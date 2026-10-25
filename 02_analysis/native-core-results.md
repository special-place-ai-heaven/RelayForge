# Native core and CLI results

M003-S01 implements `relayforge-core` as a typed Rust library and `relayforge`
as a separate CLI. The library accepts closed schema-version-1 events, enforces
bounded metadata, writes immutable rows and parent links transactionally, and
keeps session ownership until explicit attributed recovery. The CLI has no
dispatch or provider operation.

## CLI example

```text
relayforge init --database relayforge.db
relayforge append --database relayforge.db < session-open.json
relayforge append --database relayforge.db < task-created.json
relayforge show --database relayforge.db --session session-a --limit 50
relayforge reconcile --database relayforge.db --session session-a --actor owner-b --reason "recovery approved"
```

`append` reads exactly one event object from standard input. Its fixed success
response identifies `appended` or an identical `replayed` event. Failures return
a stable error code without echoing rejected content. `show` requires a session
or task scope and caps each response at 200 events.

An event contains `schema_version`, `source_id`, `event_id`, optional explicit
`parents`, and one closed `kind` variant. Variants cover session open/close and
attributed recovery, task creation, physical attempt intent and result, tool
result, component usage/cost/timing, verification, explicit acceptance, and
interruption. Attempt selection and observed identity are separate; unknown
identity, usage, cost, and timing carry a reason. Required check IDs are part
of task creation. Acceptance requires the latest attempt, a successful result,
passing current evidence for every required check, and actor/evidence
attribution.

## Verification receipt

The product verifier reports the exact compiler, dependency graph, bundled
SQLite runtime, discovered and executed test counts, and candidate tree/file
identities. The full hermetic command runs the accepted canary and this product
suite from one isolated candidate materialization; product and canary use
separate scratch databases and target directories. The full verifier also
checks source, Git index, and HEAD invariance.

Worker-observed validation includes a direct isolated-target `cargo build` and
`cargo test --workspace --all-targets` run on the pinned Windows target. The
observed test phase passed 15 tests (4 unit and 11 integration), with no failures
or ignored tests. It covered fresh-process CLI use, committed versus precommit
crash exits, active-owner protection and explicit recovery, stale recovery
rejection, exact replay, parent scope, sensitive-input rejection, and the
two-process claim race. Earlier diagnostic runs exposed missing CLI path
injection, an incomplete recoverable-session constraint, and a race-test
barrier placement deadlock; those were corrected before this observed pass.

The r2 focused product verifier passed 17/17 unit and integration tests, zero
ignored tests, and 1/1 doctest; exact dependency and runtime SQLite checks also
passed. Its first run exposed a mismatched owner in the crate doctest, which was
corrected before the final r2 focused run.

The r1 receipt at `05_governance/reviews/m003/M003-S01_r1_verification.json` is
historical and records the private-baseline ancestry failure: its inner ledger
check saw multiple pending commit operations before Rust verification. The r2
materializer fix parents the scratch baseline to observed real HEAD, checks
parent/ancestry, and runs `ledger.py check` before any build. Its worker-observed
self-test passed with `ledger: ok` and source/index/HEAD invariance. The r3
focused product verifier passed 18/18 unit and integration tests, zero ignored
tests, and 1/1 doctest. New regression tests first reproduced acceptance after
later unknown/not-run results and closing with an unresolved intent; they now
verify acceptance rejection, close refusal without writes, recovery plus
reconciliation and a late result followed by successful close, close refusal
after unknown, and successful close after explicit not-run. The authoritative
receipt for this round is
`05_governance/reviews/m003/M003-S01_r3_verification.json`; this results note is
not that receipt. This slice does not claim adapters, routing, packaging, or
release completion.
