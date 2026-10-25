//! Read-only outcome accounting and conservative descriptive comparisons.

use super::{
    AttemptOutcome, ComponentValue, CurrentState, EligibilityRequest, EligibilityResult, Event,
    EventKind, FailureCategory, InventoryProfile, InventorySnapshot, ObservedIdentity,
    SelectedIdentity, SpanRole, Store, StoreError, ToolOutcome, evaluate_eligibility, json_depth,
    safe_text, sensitive, valid_timestamp, validate_inventory_id,
};
use rusqlite::{OptionalExtension, params};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};

pub const MAX_ACCOUNTING_EVENTS: usize = 10_000;
pub const MAX_CONTEXT_BYTES: usize = 65_536;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "state", deny_unknown_fields)]
pub enum ContextLabel {
    Known { value: String, provenance: String },
    Unknown { reason: String, provenance: String },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TaskLabels {
    pub task_type: ContextLabel,
    pub difficulty: ContextLabel,
    pub readiness: ContextLabel,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TaskContext {
    pub schema_version: u16,
    pub context_id: String,
    pub task_id: String,
    pub session_id: String,
    pub inventory_snapshot_id: String,
    pub initial_profile_id: String,
    pub initial_profile_revision: String,
    pub policy_version: String,
    pub observed_at: String,
    pub labels: TaskLabels,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AccountingRequest {
    pub schema_version: u16,
    pub task_id: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ComparisonRequest {
    pub schema_version: u16,
    pub baseline_profile_id: String,
    pub evidence_window_start: String,
    pub evidence_window_end: String,
    pub eligibility: EligibilityRequest,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct UnitSubtotal {
    pub unit: String,
    pub known_subtotal: Option<u64>,
    pub reason: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CompleteTotal {
    pub value: Option<u64>,
    pub unit: Option<String>,
    pub reasons: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DimensionAccounting {
    pub known_subtotals: Vec<UnitSubtotal>,
    pub full_chain_total: CompleteTotal,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FailureOccurrence {
    pub source_id: String,
    pub event_id: String,
    pub attempt_id: Option<String>,
    pub kind: String,
    pub category: Option<FailureCategory>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AttemptAccounting {
    pub attempt_id: String,
    pub predecessor_attempt_id: Option<String>,
    pub span_role: SpanRole,
    pub selected: SelectedIdentity,
    pub outcomes: Vec<AttemptOutcome>,
    pub latest_outcome: Option<AttemptOutcome>,
    pub observed_identities: Vec<ObservedIdentity>,
    pub tool_call_count: usize,
    pub tool_failure_count: usize,
    pub interruption_count: usize,
    pub usage_cost_record_count: usize,
    pub usage: DimensionAccounting,
    pub cost: DimensionAccounting,
    pub elapsed: DimensionAccounting,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TaskAccounting {
    pub task_id: String,
    pub session_id: String,
    pub context: Option<TaskContext>,
    pub context_reason: Option<String>,
    pub observation_time: Option<String>,
    pub attempt_count: usize,
    pub retry_count: usize,
    pub parent_repair_count: usize,
    pub verification_count: usize,
    pub tool_call_count: usize,
    pub session_recovery_event_count: usize,
    pub accepted: bool,
    pub accepted_attempt_ids: Vec<String>,
    pub accepted_count: usize,
    pub current_status: String,
    pub unresolved_outcome_count: usize,
    pub not_run_count: usize,
    pub operational_failure_count: usize,
    pub failures: Vec<FailureOccurrence>,
    pub attempts: Vec<AttemptAccounting>,
    pub usage: DimensionAccounting,
    pub cost: DimensionAccounting,
    pub elapsed: DimensionAccounting,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AccountingReport {
    pub schema_version: u16,
    pub task_count: usize,
    pub attempted_task_count: usize,
    pub accepted_task_count: usize,
    pub unresolved_task_count: usize,
    pub not_run_count: usize,
    pub operational_failure_count: usize,
    pub tasks: Vec<TaskAccounting>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct WilsonInterval {
    pub lower: f64,
    pub upper: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CostPerAcceptedTask {
    pub total_cost: Option<u64>,
    pub accepted_task_count: Option<usize>,
    pub unit: Option<String>,
    pub reasons: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ProfileComparison {
    pub profile_id: String,
    pub sample_count: usize,
    pub accepted_task_count: usize,
    pub acceptance_proportion: Option<f64>,
    pub acceptance_interval_95: Option<WilsonInterval>,
    pub cost_per_accepted_task: CostPerAcceptedTask,
    pub operational_failure_count: usize,
    pub observation_start: Option<String>,
    pub observation_end: Option<String>,
    pub reasons: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ComparisonReport {
    pub schema_version: u16,
    pub method: String,
    pub baseline_profile_id: String,
    pub binding: String,
    pub as_of: Option<String>,
    pub evidence_window_start: String,
    pub evidence_window_end: String,
    pub eligible_profile_ids: Vec<String>,
    pub eligibility: EligibilityResult,
    pub comparisons: Vec<ProfileComparison>,
    pub observed_preferred_profile_ids: Vec<String>,
    pub outcome: String,
    pub reasons: Vec<String>,
    pub excluded_tasks: Vec<(String, String)>,
}

pub fn parse_task_context_bytes(bytes: &[u8]) -> Result<TaskContext, StoreError> {
    if bytes.is_empty()
        || bytes.len() > MAX_CONTEXT_BYTES
        || json_depth(bytes) > super::MAX_JSON_DEPTH
    {
        return Err(StoreError::InvalidInput(
            "context_size_or_depth_out_of_range",
        ));
    }
    let raw =
        std::str::from_utf8(bytes).map_err(|_| StoreError::InvalidInput("invalid_context_json"))?;
    if sensitive(raw) {
        return Err(StoreError::InvalidInput("sensitive_input_rejected"));
    }
    let context: TaskContext = serde_json::from_slice(bytes)
        .map_err(|_| StoreError::InvalidInput("invalid_context_json"))?;
    validate_context(&context)?;
    Ok(context)
}

pub fn parse_accounting_request_bytes(bytes: &[u8]) -> Result<AccountingRequest, StoreError> {
    if bytes.is_empty()
        || bytes.len() > MAX_CONTEXT_BYTES
        || json_depth(bytes) > super::MAX_JSON_DEPTH
    {
        return Err(StoreError::InvalidInput(
            "request_size_or_depth_out_of_range",
        ));
    }
    let raw =
        std::str::from_utf8(bytes).map_err(|_| StoreError::InvalidInput("invalid_request_json"))?;
    if sensitive(raw) {
        return Err(StoreError::InvalidInput("sensitive_input_rejected"));
    }
    let request: AccountingRequest = serde_json::from_slice(bytes)
        .map_err(|_| StoreError::InvalidInput("invalid_request_json"))?;
    validate_accounting_request(&request)?;
    Ok(request)
}

pub fn parse_comparison_request_bytes(bytes: &[u8]) -> Result<ComparisonRequest, StoreError> {
    if bytes.is_empty()
        || bytes.len() > super::MAX_INVENTORY_BYTES
        || json_depth(bytes) > super::MAX_JSON_DEPTH
    {
        return Err(StoreError::InvalidInput(
            "comparison_size_or_depth_out_of_range",
        ));
    }
    let raw = std::str::from_utf8(bytes)
        .map_err(|_| StoreError::InvalidInput("invalid_comparison_json"))?;
    if sensitive(raw) {
        return Err(StoreError::InvalidInput("sensitive_input_rejected"));
    }
    let request: ComparisonRequest = serde_json::from_slice(bytes)
        .map_err(|_| StoreError::InvalidInput("invalid_comparison_json"))?;
    validate_comparison_request(&request)?;
    Ok(request)
}

fn validate_context(context: &TaskContext) -> Result<(), StoreError> {
    if context.schema_version != 1 {
        return Err(StoreError::UnsupportedSchema);
    }
    for id in [
        &context.context_id,
        &context.task_id,
        &context.session_id,
        &context.inventory_snapshot_id,
        &context.initial_profile_id,
    ] {
        validate_inventory_id(id)?;
    }
    for value in [&context.initial_profile_revision, &context.policy_version] {
        safe_text(value, 256)?;
    }
    if !valid_timestamp(&context.observed_at) {
        return Err(StoreError::InvalidInput("invalid_context_timestamp"));
    }
    for label in [
        &context.labels.task_type,
        &context.labels.difficulty,
        &context.labels.readiness,
    ] {
        match label {
            ContextLabel::Known { value, provenance } => {
                safe_text(value, 256)?;
                safe_text(provenance, 128)?;
            }
            ContextLabel::Unknown { reason, provenance } => {
                safe_text(reason, 256)?;
                safe_text(provenance, 128)?;
            }
        }
    }
    let encoded =
        serde_json::to_vec(context).map_err(|_| StoreError::InvalidInput("invalid_context"))?;
    if encoded.len() > MAX_CONTEXT_BYTES {
        return Err(StoreError::InvalidInput("context_size_out_of_range"));
    }
    Ok(())
}

fn validate_accounting_request(request: &AccountingRequest) -> Result<(), StoreError> {
    if request.schema_version != 1 {
        return Err(StoreError::UnsupportedSchema);
    }
    if let Some(task_id) = &request.task_id {
        validate_inventory_id(task_id)?;
    }
    Ok(())
}

fn validate_comparison_request(request: &ComparisonRequest) -> Result<(), StoreError> {
    if request.schema_version != 1 {
        return Err(StoreError::UnsupportedSchema);
    }
    validate_inventory_id(&request.baseline_profile_id)?;
    if !valid_timestamp(&request.evidence_window_start)
        || !valid_timestamp(&request.evidence_window_end)
        || request.evidence_window_start > request.evidence_window_end
    {
        return Err(StoreError::InvalidInput("invalid_evidence_window"));
    }
    super::validate_request(&request.eligibility)?;
    Ok(())
}

impl Store {
    pub fn import_task_context(&mut self, context: TaskContext) -> Result<bool, StoreError> {
        validate_context(&context)?;
        let snapshot = self.inventory(&context.inventory_snapshot_id)?;
        if snapshot.policy_version != context.policy_version
            || snapshot.session_scope != context.session_id
        {
            return Err(StoreError::Conflict("context_snapshot_scope_mismatch"));
        }
        if !snapshot.profiles.iter().any(|p| {
            p.profile_id == context.initial_profile_id
                && p.revision == context.initial_profile_revision
        }) {
            return Err(StoreError::Conflict("context_profile_mismatch"));
        }
        let encoded = serde_json::to_string(&context)
            .map_err(|_| StoreError::InvalidInput("invalid_context"))?;
        let tx = self
            .connection
            .transaction_with_behavior(rusqlite::TransactionBehavior::Immediate)?;
        let task_session: Option<String> = tx.query_row(
            "SELECT session_id FROM events WHERE kind='TaskCreated' AND task_id=?1 ORDER BY sequence LIMIT 1",
            [&context.task_id], |row| row.get(0),
        ).optional()?;
        if task_session.as_deref() != Some(context.session_id.as_str()) {
            return Err(StoreError::NotFound("task_context_task"));
        }
        if let Some((existing, existing_id)) = tx.query_row(
            "SELECT context_json,context_id FROM task_contexts WHERE task_id=?1 OR context_id=?2 LIMIT 1",
            params![context.task_id, context.context_id], |row| Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?)),
        ).optional()? {
            if existing == encoded && existing_id == context.context_id { tx.commit()?; return Ok(false); }
            return Err(StoreError::Conflict("task_context_identity_reused"));
        }
        let attempts: i64 = tx.query_row(
            "SELECT count(*) FROM events WHERE kind='AttemptIntent' AND task_id=?1",
            [&context.task_id],
            |row| row.get(0),
        )?;
        if attempts != 0 {
            return Err(StoreError::Conflict("task_context_after_first_attempt"));
        }
        tx.execute(
            "INSERT INTO task_contexts(context_id,task_id,context_json) VALUES(?1,?2,?3)",
            params![context.context_id, context.task_id, encoded],
        )?;
        tx.commit()?;
        Ok(true)
    }

    pub fn accounting_report(
        &self,
        request: &AccountingRequest,
    ) -> Result<AccountingReport, StoreError> {
        validate_accounting_request(request)?;
        build_accounting_report(self, request.task_id.as_deref())
    }

    pub fn compare_outcomes(
        &self,
        request: &ComparisonRequest,
        current: Option<&CurrentState>,
    ) -> Result<ComparisonReport, StoreError> {
        compare_outcomes(self, request, current)
    }
}

#[derive(Default)]
struct AttemptWork {
    first_sequence: i64,
    predecessor: Option<String>,
    span_role: Option<SpanRole>,
    selected: Option<SelectedIdentity>,
    outcomes: Vec<AttemptOutcome>,
    result_sequence: i64,
    latest_outcome: Option<AttemptOutcome>,
    observed: Vec<ObservedIdentity>,
    interruption_sequence: i64,
    interruptions: usize,
    tool_calls: usize,
    tool_failures: usize,
    cost_records: Vec<(ComponentValue, ComponentValue, ComponentValue)>,
}

fn events(store: &Store) -> Result<Vec<(i64, Event)>, StoreError> {
    let mut statement = store
        .connection
        .prepare("SELECT sequence,event_json FROM events ORDER BY sequence LIMIT ?1")?;
    let rows = statement.query_map([(MAX_ACCOUNTING_EVENTS + 1) as i64], |row| {
        Ok((row.get::<_, i64>(0)?, row.get::<_, String>(1)?))
    })?;
    let records = rows.collect::<Result<Vec<_>, _>>()?;
    if records.len() > MAX_ACCOUNTING_EVENTS {
        return Err(StoreError::InvalidInput("accounting_event_limit_exceeded"));
    }
    records
        .into_iter()
        .map(|(sequence, raw)| {
            let event = serde_json::from_str(&raw)
                .map_err(|_| StoreError::InvalidInput("stored_event_invalid"))?;
            Ok((sequence, event))
        })
        .collect()
}

fn contexts(store: &Store) -> Result<BTreeMap<String, TaskContext>, StoreError> {
    let count: i64 =
        store
            .connection
            .query_row("SELECT count(*) FROM task_contexts", [], |row| row.get(0))?;
    if count as usize > MAX_ACCOUNTING_EVENTS {
        return Err(StoreError::InvalidInput(
            "accounting_context_limit_exceeded",
        ));
    }
    let mut statement = store
        .connection
        .prepare("SELECT context_json FROM task_contexts ORDER BY task_id")?;
    let rows = statement.query_map([], |row| row.get::<_, String>(0))?;
    rows.map(|row| {
        let raw = row?;
        serde_json::from_str::<TaskContext>(&raw)
            .map_err(|_| StoreError::InvalidInput("stored_context_invalid"))
            .map(|context| (context.task_id.clone(), context))
    })
    .collect()
}

fn component_for_attempt(
    records: &[(ComponentValue, ComponentValue, ComponentValue)],
    dimension: usize,
) -> DimensionAccounting {
    let reasons = if records.is_empty() {
        vec!["missing_component".to_owned()]
    } else if records.len() > 1 {
        vec!["ambiguous_usage_cost_records".to_owned()]
    } else {
        let component = match dimension {
            0 => &records[0].0,
            1 => &records[0].1,
            _ => &records[0].2,
        };
        match component {
            ComponentValue::Known { .. } => Vec::new(),
            ComponentValue::Unknown { reason } => vec![format!("unknown_component:{reason}")],
        }
    };
    let mut known_subtotals = Vec::new();
    let full_chain_total = if reasons.is_empty() {
        let component = match dimension {
            0 => &records[0].0,
            1 => &records[0].1,
            _ => &records[0].2,
        };
        if let ComponentValue::Known { value, unit } = component {
            known_subtotals.push(UnitSubtotal {
                unit: unit.clone(),
                known_subtotal: Some(*value),
                reason: None,
            });
            CompleteTotal {
                value: Some(*value),
                unit: Some(unit.clone()),
                reasons,
            }
        } else {
            unreachable!("known component checked above")
        }
    } else {
        CompleteTotal {
            value: None,
            unit: None,
            reasons,
        }
    };
    DimensionAccounting {
        known_subtotals,
        full_chain_total,
    }
}

fn combine_dimensions<'a>(
    dimensions: impl Iterator<Item = &'a DimensionAccounting>,
) -> DimensionAccounting {
    let dimensions: Vec<_> = dimensions.collect();
    let mut subtotals: BTreeMap<String, Option<u64>> = BTreeMap::new();
    let mut subtotal_reasons = BTreeMap::<String, String>::new();
    let mut complete_values = Vec::new();
    let mut reasons = BTreeSet::new();
    if dimensions.is_empty() {
        reasons.insert("no_attempts".to_owned());
    }
    for dimension in &dimensions {
        for subtotal in &dimension.known_subtotals {
            let entry = subtotals.entry(subtotal.unit.clone()).or_insert(Some(0));
            let Some(subtotal_value) = subtotal.known_subtotal else {
                *entry = None;
                subtotal_reasons.insert(subtotal.unit.clone(), "overflow".into());
                continue;
            };
            if let Some(value) = entry {
                match value.checked_add(subtotal_value) {
                    Some(total) => *value = total,
                    None => {
                        *entry = None;
                        subtotal_reasons.insert(subtotal.unit.clone(), "overflow".into());
                    }
                }
            }
        }
        let total = &dimension.full_chain_total;
        if let (Some(value), Some(unit)) = (total.value, total.unit.as_ref()) {
            complete_values.push((value, unit.clone()));
        } else {
            reasons.extend(total.reasons.iter().cloned());
        }
    }
    let known_subtotals: Vec<_> = subtotals
        .into_iter()
        .map(|(unit, value)| UnitSubtotal {
            reason: subtotal_reasons.get(&unit).cloned(),
            unit,
            known_subtotal: value,
        })
        .collect();
    let full_chain_total = if dimensions.is_empty() || !reasons.is_empty() {
        CompleteTotal {
            value: None,
            unit: None,
            reasons: reasons.into_iter().collect(),
        }
    } else {
        let units: BTreeSet<_> = complete_values
            .iter()
            .map(|(_, unit)| unit.clone())
            .collect();
        if units.len() != 1 {
            CompleteTotal {
                value: None,
                unit: None,
                reasons: vec!["mixed_units".into()],
            }
        } else {
            let unit = units.iter().next().cloned().unwrap_or_default();
            let total = complete_values
                .iter()
                .try_fold(0u64, |sum, (value, _)| sum.checked_add(*value));
            match total {
                Some(value) => CompleteTotal {
                    value: Some(value),
                    unit: Some(unit),
                    reasons: Vec::new(),
                },
                None => CompleteTotal {
                    value: None,
                    unit: None,
                    reasons: vec!["overflow".into()],
                },
            }
        }
    };
    DimensionAccounting {
        known_subtotals,
        full_chain_total,
    }
}

fn profile_for<'a>(
    snapshot: &'a InventorySnapshot,
    context: &TaskContext,
) -> Option<&'a InventoryProfile> {
    snapshot.profiles.iter().find(|profile| {
        profile.profile_id == context.initial_profile_id
            && profile.revision == context.initial_profile_revision
    })
}

fn context_reason(
    store: &Store,
    context: &TaskContext,
    work: &BTreeMap<String, AttemptWork>,
) -> Result<Option<String>, StoreError> {
    let snapshot = store.inventory(&context.inventory_snapshot_id)?;
    let Some(profile) = profile_for(&snapshot, context) else {
        return Ok(Some("context_profile_missing_from_snapshot".into()));
    };
    if snapshot.policy_version != context.policy_version {
        return Ok(Some("context_policy_mismatch".into()));
    }
    let Some(first) = work.values().min_by_key(|attempt| attempt.first_sequence) else {
        return Ok(Some("context_without_attempt".into()));
    };
    let Some(selected) = &first.selected else {
        return Ok(Some("context_initial_selection_missing".into()));
    };
    if selected.profile != context.initial_profile_id
        || selected.model != profile.configured_model_id
        || Some(selected.model_version.as_str()) != profile.resolved_model_version.as_deref()
        || selected.effort != profile.effort
    {
        return Ok(Some("context_first_selection_mismatch".into()));
    }
    for attempt in work.values() {
        for observed in &attempt.observed {
            match observed {
                ObservedIdentity::Unknown { .. } => {
                    return Ok(Some("unknown_execution_identity".into()));
                }
                ObservedIdentity::Known {
                    profile: actual_profile,
                    model,
                    model_version,
                    effort,
                } if actual_profile != &profile.profile_id
                    || Some(model.as_str()) != profile.resolved_model_id.as_deref()
                    || Some(model_version.as_str())
                        != profile.resolved_model_version.as_deref()
                    || effort != &profile.effort =>
                {
                    return Ok(Some("context_observed_identity_mismatch".into()));
                }
                _ => {}
            }
        }
    }
    Ok(None)
}

fn build_task(
    store: &Store,
    task_id: &str,
    session_id: &str,
    records: &[(i64, Event)],
    context: Option<TaskContext>,
    session_recoveries: usize,
) -> Result<TaskAccounting, StoreError> {
    let mut work = BTreeMap::<String, AttemptWork>::new();
    let mut failures = Vec::new();
    let mut accepted_events = Vec::<(i64, String)>::new();
    let mut latest_work_event = 0;
    let mut last_acceptance = 0;
    let mut verification_count = 0;
    let mut task_events = Vec::new();
    for (sequence, event) in records {
        let (_, _, event_task, event_attempt) = event.kind.columns();
        if event_task != Some(task_id) {
            continue;
        }
        task_events.push((*sequence, event.event_id.clone()));
        match &event.kind {
            EventKind::AttemptIntent {
                attempt_id,
                predecessor_attempt_id,
                span_role,
                selected,
                ..
            } => {
                latest_work_event = latest_work_event.max(*sequence);
                let attempt = work.entry(attempt_id.clone()).or_default();
                if attempt.first_sequence == 0 || *sequence < attempt.first_sequence {
                    attempt.first_sequence = *sequence;
                    attempt.predecessor = predecessor_attempt_id.clone();
                    attempt.span_role = Some(*span_role);
                    attempt.selected = Some(selected.clone());
                }
            }
            EventKind::AttemptResult {
                attempt_id,
                outcome,
                observed,
                ..
            } => {
                latest_work_event = latest_work_event.max(*sequence);
                let attempt = work.entry(attempt_id.clone()).or_default();
                attempt.outcomes.push(*outcome);
                attempt.observed.push(observed.clone());
                if *sequence >= attempt.result_sequence {
                    attempt.result_sequence = *sequence;
                    attempt.latest_outcome = Some(*outcome);
                }
                if *outcome == AttemptOutcome::Failure {
                    failures.push(FailureOccurrence {
                        source_id: event.source_id.clone(),
                        event_id: event.event_id.clone(),
                        attempt_id: event_attempt.map(str::to_owned),
                        kind: "attempt_failure".into(),
                        category: None,
                    });
                }
            }
            EventKind::ToolResult {
                attempt_id,
                result,
                failure_category,
                ..
            } => {
                latest_work_event = latest_work_event.max(*sequence);
                let attempt = work.entry(attempt_id.clone()).or_default();
                attempt.tool_calls += 1;
                if *result == ToolOutcome::Failure {
                    attempt.tool_failures += 1;
                    failures.push(FailureOccurrence {
                        source_id: event.source_id.clone(),
                        event_id: event.event_id.clone(),
                        attempt_id: event_attempt.map(str::to_owned),
                        kind: "tool_failure".into(),
                        category: *failure_category,
                    });
                }
            }
            EventKind::UsageCost {
                attempt_id,
                usage,
                cost,
                elapsed_ms,
                ..
            } => {
                work.entry(attempt_id.clone())
                    .or_default()
                    .cost_records
                    .push((usage.clone(), cost.clone(), elapsed_ms.clone()));
            }
            EventKind::Verification { .. } => verification_count += 1,
            EventKind::Interrupted { attempt_id, .. } => {
                latest_work_event = latest_work_event.max(*sequence);
                let attempt = work.entry(attempt_id.clone()).or_default();
                attempt.interruptions += 1;
                attempt.interruption_sequence = attempt.interruption_sequence.max(*sequence);
                failures.push(FailureOccurrence {
                    source_id: event.source_id.clone(),
                    event_id: event.event_id.clone(),
                    attempt_id: event_attempt.map(str::to_owned),
                    kind: "interruption".into(),
                    category: None,
                });
            }
            EventKind::TaskAccepted { attempt_id, .. } => {
                accepted_events.push((*sequence, attempt_id.clone()));
                last_acceptance = last_acceptance.max(*sequence);
            }
            _ => {}
        }
    }
    let mut attempts = Vec::new();
    let mut unresolved = 0;
    let mut not_run = 0;
    for (attempt_id, attempt) in &work {
        let Some(selected) = attempt.selected.clone() else {
            continue;
        };
        if attempt.outcomes.contains(&AttemptOutcome::NotRun) {
            not_run += 1;
        }
        if attempt.latest_outcome.is_none()
            || attempt.latest_outcome == Some(AttemptOutcome::Unknown)
            || attempt.interruption_sequence > attempt.result_sequence
        {
            unresolved += 1;
        }
        let usage = component_for_attempt(&attempt.cost_records, 0);
        let cost = component_for_attempt(&attempt.cost_records, 1);
        let elapsed = component_for_attempt(&attempt.cost_records, 2);
        attempts.push(AttemptAccounting {
            attempt_id: attempt_id.clone(),
            predecessor_attempt_id: attempt.predecessor.clone(),
            span_role: attempt.span_role.unwrap_or(SpanRole::Worker),
            selected,
            outcomes: attempt.outcomes.clone(),
            latest_outcome: attempt.latest_outcome,
            observed_identities: attempt.observed.clone(),
            tool_call_count: attempt.tool_calls,
            tool_failure_count: attempt.tool_failures,
            interruption_count: attempt.interruptions,
            usage_cost_record_count: attempt.cost_records.len(),
            usage,
            cost,
            elapsed,
        });
    }
    attempts.sort_by(|a, b| a.attempt_id.cmp(&b.attempt_id));
    failures.sort_by(|a, b| (&a.source_id, &a.event_id).cmp(&(&b.source_id, &b.event_id)));
    let usage = combine_dimensions(attempts.iter().map(|attempt| &attempt.usage));
    let cost = combine_dimensions(attempts.iter().map(|attempt| &attempt.cost));
    let elapsed = combine_dimensions(attempts.iter().map(|attempt| &attempt.elapsed));
    let accepted_attempt_ids: Vec<_> = accepted_events
        .iter()
        .map(|(_, attempt)| attempt.clone())
        .collect();
    let accepted = !accepted_events.is_empty();
    let accepted_count = usize::from(accepted);
    let later_work = last_acceptance > 0 && latest_work_event > last_acceptance;
    let current_status = if unresolved > 0 || later_work {
        "unresolved_or_inconsistent"
    } else if accepted {
        "accepted"
    } else if attempts.is_empty() {
        "not_attempted"
    } else {
        "not_accepted"
    };
    let context_reason = if let Some(context) = &context {
        context_reason(store, context, &work)?
    } else {
        Some("unclassified_legacy_task".into())
    };
    let observation_time = context.as_ref().map(|context| context.observed_at.clone());
    Ok(TaskAccounting {
        task_id: task_id.into(),
        session_id: session_id.into(),
        context,
        context_reason,
        observation_time,
        attempt_count: attempts.len(),
        retry_count: attempts
            .iter()
            .filter(|attempt| attempt.predecessor_attempt_id.is_some())
            .count(),
        parent_repair_count: attempts
            .iter()
            .filter(|attempt| attempt.span_role == SpanRole::ParentRepair)
            .count(),
        verification_count,
        tool_call_count: attempts.iter().map(|attempt| attempt.tool_call_count).sum(),
        session_recovery_event_count: session_recoveries,
        accepted,
        accepted_attempt_ids,
        accepted_count,
        current_status: current_status.into(),
        unresolved_outcome_count: unresolved,
        not_run_count: not_run,
        operational_failure_count: failures.len(),
        failures,
        attempts,
        usage,
        cost,
        elapsed,
    })
}

fn build_accounting_report(
    store: &Store,
    task_filter: Option<&str>,
) -> Result<AccountingReport, StoreError> {
    let records = events(store)?;
    let contexts = contexts(store)?;
    let mut grouped = BTreeMap::<(String, String), Vec<(i64, Event)>>::new();
    let mut recovery_counts = BTreeMap::<String, usize>::new();
    for (sequence, event) in &records {
        if let EventKind::SessionRecovered { session_id, .. } = &event.kind {
            *recovery_counts.entry(session_id.clone()).or_default() += 1;
        }
        let (_, session_id, task_id, _) = event.kind.columns();
        if let (Some(session_id), Some(task_id)) = (session_id, task_id)
            && task_filter.is_none_or(|filter| filter == task_id)
        {
            grouped
                .entry((session_id.to_owned(), task_id.to_owned()))
                .or_default()
                .push((*sequence, event.clone()));
        }
    }
    let tasks: Vec<_> = grouped
        .into_iter()
        .map(|((session_id, task_id), task_events)| {
            build_task(
                store,
                &task_id,
                &session_id,
                &task_events,
                contexts.get(&task_id).cloned(),
                *recovery_counts.get(&session_id).unwrap_or(&0),
            )
        })
        .collect::<Result<_, _>>()?;
    let attempted_task_count = tasks.iter().filter(|task| task.attempt_count > 0).count();
    Ok(AccountingReport {
        schema_version: 1,
        task_count: tasks.len(),
        attempted_task_count,
        accepted_task_count: tasks.iter().filter(|task| task.accepted).count(),
        unresolved_task_count: tasks
            .iter()
            .filter(|task| {
                task.unresolved_outcome_count > 0
                    || task.current_status == "unresolved_or_inconsistent"
            })
            .count(),
        not_run_count: tasks.iter().map(|task| task.not_run_count).sum(),
        operational_failure_count: tasks
            .iter()
            .map(|task| task.operational_failure_count)
            .sum(),
        tasks,
    })
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct CohortKey {
    labels: String,
    policy_version: String,
    harness: String,
    harness_version: String,
    adapter: String,
    adapter_version: String,
    configuration_fingerprint: String,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct ProfileKey {
    profile_id: String,
    revision: String,
    model_provider: String,
    configured_model_id: String,
    resolved_model_id: Option<String>,
    resolved_model_version: Option<String>,
    effort: String,
}

fn profile_key(profile: &InventoryProfile) -> ProfileKey {
    ProfileKey {
        profile_id: profile.profile_id.clone(),
        revision: profile.revision.clone(),
        model_provider: profile.model_provider.clone(),
        configured_model_id: profile.configured_model_id.clone(),
        resolved_model_id: profile.resolved_model_id.clone(),
        resolved_model_version: profile.resolved_model_version.clone(),
        effort: profile.effort.clone(),
    }
}

fn known_labels(labels: &TaskLabels) -> bool {
    matches!(labels.task_type, ContextLabel::Known { .. })
        && matches!(labels.difficulty, ContextLabel::Known { .. })
        && matches!(labels.readiness, ContextLabel::Known { .. })
}

fn cohort_key(store: &Store, context: &TaskContext) -> Result<CohortKey, StoreError> {
    let snapshot = store.inventory(&context.inventory_snapshot_id)?;
    let labels = serde_json::to_string(&context.labels)
        .map_err(|_| StoreError::InvalidInput("invalid_context"))?;
    Ok(CohortKey {
        labels,
        policy_version: context.policy_version.clone(),
        harness: snapshot.harness,
        harness_version: snapshot.harness_version,
        adapter: snapshot.adapter,
        adapter_version: snapshot.adapter_version,
        configuration_fingerprint: snapshot.configuration_fingerprint,
    })
}

fn wilson(successes: usize, samples: usize) -> Option<WilsonInterval> {
    if samples == 0 {
        return None;
    }
    let z = 1.959_963_984_540_054_f64;
    let n = samples as f64;
    let p = successes as f64 / n;
    let z2 = z * z;
    let denominator = 1.0 + z2 / n;
    let center = (p + z2 / (2.0 * n)) / denominator;
    let margin = z * ((p * (1.0 - p) / n + z2 / (4.0 * n * n)).sqrt()) / denominator;
    Some(WilsonInterval {
        lower: (center - margin).max(0.0),
        upper: (center + margin).min(1.0),
    })
}

fn comparison_metrics(profile_id: &str, tasks: &[&TaskAccounting]) -> ProfileComparison {
    let accepted_task_count = tasks.iter().filter(|task| task.accepted).count();
    let mut reasons = BTreeSet::new();
    if tasks.is_empty() {
        reasons.insert("no_matched_cohort".to_owned());
    }
    if tasks
        .iter()
        .any(|task| task.current_status == "unresolved_or_inconsistent")
    {
        reasons.insert("incomplete_outcome".to_owned());
    }
    if tasks.iter().any(|task| {
        task.usage.full_chain_total.value.is_none()
            || task.cost.full_chain_total.value.is_none()
            || task.elapsed.full_chain_total.value.is_none()
    }) {
        reasons.insert("incomplete_accounting".to_owned());
    }
    let acceptance_proportion = if tasks.is_empty() {
        None
    } else {
        Some(accepted_task_count as f64 / tasks.len() as f64)
    };
    let acceptance_interval_95 = wilson(accepted_task_count, tasks.len());
    let cost_per_accepted_task = if accepted_task_count == 0 {
        reasons.insert("no_accepted_tasks".to_owned());
        CostPerAcceptedTask {
            total_cost: None,
            accepted_task_count: None,
            unit: None,
            reasons: vec!["no_accepted_tasks".into()],
        }
    } else if tasks
        .iter()
        .any(|task| task.cost.full_chain_total.value.is_none())
    {
        let mut component_reasons: BTreeSet<_> = tasks
            .iter()
            .flat_map(|task| task.cost.full_chain_total.reasons.iter().cloned())
            .collect();
        if component_reasons.is_empty() {
            component_reasons.insert("incomplete_accounting".into());
        }
        CostPerAcceptedTask {
            total_cost: None,
            accepted_task_count: None,
            unit: None,
            reasons: component_reasons.into_iter().collect(),
        }
    } else {
        let units: BTreeSet<_> = tasks
            .iter()
            .filter_map(|task| task.cost.full_chain_total.unit.clone())
            .collect();
        if units.len() != 1 {
            reasons.insert("mixed_cost_units".to_owned());
            CostPerAcceptedTask {
                total_cost: None,
                accepted_task_count: None,
                unit: None,
                reasons: vec!["mixed_cost_units".into()],
            }
        } else {
            let unit = units.iter().next().cloned().unwrap_or_default();
            let total = tasks.iter().try_fold(0u64, |sum, task| {
                sum.checked_add(task.cost.full_chain_total.value?)
            });
            match total {
                None => {
                    reasons.insert("cost_overflow".to_owned());
                    CostPerAcceptedTask {
                        total_cost: None,
                        accepted_task_count: None,
                        unit: None,
                        reasons: vec!["overflow".into()],
                    }
                }
                Some(total) => CostPerAcceptedTask {
                    total_cost: Some(total),
                    accepted_task_count: Some(accepted_task_count),
                    unit: Some(unit),
                    reasons: Vec::new(),
                },
            }
        }
    };
    let mut observations: Vec<_> = tasks
        .iter()
        .filter_map(|task| task.observation_time.clone())
        .collect();
    observations.sort();
    ProfileComparison {
        profile_id: profile_id.into(),
        sample_count: tasks.len(),
        accepted_task_count,
        acceptance_proportion,
        acceptance_interval_95,
        cost_per_accepted_task,
        operational_failure_count: tasks
            .iter()
            .map(|task| task.operational_failure_count)
            .sum(),
        observation_start: observations.first().cloned(),
        observation_end: observations.last().cloned(),
        reasons: reasons.into_iter().collect(),
    }
}

fn compare_outcomes(
    store: &Store,
    request: &ComparisonRequest,
    current: Option<&CurrentState>,
) -> Result<ComparisonReport, StoreError> {
    validate_comparison_request(request)?;
    let snapshot = store.inventory(&request.eligibility.snapshot_id)?;
    let eligibility = evaluate_eligibility(&snapshot, &request.eligibility, current)?;
    let eligible_profile_ids = eligibility.eligible_profile_ids.clone();
    let binding = if current.is_some() {
        "available"
    } else {
        "binding_unavailable"
    }
    .to_owned();
    let mut reasons = BTreeSet::new();
    let mut excluded_tasks = Vec::new();
    let mut blocked_profiles = BTreeMap::<String, BTreeSet<String>>::new();
    if eligible_profile_ids.is_empty() {
        reasons.insert(
            if current.is_some() {
                "no_eligible_candidate"
            } else {
                "binding_unavailable"
            }
            .into(),
        );
        return Ok(ComparisonReport {
            schema_version: 1,
            method: "descriptive-v1".into(),
            baseline_profile_id: request.baseline_profile_id.clone(),
            binding,
            as_of: current.map(|state| state.now.clone()),
            evidence_window_start: request.evidence_window_start.clone(),
            evidence_window_end: request.evidence_window_end.clone(),
            eligible_profile_ids,
            eligibility,
            comparisons: Vec::new(),
            observed_preferred_profile_ids: Vec::new(),
            outcome: "no_eligible_candidate".into(),
            reasons: reasons.into_iter().collect(),
            excluded_tasks,
        });
    }
    if !eligible_profile_ids.contains(&request.baseline_profile_id) {
        reasons.insert("baseline_unavailable_or_ineligible".into());
    }
    let report = build_accounting_report(store, None)?;
    let eligible_set: BTreeSet<_> = eligible_profile_ids.iter().map(String::as_str).collect();
    let mut usable = Vec::<(&TaskAccounting, CohortKey, ProfileKey)>::new();
    for task in &report.tasks {
        let Some(context) = &task.context else {
            excluded_tasks.push((task.task_id.clone(), "unclassified_legacy_task".into()));
            continue;
        };
        if let Some(reason) = &task.context_reason {
            excluded_tasks.push((task.task_id.clone(), reason.clone()));
            if eligible_set.contains(context.initial_profile_id.as_str()) {
                blocked_profiles
                    .entry(context.initial_profile_id.clone())
                    .or_default()
                    .insert(reason.clone());
            }
            continue;
        }
        if !known_labels(&context.labels) {
            excluded_tasks.push((task.task_id.clone(), "unknown_task_labels".into()));
            if eligible_set.contains(context.initial_profile_id.as_str()) {
                blocked_profiles
                    .entry(context.initial_profile_id.clone())
                    .or_default()
                    .insert("unknown_task_labels".into());
            }
            continue;
        }
        if context.observed_at < request.evidence_window_start
            || context.observed_at > request.evidence_window_end
        {
            excluded_tasks.push((task.task_id.clone(), "outside_evidence_window".into()));
            continue;
        }
        if task.current_status == "unresolved_or_inconsistent" {
            excluded_tasks.push((
                task.task_id.clone(),
                "unresolved_or_inconsistent_outcome".into(),
            ));
            if eligible_set.contains(context.initial_profile_id.as_str()) {
                blocked_profiles
                    .entry(context.initial_profile_id.clone())
                    .or_default()
                    .insert("unresolved_or_inconsistent_outcome".into());
            }
            continue;
        }
        if task.attempt_count == 0 {
            excluded_tasks.push((task.task_id.clone(), "task_not_attempted".into()));
            continue;
        }
        if !eligible_set.contains(context.initial_profile_id.as_str()) {
            excluded_tasks.push((task.task_id.clone(), "profile_not_eligible".into()));
            continue;
        }
        let task_snapshot = store.inventory(&context.inventory_snapshot_id)?;
        let Some(profile) = profile_for(&task_snapshot, context) else {
            excluded_tasks.push((
                task.task_id.clone(),
                "context_profile_missing_from_snapshot".into(),
            ));
            continue;
        };
        let task_dimension = profile_key(profile);
        let current_dimension = snapshot
            .profiles
            .iter()
            .find(|candidate| candidate.profile_id == context.initial_profile_id)
            .map(profile_key);
        if current_dimension.as_ref() != Some(&task_dimension) {
            excluded_tasks.push((task.task_id.clone(), "profile_dimension_mismatch".into()));
            blocked_profiles
                .entry(context.initial_profile_id.clone())
                .or_default()
                .insert("profile_dimension_mismatch".into());
            continue;
        }
        usable.push((task, cohort_key(store, context)?, task_dimension));
    }
    excluded_tasks.sort();
    let baseline_profile = snapshot
        .profiles
        .iter()
        .find(|profile| profile.profile_id == request.baseline_profile_id);
    let baseline_dimension = baseline_profile.map(profile_key);
    let baseline_keys: BTreeSet<_> = usable
        .iter()
        .filter(|(task, _, dimension)| {
            task.context
                .as_ref()
                .is_some_and(|context| context.initial_profile_id == request.baseline_profile_id)
                && Some(dimension) == baseline_dimension.as_ref()
        })
        .map(|(_, key, _)| key.clone())
        .collect();
    let mut comparisons = Vec::new();
    for profile_id in &eligible_profile_ids {
        let dimension = snapshot
            .profiles
            .iter()
            .find(|profile| profile.profile_id == *profile_id)
            .map(profile_key);
        let candidate_keys: BTreeSet<_> = usable
            .iter()
            .filter(|(task, _, task_dimension)| {
                task.context
                    .as_ref()
                    .is_some_and(|context| context.initial_profile_id == *profile_id)
                    && Some(task_dimension) == dimension.as_ref()
            })
            .map(|(_, key, _)| key.clone())
            .collect();
        let matched_keys: BTreeSet<_> = if profile_id == &request.baseline_profile_id {
            baseline_keys.clone()
        } else {
            baseline_keys
                .intersection(&candidate_keys)
                .cloned()
                .collect()
        };
        let matched: Vec<_> = usable
            .iter()
            .filter(|(task, key, task_dimension)| {
                matched_keys.contains(key)
                    && task
                        .context
                        .as_ref()
                        .is_some_and(|context| context.initial_profile_id == *profile_id)
                    && Some(task_dimension) == dimension.as_ref()
            })
            .map(|(task, _, _)| *task)
            .collect();
        let mut comparison = comparison_metrics(profile_id, &matched);
        if let Some(blockers) = blocked_profiles.get(profile_id) {
            comparison.reasons.extend(blockers.iter().cloned());
            comparison.reasons.sort();
            comparison.reasons.dedup();
        }
        if profile_id == &request.baseline_profile_id {
            comparison.reasons.push("fixed_baseline".into());
        }
        comparisons.push(comparison);
    }
    let baseline = comparisons
        .iter()
        .find(|item| item.profile_id == request.baseline_profile_id);
    let mut preferred = Vec::new();
    if let Some(baseline) = baseline {
        for candidate in &comparisons {
            if candidate.profile_id == request.baseline_profile_id {
                continue;
            }
            if !candidate.reasons.is_empty()
                || !baseline.reasons.iter().all(|r| r == "fixed_baseline")
            {
                reasons.extend(
                    candidate
                        .reasons
                        .iter()
                        .filter(|r| r.as_str() != "fixed_baseline")
                        .cloned(),
                );
                continue;
            }
            let (Some(candidate_interval), Some(baseline_interval)) = (
                &candidate.acceptance_interval_95,
                &baseline.acceptance_interval_95,
            ) else {
                reasons.insert("insufficient_samples".into());
                continue;
            };
            let cost_ok = candidate
                .cost_per_accepted_task
                .total_cost
                .zip(candidate.cost_per_accepted_task.accepted_task_count)
                .zip(candidate.cost_per_accepted_task.unit.as_ref())
                .zip(
                    baseline
                        .cost_per_accepted_task
                        .total_cost
                        .zip(baseline.cost_per_accepted_task.accepted_task_count)
                        .zip(baseline.cost_per_accepted_task.unit.as_ref()),
                )
                .is_some_and(
                    |(
                        ((candidate_cost, candidate_count), candidate_unit),
                        ((baseline_cost, baseline_count), baseline_unit),
                    )| {
                        candidate_unit == baseline_unit
                            && u128::from(candidate_cost) * baseline_count as u128
                                <= u128::from(baseline_cost) * candidate_count as u128
                    },
                );
            if candidate_interval.lower > baseline_interval.upper && cost_ok {
                preferred.push(candidate.profile_id.clone());
            } else {
                reasons.insert("interval_overlap_or_cost_not_lower".into());
            }
        }
    }
    preferred.sort_by(|left, right| {
        let left_item = comparisons.iter().find(|item| &item.profile_id == left);
        let right_item = comparisons.iter().find(|item| &item.profile_id == right);
        let left_cost = left_item.and_then(|item| {
            item.cost_per_accepted_task
                .total_cost
                .zip(item.cost_per_accepted_task.accepted_task_count)
        });
        let right_cost = right_item.and_then(|item| {
            item.cost_per_accepted_task
                .total_cost
                .zip(item.cost_per_accepted_task.accepted_task_count)
        });
        match (left_cost, right_cost) {
            (Some((ln, ld)), Some((rn, rd))) => {
                (u128::from(ln) * rd as u128).cmp(&(u128::from(rn) * ld as u128))
            }
            _ => std::cmp::Ordering::Equal,
        }
        .then_with(|| left.cmp(right))
    });
    if !preferred.is_empty() {
        reasons.clear();
    }
    if comparisons
        .iter()
        .any(|comparison| comparison.sample_count == 0)
    {
        reasons.insert("insufficient_matched_evidence".into());
    }
    if reasons.is_empty() && preferred.is_empty() {
        reasons.insert("insufficient_evidence".into());
    }
    Ok(ComparisonReport {
        schema_version: 1,
        method: "descriptive-v1".into(),
        baseline_profile_id: request.baseline_profile_id.clone(),
        binding,
        as_of: current.map(|state| state.now.clone()),
        evidence_window_start: request.evidence_window_start.clone(),
        evidence_window_end: request.evidence_window_end.clone(),
        eligible_profile_ids,
        eligibility,
        comparisons,
        observed_preferred_profile_ids: preferred.clone(),
        outcome: if preferred.is_empty() {
            "insufficient_evidence"
        } else {
            "observed_preference"
        }
        .into(),
        reasons: reasons.into_iter().collect(),
        excluded_tasks,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        AttemptOutcome, ComponentValue, EventRef, ObservedIdentity, SelectedIdentity, ToolOutcome,
    };

    fn inventory_profile(id: &str) -> InventoryProfile {
        InventoryProfile {
            profile_id: id.into(),
            revision: format!("{id}-revision-1"),
            native_role: "worker".into(),
            model_provider: "provider".into(),
            configured_model_id: format!("{id}-alias"),
            resolved_model_id: Some(format!("{id}-model")),
            resolved_model_version: Some(format!("{id}-version")),
            effort: "high".into(),
            supported_efforts: vec!["high".into()],
            capabilities: Vec::new(),
        }
    }

    fn snapshot() -> InventorySnapshot {
        InventorySnapshot {
            schema_version: 1,
            snapshot_id: "snapshot-a".into(),
            revision: "inventory-revision-1".into(),
            policy_version: "policy-a".into(),
            created_at: "2026-10-25T09:00:00Z".into(),
            harness: "codex".into(),
            harness_version: "1".into(),
            adapter: "native".into(),
            adapter_version: "1".into(),
            runtime_scope: "runtime-a".into(),
            session_scope: "session-a".into(),
            configuration_fingerprint: "config-a".into(),
            workspace_fingerprint: "workspace-a".into(),
            profiles: vec![
                inventory_profile("baseline"),
                inventory_profile("alternative"),
            ],
        }
    }

    fn labels() -> TaskLabels {
        TaskLabels {
            task_type: ContextLabel::Known {
                value: "coding".into(),
                provenance: "planner-v1".into(),
            },
            difficulty: ContextLabel::Known {
                value: "medium".into(),
                provenance: "planner-v1".into(),
            },
            readiness: ContextLabel::Known {
                value: "ready".into(),
                provenance: "planner-v1".into(),
            },
        }
    }

    fn record(store: &mut Store, event_id: &str, parents: &[&str], kind: EventKind) {
        let mut event = Event::new("owner-a", event_id, kind);
        event.parents = parents
            .iter()
            .map(|event_id| EventRef {
                source_id: "owner-a".into(),
                event_id: (*event_id).into(),
            })
            .collect();
        store.append(event).unwrap();
    }

    fn base_store() -> Store {
        let mut store = Store::memory().unwrap();
        record(
            &mut store,
            "session-open",
            &[],
            EventKind::SessionOpened {
                session_id: "session-a".into(),
                owner_id: "owner-a".into(),
            },
        );
        store.import_inventory(snapshot()).unwrap();
        store
    }

    fn create_task(
        store: &mut Store,
        task_id: &str,
        profile_id: &str,
        observed_at: &str,
        task_labels: TaskLabels,
    ) -> TaskContext {
        let task_event = format!("task-create-{task_id}");
        record(
            store,
            &task_event,
            &["session-open"],
            EventKind::TaskCreated {
                session_id: "session-a".into(),
                task_id: task_id.into(),
                required_check_ids: Vec::new(),
            },
        );
        let inventory = store.inventory("snapshot-a").unwrap();
        let profile = inventory
            .profiles
            .iter()
            .find(|profile| profile.profile_id == profile_id)
            .unwrap();
        let context = TaskContext {
            schema_version: 1,
            context_id: format!("context-{task_id}"),
            task_id: task_id.into(),
            session_id: "session-a".into(),
            inventory_snapshot_id: "snapshot-a".into(),
            initial_profile_id: profile_id.into(),
            initial_profile_revision: profile.revision.clone(),
            policy_version: inventory.policy_version,
            observed_at: observed_at.into(),
            labels: task_labels,
        };
        store.import_task_context(context.clone()).unwrap();
        context
    }

    fn create_task_on_snapshot(
        store: &mut Store,
        task_id: &str,
        profile_id: &str,
        observed_at: &str,
        task_labels: TaskLabels,
        snapshot_id: &str,
    ) -> TaskContext {
        let task_event = format!("task-create-{task_id}");
        record(
            store,
            &task_event,
            &["session-open"],
            EventKind::TaskCreated {
                session_id: "session-a".into(),
                task_id: task_id.into(),
                required_check_ids: Vec::new(),
            },
        );
        let inventory = store.inventory(snapshot_id).unwrap();
        let profile = inventory
            .profiles
            .iter()
            .find(|profile| profile.profile_id == profile_id)
            .unwrap();
        let context = TaskContext {
            schema_version: 1,
            context_id: format!("context-{task_id}"),
            task_id: task_id.into(),
            session_id: "session-a".into(),
            inventory_snapshot_id: snapshot_id.into(),
            initial_profile_id: profile_id.into(),
            initial_profile_revision: profile.revision.clone(),
            policy_version: inventory.policy_version,
            observed_at: observed_at.into(),
            labels: task_labels,
        };
        store.import_task_context(context.clone()).unwrap();
        context
    }

    #[allow(clippy::too_many_arguments)]
    fn add_attempt(
        store: &mut Store,
        task_id: &str,
        attempt_id: &str,
        profile_id: &str,
        predecessor: Option<&str>,
        role: SpanRole,
        outcome: AttemptOutcome,
        cost: Option<(u64, &str)>,
        extra_cost_records: usize,
        tool_failure: bool,
        accepted: bool,
    ) {
        let inventory = store.inventory("snapshot-a").unwrap();
        let profile = inventory
            .profiles
            .iter()
            .find(|profile| profile.profile_id == profile_id)
            .unwrap();
        let intent_id = format!("intent-{attempt_id}");
        record(
            store,
            &intent_id,
            &[&format!("task-create-{task_id}")],
            EventKind::AttemptIntent {
                session_id: "session-a".into(),
                task_id: task_id.into(),
                attempt_id: attempt_id.into(),
                predecessor_attempt_id: predecessor.map(str::to_owned),
                span_id: format!("span-{attempt_id}"),
                span_role: role,
                selected: SelectedIdentity {
                    profile: profile.profile_id.clone(),
                    model: profile.configured_model_id.clone(),
                    model_version: profile.resolved_model_version.clone().unwrap(),
                    effort: profile.effort.clone(),
                },
            },
        );
        if tool_failure {
            record(
                store,
                &format!("tool-failed-{attempt_id}"),
                &[&intent_id],
                EventKind::ToolResult {
                    session_id: "session-a".into(),
                    task_id: task_id.into(),
                    attempt_id: attempt_id.into(),
                    tool_id: "build".into(),
                    operation_id: "compile".into(),
                    result: ToolOutcome::Failure,
                    failure_category: Some(FailureCategory::Infrastructure),
                    evidence_ref: Some("logs/build".into()),
                },
            );
        }
        let result_id = format!("result-{attempt_id}");
        let result_parent = if tool_failure {
            format!("tool-failed-{attempt_id}")
        } else {
            intent_id.clone()
        };
        record(
            store,
            &result_id,
            &[&result_parent],
            EventKind::AttemptResult {
                session_id: "session-a".into(),
                task_id: task_id.into(),
                attempt_id: attempt_id.into(),
                outcome,
                observed: ObservedIdentity::Known {
                    profile: profile.profile_id.clone(),
                    model: profile.resolved_model_id.clone().unwrap(),
                    model_version: profile.resolved_model_version.clone().unwrap(),
                    effort: profile.effort.clone(),
                },
                evidence_ref: Some(format!("receipts/{attempt_id}")),
            },
        );
        if let Some((value, unit)) = cost {
            for index in 0..=extra_cost_records {
                let cost_event = if extra_cost_records == 0 {
                    format!("usage-{attempt_id}")
                } else {
                    format!("usage-{attempt_id}-{index}")
                };
                record(
                    store,
                    &cost_event,
                    &[&result_id],
                    EventKind::UsageCost {
                        session_id: "session-a".into(),
                        task_id: task_id.into(),
                        attempt_id: attempt_id.into(),
                        usage: ComponentValue::Known {
                            value: 100,
                            unit: "tokens".into(),
                        },
                        cost: ComponentValue::Known {
                            value,
                            unit: unit.into(),
                        },
                        elapsed_ms: ComponentValue::Known {
                            value: 25,
                            unit: "ms".into(),
                        },
                    },
                );
            }
        }
        if accepted {
            record(
                store,
                &format!("accepted-{attempt_id}"),
                &[&result_id],
                EventKind::TaskAccepted {
                    session_id: "session-a".into(),
                    task_id: task_id.into(),
                    attempt_id: attempt_id.into(),
                    actor_id: "reviewer".into(),
                    evidence_ref: format!("reviews/{task_id}"),
                },
            );
        }
    }

    fn compare_request(selected: Option<&str>) -> ComparisonRequest {
        ComparisonRequest {
            schema_version: 1,
            baseline_profile_id: "baseline".into(),
            evidence_window_start: "2026-10-24T00:00:00Z".into(),
            evidence_window_end: "2026-10-26T00:00:00Z".into(),
            eligibility: EligibilityRequest {
                schema_version: 1,
                request_id: "compare-a".into(),
                task_id: "compare-task".into(),
                session_id: "session-a".into(),
                snapshot_id: "snapshot-a".into(),
                policy_version: "policy-a".into(),
                candidate_profile_ids: vec!["baseline".into(), "alternative".into()],
                execution_path: "native".into(),
                selected_profile_id: selected.map(str::to_owned),
                required_capabilities: Vec::new(),
                workload_labels: Vec::new(),
            },
        }
    }

    fn current_state() -> CurrentState {
        CurrentState {
            now: "2026-10-25T12:00:00Z".into(),
            execution_path: "native".into(),
            expected_snapshot_id: "snapshot-a".into(),
            policy_version: "policy-a".into(),
            harness: "codex".into(),
            harness_version: "1".into(),
            adapter: "native".into(),
            adapter_version: "1".into(),
            runtime_scope: "runtime-a".into(),
            session_scope: "session-a".into(),
            configuration_fingerprint: "config-a".into(),
            workspace_fingerprint: "workspace-a".into(),
            workspace_dirty: false,
        }
    }

    #[test]
    fn context_replay_conflict_late_rejection_and_legacy_age_are_explicit() {
        let mut store = base_store();
        let context = create_task(
            &mut store,
            "context-task",
            "baseline",
            "2026-10-25T10:00:00Z",
            labels(),
        );
        assert!(!store.import_task_context(context.clone()).unwrap());
        let mut changed = context.clone();
        changed.labels.task_type = ContextLabel::Known {
            value: "other".into(),
            provenance: "planner-v1".into(),
        };
        assert!(matches!(
            store.import_task_context(changed),
            Err(StoreError::Conflict("task_context_identity_reused"))
        ));
        add_attempt(
            &mut store,
            "context-task",
            "attempt-context",
            "baseline",
            None,
            SpanRole::Worker,
            AttemptOutcome::Success,
            None,
            0,
            false,
            false,
        );
        assert!(!store.import_task_context(context.clone()).unwrap());

        record(
            &mut store,
            "task-create-late-task",
            &["session-open"],
            EventKind::TaskCreated {
                session_id: "session-a".into(),
                task_id: "late-task".into(),
                required_check_ids: Vec::new(),
            },
        );
        add_attempt(
            &mut store,
            "late-task",
            "attempt-late",
            "baseline",
            None,
            SpanRole::Worker,
            AttemptOutcome::NotRun,
            None,
            0,
            false,
            false,
        );
        let late_context = TaskContext {
            task_id: "late-task".into(),
            context_id: "context-late".into(),
            ..context
        };
        assert!(matches!(
            store.import_task_context(late_context),
            Err(StoreError::Conflict("task_context_after_first_attempt"))
        ));
        let report = store
            .accounting_report(&AccountingRequest {
                schema_version: 1,
                task_id: None,
            })
            .unwrap();
        let late = report
            .tasks
            .iter()
            .find(|task| task.task_id == "late-task")
            .unwrap();
        assert_eq!(
            late.context_reason.as_deref(),
            Some("unclassified_legacy_task")
        );
        assert_eq!(late.observation_time, None);
        assert_eq!(late.not_run_count, 1);
        assert!(
            late.cost
                .full_chain_total
                .reasons
                .contains(&"missing_component".into())
        );
    }

    #[test]
    fn accounting_counts_repair_cost_and_preserves_ambiguous_mixed_and_overflow_values() {
        let mut store = base_store();
        create_task(
            &mut store,
            "repair-task",
            "baseline",
            "2026-10-25T10:00:00Z",
            labels(),
        );
        add_attempt(
            &mut store,
            "repair-task",
            "repair-first",
            "baseline",
            None,
            SpanRole::Worker,
            AttemptOutcome::Failure,
            Some((100, "micro")),
            0,
            true,
            false,
        );
        add_attempt(
            &mut store,
            "repair-task",
            "repair-second",
            "baseline",
            Some("repair-first"),
            SpanRole::ParentRepair,
            AttemptOutcome::Success,
            Some((50, "micro")),
            0,
            false,
            true,
        );
        let report = store
            .accounting_report(&AccountingRequest {
                schema_version: 1,
                task_id: Some("repair-task".into()),
            })
            .unwrap();
        let task = &report.tasks[0];
        assert_eq!(task.attempt_count, 2);
        assert_eq!(task.parent_repair_count, 1);
        assert_eq!(task.retry_count, 1);
        assert!(task.accepted);
        assert_eq!(task.operational_failure_count, 2);
        assert_eq!(task.cost.full_chain_total.value, Some(150));
        assert_eq!(task.failures.len(), 2);
        record(
            &mut store,
            "repair-late-cost",
            &["accepted-repair-second"],
            EventKind::UsageCost {
                session_id: "session-a".into(),
                task_id: "repair-task".into(),
                attempt_id: "repair-second".into(),
                usage: ComponentValue::Known {
                    value: 100,
                    unit: "tokens".into(),
                },
                cost: ComponentValue::Known {
                    value: 50,
                    unit: "micro".into(),
                },
                elapsed_ms: ComponentValue::Known {
                    value: 25,
                    unit: "ms".into(),
                },
            },
        );
        let report = store
            .accounting_report(&AccountingRequest {
                schema_version: 1,
                task_id: Some("repair-task".into()),
            })
            .unwrap();
        assert_eq!(report.tasks[0].current_status, "accepted");

        create_task(
            &mut store,
            "ambiguous-task",
            "baseline",
            "2026-10-25T10:00:00Z",
            labels(),
        );
        add_attempt(
            &mut store,
            "ambiguous-task",
            "ambiguous-attempt",
            "baseline",
            None,
            SpanRole::Worker,
            AttemptOutcome::Success,
            Some((2, "micro")),
            1,
            false,
            false,
        );
        let report = store
            .accounting_report(&AccountingRequest {
                schema_version: 1,
                task_id: Some("ambiguous-task".into()),
            })
            .unwrap();
        assert_eq!(report.tasks[0].attempts[0].usage_cost_record_count, 2);
        assert!(
            report.tasks[0]
                .cost
                .full_chain_total
                .reasons
                .contains(&"ambiguous_usage_cost_records".into())
        );

        create_task(
            &mut store,
            "mixed-task",
            "baseline",
            "2026-10-25T10:00:00Z",
            labels(),
        );
        add_attempt(
            &mut store,
            "mixed-task",
            "mixed-first",
            "baseline",
            None,
            SpanRole::Worker,
            AttemptOutcome::Success,
            Some((2, "micro")),
            0,
            false,
            false,
        );
        add_attempt(
            &mut store,
            "mixed-task",
            "mixed-second",
            "baseline",
            Some("mixed-first"),
            SpanRole::ParentRepair,
            AttemptOutcome::Success,
            Some((2, "USD")),
            0,
            false,
            false,
        );
        let report = store
            .accounting_report(&AccountingRequest {
                schema_version: 1,
                task_id: Some("mixed-task".into()),
            })
            .unwrap();
        assert!(
            report.tasks[0]
                .cost
                .full_chain_total
                .reasons
                .contains(&"mixed_units".into())
        );

        create_task(
            &mut store,
            "overflow-task",
            "baseline",
            "2026-10-25T10:00:00Z",
            labels(),
        );
        add_attempt(
            &mut store,
            "overflow-task",
            "overflow-first",
            "baseline",
            None,
            SpanRole::Worker,
            AttemptOutcome::Success,
            Some((u64::MAX, "micro")),
            0,
            false,
            false,
        );
        add_attempt(
            &mut store,
            "overflow-task",
            "overflow-second",
            "baseline",
            Some("overflow-first"),
            SpanRole::ParentRepair,
            AttemptOutcome::Success,
            Some((1, "micro")),
            0,
            false,
            false,
        );
        let report = store
            .accounting_report(&AccountingRequest {
                schema_version: 1,
                task_id: Some("overflow-task".into()),
            })
            .unwrap();
        assert!(
            report.tasks[0]
                .cost
                .full_chain_total
                .reasons
                .contains(&"overflow".into())
        );
        assert_eq!(report.tasks[0].cost.known_subtotals[0].known_subtotal, None);
    }

    #[test]
    fn comparison_uses_eligible_exact_cohorts_wilson_baseline_and_honest_limits() {
        let mut store = base_store();
        for i in 0..30 {
            let task_id = format!("base-task-{i}");
            create_task(
                &mut store,
                &task_id,
                "baseline",
                "2026-10-25T10:00:00Z",
                labels(),
            );
            let accepted = i == 0;
            add_attempt(
                &mut store,
                &task_id,
                &format!("base-attempt-{i}"),
                "baseline",
                None,
                SpanRole::Worker,
                if accepted {
                    AttemptOutcome::Success
                } else {
                    AttemptOutcome::Failure
                },
                Some((1, "micro")),
                0,
                false,
                accepted,
            );
        }
        for i in 0..30 {
            let task_id = format!("alt-task-{i}");
            create_task(
                &mut store,
                &task_id,
                "alternative",
                "2026-10-25T10:00:00Z",
                labels(),
            );
            add_attempt(
                &mut store,
                &task_id,
                &format!("alt-attempt-{i}"),
                "alternative",
                None,
                SpanRole::Worker,
                AttemptOutcome::Success,
                Some((1, "micro")),
                0,
                false,
                true,
            );
        }
        let report = store
            .compare_outcomes(&compare_request(None), Some(&current_state()))
            .unwrap();
        assert_eq!(report.method, "descriptive-v1");
        assert_eq!(report.observed_preferred_profile_ids, vec!["alternative"]);
        let baseline = report
            .comparisons
            .iter()
            .find(|comparison| comparison.profile_id == "baseline")
            .unwrap();
        let alternative = report
            .comparisons
            .iter()
            .find(|comparison| comparison.profile_id == "alternative")
            .unwrap();
        assert_eq!(baseline.sample_count, 30);
        assert_eq!(baseline.accepted_task_count, 1);
        assert_eq!(alternative.sample_count, 30);
        assert_eq!(alternative.acceptance_proportion, Some(1.0));
        assert_eq!(alternative.cost_per_accepted_task.total_cost, Some(30));
        assert_eq!(
            alternative.cost_per_accepted_task.accepted_task_count,
            Some(30)
        );

        let selected = store
            .compare_outcomes(
                &compare_request(Some("alternative")),
                Some(&current_state()),
            )
            .unwrap();
        assert!(selected.observed_preferred_profile_ids.is_empty());
        assert!(
            selected
                .reasons
                .contains(&"baseline_unavailable_or_ineligible".into())
        );
        let unavailable = store
            .compare_outcomes(&compare_request(None), None)
            .unwrap();
        assert_eq!(unavailable.binding, "binding_unavailable");
        assert_eq!(unavailable.outcome, "no_eligible_candidate");
        let mut empty_window = compare_request(None);
        empty_window.evidence_window_start = "2026-10-26T00:00:00Z".into();
        empty_window.evidence_window_end = "2026-10-27T00:00:00Z".into();
        let sparse = store
            .compare_outcomes(&empty_window, Some(&current_state()))
            .unwrap();
        assert_eq!(sparse.outcome, "insufficient_evidence");
        assert!(
            sparse
                .reasons
                .contains(&"insufficient_matched_evidence".into())
        );
    }

    #[test]
    fn comparison_excludes_mismatched_cohort_dimensions() {
        for field in [
            "task_label_provenance",
            "policy_version",
            "harness",
            "harness_version",
            "adapter",
            "adapter_version",
            "configuration_fingerprint",
        ] {
            let mut store = base_store();
            create_task(
                &mut store,
                "cohort-baseline",
                "baseline",
                "2026-10-25T10:00:00Z",
                labels(),
            );
            add_attempt(
                &mut store,
                "cohort-baseline",
                "cohort-baseline-attempt",
                "baseline",
                None,
                SpanRole::Worker,
                AttemptOutcome::Success,
                Some((1, "micro")),
                0,
                false,
                true,
            );

            let mut other_snapshot = snapshot();
            other_snapshot.snapshot_id = format!("snapshot-{}", field.replace('_', "-"));
            match field {
                "policy_version" => other_snapshot.policy_version = "policy-b".into(),
                "harness" => other_snapshot.harness = "other-harness".into(),
                "harness_version" => other_snapshot.harness_version = "2".into(),
                "adapter" => other_snapshot.adapter = "other-adapter".into(),
                "adapter_version" => other_snapshot.adapter_version = "2".into(),
                "configuration_fingerprint" => {
                    other_snapshot.configuration_fingerprint = "config-b".into()
                }
                _ => {}
            }
            let other_snapshot_id = other_snapshot.snapshot_id.clone();
            store.import_inventory(other_snapshot).unwrap();

            let mut other_labels = labels();
            if field == "task_label_provenance"
                && let ContextLabel::Known { provenance, .. } = &mut other_labels.task_type
            {
                *provenance = "planner-v2".into();
            }
            create_task_on_snapshot(
                &mut store,
                "cohort-alternative",
                "alternative",
                "2026-10-25T10:00:00Z",
                other_labels,
                &other_snapshot_id,
            );
            add_attempt(
                &mut store,
                "cohort-alternative",
                "cohort-alternative-attempt",
                "alternative",
                None,
                SpanRole::Worker,
                AttemptOutcome::Success,
                Some((1, "micro")),
                0,
                false,
                true,
            );

            let report = store
                .compare_outcomes(&compare_request(None), Some(&current_state()))
                .unwrap();
            let alternative = report
                .comparisons
                .iter()
                .find(|comparison| comparison.profile_id == "alternative")
                .unwrap();
            assert_eq!(alternative.sample_count, 0, "mismatch: {field}");
            assert!(
                alternative.reasons.contains(&"no_matched_cohort".into()),
                "mismatch: {field}"
            );
            assert!(
                report.observed_preferred_profile_ids.is_empty(),
                "mismatch: {field}"
            );
            assert_eq!(report.outcome, "insufficient_evidence", "mismatch: {field}");
            assert!(
                report
                    .reasons
                    .contains(&"insufficient_matched_evidence".into()),
                "mismatch: {field}"
            );
        }
    }

    #[test]
    fn comparison_excludes_profile_identity_mismatch() {
        let mut store = base_store();
        for profile_id in ["baseline", "alternative"] {
            create_task(
                &mut store,
                &format!("identity-{profile_id}"),
                profile_id,
                "2026-10-25T10:00:00Z",
                labels(),
            );
            add_attempt(
                &mut store,
                &format!("identity-{profile_id}"),
                &format!("identity-attempt-{profile_id}"),
                profile_id,
                None,
                SpanRole::Worker,
                AttemptOutcome::Success,
                Some((1, "micro")),
                0,
                false,
                true,
            );
        }

        let mut current_snapshot = snapshot();
        current_snapshot.snapshot_id = "snapshot-current".into();
        current_snapshot
            .profiles
            .iter_mut()
            .find(|profile| profile.profile_id == "alternative")
            .unwrap()
            .revision = "alternative-revision-2".into();
        store.import_inventory(current_snapshot).unwrap();

        let mut request = compare_request(None);
        request.eligibility.snapshot_id = "snapshot-current".into();
        let mut current = current_state();
        current.expected_snapshot_id = "snapshot-current".into();
        let report = store.compare_outcomes(&request, Some(&current)).unwrap();
        let alternative = report
            .comparisons
            .iter()
            .find(|comparison| comparison.profile_id == "alternative")
            .unwrap();
        assert_eq!(alternative.sample_count, 0);
        assert!(
            alternative
                .reasons
                .contains(&"profile_dimension_mismatch".into())
        );
        assert!(report.excluded_tasks.contains(&(
            "identity-alternative".into(),
            "profile_dimension_mismatch".into()
        )));
        assert!(report.observed_preferred_profile_ids.is_empty());
        assert_eq!(report.outcome, "insufficient_evidence");
        assert!(
            report
                .reasons
                .contains(&"insufficient_matched_evidence".into())
        );
    }

    #[test]
    fn comparison_does_not_prefer_overlapping_wilson_intervals() {
        let mut store = base_store();
        for profile_id in ["baseline", "alternative"] {
            let task_id = format!("overlap-{profile_id}");
            create_task(
                &mut store,
                &task_id,
                profile_id,
                "2026-10-25T10:00:00Z",
                labels(),
            );
            add_attempt(
                &mut store,
                &task_id,
                &format!("overlap-attempt-{profile_id}"),
                profile_id,
                None,
                SpanRole::Worker,
                AttemptOutcome::Success,
                Some((1, "micro")),
                0,
                false,
                true,
            );
        }

        let report = store
            .compare_outcomes(&compare_request(None), Some(&current_state()))
            .unwrap();
        assert!(report.observed_preferred_profile_ids.is_empty());
        assert_eq!(report.outcome, "insufficient_evidence");
        assert!(
            report
                .reasons
                .contains(&"interval_overlap_or_cost_not_lower".into())
        );
        let baseline = report
            .comparisons
            .iter()
            .find(|comparison| comparison.profile_id == "baseline")
            .unwrap();
        let alternative = report
            .comparisons
            .iter()
            .find(|comparison| comparison.profile_id == "alternative")
            .unwrap();
        assert_eq!(
            baseline.acceptance_interval_95,
            alternative.acceptance_interval_95
        );
    }
}
