# Review prompt: M002 — Exact Rust and SQLite canary admission (round holistic)

Read `AGENTS.md` first. Do not change product files. Use the receipt as execution
evidence; do not rerun verification unless this prompt explicitly says so.

## Objective and acceptance

Judge holistic closure of M002 across its accepted slices.

### M002-S01

- Specify pinned toolchain, SQLite binding selection evidence and isolated workspace.
- Specify exact build, test discovery, full verification and crash/replay checks.
- Map release gates G1-G6 to ordered implementation slices with observable acceptance.
- Identify architect-owned verifier/runtime setup before issuing product prompts.
- State next executable admission and continue campaign after plan acceptance.
- Carry D012 distribution gate into packaging plans; continue independent implementation while Q001 is open.

### M002-S02

- Retain a self-contained library and relayforge canary CLI under 08_pkg/canary with Rust 1.99.1 pin, PolyForm metadata and complete Cargo.lock; use rusqlite exactly 0.40.0 with bundled SQLite 3.53.1.
- Prove toolchain and target versions, locked dependency graph, formatting, warning-free Clippy, all-target build, nonempty discovered and executed unit/integration tests and doctests where declared; all checks offline after explicit dependency provisioning.
- The maintained scripts/canary_verify.py command must fail closed on missing, zero, skipped or failed required tests and include negative self-checks of those guards.
- Materialize the exact candidate Git tree in an isolated clean source directory using an alternate index without changing the real index or HEAD; retain tree and blob identities and execute the maintained full verifier there without recursion.
- Use separate temporary output/database directories and fresh CLI processes to seed records, abort after committed intent, reconcile twice without duplicates, and append failure then verification then acceptance while preserving unknown identity/cost and original failure history.
- Tests prove completion requires both verification and acceptance, capability facts are distinct from observations, duplicate events do not inflate counts, and runtime SQLite is 3.53.1 with bundled feature evidence.
- Verify source/index/HEAD remain unchanged by full verification; bounded scratch cleanup never deletes outside owned scratch; enforce per-process timeouts within the 900-second full check.
- Record portable canary evidence in 02_analysis/canary-results.md including commands, versions, counts, crash exit, snapshot and replay assertions; no secrets, private transcripts, raw runtime stores or machine paths.

## Non-goals

### M002-S01

- Product implementation or paid provider calls within this planning slice.
- Reopening accepted foundation work or changing release completion criteria.

### M002-S02

- Native adapters, real model or tool dispatch, provider calls, packaging, installation or publication.
- Reusing canary as a finished product or claiming G1-G6 release completion.

## Read first

### M002-S01

- `00_brief/CONTEXT.md`
- `00_brief/constraints.md`
- `00_brief/decisions.md`
- `00_brief/release-contract.md`
- `02_analysis/foundation-contract.md`
- `docs/project_checks.md`
- `questions/open/Q001-template-distribution-permission.md`

### M002-S02

- `08_pkg/CONTEXT.md`
- `02_analysis/CONTEXT.md`
- `00_brief/decisions.md`
- `00_brief/release-contract.md`
- `02_analysis/foundation-contract.md`
- `02_analysis/canary-plan.md`
- `docs/project_checks.md`
- `scripts/hermetic_verification.py`
- `questions/open/Q001-template-distribution-permission.md`

## Implementation boundary

Allowed prefixes: ### M002-S01

`02_analysis/canary-plan.md`

### M002-S02

`08_pkg/canary/`, `scripts/canary_verify.py`, `02_analysis/canary-results.md`

Forbidden: ### M002-S01

`00_brief/`, `05_governance/`, `prompts/`, `questions/answered/`, `roadmap.yaml`, `frutlups.toml`, `AGENTS.md`, `CLAUDE.md`

### M002-S02

`00_brief/`, `05_governance/`, `prompts/`, `questions/answered/`, `roadmap.yaml`, `frutlups.toml`, `AGENTS.md`, `CLAUDE.md` and all other paths. These describe the coder's scope;
review remains product-read-only. Read the evidence artifacts named below using
your file-reading tools. Their categories describe volume, not authority.

## Declared verification

Focused:

### M002-S01

```text
python -B scripts/roadmap.py check
```

### M002-S02

```text
python -B scripts/canary_verify.py --self-test
```

Full:

### M002-S01

```text
python -B scripts/hermetic_verification.py
```

Execution policy: runtime {}; timeout 120 seconds; observation process.

### M002-S02

```text
python -B scripts/hermetic_verification.py
```

Execution policy: runtime {}; timeout 900 seconds; observation process.

## Changed files

9 cumulative paths. Complete manifest: `05_governance/reviews/m002/M002_15492fd29ce5e197_paths.json` (sha256 `15492fd29ce5e19791faf84be22a4b5ff8ebc3089082fe5be734d435c91edc55`; read as a file, starting at line 1)

## Code diff

Bounded diff page: `05_governance/reviews/m002/M002_1d445727db9756bc_diff.md` (sha256 `1d445727db9756bc0c7eb5065fe1561668c2b442f6b40ed33f4b42fc096a90e3`; read as a file, starting at line 1)

```diff
git diff 89c0937d2fa33dceedc59ca217e3ba805c7e3f03..HEAD --stat (selected paths)
 02_analysis/canary-plan.md              | 164 ++++++++++++++
 02_analysis/canary-results.md           | 117 ++++++++++
 08_pkg/canary/Cargo.lock                |  92 ++++++++
 08_pkg/canary/Cargo.toml                |  17 ++
 08_pkg/canary/rust-toolchain.toml       |   5 +
 08_pkg/canary/src/lib.rs                | 336 +++++++++++++++++++++++++++++
 08_pkg/canary/src/main.rs               |  77 +++++++
 08_pkg/canary/tests/storage_contract.rs |  47 ++++
 scripts/canary_verify.py                | 844 ++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++
 9 files changed, 1699 insertions(+)

Per-slice diffs: current working tree against accepted base (bounded; complete manifests remain authoritative).
### M002-S02

diff --git a/08_pkg/canary/src/lib.rs b/08_pkg/canary/src/lib.rs
new file mode 100644
index 0000000..f15a583
--- /dev/null
+++ b/08_pkg/canary/src/lib.rs
@@ -0,0 +1,336 @@
+//! Minimal durable-event canary for RelayForge.
+//!
+//! ```
+//! assert!(relayforge_canary::completion_requires_both(true, true));
+//! assert!(!relayforge_canary::completion_requires_both(true, false));
+//! ```
+
+use rusqlite::{Connection, OptionalExtension, Transaction, params};
+use std::path::Path;
+
+const SCHEMA: &str = "
+PRAGMA foreign_keys = ON;
+CREATE TABLE IF NOT EXISTS events (
+    event_id TEXT PRIMARY KEY,
+    schema_version INTEGER NOT NULL,
+    created_at TEXT NOT NULL,
+    session_id TEXT NOT NULL,
+    task_id TEXT NOT NULL,
+    attempt_id TEXT,
+    kind TEXT NOT NULL,
+    result TEXT NOT NULL,
+    identity TEXT NOT NULL,
+    cost TEXT NOT NULL
+);
+CREATE TABLE IF NOT EXISTS capability_facts (
+    capability_id TEXT PRIMARY KEY,
+    status TEXT NOT NULL,
+    evidence TEXT NOT NULL
+);
+CREATE TABLE IF NOT EXISTS observations (
+    observation_id TEXT PRIMARY KEY,
+    metric TEXT NOT NULL,
+    value TEXT NOT NULL
+);
+";
+
+#[derive(Debug)]
+pub struct Snapshot {
+    pub state: &'static str,
+    pub result: &'static str,
+    pub identity: &'static str,
+    pub cost: &'static str,
+    pub complete: bool,
+    pub event_ids: Vec<String>,
+    pub event_count: usize,
+}
+
+pub fn completion_requires_both(verified: bool, accepted: bool) -> bool {
+    verified && accepted
+}
+
+pub fn open(path: &Path) -> rusqlite::Result<Connection> {
+    let connection = Connection::open(path)?;
+    connection.execute_batch(SCHEMA)?;
+    Ok(connection)
+}
+
+fn add_event(
+    tx: &Transaction<'_>,
+    id: &str,
+    attempt: Option<&str>,
+    kind: &str,
+    result: &str,
+    identity: &str,
+    cost: &str,
+) -> rusqlite::Result<()> {
+    tx.execute(
+        "INSERT INTO events(
+             event_id, schema_version, created_at, session_id, task_id, attempt_id,
+             kind, result, identity, cost
+         ) VALUES (
+             ?1, 1, strftime('%Y-%m-%dT%H:%M:%fZ', 'now'), 'session-001', 'task-001',
+             ?2, ?3, ?4, ?5, ?6
+         )
+         ON CONFLICT(event_id) DO NOTHING",
+        params![id, attempt, kind, result, identity, cost],
+    )?;
+    Ok(())
+}
+
+pub fn seed(connection: &mut Connection) -> rusqlite::Result<()> {
+    let tx = connection.transaction()?;
+    add_event(
+        &tx,
+        "session.created",
+        None,
+        "session",
+        "recorded",
+        "unknown",
+        "unknown",
+    )?;
+    add_event(
+        &tx,
+        "task.created",
+        None,
+        "task",
+        "recorded",
+        "unknown",
+        "unknown",
+    )?;
+    tx.execute(
+        "INSERT INTO capability_facts VALUES ('tool-observation', 'configured', 'synthetic fixture')
+         ON CONFLICT(capability_id) DO NOTHING",
+        [],
+    )?;
+    tx.execute(
+        "INSERT INTO observations VALUES ('observation-001', 'tool-call-result', 'failure')
+         ON CONFLICT(observation_id) DO NOTHING",
+        [],
+    )?;
+    tx.commit()
+}
+
+pub fn commit_intent(connection: &mut Connection) -> rusqlite::Result<()> {
+    let tx = connection.transaction()?;
+    add_event(
+        &tx,
+        "attempt.intent",
+        Some("attempt-001"),
+        "dispatch_intent",
+        "pending",
+        "unknown",
+        "unknown",
+    )?;
+    tx.commit()
+}
+
+pub fn append_failure(connection: &mut Connection) -> rusqlite::Result<()> {
+    let tx = connection.transaction()?;
+    add_event(
+        &tx,
+        "tool.failure",
+        Some("attempt-001"),
+        "tool_call",
+        "failure",
+        "unknown",
+        "unknown",
+    )?;
+    tx.commit()
+}
+
+pub fn append_verification(connection: &mut Connection) -> rusqlite::Result<()> {
+    let tx = connection.transaction()?;
+    add_event(
+        &tx,
+        "task.verified",
+        Some("attempt-001"),
+        "verification",
+    
[diff truncated; complete manifest remains authoritative]
### M002-S02

diff --git a/08_pkg/canary/src/main.rs b/08_pkg/canary/src/main.rs
new file mode 100644
index 0000000..c1b9f0d
--- /dev/null
+++ b/08_pkg/canary/src/main.rs
@@ -0,0 +1,77 @@
+use relayforge_canary::{
+    append_acceptance, append_failure, append_verification, commit_intent, open, reconcile, seed,
+    snapshot,
+};
+use rusqlite::Connection;
+use std::env;
+use std::path::PathBuf;
+use std::process;
+
+fn database(args: &[String]) -> Result<PathBuf, String> {
+    args.windows(2)
+        .find(|pair| pair[0] == "--database")
+        .map(|pair| PathBuf::from(&pair[1]))
+        .ok_or_else(|| "missing --database path".to_owned())
+}
+
+fn print_snapshot(connection: &Connection) -> rusqlite::Result<()> {
+    let state = snapshot(connection)?;
+    println!(
+        "{{\"state\":\"{}\",\"result\":\"{}\",\"identity\":\"{}\",\"cost\":\"{}\",\"complete\":{},\"event_count\":{},\"event_ids\":[{}]}}",
+        state.state,
+        state.result,
+        state.identity,
+        state.cost,
+        state.complete,
+        state.event_count,
+        state
+            .event_ids
+            .iter()
+            .map(|id| format!("\"{id}\""))
+            .collect::<Vec<_>>()
+            .join(",")
+    );
+    Ok(())
+}
+
+fn run() -> Result<(), String> {
+    let args = env::args().skip(1).collect::<Vec<_>>();
+    if args.first().map(String::as_str) == Some("sqlite-version") {
+        let connection = Connection::open_in_memory().map_err(|error| error.to_string())?;
+        let version: String = connection
+            .query_row("SELECT sqlite_version()", [], |row| row.get(0))
+            .map_err(|error| error.to_string())?;
+        println!("{version}");
+        return Ok(());
+    }
+    if args.first().map(String::as_str) != Some("canary") {
+        return Err("usage: relayforge canary <command> --database PATH".to_owned());
+    }
+    let command = args.get(1).map(String::as_str).unwrap_or_default();
+    let path = database(&args)?;
+    let mut connection = open(&path).map_err(|error| error.to_string())?;
+    match command {
+        "seed" => seed(&mut connection),
+        "crash-after-intent" => {
+            commit_intent(&mut connection).map_err(|error| error.to_string())?;
+            process::abort();
+        }
+        "reconcile" => {
+            reconcile(&mut connection).map_err(|error| error.to_string())?;
+            return print_snapshot(&connection).map_err(|error| error.to_string());
+        }
+        "record-failure" => append_failure(&mut connection),
+        "verify" => append_verification(&mut connection),
+        "accept" => append_acceptance(&mut connection),
+        _ => return Err(format!("unknown canary command: {command}")),
+    }
+    .map_err(|error| error.to_string())?;
+    print_snapshot(&connection).map_err(|error| error.to_string())
+}
+
+fn main() {
+    if let Err(error) = run() {
+        eprintln!("relayforge: {error}");
+        process::exit(2);
+    }
+}
### M002-S02

diff --git a/08_pkg/canary/tests/storage_contract.rs b/08_pkg/canary
```

[Inline excerpt bounded; read the full diff page above.]

## Verification receipt

### M002-S01

Verification passed. Complete receipt: `05_governance/reviews/m002/M002-S01_r2_verification.json` (sha256 `c89efd43ee108032ea6ae90c11d5e12679d7df62eb0d5fa8e580f48f52d653d2`)

```json
{"schema":"frutlups.receipt/2","slice":"M002-S01","round":2,"t":"2026-10-25T14:00:03Z","base_commit":"89c0937d2fa33dceedc59ca217e3ba805c7e3f03","tree_dirty_before":true,"commands":[{"label":"full","argv":["python","-B","scripts/hermetic_verification.py"],"exit":0,"secs":0.547,"stdout_tail":"roadmap: ok\r\nledger: ok\r\n","stderr_tail":"","timed_out":false}],"tree_clean_after":false,"ok":true,"manifest":{"path":"05_governance/reviews/m002/M002-S01_r2_manifest_5216e1384c2a8258.json","sha":"5216e1384c2a8258cdd04adbf63ce57b9434c9dff1a8b0a356f81b0e3f6d5d3b"},"witness":{"before":"06233ac55a8dd4e755981007ef28247c3cd44fdb7784f75976da94f52f8497f5","after":"06233ac55a8dd4e755981007ef28247c3cd44fdb7784f75976da94f52f8497f5","stable":true,"head":"89c0937d2fa33dceedc59ca217e3ba805c7e3f03","index":"f442345d717acf6c5b0dcee3b1508374fe0a33da7631eb33878e8ea03b960679","product":"02a8a4e6b6b1f196aa2918343986db7d9dd50520a8c66d4fea3c8814bc649987"},"runtime":{"python":"3.12.10","implementation":"CPython","source":"invoking_python","sha":"964fbaadf28592f4f81be6e9bccfdd48722d3d3c188e1d71d4d5dd72e0c2f8a1"},"observation":"process"}
```

Review: `05_governance/reviews/m002/M002-S01_r2_review.md` (sha256 `2c1cfda3ba3d9e6aecec5f836f4d97c311180cd16d17a1af697c05d72aff6c51`)

### M002-S02

Verification passed. Complete receipt: `05_governance/reviews/m002/M002-S02_r4_verification.json` (sha256 `0cf2de0910e1e6e64b39357710c849642b37fbfe4f654ed13a4bb3bcb5cbcb50`)

Review: `05_governance/reviews/m002/M002-S02_r4_review.md` (sha256 `255d093f916e28b8006dcad673b18a5a09e7bed610bdecffb77b3ee6bf79e5d3`)

## Prior findings

Source review: `05_governance/reviews/m002/M002-S01_r1_review.md` (sha256 `36e0871170a1e07dc60a8bb9f68952d81480549e2d376f9edb2023067db68d82`)
| M002-S01-001 | P1 | open | The canary plan says both to keep source in a disposable TEMP directory and delete it after retaining evidence, and that the canary coder owns tracked workspace files and a maintained verifier launcher. It does not explain how those files persist in the checkout or how the clean-checkout verifier exercises them. This leaves the immediate next admission without a reproducible source and verification arrangement. |

Source review: `05_governance/reviews/m002/M002-S02_r3_review.md` (sha256 `4152e25ce931ba8fa1867747df2af6f24635b84e8f9052a0f93206e7adde976a`)
| M002-S02-P2-2 | P2 | open | `02_analysis/canary-results.md` still says the r3 authoritative full verification is pending. It records earlier candidate identities, but not the successful r3 run’s candidate tree and blob identities or final verification evidence, as required. |

## Output

Autonomous seats return the complete report for the runner to save. In manual
mode, write only `05_governance/reviews/m002/M002_holistic_review.md` when that tool is granted, or return it for
the architect to save.

Use this exact contract:

```markdown
# Review: M002 round holistic

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
