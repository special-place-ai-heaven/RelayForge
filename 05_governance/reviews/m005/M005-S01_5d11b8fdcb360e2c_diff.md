```diff
HEAD diff filtered to current-round paths; this is not a prior-round delta.
diff --git a/08_pkg/relayforge-core/src/lib.rs b/08_pkg/relayforge-core/src/lib.rs
index 36586bd..424da95 100644
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
diff --git a/scripts/product_verify.py b/scripts/product_verify.py
index 7dd4a8e..5d8b848 100644
--- a/scripts/product_verify.py
+++ b/scripts/product_verify.py
@@ -16,6 +16,12 @@ ROOT = Path(__file__).resolve().parents[1]
 MANIFEST = ROOT / "Cargo.toml"
 STAGES = ("fmt", "clippy", "unit", "integration", "doc")
 REQUIRED = {
+    "tests::inventory_schema_size_privacy_and_duplicate_keys_are_rejected",
+    "tests::inventory_import_replay_conflict_and_reopen_are_immutable",
+    "tests::eligibility_requires_separate_current_state_and_exact_confirmed_scope",
+    "tests::capability_statuses_and_evidence_freshness_have_explicit_reasons",
+    "tests::capability_statuses_require_applicability_before_classification",
+    "tests::eligibility_rejects_stale_state_dirty_workspace_and_never_substitutes_selection",
     "tests::unknown_fields_and_schema_versions_are_rejected",
     "tests::event_size_and_depth_are_bounded",
     "tests::session_open_is_a_durable_owner_claim",
@@ -27,6 +33,7 @@ REQUIRED = {
     "explicit_recovery_protects_active_owner_and_reconcile_is_repeatable",
     "identical_replay_is_noop_and_conflicting_duplicate_writes_nothing",
     "immutable_rows_reject_mutation_and_sources_can_reuse_event_ids",
+    "actual_cli_fresh_processes_inventory_import_show_and_binding_diagnostics",
     "parent_scope_mismatch_and_failed_append_are_atomic",
     "synthetic_sensitive_input_is_rejected_without_echo_or_storage",
     "two_process_claim_race_has_exactly_one_owner",

Diff truncated at 32 KB or the smaller per-file evidence allowance.
```
