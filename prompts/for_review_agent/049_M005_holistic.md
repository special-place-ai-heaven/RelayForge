# Review prompt: M005 — Exact inventory and useful deterministic routing evidence (round holistic)

Read `AGENTS.md` first. Do not change product files. Use the receipt as execution
evidence; do not rerun verification unless this prompt explicitly says so.

## Objective and acceptance

Judge holistic closure of M005 across its accepted slices.

### M005-S01

- Implement the exact identity, immutable snapshot, privacy, freshness and current-state contracts in 02_analysis/inventory-plan.md using existing dependencies.
- Persist transactional idempotent snapshots and reject same-ID conflicts without writes; preserve existing databases and event behavior.
- Only exact fresh runtime-confirmed capabilities qualify against independent current state; missing or stale state and dirty-workspace mismatches remain unresolved.
- Honor explicit selection without substitution and return deterministic per-candidate reasons; never dispatch or allow a request to assert trusted state.
- Exercise fresh-process CLI import/show/eligibility diagnostics, including unavailable standalone runtime binding, with discovered positive and negative library tests.
- Preserve existing recovery/privacy/ownership tests, extend verifier discovery gates, pass focused and full checks, and document observed limits without claiming G3/G4 completion.

### M005-S02

- Implement 02_analysis/accounting-plan.md using existing dependencies, preserving accepted storage and inventory behavior.
- Persist immutable pre-attempt task context; reject retroactive labels and preserve unclassified legacy observations with unknown age.
- Account for every physical attempt and parent repair; distinguish explicit acceptance from worker success and expose unresolved and operational failures.
- Report per-unit known subtotals and unknown complete totals for missing, ambiguous, mixed-unit or overflowing components without silent truncation.
- Apply exact cohort matching and S01 eligibility before fixed-baseline descriptive-v1 comparisons; report uncertainty and insufficient evidence honestly, with no dispatch or automatic promotion.
- Exercise documented fresh-process CLI context, accounting and comparison paths, required discovery tests and the authoritative full check; disclose synthetic and unavailable native-binding limits.

## Non-goals

### M005-S01

- Statistics/ranking, provider calls, native harness hooks, dispatch, tool bundles, new dependencies or packaging.

### M005-S02

- Native adapters, provider calls, calibrated dispatch thresholds, dependency additions, tool bundles or packaging.

## Read first

### M005-S01

- `08_pkg/CONTEXT.md`
- `07_app/CONTEXT.md`
- `02_analysis/CONTEXT.md`
- `02_analysis/inventory-plan.md`
- `02_analysis/foundation-contract.md`
- `02_analysis/native-core-plan.md`

### M005-S02

- `08_pkg/CONTEXT.md`
- `07_app/CONTEXT.md`
- `02_analysis/CONTEXT.md`
- `02_analysis/accounting-plan.md`
- `02_analysis/inventory-plan.md`
- `02_analysis/foundation-contract.md`

## Implementation boundary

Allowed prefixes: ### M005-S01

`08_pkg/relayforge-core/`, `07_app/relayforge-cli/`, `scripts/product_verify.py`, `02_analysis/inventory-results.md`

### M005-S02

`08_pkg/relayforge-core/`, `07_app/relayforge-cli/`, `scripts/product_verify.py`, `02_analysis/accounting-results.md`

Forbidden: ### M005-S01

`00_brief/`, `05_governance/`, `prompts/`, `questions/answered/`, `roadmap.yaml`, `frutlups.toml`, `AGENTS.md`, `CLAUDE.md`

### M005-S02

`00_brief/`, `05_governance/`, `prompts/`, `questions/answered/`, `roadmap.yaml`, `frutlups.toml`, `AGENTS.md`, `CLAUDE.md` and all other paths. These describe the coder's scope;
review remains product-read-only. Read the evidence artifacts named below using
your file-reading tools. Their categories describe volume, not authority.

## Declared verification

Focused:

### M005-S01

```text
python -B scripts/product_verify.py --focused
```

### M005-S02

```text
python -B scripts/product_verify.py --focused
```

Full:

### M005-S01

```text
python -B scripts/hermetic_verification.py
```

Execution policy: runtime {}; timeout 900 seconds; observation process.

### M005-S02

```text
python -B scripts/hermetic_verification.py
```

Execution policy: runtime {}; timeout 900 seconds; observation process.

## Changed files

11 cumulative paths. Complete manifest: `05_governance/reviews/m005/M005_4e769948e84d0b16_paths.json` (sha256 `4e769948e84d0b169418604868ee78c5e01d782bc291525ffca4c4a35c88ccee`; read as a file, starting at line 1)

## Code diff

Bounded diff page: `05_governance/reviews/m005/M005_7c1edd6d8a9e74fc_diff.md` (sha256 `7c1edd6d8a9e74fcdf3be2a66a532398b075f2a73133e320884018193d761df8`; read as a file, starting at line 1)

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
+    pub event_id: String
```

[Inline excerpt bounded; read the full diff page above.]

## Verification receipt

### M005-S01

Verification passed. Complete receipt: `05_governance/reviews/m005/M005-S01_r2_verification.json` (sha256 `9c88cee10d572a5f1082a16f725589450e1152daa85e4bcdb9498207ab4721fd`)

Review: `05_governance/reviews/m005/M005-S01_r2_review.md` (sha256 `7c1c9240f51d26f2e031df1a88851b8f74b7017a3bd8940f762efd9178321578`)

### M005-S02

Verification passed. Complete receipt: `05_governance/reviews/m005/M005-S02_r2_verification.json` (sha256 `7ad395de2249f303c2fc01da6b8f9821b0573a1d9156ac5ed4f491e9de1b4473`)

Review: `05_governance/reviews/m005/M005-S02_r2_review.md` (sha256 `09f67d05a09d07b5f9507f62e92cadd0683cdf4002807f369b7869ed14a3f1f1`)

## Prior findings

Source review: `05_governance/reviews/m005/M005-S01_r1_review.md` (sha256 `b7f845fb87bd51c9050dadc38eaab223761ac885dae68c17fa6b3535a73cc63a`)
| M005-S01-001 | P2 | open | evaluate_eligibility classifies Configured and Unsupported facts before checking whether their profile revision, harness/configuration, and runtime scope match current state. A fact from another scope can therefore report unresolved_configured_only or excluded_unsupported for the current scope. Check applicability first and cover both cases in tests. |

Source review: `05_governance/reviews/m005/M005-S02_r1_review.md` (sha256 `a47c4d8821fec6255c3d3ce0ba69d34fe2d577d520a719cd6a92df9e2a25b83e`)
| M005-S02-001 | P2 | open | The comparison test covers one shared cohort and a clearly separated result, but not the required mismatched cohort/profile identity or overlapping Wilson interval cases. Add coverage for these conservative comparison boundaries before pass. |

## Output

Autonomous seats return the complete report for the runner to save. In manual
mode, write only `05_governance/reviews/m005/M005_holistic_review.md` when that tool is granted, or return it for
the architect to save.

Use this exact contract:

```markdown
# Review: M005 round holistic

## Findings
| id | severity | disposition | summary |
| --- | --- | --- | --- |

## Closure Decision
Objective status: achieved | not_achieved | indeterminate
Objective evidence: one sentence tied to acceptance and the receipt

## Verdict
Verdict: pass|needs_work|blocked - next: one move
```

Return the report as the plain text of your final message, in exactly this shape, not enclosed in a code fence. Use one allowed value on each choice line. A pass requires zero open P0-P2.
Every P0-P2 finding ID must start with the affected slice ID, for example `M001-S02-H1-F1`.

To change an older finding, add `## Finding updates` before Closure Decision with columns `source | sha | id | disposition | related`. Name the original report path, its SHA-256, exact finding ID and explicit disposition; related is linked IDs or `-`. An unrelated pass closes nothing. Only a human may waive findings.
