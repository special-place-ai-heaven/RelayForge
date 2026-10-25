```diff
git diff 1a82d197ca5f87273d504416b199402832e1ca80..HEAD --stat (selected paths)
 02_analysis/accounting-results.md                |   44 +
 02_analysis/inventory-results.md                 |   31 +
 07_app/relayforge-cli/src/main.rs                |   89 +-
 08_pkg/relayforge-core/src/accounting.rs         | 2385 ++++++++++++++++++++++
 08_pkg/relayforge-core/src/lib.rs                | 1006 +++++++++
 08_pkg/relayforge-core/tests/storage_contract.rs |  377 +++-
 scripts/product_verify.py                        |   18 +-
 7 files changed, 3944 insertions(+), 6 deletions(-)

Per-slice diffs: current working tree against accepted base (bounded; complete manifests remain authoritative).
### M005-S01

diff --git a/07_app/relayforge-cli/src/main.rs b/07_app/relayforge-cli/src/main.rs
index 31b1bdb..239b0e2 100644
--- a/07_app/relayforge-cli/src/main.rs
+++ b/07_app/relayforge-cli/src/main.rs
@@ -1,4 +1,8 @@
-use relayforge_core::{Event, open, parse_event_bytes};
+use relayforge_core::{
+    AccountingRequest, Event, MAX_INVENTORY_BYTES, evaluate_eligibility, open,
+    parse_comparison_request_bytes, parse_eligibility_request_bytes, parse_event_bytes,
+    parse_inventory_bytes, parse_task_context_bytes,
+};
 use rusqlite::Connection;
 use serde_json::json;
 use std::env;
@@ -54,6 +58,20 @@ fn read_event() -> Result<Event, String> {
     }
     parse_event_bytes(&input).map_err(|e| e.to_string())
 }
+fn read_json_file(path: &str) -> Result<Vec<u8>, String> {
+    let file = std::fs::File::open(path).map_err(|_| "input_read_failed")?;
+    let mut bytes = Vec::new();
+    file.take((MAX_INVENTORY_BYTES + 1) as u64)
+        .read_to_end(&mut bytes)
+        .map_err(|_| "input_read_failed")?;
+    if bytes.is_empty() || bytes.len() > MAX_INVENTORY_BYTES {
+        return Err("input_size_out_of_range".into());
+    }
+    Ok(bytes)
+}
+fn db_path(args: &[String]) -> Result<PathBuf, String> {
+    Ok(PathBuf::from(required(args, "--db")?))
+}
 fn run() -> Result<serde_json::Value, String> {
     let args: Vec<String> = env::args().skip(1).collect();
     let command = args.first().map(String::as_str).unwrap_or("");
@@ -104,6 +122,73 @@ fn run() -> Result<serde_json::Value, String> {
                 .map_err(|e| e.to_string())?;
             Ok(json!({"ok":true,"count":events.len(),"events":events}))
         }
+        "inventory-import" => {
+            check_options(&args[1..], &["--db", "--input"])?;
+            let path = db_path(&args[1..])?;
+            let input = required(&args[1..], "--input")?;
+            let snapshot = parse_inventory_bytes(&read_json_file(&input)?)
+                .map_err(|error| error.to_string())?;
+            let mut store = open(&path).map_err(|error| error.to_string())?;
+            let inserted = store
+                .import_inventory(snapshot.clone())
+                .map_err(|error| error.to_string())?;
+            Ok(json!({"ok":true,"command":"inventory-import","status":if inserted {"imported"} else {"replayed"},"snapshot_id":snapshot.snapshot_id}))
+        }
+        "context-import" => {
+            check_options(&args[1..], &["--db", "--input"])?;
+            let path = db_path(&args[1..])?;
+            let input = required(&args[1..], "--input")?;
+            let context = parse_task_context_bytes(&read_json_file(&input)?)
+                .map_err(|error| error.to_string())?;
+            let mut store = open(&path).map_err(|error| error.to_string())?;
+            let inserted = store
+                .import_task_context(context.clone())
+                .map_err(|error| error.to_string())?;
+            Ok(json!({"ok":true,"command":"context-import","status":if inserted {"imported"} else {"replayed"},"task_id":context.task_id}))
+        }
+        "accounting" => {
+            check_options(&args[1..], &["--db", "--task"])?;
+            let path = db_path(&args[1..])?;
+            let task = option(&args[1..], "--task")?;
+            let store = open(&path).map_err(|error| error.to_string())?;
+            let report = store
+                .accounting_report(&AccountingRequest { schema_version: 1, task_id: task })
+                .map_err(|error| error.to_string())?;
+            Ok(json!({"ok":true,"command":"accounting","report":report}))
+        }
+        "compare" => {
+            check_options(&args[1..], &["--db", "--input"])?;
+            let path = db_path(&args[1..])?;
+            let input = required(&args[1..], "--input")?;
+            let request = parse_comparison_request_bytes(&read_json_file(&input)?)
+                .map_err(|error| error.to_string())?;
+            let store = open(&path).map_err(|error| error.to_string())?;
+            let report = store
+          
[diff truncated; complete manifest remains authoritative]
### M005-S02

diff --git a/08_pkg/relayforge-core/src/accounting.rs b/08_pkg/relayforge-core/src/accounting.rs
new file mode 100644
index 0000000..75a2785
--- /dev/null
+++ b/08_pkg/relayforge-core/src/accounting.rs
@@ -0,0 +1,2385 @@
+//! Read-only outcome accounting and conservative descriptive comparisons.
+
+use super::{
+    AttemptOutcome, ComponentValue, CurrentState, EligibilityRequest, EligibilityResult, Event,
+    EventKind, FailureCategory, InventoryProfile, InventorySnapshot, ObservedIdentity,
+    SelectedIdentity, SpanRole, Store, StoreError, ToolOutcome, evaluate_eligibility, json_depth,
+    safe_text, sensitive, valid_timestamp, validate_inventory_id,
+};
+use rusqlite::{OptionalExtension, params};
+use serde::{Deserialize, Serialize};
+use std::collections::{BTreeMap, BTreeSet};
+
+pub const MAX_ACCOUNTING_EVENTS: usize = 10_000;
+pub const MAX_CONTEXT_BYTES: usize = 65_536;
+
+#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
+#[serde(tag = "state", deny_unknown_fields)]
+pub enum ContextLabel {
+    Known { value: String, provenance: String },
+    Unknown { reason: String, provenance: String },
+}
+
+#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
+#[serde(deny_unknown_fields)]
+pub struct TaskLabels {
+    pub task_type: ContextLabel,
+    pub difficulty: ContextLabel,
+    pub readiness: ContextLabel,
+}
+
+#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
+#[serde(deny_unknown_fields)]
+pub struct TaskContext {
+    pub schema_version: u16,
+    pub context_id: String,
+    pub task_id: String,
+    pub session_id: String,
+    pub inventory_snapshot_id: String,
+    pub initial_profile_id: String,
+    pub initial_profile_revision: String,
+    pub policy_version: String,
+    pub observed_at: String,
+    pub labels: TaskLabels,
+}
+
+#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
+#[serde(deny_unknown_fields)]
+pub struct AccountingRequest {
+    pub schema_version: u16,
+    pub task_id: Option<String>,
+}
+
+#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
+#[serde(deny_unknown_fields)]
+pub struct ComparisonRequest {
+    pub schema_version: u16,
+    pub baseline_profile_id: String,
+    pub evidence_window_start: String,
+    pub evidence_window_end: String,
+    pub eligibility: EligibilityRequest,
+}
+
+#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
+#[serde(deny_unknown_fields)]
+pub struct UnitSubtotal {
+    pub unit: String,
+    pub known_subtotal: Option<u64>,
+    pub reason: Option<String>,
+}
+
+#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
+#[serde(deny_unknown_fields)]
+pub struct CompleteTotal {
+    pub value: Option<u64>,
+    pub unit: Option<String>,
+    pub reasons: Vec<String>,
+}
+
+#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
+#[serde(deny_unknown_fields)]
+pub struct DimensionAccounting {
+    pub known_subtotals: Vec<UnitSubtotal>,
+    pub full_chain_total: CompleteTotal,
+}
+
+#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
+#[serde(deny_unknown_fields)]
+pub struct FailureOccurrence {
+    pub source_id: String,
+    pub event_id: String,
+    pub attempt_id: Option<String>,
+    pub kind: String,
+    pub category: Option<FailureCategory>,
+}
+
+#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
+#[serde(deny_unknown_fields)]
+pub struct AttemptAccounting {
+    pub attempt_id: String,
+    pub predecessor_attempt_id: Option<String>,
+    pub span_role: SpanRole,
+    pub selected: SelectedIdentity,
+    pub outcomes: Vec<AttemptOutcome>,
+    pub latest_outcome: Option<AttemptOutcome>,
+    pub observed_identities: Vec<ObservedIdentity>,
+    pub tool_call_count: usize,
+    pub tool_failure_count: usize,
+    pub interruption_count: usize,
+    pub usage_cost_record_count: usize,
+    pub usage: DimensionAccounting,
+    pub cost: DimensionAccounting,
+    pub elapsed: DimensionAccounting,
+}
+
+#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
+#[serde(d
[diff truncated; complete manifest remains authoritative]
### M005-S01

diff --git a/08_pkg/relayforge-core/src/lib.rs b/08_pkg/relayforge-core/src/lib.rs
index 36586bd..7007323 100644
--- a/08_pkg/relayforge-core/src/lib.rs
+++ b/08_pkg/relayforge-core/src/lib.rs
@@ -15,6 +15,14 @@ use rusqlite::{Connection, OptionalExtension, Transaction, TransactionBehavior,
 use serde::{Deserialize, Serialize};
 use std::path::Path;
 
+mod accounting;
+pub use accounting::{
+    AccountingReport, AccountingRequest, AttemptAccounting, ComparisonReport, ComparisonRequest,
+    CompleteTotal, ContextLabel, CostPerAcceptedTask, DimensionAccounting, FailureOccurrence,
+    ProfileComparison, TaskAccounting, TaskContext, TaskLabels, UnitSubtotal, WilsonInterval,
+    parse_accounting_request_bytes, parse_comparison_request_bytes, parse_task_context_bytes,
+};
+
 pub const EVENT_SCHEMA_VERSION: u16 = 1;
 pub const DATABASE_SCHEMA_VERSION: i64 = 1;
 pub const MAX_EVENT_BYTES: usize = 65_536;
@@ -58,6 +66,19 @@ CREATE TABLE IF NOT EXISTS attempts (
     intent_event_id TEXT NOT NULL,
     selected_json TEXT NOT NULL
 );
+CREATE TABLE IF NOT EXISTS inventory_snapshots (
+    snapshot_id TEXT PRIMARY KEY,
+    snapshot_json TEXT NOT NULL
+);
+CREATE TABLE IF NOT EXISTS task_contexts (
+    context_id TEXT PRIMARY KEY,
+    task_id TEXT NOT NULL UNIQUE,
+    context_json TEXT NOT NULL
+);
+CREATE TRIGGER IF NOT EXISTS task_contexts_no_update BEFORE UPDATE ON task_contexts BEGIN SELECT RAISE(ABORT,'immutable task context'); END;
+CREATE TRIGGER IF NOT EXISTS task_contexts_no_delete BEFORE DELETE ON task_contexts BEGIN SELECT RAISE(ABORT,'immutable task context'); END;
+CREATE TRIGGER IF NOT EXISTS inventory_snapshots_no_update BEFORE UPDATE ON inventory_snapshots BEGIN SELECT RAISE(ABORT,'immutable inventory snapshot'); END;
+CREATE TRIGGER IF NOT EXISTS inventory_snapshots_no_delete BEFORE DELETE ON inventory_snapshots BEGIN SELECT RAISE(ABORT,'immutable inventory snapshot'); END;
 CREATE TRIGGER IF NOT EXISTS events_no_update BEFORE UPDATE ON events BEGIN SELECT RAISE(ABORT,'immutable event'); END;
 CREATE TRIGGER IF NOT EXISTS events_no_delete BEFORE DELETE ON events BEGIN SELECT RAISE(ABORT,'immutable event'); END;
 CREATE TRIGGER IF NOT EXISTS parents_no_update BEFORE UPDATE ON parents BEGIN SELECT RAISE(ABORT,'immutable parent link'); END;
@@ -66,6 +87,24 @@ CREATE TRIGGER IF NOT EXISTS attempts_no_update BEFORE UPDATE ON attempts BEGIN
 CREATE TRIGGER IF NOT EXISTS attempts_no_delete BEFORE DELETE ON attempts BEGIN SELECT RAISE(ABORT,'immutable attempt'); END;
 ";
 
+const INVENTORY_SCHEMA: &str = "
+BEGIN IMMEDIATE;
+CREATE TABLE IF NOT EXISTS inventory_snapshots (
+    snapshot_id TEXT PRIMARY KEY,
+    snapshot_json TEXT NOT NULL
+);
+CREATE TABLE IF NOT EXISTS task_contexts (
+    context_id TEXT PRIMARY KEY,
+    task_id TEXT NOT NULL UNIQUE,
+    context_json TEXT NOT NULL
+);
+CREATE TRIGGER IF NOT EXISTS task_contexts_no_update BEFORE UPDATE ON task_contexts BEGIN SELECT RAISE(ABORT,'immutable task context'); END;
+CREATE TRIGGER IF NOT EXISTS task_contexts_no_delete BEFORE DELETE ON task_contexts BEGIN SELECT RAISE(ABORT,'immutable task context'); END;
+CREATE TRIGGER IF NOT EXISTS inventory_snapshots_no_update BEFORE UPDATE ON inventory_snapshots BEGIN SELECT RAISE(ABORT,'immutable inventory snapshot'); END;
+CREATE TRIGGER IF NOT EXISTS inventory_snapshots_no_delete BEFORE DELETE ON inventory_snapshots BEGIN SELECT RAISE(ABORT,'immutable inventory snapshot'); END;
+COMMIT;
+";
+
 #[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
 #[serde(deny_unknown_fields)]
 pub struct Event {
@@ -255,6 +294,137 @@ pub enum StoreError {
     Sql(String),
 }
 
+pub const MAX_INVENTORY_BYTES: usize = 1_048_576;
+pub const MAX_INVENTORY_PROFILES: usize = 64;
+pub const MAX_CAPABILITIES_PER_PROFILE: usize = 64;
+
+/// Versioned inventory metadata is declared by a caller and is not proof of runtime state.
+#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
+#[serde(deny_unknown_fields)]
+pub struct InventorySnapshot {
+    pub schema_ve
[diff truncated; complete manifest remains authoritative]
### M005-S01

diff --git a/08_pkg/relayforge-core/tests/storage_contract.rs b/08_pkg/relayforge-core/tests/storage_contract.rs
index 4484062..04a00d8 100644
--- a/08_pkg/relayforge-core/tests/storage_contract.rs
+++ b/08_pkg/relayforge-core/tests/storage_contract.rs
@@ -1,6 +1,8 @@
 use relayforge_core::{
-    AppendOutcome, AttemptOutcome, ComponentValue, Event, EventKind, EventRef, FailureCategory,
-    ObservedIdentity, SelectedIdentity, SpanRole, Store, ToolOutcome, open,
+    AppendOutcome, AttemptOutcome, CapabilityFact, CapabilityStatus, ComparisonRequest,
+    ComponentValue, ContextLabel, EligibilityRequest, Event, EventKind, EventRef, FailureCategory,
+    InventoryProfile, InventorySnapshot, ObservedIdentity, RequiredCapability, SelectedIdentity,
+    SpanRole, Store, TaskContext, TaskLabels, ToolOutcome, WorkloadLabel, open,
 };
 use rusqlite::Connection;
 use std::io::Write;
@@ -1103,6 +1105,377 @@ fn actual_cli_fresh_processes_init_append_show_reconcile_accept() {
     );
 }
 
+#[test]
+fn actual_cli_fresh_processes_inventory_import_show_and_binding_diagnostics() {
+    let (dir, path) = db();
+    let snapshot = InventorySnapshot {
+        schema_version: 1,
+        snapshot_id: "cli-snapshot".into(),
+        revision: "revision-1".into(),
+        policy_version: "policy-1".into(),
+        created_at: "2026-10-25T10:00:00Z".into(),
+        harness: "codex".into(),
+        harness_version: "1".into(),
+        adapter: "native".into(),
+        adapter_version: "1".into(),
+        runtime_scope: "runtime-1".into(),
+        session_scope: "session-1".into(),
+        configuration_fingerprint: "config-1".into(),
+        workspace_fingerprint: "workspace-1".into(),
+        profiles: vec![InventoryProfile {
+            profile_id: "profile-1".into(),
+            revision: "profile-rev-1".into(),
+            native_role: "worker".into(),
+            model_provider: "provider-1".into(),
+            configured_model_id: "alias-1".into(),
+            resolved_model_id: None,
+            resolved_model_version: None,
+            effort: "high".into(),
+            supported_efforts: vec!["high".into()],
+            capabilities: vec![CapabilityFact {
+                capability_id: "cap-1".into(),
+                value: "enabled".into(),
+                status: CapabilityStatus::Configured,
+                source_ref: "inventory/cap-1".into(),
+                observed_at: "2026-10-25T10:00:00Z".into(),
+                expires_at: "2026-10-26T10:00:00Z".into(),
+                profile_revision: "profile-rev-1".into(),
+                harness_version: "1".into(),
+                configuration_fingerprint: "config-1".into(),
+                runtime_scope: "runtime-1".into(),
+            }],
+        }],
+    };
+    let request = EligibilityRequest {
+        schema_version: 1,
+        request_id: "cli-request".into(),
+        task_id: "task-1".into(),
+        session_id: "session-1".into(),
+        snapshot_id: "cli-snapshot".into(),
+        policy_version: "policy-1".into(),
+        candidate_profile_ids: vec!["profile-1".into()],
+        execution_path: "native".into(),
+        selected_profile_id: None,
+        required_capabilities: vec![RequiredCapability {
+            capability_id: "cap-1".into(),
+            value: "enabled".into(),
+        }],
+        workload_labels: vec![WorkloadLabel {
+            name: "unknown".into(),
+            value: "preserved".into(),
+            provenance: "user".into(),
+        }],
+    };
+    let snapshot_path = dir.path().join("snapshot.json");
+    let request_path = dir.path().join("request.json");
+    std::fs::write(&snapshot_path, serde_json::to_vec(&snapshot).unwrap()).unwrap();
+    std::fs::write(&request_path, serde_json::to_vec(&request).unwrap()).unwrap();
+    let db_path = path.to_str().unwrap();
+    let snapshot_input = snapshot_path.to_str().unwrap();
+    let request_input = request_path.to_str().unwrap();
+    let (ok, imported) = cli(
+        &[
+            "inven
[diff truncated; complete manifest remains authoritative]
### M005-S01

diff --git a/scripts/product_verify.py b/scripts/product_verify.py
index 7dd4a8e..6fcf7db 100644
--- a/scripts/product_verify.py
+++ b/scripts/product_verify.py
@@ -16,6 +16,18 @@ ROOT = Path(__file__).resolve().parents[1]
 MANIFEST = ROOT / "Cargo.toml"
 STAGES = ("fmt", "clippy", "unit", "integration", "doc")
 REQUIRED = {
+    "tests::inventory_schema_size_privacy_and_duplicate_keys_are_rejected",
+    "tests::inventory_import_replay_conflict_and_reopen_are_immutable",
+    "tests::eligibility_requires_separate_current_state_and_exact_confirmed_scope",
+    "tests::capability_statuses_and_evidence_freshness_have_explicit_reasons",
+    "tests::capability_statuses_require_applicability_before_classification",
+    "accounting::tests::context_replay_conflict_late_rejection_and_legacy_age_are_explicit",
+    "accounting::tests::accounting_counts_repair_cost_and_preserves_ambiguous_mixed_and_overflow_values",
+    "accounting::tests::comparison_uses_eligible_exact_cohorts_wilson_baseline_and_honest_limits",
+    "accounting::tests::comparison_excludes_mismatched_cohort_dimensions",
+    "accounting::tests::comparison_excludes_profile_identity_mismatch",
+    "accounting::tests::comparison_does_not_prefer_overlapping_wilson_intervals",
+    "tests::eligibility_rejects_stale_state_dirty_workspace_and_never_substitutes_selection",
     "tests::unknown_fields_and_schema_versions_are_rejected",
     "tests::event_size_and_depth_are_bounded",
     "tests::session_open_is_a_durable_owner_claim",
@@ -27,6 +39,8 @@ REQUIRED = {
     "explicit_recovery_protects_active_owner_and_reconcile_is_repeatable",
     "identical_replay_is_noop_and_conflicting_duplicate_writes_nothing",
     "immutable_rows_reject_mutation_and_sources_can_reuse_event_ids",
+    "actual_cli_fresh_processes_inventory_import_show_and_binding_diagnostics",
+    "actual_cli_fresh_processes_context_accounting_and_comparison",
     "parent_scope_mismatch_and_failed_append_are_atomic",
     "synthetic_sensitive_input_is_rejected_without_echo_or_storage",
     "two_process_claim_race_has_exactly_one_owner",
@@ -75,8 +89,8 @@ def run_product(root: Path, scratch: Path, stages: tuple[str, ...] = STAGES) ->
     if "clippy" in stages:
         canary._run(cargo("clippy", "--manifest-path", str(MANIFEST), "--workspace", "--all-targets", "--target", target, "--locked", "--offline", "--", "-D", "warnings"), cwd=root, env=env, deadline=deadline, scratch=scratch)
     for stage, selector, required in (
-        ("unit", ["--workspace", "--lib"], {name for name in REQUIRED if name.startswith("tests::")}),
-        ("integration", ["--manifest-path", str(root / "08_pkg/relayforge-core/Cargo.toml"), "--test", "storage_contract"], {name for name in REQUIRED if not name.startswith("tests::")}),
+        ("unit", ["--workspace", "--lib"], {name for name in REQUIRED if name.startswith(("tests::", "accounting::tests::"))}),
+        ("integration", ["--manifest-path", str(root / "08_pkg/relayforge-core/Cargo.toml"), "--test", "storage_contract"], {name for name in REQUIRED if not name.startswith(("tests::", "accounting::tests::"))}),
         ("doc", ["--workspace", "--doc"], set()),
     ):
         if stage not in stages:
### M005-S02

diff --git a/02_analysis/accounting-results.md b/02_analysis/accounting-results.md
new file mode 100644
index 0000000..658bc08
--- /dev/null
+++ b/02_analysis/accounting-results.md
@@ -0,0 +1,44 @@
+# M005-S02 accounting results
+
+The core stores an immutable task-context record before the first attempt and
+reports physical attempts, parent repairs, explicit acceptance, failures, and
+usage/cost/elapsed accounting. The CLI exposes the same read-only reports in a
+fresh process:
+
+```text
+relayforge context-import --db run.db --input task-context.json
+relayforge accounting --db run.db [--task TASK_ID]
+relayforge compare --db run.db --input comparison-request.json
+```
+
+`task-context.json` is a version-1 `TaskContext`: it names the task and session,
+the inventory snapshot, initial profile and revision, policy, observation time,
+and task-type, difficulty, and readiness labels. Each label carries provenance
+or an explicit unknown state. Import is idempotent for identical content and
+rejects changed content or a first import after an attempt has begun. Existing
+tasks without context remain unclassified; database and event ordering do not
+imply their age or cohort.
+
+The accounting report preserves every observed attempt and failure. A physical
+attempt with multiple legacy `UsageCost` rows is ambiguous because the event
+schema does not identify deltas or snapshots. Totals retain known subtotals by
+unit and mark the full chain unknown for missing, ambiguous, mixed-unit, or
+overflowing components. Acceptance is taken from explicit `TaskAccepted`
+events; a later work attempt can make current status unresolved while retaining
+historical acceptance.
+
+Comparison requests name a fixed baseline, evidence window, and existing S01
+eligibility request. Comparisons use exact known label provenance, policy,
+harness, adapter, and configuration cohorts, and profile identity dimensions.
+The Wilson 95% intervals and exact same-unit cost-per-accepted-task comparison
+are descriptive only. The library comparison requires caller-supplied current
+state for positive eligibility. The standalone CLI has no trusted runtime
+binding and reports `binding_unavailable`; it cannot select a runtime or promote
+a profile. Event scans are capped at 10,000 records and fail rather than
+truncate.
+
+Focused tests cover context replay and late-import rejection, repair-chain
+accounting, ambiguous/mixed/overflow values, exact eligibility and cohort
+comparison, uncertainty, and fresh-process CLI paths. These comparison fixtures
+are synthetic; they do not establish native adapter behavior or production
+calibration.
### M005-S01

diff --git a/02_analysis/inventory-results.md b/02_analysis/inventory-results.md
new file mode 100644
index 0000000..6c33185
--- /dev/null
+++ b/02_analysis/inventory-results.md
@@ -0,0 +1,31 @@
+# Inventory implementation results
+
+M005-S01 adds strict version-1 inventory and request JSON, immutable SQLite
+snapshot import, exact-scope capability eligibility, and explicit CLI
+diag
[diff truncated; complete manifest remains authoritative]
```
