```diff
HEAD diff filtered to current-round paths; this is not a prior-round delta.
diff --git a/07_app/relayforge-cli/src/main.rs b/07_app/relayforge-cli/src/main.rs
index 31b1bdb..b0c3c06 100644
--- a/07_app/relayforge-cli/src/main.rs
+++ b/07_app/relayforge-cli/src/main.rs
@@ -1,4 +1,7 @@
-use relayforge_core::{Event, open, parse_event_bytes};
+use relayforge_core::{
+    Event, MAX_INVENTORY_BYTES, evaluate_eligibility, open, parse_eligibility_request_bytes,
+    parse_event_bytes, parse_inventory_bytes,
+};
 use rusqlite::Connection;
 use serde_json::json;
 use std::env;
@@ -54,6 +57,20 @@ fn read_event() -> Result<Event, String> {
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
@@ -104,6 +121,39 @@ fn run() -> Result<serde_json::Value, String> {
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
+        "inventory-show" => {
+            check_options(&args[1..], &["--db", "--snapshot"])?;
+            let path = db_path(&args[1..])?;
+            let snapshot_id = required(&args[1..], "--snapshot")?;
+            let store = open(&path).map_err(|error| error.to_string())?;
+            let snapshot = store.inventory(&snapshot_id).map_err(|error| error.to_string())?;
+            Ok(json!({"ok":true,"command":"inventory-show","snapshot":snapshot}))
+        }
+        "eligibility" => {
+            check_options(&args[1..], &["--db", "--snapshot", "--input"])?;
+            let path = db_path(&args[1..])?;
+            let snapshot_id = required(&args[1..], "--snapshot")?;
+            let input = required(&args[1..], "--input")?;
+            let request = parse_eligibility_request_bytes(&read_json_file(&input)?)
+                .map_err(|error| error.to_string())?;
+            let store = open(&path).map_err(|error| error.to_string())?;
+            let snapshot = store.inventory(&snapshot_id).map_err(|error| error.to_string())?;
+            let result = evaluate_eligibility(&snapshot, &request, None)
+                .map_err(|error| error.to_string())?;
+            Ok(json!({"ok":true,"command":"eligibility","binding":"binding_unavailable","eligible_profile_ids":[],"candidates":result.candidates,"outcome":"no_eligible_candidate"}))
+        }
         "reconcile" => {
             check_options(
                 &args[1..],
@@ -120,7 +170,7 @@ fn run() -> Result<serde_json::Value, String> {
             Ok(json!({"ok":true,"appended":appended,"session_id":session}))
         }
         _ => Err(
-            "usage: relayforge <init|append|show|reconcile|sqlite-version> --database PATH".into(),
+            "usage: re
[diff truncated; complete manifest remains authoritative]
diff --git a/08_pkg/relayforge-core/src/lib.rs b/08_pkg/relayforge-core/src/lib.rs
index 36586bd..40948ae 100644
--- a/08_pkg/relayforge-core/src/lib.rs
+++ b/08_pkg/relayforge-core/src/lib.rs
@@ -58,6 +58,12 @@ CREATE TABLE IF NOT EXISTS attempts (
     intent_event_id TEXT NOT NULL,
     selected_json TEXT NOT NULL
 );
+CREATE TABLE IF NOT EXISTS inventory_snapshots (
+    snapshot_id TEXT PRIMARY KEY,
+    snapshot_json TEXT NOT NULL
+);
+CREATE TRIGGER IF NOT EXISTS inventory_snapshots_no_update BEFORE UPDATE ON inventory_snapshots BEGIN SELECT RAISE(ABORT,'immutable inventory snapshot'); END;
+CREATE TRIGGER IF NOT EXISTS inventory_snapshots_no_delete BEFORE DELETE ON inventory_snapshots BEGIN SELECT RAISE(ABORT,'immutable inventory snapshot'); END;
 CREATE TRIGGER IF NOT EXISTS events_no_update BEFORE UPDATE ON events BEGIN SELECT RAISE(ABORT,'immutable event'); END;
 CREATE TRIGGER IF NOT EXISTS events_no_delete BEFORE DELETE ON events BEGIN SELECT RAISE(ABORT,'immutable event'); END;
 CREATE TRIGGER IF NOT EXISTS parents_no_update BEFORE UPDATE ON parents BEGIN SELECT RAISE(ABORT,'immutable parent link'); END;
@@ -66,6 +72,17 @@ CREATE TRIGGER IF NOT EXISTS attempts_no_update BEFORE UPDATE ON attempts BEGIN
 CREATE TRIGGER IF NOT EXISTS attempts_no_delete BEFORE DELETE ON attempts BEGIN SELECT RAISE(ABORT,'immutable attempt'); END;
 ";
 
+const INVENTORY_SCHEMA: &str = "
+BEGIN IMMEDIATE;
+CREATE TABLE IF NOT EXISTS inventory_snapshots (
+    snapshot_id TEXT PRIMARY KEY,
+    snapshot_json TEXT NOT NULL
+);
+CREATE TRIGGER IF NOT EXISTS inventory_snapshots_no_update BEFORE UPDATE ON inventory_snapshots BEGIN SELECT RAISE(ABORT,'immutable inventory snapshot'); END;
+CREATE TRIGGER IF NOT EXISTS inventory_snapshots_no_delete BEFORE DELETE ON inventory_snapshots BEGIN SELECT RAISE(ABORT,'immutable inventory snapshot'); END;
+COMMIT;
+";
+
 #[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
 #[serde(deny_unknown_fields)]
 pub struct Event {
@@ -255,6 +272,137 @@ pub enum StoreError {
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
+    pub schema_version: u16,
+    pub snapshot_id: String,
+    pub revision: String,
+    pub policy_version: String,
+    pub created_at: String,
+    pub harness: String,
+    pub harness_version: String,
+    pub adapter: String,
+    pub adapter_version: String,
+    pub runtime_scope: String,
+    pub session_scope: String,
+    pub configuration_fingerprint: String,
+    pub workspace_fingerprint: String,
+    pub profiles: Vec<InventoryProfile>,
+}
+
+#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
+#[serde(deny_unknown_fields)]
+pub struct InventoryProfile {
+    pub profile_id: String,
+    pub revision: String,
+    pub native_role: String,
+    pub model_provider: String,
+    pub configured_model_id: String,
+    pub resolved_model_id: Option<String>,
+    pub resolved_model_version: Option<String>,
+    pub effort: String,
+    pub supported_efforts: Vec<String>,
+    pub capabilities: Vec<CapabilityFact>,
+}
+
+#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
+#[serde(rename_all = "snake_case")]
+pub enum CapabilityStatus {
+    Configured,
+    RuntimeConfirmed,
+    Unsupported,
+    Unknown,
+}
+
+#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
+#[serde(deny_unknown_fields)]
+pub struct CapabilityFact {
+    pub capability_id: String,
+    pub value: String,
+    pub status: CapabilityStatus,
+    pub source_ref: String,
+    pub observed_at: String,
+    pub expires_at: String,
+    pub profile_revision: String,
+    pub harness_version: String,
+    pub configuration_fingerprint: String,
+    pub runtime_scop
[diff truncated; complete manifest remains authoritative]
diff --git a/08_pkg/relayforge-core/tests/storage_contract.rs b/08_pkg/relayforge-core/tests/storage_contract.rs
index 4484062..9f992d5 100644
--- a/08_pkg/relayforge-core/tests/storage_contract.rs
+++ b/08_pkg/relayforge-core/tests/storage_contract.rs
@@ -1,6 +1,8 @@
 use relayforge_core::{
-    AppendOutcome, AttemptOutcome, ComponentValue, Event, EventKind, EventRef, FailureCategory,
-    ObservedIdentity, SelectedIdentity, SpanRole, Store, ToolOutcome, open,
+    AppendOutcome, AttemptOutcome, CapabilityFact, CapabilityStatus, ComponentValue,
+    EligibilityRequest, Event, EventKind, EventRef, FailureCategory, InventoryProfile,
+    InventorySnapshot, ObservedIdentity, RequiredCapability, SelectedIdentity, SpanRole, Store,
+    ToolOutcome, WorkloadLabel, open,
 };
 use rusqlite::Connection;
 use std::io::Write;
@@ -1103,6 +1105,132 @@ fn actual_cli_fresh_processes_init_append_show_reconcile_accept() {
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
+            "inventory-import",
+            "--db",
+            db_path,
+
[diff truncated; complete manifest remains authoritative]
diff --git a/scripts/product_verify.py b/scripts/product_verify.py
index 7dd4a8e..7f6e726 100644
--- a/scripts/product_verify.py
+++ b/scripts/product_verify.py
@@ -16,6 +16,11 @@ ROOT = Path(__file__).resolve().parents[1]
 MANIFEST = ROOT / "Cargo.toml"
 STAGES = ("fmt", "clippy", "unit", "integration", "doc")
 REQUIRED = {
+    "tests::inventory_schema_size_privacy_and_duplicate_keys_are_rejected",
+    "tests::inventory_import_replay_conflict_and_reopen_are_immutable",
+    "tests::eligibility_requires_separate_current_state_and_exact_confirmed_scope",
+    "tests::capability_statuses_and_evidence_freshness_have_explicit_reasons",
+    "tests::eligibility_rejects_stale_state_dirty_workspace_and_never_substitutes_selection",
     "tests::unknown_fields_and_schema_versions_are_rejected",
     "tests::event_size_and_depth_are_bounded",
     "tests::session_open_is_a_durable_owner_claim",
@@ -27,6 +32,7 @@ REQUIRED = {
     "explicit_recovery_protects_active_owner_and_reconcile_is_repeatable",
     "identical_replay_is_noop_and_conflicting_duplicate_writes_nothing",
     "immutable_rows_reject_mutation_and_sources_can_reuse_event_ids",
+    "actual_cli_fresh_processes_inventory_import_show_and_binding_diagnostics",
     "parent_scope_mismatch_and_failed_append_are_atomic",
     "synthetic_sensitive_input_is_rejected_without_echo_or_storage",
     "two_process_claim_race_has_exactly_one_owner",

Added file: 02_analysis/inventory-results.md
# Inventory implementation results

M005-S01 adds strict version-1 inventory and request JSON, immutable SQLite
snapshot import, exact-scope capability eligibility, and explicit CLI
diagnostics. Snapshots remain declared metadata. A library caller must supply a
separate `CurrentState`; the API documentation says that callers own trustworthy
capture and that matching fields do not authenticate it.

## CLI

The explicit-path commands are:

```text
relayforge inventory-import --db <db> --input <snapshot.json>
relayforge inventory-show --db <db> --snapshot <id>
relayforge eligibility --db <db> --snapshot <id> --input <request.json>
```

Fresh-process integration coverage imported and replayed a snapshot, read it by
ID, and ran eligibility diagnostics. The standalone command returned
`binding_unavailable`, `no_eligible_candidate`, and `missing_current_state`;
it does not turn JSON assertions into runtime confirmation.

## Limits

Focused verification exercised inventory schema/privacy/size checks, immutable
replay and conflict behavior, retained pre-existing event rows, current-state
matching, capability statuses, future/expired evidence, selection, dirty
workspaces, stable ordering, and fresh-process CLI behavior. The tests use
synthetic metadata. No live runtime observation, G4 host binding, ranking,
provider call, or dispatch was exercised or claimed.

Diff truncated at 32 KB or the smaller per-file evidence allowance.
```
