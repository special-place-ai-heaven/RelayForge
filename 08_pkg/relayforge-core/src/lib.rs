//! Typed, bounded durable metadata for RelayForge.
//!
//! This crate stores constrained metadata only. It does not dispatch work.
//!
//! ```
//! use relayforge_core::{Event, EventKind, open_memory};
//! let mut store = open_memory().unwrap();
//! let event = Event::new("source-a", "event-a", EventKind::SessionOpened {
//!     session_id: "session-a".into(), owner_id: "source-a".into(),
//! });
//! store.append(event).unwrap();
//! ```

use rusqlite::{Connection, OptionalExtension, Transaction, TransactionBehavior, params};
use serde::{Deserialize, Serialize};
use std::path::Path;

pub const EVENT_SCHEMA_VERSION: u16 = 1;
pub const DATABASE_SCHEMA_VERSION: i64 = 1;
pub const MAX_EVENT_BYTES: usize = 65_536;
pub const MAX_JSON_DEPTH: usize = 16;
pub const MAX_EVENTS_PER_SHOW: usize = 200;

const SCHEMA: &str = "
CREATE TABLE IF NOT EXISTS sessions (
    session_id TEXT PRIMARY KEY,
    owner_id TEXT NOT NULL,
    state TEXT NOT NULL CHECK(state IN ('active','recoverable','closed'))
);
CREATE TABLE IF NOT EXISTS events (
    sequence INTEGER PRIMARY KEY AUTOINCREMENT,
    source_id TEXT NOT NULL,
    event_id TEXT NOT NULL,
    schema_version INTEGER NOT NULL,
    event_json TEXT NOT NULL,
    kind TEXT NOT NULL,
    session_id TEXT,
    task_id TEXT,
    attempt_id TEXT,
    created_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ','now')),
    UNIQUE(source_id, event_id)
);
CREATE TABLE IF NOT EXISTS parents (
    source_id TEXT NOT NULL,
    event_id TEXT NOT NULL,
    parent_source_id TEXT NOT NULL,
    parent_event_id TEXT NOT NULL,
    PRIMARY KEY(source_id,event_id,parent_source_id,parent_event_id),
    FOREIGN KEY(source_id,event_id) REFERENCES events(source_id,event_id),
    FOREIGN KEY(parent_source_id,parent_event_id) REFERENCES events(source_id,event_id)
);
CREATE TABLE IF NOT EXISTS attempts (
    attempt_id TEXT PRIMARY KEY,
    session_id TEXT NOT NULL,
    task_id TEXT NOT NULL,
    predecessor_attempt_id TEXT,
    intent_source_id TEXT NOT NULL,
    intent_event_id TEXT NOT NULL,
    selected_json TEXT NOT NULL
);
CREATE TABLE IF NOT EXISTS inventory_snapshots (
    snapshot_id TEXT PRIMARY KEY,
    snapshot_json TEXT NOT NULL
);
CREATE TRIGGER IF NOT EXISTS inventory_snapshots_no_update BEFORE UPDATE ON inventory_snapshots BEGIN SELECT RAISE(ABORT,'immutable inventory snapshot'); END;
CREATE TRIGGER IF NOT EXISTS inventory_snapshots_no_delete BEFORE DELETE ON inventory_snapshots BEGIN SELECT RAISE(ABORT,'immutable inventory snapshot'); END;
CREATE TRIGGER IF NOT EXISTS events_no_update BEFORE UPDATE ON events BEGIN SELECT RAISE(ABORT,'immutable event'); END;
CREATE TRIGGER IF NOT EXISTS events_no_delete BEFORE DELETE ON events BEGIN SELECT RAISE(ABORT,'immutable event'); END;
CREATE TRIGGER IF NOT EXISTS parents_no_update BEFORE UPDATE ON parents BEGIN SELECT RAISE(ABORT,'immutable parent link'); END;
CREATE TRIGGER IF NOT EXISTS parents_no_delete BEFORE DELETE ON parents BEGIN SELECT RAISE(ABORT,'immutable parent link'); END;
CREATE TRIGGER IF NOT EXISTS attempts_no_update BEFORE UPDATE ON attempts BEGIN SELECT RAISE(ABORT,'immutable attempt'); END;
CREATE TRIGGER IF NOT EXISTS attempts_no_delete BEFORE DELETE ON attempts BEGIN SELECT RAISE(ABORT,'immutable attempt'); END;
";

const INVENTORY_SCHEMA: &str = "
BEGIN IMMEDIATE;
CREATE TABLE IF NOT EXISTS inventory_snapshots (
    snapshot_id TEXT PRIMARY KEY,
    snapshot_json TEXT NOT NULL
);
CREATE TRIGGER IF NOT EXISTS inventory_snapshots_no_update BEFORE UPDATE ON inventory_snapshots BEGIN SELECT RAISE(ABORT,'immutable inventory snapshot'); END;
CREATE TRIGGER IF NOT EXISTS inventory_snapshots_no_delete BEFORE DELETE ON inventory_snapshots BEGIN SELECT RAISE(ABORT,'immutable inventory snapshot'); END;
COMMIT;
";

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Event {
    pub schema_version: u16,
    pub source_id: String,
    pub event_id: String,
    #[serde(default)]
    pub parents: Vec<EventRef>,
    pub kind: EventKind,
}

impl Event {
    pub fn new(source_id: impl Into<String>, event_id: impl Into<String>, kind: EventKind) -> Self {
        Self {
            schema_version: EVENT_SCHEMA_VERSION,
            source_id: source_id.into(),
            event_id: event_id.into(),
            parents: Vec::new(),
            kind,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EventRef {
    pub source_id: String,
    pub event_id: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", content = "data", deny_unknown_fields)]
pub enum EventKind {
    SessionOpened {
        session_id: String,
        owner_id: String,
    },
    SessionClosed {
        session_id: String,
        owner_id: String,
    },
    SessionRecovered {
        session_id: String,
        expected_owner_id: String,
        actor_id: String,
        reason: String,
        evidence_ref: String,
    },
    TaskCreated {
        session_id: String,
        task_id: String,
        required_check_ids: Vec<String>,
    },
    AttemptIntent {
        session_id: String,
        task_id: String,
        attempt_id: String,
        predecessor_attempt_id: Option<String>,
        span_id: String,
        span_role: SpanRole,
        selected: SelectedIdentity,
    },
    AttemptResult {
        session_id: String,
        task_id: String,
        attempt_id: String,
        outcome: AttemptOutcome,
        observed: ObservedIdentity,
        evidence_ref: Option<String>,
    },
    ToolResult {
        session_id: String,
        task_id: String,
        attempt_id: String,
        tool_id: String,
        operation_id: String,
        result: ToolOutcome,
        failure_category: Option<FailureCategory>,
        evidence_ref: Option<String>,
    },
    UsageCost {
        session_id: String,
        task_id: String,
        attempt_id: String,
        usage: ComponentValue,
        cost: ComponentValue,
        elapsed_ms: ComponentValue,
    },
    Verification {
        session_id: String,
        task_id: String,
        attempt_id: String,
        check_id: String,
        passed: bool,
        evidence_ref: String,
    },
    TaskAccepted {
        session_id: String,
        task_id: String,
        attempt_id: String,
        actor_id: String,
        evidence_ref: String,
    },
    Interrupted {
        session_id: String,
        task_id: String,
        attempt_id: String,
        reason: String,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SpanRole {
    Worker,
    ParentRepair,
    Tool,
    Verification,
    Recovery,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SelectedIdentity {
    pub profile: String,
    pub model: String,
    pub model_version: String,
    pub effort: String,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "state", deny_unknown_fields)]
pub enum ObservedIdentity {
    Known {
        profile: String,
        model: String,
        model_version: String,
        effort: String,
    },
    Unknown {
        reason: String,
    },
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AttemptOutcome {
    Success,
    Failure,
    NotRun,
    Unknown,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ToolOutcome {
    Success,
    Failure,
    NotRun,
    Unknown,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FailureCategory {
    InvalidCall,
    Permission,
    Unavailable,
    Infrastructure,
    Timeout,
    Unresolved,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "state", deny_unknown_fields)]
pub enum ComponentValue {
    Known { value: u64, unit: String },
    Unknown { reason: String },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AppendOutcome {
    Appended { sequence: i64 },
    Replayed { sequence: i64 },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StoreError {
    InvalidInput(&'static str),
    UnsupportedSchema,
    Conflict(&'static str),
    NotFound(&'static str),
    Sql(String),
}

pub const MAX_INVENTORY_BYTES: usize = 1_048_576;
pub const MAX_INVENTORY_PROFILES: usize = 64;
pub const MAX_CAPABILITIES_PER_PROFILE: usize = 64;

/// Versioned inventory metadata is declared by a caller and is not proof of runtime state.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct InventorySnapshot {
    pub schema_version: u16,
    pub snapshot_id: String,
    pub revision: String,
    pub policy_version: String,
    pub created_at: String,
    pub harness: String,
    pub harness_version: String,
    pub adapter: String,
    pub adapter_version: String,
    pub runtime_scope: String,
    pub session_scope: String,
    pub configuration_fingerprint: String,
    pub workspace_fingerprint: String,
    pub profiles: Vec<InventoryProfile>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct InventoryProfile {
    pub profile_id: String,
    pub revision: String,
    pub native_role: String,
    pub model_provider: String,
    pub configured_model_id: String,
    pub resolved_model_id: Option<String>,
    pub resolved_model_version: Option<String>,
    pub effort: String,
    pub supported_efforts: Vec<String>,
    pub capabilities: Vec<CapabilityFact>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CapabilityStatus {
    Configured,
    RuntimeConfirmed,
    Unsupported,
    Unknown,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CapabilityFact {
    pub capability_id: String,
    pub value: String,
    pub status: CapabilityStatus,
    pub source_ref: String,
    pub observed_at: String,
    pub expires_at: String,
    pub profile_revision: String,
    pub harness_version: String,
    pub configuration_fingerprint: String,
    pub runtime_scope: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EligibilityRequest {
    pub schema_version: u16,
    pub request_id: String,
    pub task_id: String,
    pub session_id: String,
    pub snapshot_id: String,
    pub policy_version: String,
    pub candidate_profile_ids: Vec<String>,
    pub execution_path: String,
    pub selected_profile_id: Option<String>,
    pub required_capabilities: Vec<RequiredCapability>,
    pub workload_labels: Vec<WorkloadLabel>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RequiredCapability {
    pub capability_id: String,
    pub value: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct WorkloadLabel {
    pub name: String,
    pub value: String,
    pub provenance: String,
}

/// This is supplied separately by a host integration that owns trustworthy capture.
/// Matching names and digests are not authentication; G4 adapters must derive these
/// values independently and observe the runtime before constructing this value.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CurrentState {
    pub now: String,
    pub execution_path: String,
    pub expected_snapshot_id: String,
    pub policy_version: String,
    pub harness: String,
    pub harness_version: String,
    pub adapter: String,
    pub adapter_version: String,
    pub runtime_scope: String,
    pub session_scope: String,
    pub configuration_fingerprint: String,
    pub workspace_fingerprint: String,
    pub workspace_dirty: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CandidateEligibility {
    pub profile_id: String,
    pub eligible: bool,
    pub reasons: Vec<String>,
    pub evidence: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EligibilityResult {
    pub eligible_profile_ids: Vec<String>,
    pub candidates: Vec<CandidateEligibility>,
    pub outcome: String,
}

impl std::fmt::Display for StoreError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidInput(reason) => write!(f, "invalid_event: {reason}"),
            Self::UnsupportedSchema => f.write_str("unsupported_schema"),
            Self::Conflict(reason) => write!(f, "conflict: {reason}"),
            Self::NotFound(reason) => write!(f, "not_found: {reason}"),
            Self::Sql(_) => f.write_str("storage_error"),
        }
    }
}
impl std::error::Error for StoreError {}
impl From<rusqlite::Error> for StoreError {
    fn from(value: rusqlite::Error) -> Self {
        Self::Sql(value.to_string())
    }
}

pub struct Store {
    connection: Connection,
}

pub fn open(path: &Path) -> Result<Store, StoreError> {
    Store::open(path)
}
pub fn open_memory() -> Result<Store, StoreError> {
    Store::memory()
}
pub fn parse_event_bytes(input: &[u8]) -> Result<Event, StoreError> {
    if input.is_empty() || input.len() > MAX_EVENT_BYTES {
        return Err(StoreError::InvalidInput("event_size_out_of_range"));
    }
    if json_depth(input) > MAX_JSON_DEPTH {
        return Err(StoreError::InvalidInput("event_too_deep"));
    }
    let text =
        std::str::from_utf8(input).map_err(|_| StoreError::InvalidInput("invalid_event_json"))?;
    if sensitive(text) {
        return Err(StoreError::InvalidInput("sensitive_input_rejected"));
    }
    let event: Event = serde_json::from_slice(input)
        .map_err(|_| StoreError::InvalidInput("invalid_event_json"))?;
    validate_event(&event)?;
    Ok(event)
}

impl Store {
    pub fn open(path: &Path) -> Result<Self, StoreError> {
        let connection = Connection::open(path)?;
        Self::from_connection(connection)
    }
    pub fn memory() -> Result<Self, StoreError> {
        Self::from_connection(Connection::open_in_memory()?)
    }
    fn from_connection(connection: Connection) -> Result<Self, StoreError> {
        connection.pragma_update(None, "foreign_keys", "ON")?;
        connection.busy_timeout(std::time::Duration::from_secs(5))?;
        let version: i64 = connection.pragma_query_value(None, "user_version", |row| row.get(0))?;
        if version != 0 && version != DATABASE_SCHEMA_VERSION {
            return Err(StoreError::UnsupportedSchema);
        }
        if version == 0 {
            connection.pragma_update(None, "journal_mode", "WAL")?;
            connection.pragma_update(None, "synchronous", "FULL")?;
            connection.execute_batch(SCHEMA)?;
            connection.pragma_update(None, "user_version", DATABASE_SCHEMA_VERSION)?;
        } else {
            connection.execute_batch(INVENTORY_SCHEMA)?;
        }
        Ok(Self { connection })
    }
    pub fn append(&mut self, event: Event) -> Result<AppendOutcome, StoreError> {
        validate_event(&event)?;
        let encoded = serde_json::to_string(&event)
            .map_err(|_| StoreError::InvalidInput("malformed_event"))?;
        let tx = self
            .connection
            .transaction_with_behavior(TransactionBehavior::Immediate)?;
        if let Some((existing, sequence)) = tx
            .query_row(
                "SELECT event_json,sequence FROM events WHERE source_id=?1 AND event_id=?2",
                params![event.source_id, event.event_id],
                |row| Ok((row.get::<_, String>(0)?, row.get::<_, i64>(1)?)),
            )
            .optional()?
        {
            if existing == encoded {
                tx.commit()?;
                return Ok(AppendOutcome::Replayed { sequence });
            }
            return Err(StoreError::Conflict("event_identity_reused"));
        }
        validate_parents(&tx, &event)?;
        validate_scope_and_claim(&tx, &event)?;
        validate_kind_state(&tx, &event)?;
        let (kind, session_id, task_id, attempt_id) = event.kind.columns();
        tx.execute("INSERT INTO events(source_id,event_id,schema_version,event_json,kind,session_id,task_id,attempt_id) VALUES(?1,?2,?3,?4,?5,?6,?7,?8)", params![event.source_id,event.event_id,event.schema_version,encoded,kind,session_id,task_id,attempt_id])?;
        let sequence = tx.last_insert_rowid();
        for parent in &event.parents {
            tx.execute("INSERT INTO parents(source_id,event_id,parent_source_id,parent_event_id) VALUES(?1,?2,?3,?4)", params![event.source_id,event.event_id,parent.source_id,parent.event_id])?;
        }
        apply_claim(&tx, &event)?;
        tx.commit()?;
        Ok(AppendOutcome::Appended { sequence })
    }
    pub fn reconcile(
        &mut self,
        session_id: &str,
        actor_id: &str,
        reason: &str,
    ) -> Result<usize, StoreError> {
        validate_id(session_id)?;
        validate_id(actor_id)?;
        validate_text(reason, 256)?;
        let tx = self
            .connection
            .transaction_with_behavior(TransactionBehavior::Immediate)?;
        let claim: Option<(String, String)> = tx
            .query_row(
                "SELECT owner_id,state FROM sessions WHERE session_id=?1",
                [session_id],
                |row| Ok((row.get(0)?, row.get(1)?)),
            )
            .optional()?;
        let Some((owner, state)) = claim else {
            return Err(StoreError::NotFound("session"));
        };
        if state == "active" {
            tx.commit()?;
            return Ok(0);
        }
        if state != "recoverable" {
            return Err(StoreError::Conflict("session_not_recoverable"));
        }
        if owner != actor_id {
            return Err(StoreError::Conflict(
                "reconcile_actor_must_own_recovered_session",
            ));
        }
        let mut query = tx.prepare("SELECT attempt_id,task_id,intent_source_id,intent_event_id FROM attempts WHERE session_id=?1 AND NOT EXISTS (SELECT 1 FROM events e WHERE e.kind='AttemptResult' AND e.attempt_id=attempts.attempt_id) AND NOT EXISTS (SELECT 1 FROM events e WHERE e.kind='Interrupted' AND e.attempt_id=attempts.attempt_id) ORDER BY attempt_id")?;
        let pending = query
            .query_map([session_id], |row| {
                Ok((
                    row.get::<_, String>(0)?,
                    row.get::<_, String>(1)?,
                    row.get::<_, String>(2)?,
                    row.get::<_, String>(3)?,
                ))
            })?
            .collect::<Result<Vec<_>, _>>()?;
        drop(query);
        let mut count = 0;
        for (attempt_id, task_id, source_id, intent_event_id) in pending {
            let event = Event {
                schema_version: EVENT_SCHEMA_VERSION,
                source_id: actor_id.to_owned(),
                event_id: format!("interrupted-{attempt_id}"),
                parents: vec![EventRef {
                    source_id,
                    event_id: intent_event_id,
                }],
                kind: EventKind::Interrupted {
                    session_id: session_id.to_owned(),
                    task_id,
                    attempt_id,
                    reason: reason.to_owned(),
                },
            };
            validate_event(&event)?;
            let encoded = serde_json::to_string(&event)
                .map_err(|_| StoreError::InvalidInput("malformed_event"))?;
            let (kind, sid, tid, aid) = event.kind.columns();
            tx.execute("INSERT OR IGNORE INTO events(source_id,event_id,schema_version,event_json,kind,session_id,task_id,attempt_id) VALUES(?1,?2,1,?3,?4,?5,?6,?7)",params![event.source_id,event.event_id,encoded,kind,sid,tid,aid])?;
            if tx.changes() > 0 {
                tx.execute("INSERT INTO parents(source_id,event_id,parent_source_id,parent_event_id) VALUES(?1,?2,?3,?4)",params![event.source_id,event.event_id,event.parents[0].source_id,event.parents[0].event_id])?;
                count += 1;
            }
        }
        tx.commit()?;
        Ok(count)
    }
    pub fn show(
        &self,
        session_id: Option<&str>,
        task_id: Option<&str>,
        limit: usize,
    ) -> Result<Vec<Event>, StoreError> {
        if limit == 0 || limit > MAX_EVENTS_PER_SHOW {
            return Err(StoreError::InvalidInput("show_limit_out_of_range"));
        }
        if session_id.is_none() && task_id.is_none() {
            return Err(StoreError::InvalidInput("show_scope_required"));
        }
        let mut statement = self.connection.prepare("SELECT event_json FROM events WHERE (?1 IS NULL OR session_id=?1) AND (?2 IS NULL OR task_id=?2) ORDER BY sequence LIMIT ?3")?;
        let rows = statement.query_map(params![session_id, task_id, limit as i64], |row| {
            row.get::<_, String>(0)
        })?;
        rows.map(|row| {
            let json = row?;
            serde_json::from_str(&json)
                .map_err(|_| rusqlite::Error::InvalidQuery)
                .map_err(StoreError::from)
        })
        .collect()
    }

    pub fn import_inventory(&mut self, snapshot: InventorySnapshot) -> Result<bool, StoreError> {
        validate_snapshot(&snapshot)?;
        let encoded = serde_json::to_string(&snapshot)
            .map_err(|_| StoreError::InvalidInput("invalid_inventory"))?;
        let tx = self
            .connection
            .transaction_with_behavior(TransactionBehavior::Immediate)?;
        let existing: Option<String> = tx
            .query_row(
                "SELECT snapshot_json FROM inventory_snapshots WHERE snapshot_id=?1",
                [&snapshot.snapshot_id],
                |row| row.get(0),
            )
            .optional()?;
        if let Some(existing) = existing {
            if existing == encoded {
                tx.commit()?;
                return Ok(false);
            }
            return Err(StoreError::Conflict("snapshot_identity_reused"));
        }
        tx.execute(
            "INSERT INTO inventory_snapshots(snapshot_id,snapshot_json) VALUES(?1,?2)",
            params![snapshot.snapshot_id, encoded],
        )?;
        tx.commit()?;
        Ok(true)
    }

    pub fn inventory(&self, snapshot_id: &str) -> Result<InventorySnapshot, StoreError> {
        validate_id(snapshot_id)?;
        let raw: String = self
            .connection
            .query_row(
                "SELECT snapshot_json FROM inventory_snapshots WHERE snapshot_id=?1",
                [snapshot_id],
                |row| row.get(0),
            )
            .optional()?
            .ok_or(StoreError::NotFound("inventory_snapshot"))?;
        serde_json::from_str(&raw).map_err(|_| StoreError::InvalidInput("stored_inventory_invalid"))
    }
}

impl EventKind {
    fn columns(&self) -> (&'static str, Option<&str>, Option<&str>, Option<&str>) {
        match self {
            Self::SessionOpened { session_id, .. } => {
                ("SessionOpened", Some(session_id), None, None)
            }
            Self::SessionClosed { session_id, .. } => {
                ("SessionClosed", Some(session_id), None, None)
            }
            Self::SessionRecovered { session_id, .. } => {
                ("SessionRecovered", Some(session_id), None, None)
            }
            Self::TaskCreated {
                session_id,
                task_id,
                ..
            } => ("TaskCreated", Some(session_id), Some(task_id), None),
            Self::AttemptIntent {
                session_id,
                task_id,
                attempt_id,
                ..
            } => (
                "AttemptIntent",
                Some(session_id),
                Some(task_id),
                Some(attempt_id),
            ),
            Self::AttemptResult {
                session_id,
                task_id,
                attempt_id,
                ..
            } => (
                "AttemptResult",
                Some(session_id),
                Some(task_id),
                Some(attempt_id),
            ),
            Self::ToolResult {
                session_id,
                task_id,
                attempt_id,
                ..
            } => (
                "ToolResult",
                Some(session_id),
                Some(task_id),
                Some(attempt_id),
            ),
            Self::UsageCost {
                session_id,
                task_id,
                attempt_id,
                ..
            } => (
                "UsageCost",
                Some(session_id),
                Some(task_id),
                Some(attempt_id),
            ),
            Self::Verification {
                session_id,
                task_id,
                attempt_id,
                ..
            } => (
                "Verification",
                Some(session_id),
                Some(task_id),
                Some(attempt_id),
            ),
            Self::TaskAccepted {
                session_id,
                task_id,
                attempt_id,
                ..
            } => (
                "TaskAccepted",
                Some(session_id),
                Some(task_id),
                Some(attempt_id),
            ),
            Self::Interrupted {
                session_id,
                task_id,
                attempt_id,
                ..
            } => (
                "Interrupted",
                Some(session_id),
                Some(task_id),
                Some(attempt_id),
            ),
        }
    }
}

pub fn parse_inventory_bytes(input: &[u8]) -> Result<InventorySnapshot, StoreError> {
    if input.is_empty() || input.len() > MAX_INVENTORY_BYTES {
        return Err(StoreError::InvalidInput("inventory_size_out_of_range"));
    }
    if json_depth(input) > MAX_JSON_DEPTH {
        return Err(StoreError::InvalidInput("inventory_too_deep"));
    }
    let text = std::str::from_utf8(input)
        .map_err(|_| StoreError::InvalidInput("invalid_inventory_json"))?;
    if sensitive(text) {
        return Err(StoreError::InvalidInput("sensitive_input_rejected"));
    }
    let snapshot: InventorySnapshot = serde_json::from_slice(input)
        .map_err(|_| StoreError::InvalidInput("invalid_inventory_json"))?;
    validate_snapshot(&snapshot)?;
    Ok(snapshot)
}

pub fn parse_eligibility_request_bytes(input: &[u8]) -> Result<EligibilityRequest, StoreError> {
    if input.is_empty() || input.len() > MAX_INVENTORY_BYTES {
        return Err(StoreError::InvalidInput("request_size_out_of_range"));
    }
    if json_depth(input) > MAX_JSON_DEPTH {
        return Err(StoreError::InvalidInput("request_too_deep"));
    }
    let text =
        std::str::from_utf8(input).map_err(|_| StoreError::InvalidInput("invalid_request_json"))?;
    if sensitive(text) {
        return Err(StoreError::InvalidInput("sensitive_input_rejected"));
    }
    let request: EligibilityRequest = serde_json::from_slice(input)
        .map_err(|_| StoreError::InvalidInput("invalid_request_json"))?;
    validate_request(&request)?;
    Ok(request)
}

fn safe_text(value: &str, max: usize) -> Result<(), StoreError> {
    validate_text(value, max)?;
    if sensitive(value) {
        return Err(StoreError::InvalidInput("sensitive_input_rejected"));
    }
    Ok(())
}

fn valid_timestamp(value: &str) -> bool {
    let bytes = value.as_bytes();
    if bytes.len() != 20
        || bytes[4] != b'-'
        || bytes[7] != b'-'
        || bytes[10] != b'T'
        || bytes[13] != b':'
        || bytes[16] != b':'
        || bytes[19] != b'Z'
    {
        return false;
    }
    let digits = [0..4, 5..7, 8..10, 11..13, 14..16, 17..19];
    if digits
        .iter()
        .any(|r| !bytes[r.clone()].iter().all(u8::is_ascii_digit))
    {
        return false;
    }
    let num = |r: std::ops::Range<usize>| value[r].parse::<u32>().unwrap_or(0);
    let (year, month, day, hour, minute, second) = (
        num(0..4),
        num(5..7),
        num(8..10),
        num(11..13),
        num(14..16),
        num(17..19),
    );
    let leap = year % 4 == 0 && (year % 100 != 0 || year % 400 == 0);
    let days = match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 if leap => 29,
        2 => 28,
        _ => 0,
    };
    year > 0 && (1..=days).contains(&day) && hour < 24 && minute < 60 && second < 60
}

fn valid_reference(value: &str) -> bool {
    !value.starts_with('/')
        && !value.starts_with('~')
        && !value.contains('\\')
        && !value.contains("://")
        && !value.as_bytes().get(1).is_some_and(|b| *b == b':')
        && value
            .split('/')
            .all(|part| !part.is_empty() && part != "." && part != "..")
        && value
            .bytes()
            .all(|b| b.is_ascii_alphanumeric() || b"._:/-".contains(&b))
}

fn validate_inventory_id(value: &str) -> Result<(), StoreError> {
    validate_id(value)?;
    if sensitive(value) {
        return Err(StoreError::InvalidInput("sensitive_input_rejected"));
    }
    Ok(())
}

fn validate_snapshot(snapshot: &InventorySnapshot) -> Result<(), StoreError> {
    if snapshot.schema_version != 1 {
        return Err(StoreError::UnsupportedSchema);
    }
    validate_inventory_id(&snapshot.snapshot_id)?;
    for value in [
        &snapshot.revision,
        &snapshot.policy_version,
        &snapshot.harness,
        &snapshot.harness_version,
        &snapshot.adapter,
        &snapshot.adapter_version,
        &snapshot.runtime_scope,
        &snapshot.session_scope,
        &snapshot.configuration_fingerprint,
        &snapshot.workspace_fingerprint,
    ] {
        safe_text(value, 256)?;
    }
    if !valid_timestamp(&snapshot.created_at)
        || snapshot.profiles.is_empty()
        || snapshot.profiles.len() > MAX_INVENTORY_PROFILES
    {
        return Err(StoreError::InvalidInput("invalid_inventory_bounds"));
    }
    let mut profile_ids = std::collections::BTreeSet::new();
    for profile in &snapshot.profiles {
        validate_inventory_id(&profile.profile_id)?;
        if !profile_ids.insert(&profile.profile_id) {
            return Err(StoreError::InvalidInput("duplicate_profile"));
        }
        for value in [
            &profile.revision,
            &profile.native_role,
            &profile.model_provider,
            &profile.configured_model_id,
            &profile.effort,
        ] {
            safe_text(value, 256)?;
        }
        if profile.resolved_model_id.is_some() != profile.resolved_model_version.is_some() {
            return Err(StoreError::InvalidInput("incomplete_resolved_identity"));
        }
        for value in [&profile.resolved_model_id, &profile.resolved_model_version]
            .into_iter()
            .flatten()
        {
            safe_text(value, 256)?;
        }
        if profile.supported_efforts.is_empty() || profile.supported_efforts.len() > 32 {
            return Err(StoreError::InvalidInput("invalid_efforts"));
        }
        let mut efforts = std::collections::BTreeSet::new();
        for effort in &profile.supported_efforts {
            safe_text(effort, 64)?;
            if !efforts.insert(effort) {
                return Err(StoreError::InvalidInput("duplicate_effort"));
            }
        }
        if profile.capabilities.len() > MAX_CAPABILITIES_PER_PROFILE {
            return Err(StoreError::InvalidInput("too_many_capabilities"));
        }
        let mut capability_ids = std::collections::BTreeSet::new();
        for capability in &profile.capabilities {
            validate_inventory_id(&capability.capability_id)?;
            if !capability_ids.insert(&capability.capability_id) {
                return Err(StoreError::InvalidInput("duplicate_capability"));
            }
            for value in [
                &capability.value,
                &capability.profile_revision,
                &capability.harness_version,
                &capability.configuration_fingerprint,
                &capability.runtime_scope,
            ] {
                safe_text(value, 256)?;
            }
            if !valid_reference(&capability.source_ref)
                || sensitive(&capability.source_ref)
                || !valid_timestamp(&capability.observed_at)
                || !valid_timestamp(&capability.expires_at)
                || capability.expires_at <= capability.observed_at
            {
                return Err(StoreError::InvalidInput("invalid_capability_evidence"));
            }
        }
    }
    let encoded =
        serde_json::to_vec(snapshot).map_err(|_| StoreError::InvalidInput("invalid_inventory"))?;
    if encoded.len() > MAX_INVENTORY_BYTES || json_depth(&encoded) > MAX_JSON_DEPTH {
        return Err(StoreError::InvalidInput("inventory_size_out_of_range"));
    }
    Ok(())
}

fn validate_request(request: &EligibilityRequest) -> Result<(), StoreError> {
    if request.schema_version != 1 {
        return Err(StoreError::UnsupportedSchema);
    }
    for value in [
        &request.request_id,
        &request.task_id,
        &request.session_id,
        &request.snapshot_id,
    ] {
        validate_inventory_id(value)?;
    }
    safe_text(&request.policy_version, 256)?;
    safe_text(&request.execution_path, 256)?;
    if request.candidate_profile_ids.is_empty()
        || request.candidate_profile_ids.len() > MAX_INVENTORY_PROFILES
        || request.required_capabilities.len() > MAX_CAPABILITIES_PER_PROFILE
        || request.workload_labels.len() > 64
    {
        return Err(StoreError::InvalidInput("request_bounds_out_of_range"));
    }
    if let Some(selected) = &request.selected_profile_id {
        validate_inventory_id(selected)?;
    }
    let mut candidates = std::collections::BTreeSet::new();
    for candidate in &request.candidate_profile_ids {
        validate_inventory_id(candidate)?;
        if !candidates.insert(candidate) {
            return Err(StoreError::InvalidInput("duplicate_candidate"));
        }
    }
    let mut requirements = std::collections::BTreeSet::new();
    for required in &request.required_capabilities {
        validate_inventory_id(&required.capability_id)?;
        safe_text(&required.value, 256)?;
        if !requirements.insert((&required.capability_id, &required.value)) {
            return Err(StoreError::InvalidInput("duplicate_requirement"));
        }
    }
    for label in &request.workload_labels {
        safe_text(&label.name, 128)?;
        safe_text(&label.value, 256)?;
        safe_text(&label.provenance, 256)?;
    }
    Ok(())
}

/// Evaluate only against a separately supplied host state. API callers own trustworthy capture.
pub fn evaluate_eligibility(
    snapshot: &InventorySnapshot,
    request: &EligibilityRequest,
    current: Option<&CurrentState>,
) -> Result<EligibilityResult, StoreError> {
    validate_snapshot(snapshot)?;
    validate_request(request)?;
    if request.snapshot_id != snapshot.snapshot_id {
        return Err(StoreError::Conflict("snapshot_request_mismatch"));
    }
    let by_id: std::collections::BTreeMap<_, _> = snapshot
        .profiles
        .iter()
        .map(|p| (p.profile_id.as_str(), p))
        .collect();
    let selected_unknown = request
        .selected_profile_id
        .as_deref()
        .is_some_and(|id| !by_id.contains_key(id));
    let mut ids = request.candidate_profile_ids.clone();
    if let Some(selected) = &request.selected_profile_id {
        ids = vec![selected.clone()];
    }
    ids.sort();
    let mut candidates = Vec::with_capacity(ids.len());
    for id in ids {
        let Some(profile) = by_id.get(id.as_str()) else {
            candidates.push(CandidateEligibility {
                profile_id: id,
                eligible: false,
                reasons: vec![
                    if selected_unknown {
                        "unknown_selected_profile"
                    } else {
                        "unknown_candidate_profile"
                    }
                    .into(),
                ],
                evidence: Vec::new(),
            });
            continue;
        };
        let mut reasons = Vec::new();
        let mut evidence = Vec::new();
        let Some(state) = current else {
            reasons.push("missing_current_state".into());
            candidates.push(CandidateEligibility {
                profile_id: id,
                eligible: false,
                reasons,
                evidence,
            });
            continue;
        };
        if !valid_timestamp(&state.now) {
            return Err(StoreError::InvalidInput("invalid_current_time"));
        }
        if state.expected_snapshot_id != snapshot.snapshot_id {
            reasons.push("snapshot_mismatch".into());
        }
        if state.now < snapshot.created_at {
            reasons.push("snapshot_from_future".into());
        }
        if state.execution_path != request.execution_path {
            reasons.push("execution_path_mismatch".into());
        }
        if state.policy_version != snapshot.policy_version
            || request.policy_version != snapshot.policy_version
        {
            reasons.push("policy_mismatch".into());
        }
        if state.harness != snapshot.harness || state.harness_version != snapshot.harness_version {
            reasons.push("harness_mismatch".into());
        }
        if state.adapter != snapshot.adapter || state.adapter_version != snapshot.adapter_version {
            reasons.push("adapter_mismatch".into());
        }
        if state.runtime_scope != snapshot.runtime_scope
            || state.session_scope != snapshot.session_scope
            || request.session_id != snapshot.session_scope
        {
            reasons.push("runtime_scope_mismatch".into());
        }
        if state.configuration_fingerprint != snapshot.configuration_fingerprint {
            reasons.push("configuration_mismatch".into());
        }
        if state.workspace_dirty {
            reasons.push("workspace_dirty".into());
        } else if state.workspace_fingerprint != snapshot.workspace_fingerprint {
            reasons.push("workspace_mismatch".into());
        }
        if profile.resolved_model_id.is_none() || profile.resolved_model_version.is_none() {
            reasons.push("unknown_execution_identity".into());
        }
        if !profile.supported_efforts.contains(&profile.effort) {
            reasons.push("unsupported_effort".into());
        }
        for required in &request.required_capabilities {
            let facts: Vec<_> = profile
                .capabilities
                .iter()
                .filter(|fact| {
                    fact.capability_id == required.capability_id && fact.value == required.value
                })
                .collect();
            let mut accepted = false;
            let mut requirement_reason = None;
            for fact in facts {
                if fact.profile_revision != profile.revision
                    || fact.harness_version != state.harness_version
                    || fact.configuration_fingerprint != state.configuration_fingerprint
                    || fact.runtime_scope != state.runtime_scope
                {
                    requirement_reason = Some("unresolved_scope_mismatch");
                    continue;
                }
                if fact.observed_at > state.now {
                    requirement_reason = Some("unresolved_future_evidence");
                    continue;
                }
                if fact.expires_at <= state.now {
                    requirement_reason = Some("unresolved_stale_evidence");
                    continue;
                }
                if fact.status == CapabilityStatus::Unsupported {
                    requirement_reason = Some("excluded_unsupported");
                    continue;
                }
                if fact.status == CapabilityStatus::Configured {
                    requirement_reason = Some("unresolved_configured_only");
                    continue;
                }
                if fact.status != CapabilityStatus::RuntimeConfirmed {
                    requirement_reason.get_or_insert("unresolved_unknown_evidence");
                    continue;
                }
                accepted = true;
                evidence.push(fact.source_ref.clone());
                break;
            }
            if !accepted {
                reasons.push(
                    requirement_reason
                        .unwrap_or("unresolved_missing_evidence")
                        .into(),
                );
            }
        }
        reasons.sort();
        reasons.dedup();
        evidence.sort();
        evidence.dedup();
        candidates.push(CandidateEligibility {
            eligible: reasons.is_empty(),
            profile_id: id,
            reasons,
            evidence,
        });
    }
    let eligible_profile_ids: Vec<_> = candidates
        .iter()
        .filter(|c| c.eligible)
        .map(|c| c.profile_id.clone())
        .collect();
    let outcome = if eligible_profile_ids.is_empty() {
        "no_eligible_candidate"
    } else {
        "eligible_candidates"
    }
    .to_owned();
    Ok(EligibilityResult {
        eligible_profile_ids,
        candidates,
        outcome,
    })
}

fn validate_event(event: &Event) -> Result<(), StoreError> {
    if event.schema_version != EVENT_SCHEMA_VERSION {
        return Err(StoreError::UnsupportedSchema);
    }
    validate_id(&event.source_id)?;
    validate_id(&event.event_id)?;
    if event.parents.len() > 16 {
        return Err(StoreError::InvalidInput("too_many_parents"));
    }
    let json =
        serde_json::to_vec(event).map_err(|_| StoreError::InvalidInput("malformed_event"))?;
    if json.len() > MAX_EVENT_BYTES {
        return Err(StoreError::InvalidInput("event_too_large"));
    }
    if json_depth(&json) > MAX_JSON_DEPTH {
        return Err(StoreError::InvalidInput("event_too_deep"));
    }
    let text =
        std::str::from_utf8(&json).map_err(|_| StoreError::InvalidInput("malformed_event"))?;
    if sensitive(text) {
        return Err(StoreError::InvalidInput("sensitive_input_rejected"));
    }
    for p in &event.parents {
        validate_id(&p.source_id)?;
        validate_id(&p.event_id)?;
    }
    let (kind, sid, tid, aid) = event.kind.columns();
    let _ = kind;
    for id in sid.into_iter().chain(tid).chain(aid) {
        validate_id(id)?;
    }
    match &event.kind {
        EventKind::SessionOpened { owner_id, .. } | EventKind::SessionClosed { owner_id, .. } => {
            validate_id(owner_id)?
        }
        EventKind::SessionRecovered {
            expected_owner_id,
            actor_id,
            reason,
            evidence_ref,
            ..
        } => {
            validate_id(expected_owner_id)?;
            validate_id(actor_id)?;
            validate_text(reason, 256)?;
            validate_evidence(evidence_ref)?;
        }
        EventKind::TaskCreated {
            required_check_ids, ..
        } => {
            if required_check_ids.len() > 32 {
                return Err(StoreError::InvalidInput("too_many_required_checks"));
            }
            for id in required_check_ids {
                validate_id(id)?;
            }
        }
        EventKind::AttemptIntent {
            predecessor_attempt_id,
            span_id,
            selected,
            ..
        } => {
            if let Some(id) = predecessor_attempt_id {
                validate_id(id)?;
            }
            validate_id(span_id)?;
            validate_identity(&selected.profile)?;
            validate_identity(&selected.model)?;
            validate_identity(&selected.model_version)?;
            validate_identity(&selected.effort)?;
        }
        EventKind::AttemptResult {
            observed,
            evidence_ref,
            ..
        } => {
            validate_observed(observed)?;
            if let Some(r) = evidence_ref {
                validate_evidence(r)?;
            }
        }
        EventKind::ToolResult {
            tool_id,
            operation_id,
            failure_category,
            result,
            evidence_ref,
            ..
        } => {
            validate_id(tool_id)?;
            validate_id(operation_id)?;
            if (*result == ToolOutcome::Failure) != failure_category.is_some() {
                return Err(StoreError::InvalidInput("failure_category_mismatch"));
            }
            if let Some(r) = evidence_ref {
                validate_evidence(r)?;
            }
        }
        EventKind::UsageCost {
            usage,
            cost,
            elapsed_ms,
            ..
        } => {
            validate_component(usage)?;
            validate_component(cost)?;
            validate_component(elapsed_ms)?;
        }
        EventKind::Verification {
            check_id,
            evidence_ref,
            ..
        } => {
            validate_id(check_id)?;
            validate_evidence(evidence_ref)?;
        }
        EventKind::TaskAccepted {
            actor_id,
            evidence_ref,
            ..
        } => {
            validate_id(actor_id)?;
            validate_evidence(evidence_ref)?;
        }
        EventKind::Interrupted { reason, .. } => validate_text(reason, 256)?,
    }
    Ok(())
}
fn validate_identity(v: &str) -> Result<(), StoreError> {
    validate_text(v, 128)?;
    if v.trim().is_empty() {
        return Err(StoreError::InvalidInput("empty_identity"));
    }
    Ok(())
}
fn validate_component(v: &ComponentValue) -> Result<(), StoreError> {
    if let ComponentValue::Known { unit, .. } = v {
        validate_identity(unit)?;
    }
    if let ComponentValue::Unknown { reason } = v {
        validate_text(reason, 256)?;
    }
    Ok(())
}
fn validate_observed(v: &ObservedIdentity) -> Result<(), StoreError> {
    match v {
        ObservedIdentity::Known {
            profile,
            model,
            model_version,
            effort,
        } => {
            validate_identity(profile)?;
            validate_identity(model)?;
            validate_identity(model_version)?;
            validate_identity(effort)?;
        }
        ObservedIdentity::Unknown { reason } => validate_text(reason, 256)?,
    }
    Ok(())
}
fn validate_id(v: &str) -> Result<(), StoreError> {
    if v.is_empty()
        || v.len() > 128
        || !v
            .bytes()
            .all(|b| b.is_ascii_alphanumeric() || b"._:-".contains(&b))
        || !v.as_bytes()[0].is_ascii_alphanumeric()
    {
        return Err(StoreError::InvalidInput("invalid_identifier"));
    }
    Ok(())
}
fn validate_text(v: &str, max: usize) -> Result<(), StoreError> {
    if v.is_empty() || v.len() > max || v.chars().any(char::is_control) {
        return Err(StoreError::InvalidInput("invalid_text"));
    }
    Ok(())
}
fn validate_evidence(v: &str) -> Result<(), StoreError> {
    validate_text(v, 512)?;
    if v.starts_with('/')
        || v.contains('\\')
        || v.contains(':')
        || v.split('/').any(|p| p.is_empty() || p == "." || p == "..")
        || v.split('/').count() > 8
    {
        return Err(StoreError::InvalidInput("invalid_evidence_reference"));
    }
    Ok(())
}
fn sensitive(v: &str) -> bool {
    let l = v.to_ascii_lowercase();
    [
        "api_key",
        "apikey",
        "access_token",
        "refresh_token",
        "password",
        "private_key",
        "bearer ",
        "authorization",
        "secret",
    ]
    .iter()
    .any(|p| l.contains(p))
}
fn json_depth(bytes: &[u8]) -> usize {
    let mut depth: usize = 0;
    let mut max = 0;
    let mut string = false;
    let mut escape = false;
    for b in bytes {
        if string {
            if escape {
                escape = false;
            } else if *b == b'\\' {
                escape = true;
            } else if *b == b'"' {
                string = false;
            }
            continue;
        }
        match *b {
            b'"' => string = true,
            b'{' | b'[' => {
                depth += 1;
                max = max.max(depth);
            }
            b'}' | b']' => depth = depth.saturating_sub(1),
            _ => {}
        }
    }
    max
}

fn validate_parents(tx: &Transaction<'_>, event: &Event) -> Result<(), StoreError> {
    if event.parents.is_empty() && !matches!(&event.kind, EventKind::SessionOpened { .. }) {
        return Err(StoreError::InvalidInput("parent_link_required"));
    }
    for parent in &event.parents {
        let row: Option<(Option<String>, Option<String>)> = tx
            .query_row(
                "SELECT session_id,task_id FROM events WHERE source_id=?1 AND event_id=?2",
                params![parent.source_id, parent.event_id],
                |r| Ok((r.get(0)?, r.get(1)?)),
            )
            .optional()?;
        let Some((sid, tid)) = row else {
            return Err(StoreError::NotFound("parent_event"));
        };
        let (_, child_sid, child_tid, _) = event.kind.columns();
        if let Some(cs) = child_sid
            && sid.as_deref() != Some(cs)
        {
            return Err(StoreError::Conflict("parent_session_mismatch"));
        }
        if let Some(ct) = child_tid
            && !matches!(&event.kind, EventKind::TaskCreated { .. })
            && tid.as_deref() != Some(ct)
        {
            return Err(StoreError::Conflict("parent_task_mismatch"));
        }
    }
    Ok(())
}

fn validate_scope_and_claim(tx: &Transaction<'_>, event: &Event) -> Result<(), StoreError> {
    match &event.kind {
        EventKind::SessionOpened {
            session_id,
            owner_id,
        } => {
            if owner_id != &event.source_id {
                return Err(StoreError::Conflict("owner_must_match_source"));
            }
            let exists: bool = tx.query_row(
                "SELECT EXISTS(SELECT 1 FROM sessions WHERE session_id=?1)",
                [session_id],
                |r| r.get(0),
            )?;
            if exists {
                return Err(StoreError::Conflict("session_already_claimed"));
            }
        }
        EventKind::SessionRecovered {
            session_id,
            expected_owner_id,
            actor_id,
            ..
        } => {
            if expected_owner_id == actor_id {
                return Err(StoreError::Conflict("recovery_requires_new_actor"));
            }
            if actor_id != &event.source_id {
                return Err(StoreError::Conflict("recovery_actor_must_match_source"));
            }
            let owner: Option<(String, String)> = tx
                .query_row(
                    "SELECT owner_id,state FROM sessions WHERE session_id=?1",
                    [session_id],
                    |r| Ok((r.get(0)?, r.get(1)?)),
                )
                .optional()?;
            match owner {
                Some((current, state)) if current == *expected_owner_id && state == "active" => {}
                Some((current, state)) if current == *actor_id && state == "recoverable" => {}
                Some(_) => return Err(StoreError::Conflict("recovery_owner_mismatch")),
                None => return Err(StoreError::NotFound("session")),
            }
        }
        kind => {
            let (_, sid, _, _) = kind.columns();
            if let Some(sid) = sid {
                let owner: Option<(String, String)> = tx
                    .query_row(
                        "SELECT owner_id,state FROM sessions WHERE session_id=?1",
                        [sid],
                        |r| Ok((r.get(0)?, r.get(1)?)),
                    )
                    .optional()?;
                let Some((owner, state)) = owner else {
                    return Err(StoreError::NotFound("session"));
                };
                let actor = match kind {
                    EventKind::SessionClosed { owner_id, .. } => owner_id,
                    _ => &event.source_id,
                };
                if owner != *actor || event.source_id != owner {
                    return Err(StoreError::Conflict("session_owned_by_another_actor"));
                }
                if state == "closed" {
                    return Err(StoreError::Conflict("session_closed"));
                }
            }
        }
    }
    Ok(())
}

fn validate_kind_state(tx: &Transaction<'_>, event: &Event) -> Result<(), StoreError> {
    match &event.kind {
        EventKind::TaskCreated {
            session_id,
            task_id,
            ..
        } => {
            let exists: bool = tx.query_row("SELECT EXISTS(SELECT 1 FROM events WHERE kind='TaskCreated' AND session_id=?1 AND task_id=?2)", params![session_id, task_id], |r| r.get(0))?;
            if exists {
                return Err(StoreError::Conflict("task_already_exists"));
            }
        }
        EventKind::AttemptIntent {
            session_id,
            task_id,
            attempt_id: _,
            predecessor_attempt_id,
            ..
        } => {
            let exists: bool = tx.query_row("SELECT EXISTS(SELECT 1 FROM events WHERE kind='TaskCreated' AND session_id=?1 AND task_id=?2)", params![session_id, task_id], |r| r.get(0))?;
            if !exists {
                return Err(StoreError::NotFound("task"));
            }
            let accepted: bool = tx.query_row(
                "SELECT EXISTS(SELECT 1 FROM events WHERE kind='TaskAccepted' AND session_id=?1 AND task_id=?2)",
                params![session_id, task_id],
                |r| r.get(0),
            )?;
            if accepted {
                return Err(StoreError::Conflict("task_already_accepted"));
            }
            if let Some(previous) = predecessor_attempt_id {
                let exists: bool = tx.query_row(
                    "SELECT EXISTS(SELECT 1 FROM attempts WHERE attempt_id=?1 AND task_id=?2 AND session_id=?3)",
                    params![previous, task_id, session_id],
                    |r| r.get(0),
                )?;
                if !exists {
                    return Err(StoreError::NotFound("predecessor_attempt"));
                }
            }
        }
        EventKind::AttemptResult {
            session_id,
            task_id,
            attempt_id,
            ..
        }
        | EventKind::ToolResult {
            session_id,
            task_id,
            attempt_id,
            ..
        }
        | EventKind::UsageCost {
            session_id,
            task_id,
            attempt_id,
            ..
        }
        | EventKind::Verification {
            session_id,
            task_id,
            attempt_id,
            ..
        }
        | EventKind::Interrupted {
            session_id,
            task_id,
            attempt_id,
            ..
        } => {
            let exists: bool = tx.query_row("SELECT EXISTS(SELECT 1 FROM attempts WHERE attempt_id=?1 AND session_id=?2 AND task_id=?3)", params![attempt_id,session_id,task_id], |r| r.get(0))?;
            if !exists {
                return Err(StoreError::NotFound("attempt"));
            }
        }
        EventKind::SessionClosed {
            session_id,
            owner_id,
        } => {
            let current: Option<String> = tx
                .query_row(
                    "SELECT owner_id FROM sessions WHERE session_id=?1",
                    [session_id],
                    |r| r.get(0),
                )
                .optional()?;
            if current.as_deref() != Some(owner_id) {
                return Err(StoreError::Conflict("session_owner_mismatch"));
            }
            let mut statement = tx.prepare(
                "SELECT (SELECT event_json FROM events WHERE attempt_id=a.attempt_id AND kind='AttemptResult' ORDER BY sequence DESC LIMIT 1) FROM attempts a WHERE a.session_id=?1",
            )?;
            let rows = statement.query_map([session_id], |r| r.get::<_, Option<String>>(0))?;
            let latest_results = rows.collect::<Result<Vec<_>, _>>()?;
            drop(statement);
            for raw in latest_results {
                let Some(raw) = raw else {
                    return Err(StoreError::Conflict("session_has_unresolved_attempts"));
                };
                let result: Event = serde_json::from_str(&raw)
                    .map_err(|_| StoreError::InvalidInput("stored_event_invalid"))?;
                if matches!(
                    result.kind,
                    EventKind::AttemptResult {
                        outcome: AttemptOutcome::Unknown,
                        ..
                    }
                ) {
                    return Err(StoreError::Conflict("session_has_unresolved_attempts"));
                }
            }
        }
        EventKind::TaskAccepted {
            session_id,
            task_id,
            attempt_id,
            ..
        } => {
            let latest: Option<String> = tx
                .query_row(
                    "SELECT attempt_id FROM attempts WHERE task_id=?1 AND session_id=?2 ORDER BY rowid DESC LIMIT 1",
                    params![task_id, session_id],
                    |r| r.get(0),
                )
                .optional()?;
            if latest.as_deref() != Some(attempt_id) {
                return Err(StoreError::Conflict("acceptance_attempt_is_stale"));
            }
            let task_json: Option<String> = tx.query_row("SELECT event_json FROM events WHERE kind='TaskCreated' AND session_id=?1 AND task_id=?2 ORDER BY sequence LIMIT 1", params![session_id, task_id], |r| r.get(0)).optional()?;
            let Some(task_json) = task_json else {
                return Err(StoreError::NotFound("task"));
            };
            let task: Event = serde_json::from_str(&task_json)
                .map_err(|_| StoreError::InvalidInput("stored_event_invalid"))?;
            let required = match task.kind {
                EventKind::TaskCreated {
                    required_check_ids, ..
                } => required_check_ids,
                _ => Vec::new(),
            };
            let mut statement = tx.prepare(
                "SELECT sequence,event_json FROM events WHERE attempt_id=?1 ORDER BY sequence",
            )?;
            let rows = statement.query_map([attempt_id], |r| {
                Ok((r.get::<_, i64>(0)?, r.get::<_, String>(1)?))
            })?;
            let mut latest_failure = 0_i64;
            let mut latest_success = 0_i64;
            let mut latest_attempt_result = None;
            for row in rows {
                let (sequence, raw) = row?;
                let attempt_event: Event = serde_json::from_str(&raw)
                    .map_err(|_| StoreError::InvalidInput("stored_event_invalid"))?;
                match attempt_event.kind {
                    EventKind::AttemptResult { outcome, .. } => {
                        latest_attempt_result = Some(outcome);
                        match outcome {
                            AttemptOutcome::Failure => latest_failure = sequence,
                            AttemptOutcome::Success => latest_success = sequence,
                            AttemptOutcome::Unknown | AttemptOutcome::NotRun => {}
                        }
                    }
                    EventKind::ToolResult {
                        result: ToolOutcome::Failure,
                        ..
                    } => latest_failure = sequence,
                    _ => {}
                }
            }
            drop(statement);
            let succeeded = latest_success > latest_failure
                && matches!(latest_attempt_result, Some(AttemptOutcome::Success));
            if !succeeded {
                return Err(StoreError::Conflict("successful_attempt_result_missing"));
            }
            for check in required {
                let mut statement = tx.prepare("SELECT sequence,event_json FROM events WHERE kind='Verification' AND task_id=?1 AND attempt_id=?2 ORDER BY sequence DESC")?;
                let rows = statement.query_map(params![task_id, attempt_id], |r| {
                    Ok((r.get::<_, i64>(0)?, r.get::<_, String>(1)?))
                })?;
                let mut passed = false;
                for row in rows {
                    let (sequence, raw) = row?;
                    if sequence <= latest_failure {
                        continue;
                    }
                    let verification: Event = serde_json::from_str(&raw)
                        .map_err(|_| StoreError::InvalidInput("stored_event_invalid"))?;
                    if let EventKind::Verification {
                        check_id,
                        passed: value,
                        ..
                    } = verification.kind
                        && check_id == check
                    {
                        passed = value;
                        break;
                    }
                }
                if !passed {
                    return Err(StoreError::Conflict(
                        "required_verification_missing_or_failed",
                    ));
                }
            }
        }
        _ => {}
    }
    Ok(())
}

fn apply_claim(tx: &Transaction<'_>, event: &Event) -> Result<(), StoreError> {
    match &event.kind {
        EventKind::SessionOpened {
            session_id,
            owner_id,
        } => {
            tx.execute(
                "INSERT INTO sessions(session_id,owner_id,state) VALUES(?1,?2,'active')",
                params![session_id, owner_id],
            )?;
        }
        EventKind::SessionClosed { session_id, .. } => {
            tx.execute(
                "UPDATE sessions SET state='closed' WHERE session_id=?1",
                [session_id],
            )?;
        }
        EventKind::SessionRecovered {
            session_id,
            actor_id,
            ..
        } => {
            tx.execute(
                "UPDATE sessions SET owner_id=?2,state='recoverable' WHERE session_id=?1",
                params![session_id, actor_id],
            )?;
        }
        EventKind::AttemptIntent {
            session_id,
            task_id,
            attempt_id,
            predecessor_attempt_id,
            selected,
            ..
        } => {
            let selection = serde_json::to_string(selected)
                .map_err(|_| StoreError::InvalidInput("malformed_event"))?;
            tx.execute("INSERT INTO attempts(attempt_id,session_id,task_id,predecessor_attempt_id,intent_source_id,intent_event_id,selected_json) VALUES(?1,?2,?3,?4,?5,?6,?7)", params![attempt_id, session_id, task_id, predecessor_attempt_id, event.source_id, event.event_id, selection])?;
        }
        _ => {}
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_snapshot(status: CapabilityStatus) -> InventorySnapshot {
        InventorySnapshot {
            schema_version: 1,
            snapshot_id: "snap-a".into(),
            revision: "rev-a".into(),
            policy_version: "policy-a".into(),
            created_at: "2026-10-25T10:00:00Z".into(),
            harness: "codex".into(),
            harness_version: "1".into(),
            adapter: "native".into(),
            adapter_version: "1".into(),
            runtime_scope: "runtime-a".into(),
            session_scope: "session-a".into(),
            configuration_fingerprint: "config-a".into(),
            workspace_fingerprint: "workspace-a".into(),
            profiles: vec![InventoryProfile {
                profile_id: "profile-a".into(),
                revision: "profile-rev-a".into(),
                native_role: "worker".into(),
                model_provider: "provider-a".into(),
                configured_model_id: "alias-a".into(),
                resolved_model_id: Some("model-a".into()),
                resolved_model_version: Some("version-a".into()),
                effort: "high".into(),
                supported_efforts: vec!["high".into()],
                capabilities: vec![CapabilityFact {
                    capability_id: "cap-a".into(),
                    value: "value-a".into(),
                    status,
                    source_ref: "evidence/cap-a".into(),
                    observed_at: "2026-10-25T10:00:00Z".into(),
                    expires_at: "2026-10-26T10:00:00Z".into(),
                    profile_revision: "profile-rev-a".into(),
                    harness_version: "1".into(),
                    configuration_fingerprint: "config-a".into(),
                    runtime_scope: "runtime-a".into(),
                }],
            }],
        }
    }

    fn sample_request() -> EligibilityRequest {
        EligibilityRequest {
            schema_version: 1,
            request_id: "request-a".into(),
            task_id: "task-a".into(),
            session_id: "session-a".into(),
            snapshot_id: "snap-a".into(),
            policy_version: "policy-a".into(),
            candidate_profile_ids: vec!["profile-a".into()],
            execution_path: "native".into(),
            selected_profile_id: None,
            required_capabilities: vec![RequiredCapability {
                capability_id: "cap-a".into(),
                value: "value-a".into(),
            }],
            workload_labels: vec![WorkloadLabel {
                name: "unknown-label".into(),
                value: "preserved".into(),
                provenance: "user".into(),
            }],
        }
    }

    fn sample_current() -> CurrentState {
        CurrentState {
            now: "2026-10-25T11:00:00Z".into(),
            execution_path: "native".into(),
            expected_snapshot_id: "snap-a".into(),
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
    fn inventory_schema_size_privacy_and_duplicate_keys_are_rejected() {
        let snapshot = sample_snapshot(CapabilityStatus::RuntimeConfirmed);
        let bytes = serde_json::to_vec(&snapshot).unwrap();
        assert_eq!(parse_inventory_bytes(&bytes).unwrap(), snapshot);
        let mut unknown: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
        unknown["unrecognized"] = serde_json::json!("x");
        assert!(parse_inventory_bytes(&serde_json::to_vec(&unknown).unwrap()).is_err());
        let oversized = vec![b' '; MAX_INVENTORY_BYTES + 1];
        assert!(parse_inventory_bytes(&oversized).is_err());
        let mut duplicate = sample_snapshot(CapabilityStatus::RuntimeConfirmed);
        duplicate.profiles.push(duplicate.profiles[0].clone());
        assert!(validate_snapshot(&duplicate).is_err());
        let mut private = sample_snapshot(CapabilityStatus::RuntimeConfirmed);
        private.profiles[0].configured_model_id = "password-fixture".into();
        assert!(validate_snapshot(&private).is_err());
        let mut invalid_date = sample_snapshot(CapabilityStatus::RuntimeConfirmed);
        invalid_date.created_at = "2026-02-31T10:00:00Z".into();
        assert!(validate_snapshot(&invalid_date).is_err());
        let mut absolute_reference = sample_snapshot(CapabilityStatus::RuntimeConfirmed);
        absolute_reference.profiles[0].capabilities[0].source_ref = "C:/private/evidence".into();
        assert!(validate_snapshot(&absolute_reference).is_err());
    }

    #[test]
    fn inventory_import_replay_conflict_and_reopen_are_immutable() {
        let mut store = Store::memory().unwrap();
        let snapshot = sample_snapshot(CapabilityStatus::RuntimeConfirmed);
        assert!(store.import_inventory(snapshot.clone()).unwrap());
        assert!(!store.import_inventory(snapshot.clone()).unwrap());
        let mut changed = snapshot.clone();
        changed.revision = "rev-b".into();
        assert_eq!(
            store.import_inventory(changed),
            Err(StoreError::Conflict("snapshot_identity_reused"))
        );
        assert_eq!(store.inventory("snap-a").unwrap(), snapshot);
        let count: i64 = store
            .connection
            .query_row("SELECT count(*) FROM inventory_snapshots", [], |row| {
                row.get(0)
            })
            .unwrap();
        assert_eq!(count, 1);

        let existing = Connection::open_in_memory().unwrap();
        existing
            .execute_batch(
                "CREATE TABLE events(event_id TEXT); INSERT INTO events VALUES('retained');",
            )
            .unwrap();
        existing
            .pragma_update(None, "user_version", DATABASE_SCHEMA_VERSION)
            .unwrap();
        let migrated = Store::from_connection(existing).unwrap();
        let retained: String = migrated
            .connection
            .query_row("SELECT event_id FROM events", [], |row| row.get(0))
            .unwrap();
        assert_eq!(retained, "retained");
    }

    #[test]
    fn eligibility_requires_separate_current_state_and_exact_confirmed_scope() {
        let snapshot = sample_snapshot(CapabilityStatus::RuntimeConfirmed);
        let request = sample_request();
        let missing = evaluate_eligibility(&snapshot, &request, None).unwrap();
        assert_eq!(missing.candidates[0].reasons, vec!["missing_current_state"]);
        let result = evaluate_eligibility(&snapshot, &request, Some(&sample_current())).unwrap();
        assert_eq!(result.eligible_profile_ids, vec!["profile-a"]);
        assert_eq!(result.candidates[0].evidence, vec!["evidence/cap-a"]);
    }

    #[test]
    fn capability_statuses_and_evidence_freshness_have_explicit_reasons() {
        for (status, expected) in [
            (CapabilityStatus::Configured, "unresolved_configured_only"),
            (CapabilityStatus::Unsupported, "excluded_unsupported"),
            (CapabilityStatus::Unknown, "unresolved_unknown_evidence"),
        ] {
            let snapshot = sample_snapshot(status);
            let result =
                evaluate_eligibility(&snapshot, &sample_request(), Some(&sample_current()))
                    .unwrap();
            assert_eq!(result.candidates[0].reasons, vec![expected]);
        }
        let mut stale = sample_snapshot(CapabilityStatus::RuntimeConfirmed);
        stale.profiles[0].capabilities[0].expires_at = "2026-10-25T11:00:00Z".into();
        let result =
            evaluate_eligibility(&stale, &sample_request(), Some(&sample_current())).unwrap();
        assert_eq!(
            result.candidates[0].reasons,
            vec!["unresolved_stale_evidence"]
        );
        let mut future = sample_snapshot(CapabilityStatus::RuntimeConfirmed);
        future.profiles[0].capabilities[0].observed_at = "2026-10-25T12:00:00Z".into();
        let result =
            evaluate_eligibility(&future, &sample_request(), Some(&sample_current())).unwrap();
        assert_eq!(
            result.candidates[0].reasons,
            vec!["unresolved_future_evidence"]
        );
    }

    #[test]
    fn capability_statuses_require_applicability_before_classification() {
        let statuses = [
            (
                CapabilityStatus::Configured,
                Some("unresolved_configured_only"),
            ),
            (CapabilityStatus::Unsupported, Some("excluded_unsupported")),
            (
                CapabilityStatus::Unknown,
                Some("unresolved_unknown_evidence"),
            ),
            (CapabilityStatus::RuntimeConfirmed, None),
        ];
        for (status, status_reason) in statuses {
            let applicable = sample_snapshot(status);
            let baseline =
                evaluate_eligibility(&applicable, &sample_request(), Some(&sample_current()))
                    .unwrap();
            if let Some(reason) = status_reason {
                assert_eq!(baseline.candidates[0].reasons, vec![reason]);
            } else {
                assert!(baseline.candidates[0].eligible);
            }

            for mismatch in [
                "profile_revision",
                "harness_version",
                "configuration_fingerprint",
                "runtime_scope",
            ] {
                let mut snapshot = sample_snapshot(status);
                let fact = &mut snapshot.profiles[0].capabilities[0];
                match mismatch {
                    "profile_revision" => fact.profile_revision = "other-revision".into(),
                    "harness_version" => fact.harness_version = "other-harness".into(),
                    "configuration_fingerprint" => {
                        fact.configuration_fingerprint = "other-config".into()
                    }
                    "runtime_scope" => fact.runtime_scope = "other-runtime".into(),
                    _ => unreachable!(),
                }
                let result =
                    evaluate_eligibility(&snapshot, &sample_request(), Some(&sample_current()))
                        .unwrap();
                assert!(
                    result.candidates[0]
                        .reasons
                        .contains(&"unresolved_scope_mismatch".into()),
                    "status {status:?} mismatch {mismatch}"
                );
                if let Some(reason) = status_reason {
                    assert!(!result.candidates[0].reasons.contains(&reason.into()));
                }
            }

            let mut future = sample_snapshot(status);
            future.profiles[0].capabilities[0].observed_at = "2026-10-25T12:00:00Z".into();
            future.profiles[0].capabilities[0].expires_at = "2026-10-25T13:00:00Z".into();
            let result =
                evaluate_eligibility(&future, &sample_request(), Some(&sample_current())).unwrap();
            assert!(
                result.candidates[0]
                    .reasons
                    .contains(&"unresolved_future_evidence".into()),
                "status {status:?} future observation"
            );
            if let Some(reason) = status_reason {
                assert!(!result.candidates[0].reasons.contains(&reason.into()));
            }

            let mut stale = sample_snapshot(status);
            stale.profiles[0].capabilities[0].expires_at = "2026-10-25T11:00:00Z".into();
            let result =
                evaluate_eligibility(&stale, &sample_request(), Some(&sample_current())).unwrap();
            assert!(
                result.candidates[0]
                    .reasons
                    .contains(&"unresolved_stale_evidence".into()),
                "status {status:?} stale expiry"
            );
            if let Some(reason) = status_reason {
                assert!(!result.candidates[0].reasons.contains(&reason.into()));
            }
        }
    }

    #[test]
    fn eligibility_rejects_stale_state_dirty_workspace_and_never_substitutes_selection() {
        let snapshot = sample_snapshot(CapabilityStatus::RuntimeConfirmed);
        let mut current = sample_current();
        current.configuration_fingerprint = "different".into();
        current.workspace_dirty = true;
        let mut request = sample_request();
        request.selected_profile_id = Some("unknown-profile".into());
        let result = evaluate_eligibility(&snapshot, &request, Some(&current)).unwrap();
        assert_eq!(result.candidates.len(), 1);
        assert_eq!(result.candidates[0].profile_id, "unknown-profile");
        assert_eq!(
            result.candidates[0].reasons,
            vec!["unknown_selected_profile"]
        );
        let mut request = sample_request();
        request.candidate_profile_ids.push("profile-b".into());
        let mut multi = snapshot.clone();
        let mut second = multi.profiles[0].clone();
        second.profile_id = "profile-b".into();
        second.revision = "profile-rev-b".into();
        second.capabilities[0].profile_revision = "profile-rev-b".into();
        multi.profiles.push(second);
        let result = evaluate_eligibility(&multi, &request, Some(&sample_current())).unwrap();
        assert_eq!(
            result
                .candidates
                .iter()
                .map(|candidate| candidate.profile_id.as_str())
                .collect::<Vec<_>>(),
            vec!["profile-a", "profile-b"]
        );
        request.candidate_profile_ids.reverse();
        assert_eq!(
            result,
            evaluate_eligibility(&multi, &request, Some(&sample_current())).unwrap()
        );

        let mut configured = sample_snapshot(CapabilityStatus::Configured);
        let mut eligible_alternative = configured.profiles[0].clone();
        eligible_alternative.profile_id = "profile-b".into();
        eligible_alternative.revision = "profile-rev-b".into();
        eligible_alternative.capabilities[0].status = CapabilityStatus::RuntimeConfirmed;
        eligible_alternative.capabilities[0].profile_revision = "profile-rev-b".into();
        configured.profiles.push(eligible_alternative);
        let mut selected = sample_request();
        selected.selected_profile_id = Some("profile-a".into());
        selected.candidate_profile_ids.push("profile-b".into());
        let selected_result =
            evaluate_eligibility(&configured, &selected, Some(&sample_current())).unwrap();
        assert_eq!(selected_result.candidates.len(), 1);
        assert_eq!(selected_result.candidates[0].profile_id, "profile-a");
        assert_eq!(
            selected_result.candidates[0].reasons,
            vec!["unresolved_configured_only"]
        );

        let mut request = sample_request();
        let result = evaluate_eligibility(&snapshot, &request, Some(&current)).unwrap();
        assert!(
            result.candidates[0]
                .reasons
                .contains(&"configuration_mismatch".into())
        );
        assert!(
            result.candidates[0]
                .reasons
                .contains(&"workspace_dirty".into())
        );
        request.candidate_profile_ids.reverse();
        assert_eq!(
            result,
            evaluate_eligibility(&snapshot, &request, Some(&current)).unwrap()
        );
        let mut path_mismatch = sample_current();
        path_mismatch.execution_path = "different".into();
        let result =
            evaluate_eligibility(&snapshot, &sample_request(), Some(&path_mismatch)).unwrap();
        assert!(
            result.candidates[0]
                .reasons
                .contains(&"execution_path_mismatch".into())
        );
        path_mismatch.now = "2026-10-25T09:00:00Z".into();
        let result =
            evaluate_eligibility(&snapshot, &sample_request(), Some(&path_mismatch)).unwrap();
        assert!(
            result.candidates[0]
                .reasons
                .contains(&"snapshot_from_future".into())
        );
    }

    #[test]
    fn unknown_fields_and_schema_versions_are_rejected() {
        let raw = r#"{"schema_version":1,"source_id":"s1","event_id":"e1","extra":"x","kind":{"type":"SessionOpened","data":{"session_id":"ss","owner_id":"s1"}}}"#;
        assert!(serde_json::from_str::<Event>(raw).is_err());
        assert_eq!(
            validate_event(&Event {
                schema_version: 8,
                source_id: "s1".into(),
                event_id: "e1".into(),
                parents: vec![],
                kind: EventKind::SessionOpened {
                    session_id: "ss".into(),
                    owner_id: "s1".into()
                }
            }),
            Err(StoreError::UnsupportedSchema)
        );
    }

    #[test]
    fn event_size_and_depth_are_bounded() {
        let huge = Event::new(
            "s1",
            "e1",
            EventKind::Interrupted {
                session_id: "ss".into(),
                task_id: "tt".into(),
                attempt_id: "aa".into(),
                reason: "x".repeat(MAX_EVENT_BYTES),
            },
        );
        assert!(validate_event(&huge).is_err());
        assert!(json_depth(b"[[[[[[[[[[[[[[[[[0]]]]]]]]]]]]]]]]]") > MAX_JSON_DEPTH);
        let nested = format!(
            "{}0{}",
            "[".repeat(MAX_JSON_DEPTH + 1),
            "]".repeat(MAX_JSON_DEPTH + 1)
        );
        let raw = format!(
            r#"{{"schema_version":1,"source_id":"s1","event_id":"e1","kind":{{"type":"SessionOpened","data":{{"session_id":"ss","owner_id":"s1"}}}},"extra":{nested}}}"#
        );
        assert!(matches!(
            parse_event_bytes(raw.as_bytes()),
            Err(StoreError::InvalidInput("event_too_deep"))
        ));
    }

    #[test]
    fn session_open_is_a_durable_owner_claim() {
        let mut store = Store::memory().unwrap();
        store
            .append(Event::new(
                "owner1",
                "open",
                EventKind::SessionOpened {
                    session_id: "session1".into(),
                    owner_id: "owner1".into(),
                },
            ))
            .unwrap();
        assert!(matches!(
            store.append(Event::new(
                "owner2",
                "open2",
                EventKind::SessionOpened {
                    session_id: "session1".into(),
                    owner_id: "owner2".into()
                }
            )),
            Err(StoreError::Conflict(_))
        ));
    }

    #[test]
    fn unknown_identity_and_cost_are_explicit_values() {
        let identity = ObservedIdentity::Unknown {
            reason: "provider receipt absent".into(),
        };
        let cost = ComponentValue::Unknown {
            reason: "billing data absent".into(),
        };
        assert!(matches!(identity, ObservedIdentity::Unknown { .. }));
        assert!(matches!(cost, ComponentValue::Unknown { .. }));
    }

    #[test]
    fn unsupported_database_schema_is_not_opened_for_migration() {
        let connection = Connection::open_in_memory().unwrap();
        connection
            .pragma_update(None, "user_version", DATABASE_SCHEMA_VERSION + 1)
            .unwrap();
        assert!(matches!(
            Store::from_connection(connection),
            Err(StoreError::UnsupportedSchema)
        ));
    }
}
