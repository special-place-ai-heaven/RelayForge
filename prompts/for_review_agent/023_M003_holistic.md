# Review prompt: M003 — Durable native metadata core and CLI (round holistic)

Read `AGENTS.md` first. Do not change product files. Use the receipt as execution
evidence; do not rerun verification unless this prompt explicitly says so.

## Objective and acceptance

Judge holistic closure of M003 across its accepted slices.

### M003-S01

- Implement all metadata, persistence, ownership, recovery and CLI contracts in 02_analysis/native-core-plan.md; use closed bounded versioned types and no raw payload maps.
- Pin Rust 1.99.1, rusqlite 0.40.0 with bundled SQLite 3.53.1, serde 1.0.228 and serde_json 1.0.154; track root lockfile and exclude the canary from the product workspace.
- Prove immutable transactional append, parent/scope validation, identical replay no-op and conflicting duplicate refusal without writes; reject unsupported schema versions safely.
- Prove committed-before-return intents, crash before and after commit, explicit attributed recovery, active-session protection, repeatable reconciliation and linked late results using fresh processes.
- Race two processes for one session claim and observe exactly one owner; no implicit expiry or takeover. Completion requires all current required checks plus explicit acceptance.
- Preserve every physical attempt, predecessor/repair link, failure, selected versus observed identity and unknown cost/usage component for later honest accounting.
- Reject sensitive or raw input before persistence without echoing it; verify synthetic sensitive values absent from database/WAL and stdout/stderr.
- Exercise actual built CLI init, bounded JSON append/show and reconcile across process boundaries, including failed tool result and verification/acceptance; document usable examples.
- Full verifier materializes once and executes canary plus product offline with fmt, Clippy, build and nonempty required discovered/executed tests; fail on missing/skipped tests and retain candidate identities and source/index/HEAD invariance.
- Record portable observations and historical receipt references in 02_analysis/native-core-results.md; make no packaging, adapter, routing or G1-G6 completion claim.

## Non-goals

### M003-S01

- Model/tool dispatch, inventory ranking, providers, harness hooks, packaging or installation.
- Rewriting accepted canary evidence, importing the community harness or adding unapproved direct dependencies.

## Read first

### M003-S01

- `08_pkg/CONTEXT.md`
- `07_app/CONTEXT.md`
- `02_analysis/CONTEXT.md`
- `00_brief/decisions.md`
- `00_brief/release-contract.md`
- `02_analysis/native-core-plan.md`
- `02_analysis/foundation-contract.md`
- `scripts/canary_verify.py`
- `scripts/hermetic_verification.py`
- `docs/project_checks.md`

## Implementation boundary

Allowed prefixes: ### M003-S01

`08_pkg/relayforge-core/`, `07_app/relayforge-cli/`, `Cargo.toml`, `Cargo.lock`, `rust-toolchain.toml`, `scripts/product_verify.py`, `scripts/canary_verify.py`, `02_analysis/native-core-results.md`

Forbidden: ### M003-S01

`00_brief/`, `05_governance/`, `prompts/`, `questions/answered/`, `roadmap.yaml`, `frutlups.toml`, `AGENTS.md`, `CLAUDE.md` and all other paths. These describe the coder's scope;
review remains product-read-only. Read the evidence artifacts named below using
your file-reading tools. Their categories describe volume, not authority.

## Declared verification

Focused:

### M003-S01

```text
python -B scripts/product_verify.py --focused
```

Full:

### M003-S01

```text
python -B scripts/hermetic_verification.py
```

Execution policy: runtime {}; timeout 900 seconds; observation process.

## Changed files

11 cumulative paths. Complete manifest: `05_governance/reviews/m003/M003_f394a2395caadc4f_paths.json` (sha256 `f394a2395caadc4f3c305c16ddc8a0b21591efcd2d9fe375a24c879df71ac4f9`; read as a file, starting at line 1)

## Code diff

Bounded diff page: `05_governance/reviews/m003/M003_4f800f19c01ee14b_diff.md` (sha256 `4f800f19c01ee14bb1b600199b5f909849b7d3977afc8b32424429c32d072443`; read as a file, starting at line 1)

```diff
git diff 536bd280706f2a1b56c9feb484515af79477ceb5..HEAD --stat (selected paths)
 02_analysis/native-core-results.md               |   73 ++
 07_app/relayforge-cli/Cargo.toml                 |   15 +
 07_app/relayforge-cli/src/main.rs                |  135 +++
 08_pkg/relayforge-core/Cargo.toml                |   11 +
 08_pkg/relayforge-core/src/lib.rs                | 1259 ++++++++++++++++++++++
 08_pkg/relayforge-core/tests/storage_contract.rs | 1168 ++++++++++++++++++++
 Cargo.lock                                       |  357 ++++++
 Cargo.toml                                       |   15 +
 rust-toolchain.toml                              |    5 +
 scripts/canary_verify.py                         |   40 +-
 scripts/product_verify.py                        |  115 ++
 11 files changed, 3192 insertions(+), 1 deletion(-)

Per-slice diffs: current working tree against accepted base (bounded; complete manifests remain authoritative).
### M003-S01

diff --git a/07_app/relayforge-cli/src/main.rs b/07_app/relayforge-cli/src/main.rs
new file mode 100644
index 0000000..31b1bdb
--- /dev/null
+++ b/07_app/relayforge-cli/src/main.rs
@@ -0,0 +1,135 @@
+use relayforge_core::{Event, open, parse_event_bytes};
+use rusqlite::Connection;
+use serde_json::json;
+use std::env;
+use std::io::{self, Read};
+use std::path::PathBuf;
+use std::process;
+
+const MAX_STDIN: usize = 65_536;
+
+fn option(args: &[String], name: &str) -> Result<Option<String>, String> {
+    let mut found = None;
+    let mut index = 0;
+    while index < args.len() {
+        if args[index] == name {
+            if found.is_some() || index + 1 == args.len() {
+                return Err("invalid_arguments".into());
+            }
+            found = Some(args[index + 1].clone());
+            index += 2;
+        } else {
+            index += 1;
+        }
+    }
+    Ok(found)
+}
+fn required(args: &[String], name: &str) -> Result<String, String> {
+    option(args, name)?.ok_or_else(|| "missing_required_option".into())
+}
+fn check_options(args: &[String], allowed: &[&str]) -> Result<(), String> {
+    let mut index = 0;
+    while index < args.len() {
+        if !allowed.contains(&args[index].as_str())
+            || index + 1 == args.len()
+            || args[index + 1].starts_with("--")
+        {
+            return Err("invalid_arguments".into());
+        }
+        index += 2;
+    }
+    Ok(())
+}
+fn database(args: &[String]) -> Result<PathBuf, String> {
+    Ok(PathBuf::from(required(args, "--database")?))
+}
+fn read_event() -> Result<Event, String> {
+    let mut input = Vec::new();
+    io::stdin()
+        .take((MAX_STDIN + 1) as u64)
+        .read_to_end(&mut input)
+        .map_err(|_| "input_read_failed")?;
+    if input.is_empty() || input.len() > MAX_STDIN {
+        return Err("event_size_out_of_range".into());
+    }
+    parse_event_bytes(&input).map_err(|e| e.to_string())
+}
+fn run() -> Result<serde_json::Value, String> {
+    let args: Vec<String> = env::args().skip(1).collect();
+    let command = args.first().map(String::as_str).unwrap_or("");
+    match command {
+        "sqlite-version" => {
+            let connection = Connection::open_in_memory().map_err(|_| "storage_error")?;
+            let version: String = connection
+                .query_row("SELECT sqlite_version()", [], |r| r.get(0))
+                .map_err(|_| "storage_error")?;
+            Ok(json!({"ok":true,"sqlite_version":version}))
+        }
+        "init" => {
+            check_options(&args[1..], &["--database"])?;
+            let path = database(&args[1..])?;
+            let _store = open(&path).map_err(|e| e.to_string())?;
+            Ok(json!({"ok":true,"command":"init","schema_version":1}))
+        }
+        "append" => {
+            check_options(&args[1..], &["--database"])?;
+            let path = database(&args[1..])?;
+            let event = read_event()?;
+            let mut store = open(&path).map_err(|e| e.to_string())?;
+            let result = store.append(event).map_err(|e| e.to_string())?;
+            match result {
+                relayforge_core::AppendOutcome::Appended { sequence } => {
+                    Ok(json!({"ok":true,"status":"appended","sequence":sequence}))
+                }
+                relayforge_core::AppendOutcome::Replayed { sequence } => {
+                    Ok(json!({"ok":true,"status":"replayed","sequence":sequence}))
+                }
+            }
+        }
+        "show" => {
+            check_options(
+                &args[1..],
+                &["--database", "--session", "--task", "--limit"],
+            )?;
+            let path = database(&args[1..])?;
+            let session = option(&args[1..], "--session")?;
+            let task = option(&args[1..], "--task")?;
+            let limit = option(&args[1..], "--limit")?
+                .map(|v| v.parse::<usize>().map_err(|_| "invalid_limit"))
+                .transpose()?
+                .unwrap_or(100);
[diff truncated; complete manifest remains authoritative]
### M003-S01

diff --git a/08_pkg/relayforge-core/src/lib.rs b/08_pkg/relayforge-core/src/lib.rs
new file mode 100644
index 0000000..36586bd
--- /dev/null
+++ b/08_pkg/relayforge-core/src/lib.rs
@@ -0,0 +1,1259 @@
+//! Typed, bounded durable metadata for RelayForge.
+//!
+//! This crate stores constrained metadata only. It does not dispatch work.
+//!
+//! ```
+//! use relayforge_core::{Event, EventKind, open_memory};
+//! let mut store = open_memory().unwrap();
+//! let event = Event::new("source-a", "event-a", EventKind::SessionOpened {
+//!     session_id: "session-a".into(), owner_id: "source-a".into(),
+//! });
+//! store.append(event).unwrap();
+//! ```
+
+use rusqlite::{Connection, OptionalExtension, Transaction, TransactionBehavior, params};
+use serde::{Deserialize, Serialize};
+use std::path::Path;
+
+pub const EVENT_SCHEMA_VERSION: u16 = 1;
+pub const DATABASE_SCHEMA_VERSION: i64 = 1;
+pub const MAX_EVENT_BYTES: usize = 65_536;
+pub const MAX_JSON_DEPTH: usize = 16;
+pub const MAX_EVENTS_PER_SHOW: usize = 200;
+
+const SCHEMA: &str = "
+CREATE TABLE IF NOT EXISTS sessions (
+    session_id TEXT PRIMARY KEY,
+    owner_id TEXT NOT NULL,
+    state TEXT NOT NULL CHECK(state IN ('active','recoverable','closed'))
+);
+CREATE TABLE IF NOT EXISTS events (
+    sequence INTEGER PRIMARY KEY AUTOINCREMENT,
+    source_id TEXT NOT NULL,
+    event_id TEXT NOT NULL,
+    schema_version INTEGER NOT NULL,
+    event_json TEXT NOT NULL,
+    kind TEXT NOT NULL,
+    session_id TEXT,
+    task_id TEXT,
+    attempt_id TEXT,
+    created_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ','now')),
+    UNIQUE(source_id, event_id)
+);
+CREATE TABLE IF NOT EXISTS parents (
+    source_id TEXT NOT NULL,
+    event_id TEXT NOT NULL,
+    parent_source_id TEXT NOT NULL,
+    parent_event_id TEXT NOT NULL,
+    PRIMARY KEY(source_id,event_id,parent_source_id,parent_event_id),
+    FOREIGN KEY(source_id,event_id) REFERENCES events(source_id,event_id),
+    FOREIGN KEY(parent_source_id,parent_event_id) REFERENCES events(source_id,event_id)
+);
+CREATE TABLE IF NOT EXISTS attempts (
+    attempt_id TEXT PRIMARY KEY,
+    session_id TEXT NOT NULL,
+    task_id TEXT NOT NULL,
+    predecessor_attempt_id TEXT,
+    intent_source_id TEXT NOT NULL,
+    intent_event_id TEXT NOT NULL,
+    selected_json TEXT NOT NULL
+);
+CREATE TRIGGER IF NOT EXISTS events_no_update BEFORE UPDATE ON events BEGIN SELECT RAISE(ABORT,'immutable event'); END;
+CREATE TRIGGER IF NOT EXISTS events_no_delete BEFORE DELETE ON events BEGIN SELECT RAISE(ABORT,'immutable event'); END;
+CREATE TRIGGER IF NOT EXISTS parents_no_update BEFORE UPDATE ON parents BEGIN SELECT RAISE(ABORT,'immutable parent link'); END;
+CREATE TRIGGER IF NOT EXISTS parents_no_delete BEFORE DELETE ON parents BEGIN SELECT RAISE(ABORT,'immutable parent link'); END;
+CREATE TRIGGER IF NOT EXISTS attempts_no_update BEFORE UPDATE ON attempts BEGIN SELECT RA
```

[Inline excerpt bounded; read the full diff page above.]

## Verification receipt

### M003-S01

Verification passed. Complete receipt: `05_governance/reviews/m003/M003-S01_r3_verification.json` (sha256 `7324b37f76bde13a47158d899ef1be5caed8fbb3f121c0fb93f16724c169bc50`)

Review: `05_governance/reviews/m003/M003-S01_r3_review.md` (sha256 `f168c912269450069c883cb0b75d1330ff91dc9e4e86c079c6bb6b4a936e9811`)

## Prior findings

Source review: `05_governance/reviews/m003/M003-S01_r2_review.md` (sha256 `23f0f0f2bc698409ab5d7be2ee4753ed4bdedf872073111bc472644ebe0ebf1c`)
| M003-S01-001 | P1 | open | `08_pkg/relayforge-core/src/lib.rs:1028-1053` records only the latest success and failure. After a success and passing verification, a later `AttemptResult::Unknown` or `NotRun` leaves `latest_success > latest_failure`, so `TaskAccepted` can still be appended. Validate the latest attempt result’s outcome before accepting; add regression coverage for later unknown and not-run results. |

Source review: `05_governance/reviews/m003/M003-S01_r2_review.md` (sha256 `23f0f0f2bc698409ab5d7be2ee4753ed4bdedf872073111bc472644ebe0ebf1c`)
| M003-S01-002 | P1 | open | `08_pkg/relayforge-core/src/lib.rs:979-993, 1091-1116` allows `SessionClosed` with unresolved intents and marks the session closed. `reconcile` rejects closed sessions (`383-388`), recovery rejects them (`867-870`), and closed sessions cannot accept late results (`893-895`). Reject closing with unresolved attempts, or record a recoverable resolution atomically so those intents remain resolvable. Add a regression covering close followed by recovery. |

## Output

Autonomous seats return the complete report for the runner to save. In manual
mode, write only `05_governance/reviews/m003/M003_holistic_review.md` when that tool is granted, or return it for
the architect to save.

Use this exact contract:

```markdown
# Review: M003 round holistic

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
