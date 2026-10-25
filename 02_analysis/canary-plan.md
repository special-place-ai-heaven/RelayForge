# M002-S01 canary and release plan

Status: proposed execution plan after this documentation slice is accepted. This
plan admits no product code and does not claim a canary or release has passed.

## Next admission: disposable durability canary

After plan acceptance, the architect admits **M002-S02 — native event durability
and recovery canary**. It is a single bounded qualification of the selected
toolchain, SQLite binding, test discovery, and crash/replay contract; it is not a
product release gate. Independent G1-G6 implementation proceeds after canary
acceptance, one admitted slice at a time.

- Target: `x86_64-pc-windows-msvc`; Rust `1.99.1`, with `rustfmt` and `clippy`.
  The architect observed `rustc 1.99.1 (58cf8f9c3 2026-10-13)` and
  `cargo 1.99.1 (5f94df478 2026-08-27)` through `rustup run`; global default
  remains `1.98.1`. This establishes tool availability only, not linker,
  dependency, SQLite, or application qualification. The product workspace pins
  `channel = "1.99.1"`, `profile = "minimal"`, both components, and the target
  in `rust-toolchain.toml`; leave the global default untouched.
- Binding candidate: `rusqlite = { version = "=0.40.0", default-features =
  false, features = ["bundled"] }`. The upstream v0.40.0 release notes say it
  bundles SQLite 3.53.1; its tagged manifest maps `bundled` to
  `libsqlite3-sys/bundled`, and that tagged crate is 0.38.0. This is a
  source-backed candidate, not a compatibility claim: the canary must resolve,
  build, and run it on Rust 1.99.1/MSVC. Pin every resolution in `Cargo.lock`,
  commit the lock with the canary, and record `cargo tree --locked -i
  libsqlite3-sys` plus runtime `SELECT sqlite_version()` output. Require the
  runtime value `3.53.1`; fail if system SQLite was linked. Sources:
  [rusqlite v0.40.0 release](https://github.com/rusqlite/rusqlite/releases/tag/v0.40.0),
  [tagged feature manifest](https://github.com/rusqlite/rusqlite/blob/v0.40.0/Cargo.toml),
  [tagged libsqlite3-sys manifest](https://github.com/rusqlite/rusqlite/blob/v0.40.0/libsqlite3-sys/Cargo.toml).
- Tracked source: keep `Cargo.toml`, `Cargo.lock`, `rust-toolchain.toml`, Rust
  source, tests, and fixtures in `08_pkg/canary/`; keep the stdlib-only
  orchestrator in `scripts/canary_verify.py`. These are reviewable source, not
  disposable scratch. The canary Cargo manifest is self-contained at
  `08_pkg/canary/Cargo.toml` so it does not rely on an untracked root workspace.
- Isolated runtime/build output: create a unique directory under `$env:TEMP`,
  outside the checkout, for `CARGO_TARGET_DIR`, the SQLite database, logs, and
  crash artifacts. Do not copy source there. Record the exact candidate Git tree
  object ID and each included file's Git blob ID, including the canary manifest,
  lockfile, toolchain file, source, tests, fixtures, and launcher. Run checks from a clean
  materialization of that candidate tree so all tested source and launcher files
  are tracked and reproducible. Capture commands, exit codes, test counts,
  SQLite version, and replay assertions in slice evidence. Delete only the
  owned temporary output directory after retaining evidence; keep tracked source.
- Scope: minimal library plus CLI and SQLite schema, synthetic records only; no
  adapters, provider calls, live dispatch, credentials, or private transcripts.
  Write a session, task, pre-dispatch attempt, simulated tool failure, later
  verification, and explicit acceptance. Keep capability facts separate from
  performance observations. Use stable IDs and retain the failure after
  acceptance.

## Canary verification contract

Architect-owned setup precedes the coding prompt: prepare/select Python for the
existing verifier without adding a tracked machine path; configure
`scripts/hermetic_verification.py` to call the planned tracked launcher
`python -B scripts/canary_verify.py`; ensure Rust 1.99.1, `rustfmt`, `clippy`,
and the MSVC target are available; and establish the exact candidate-tree
materialization and evidence boundary. The launcher and canary source are
created by the admitted coding slice at their tracked paths, then the configured
outer verifier runs only after those files exist in the clean materialized
candidate. This separates pre-prompt environment/command setup from post-code
product verification: no missing-script baseline is described as passing, and
the verifier does not depend on temporary source. The canary coder owns
`08_pkg/canary/rust-toolchain.toml`, `Cargo.lock`, source/tests/fixtures, and
`scripts/canary_verify.py`. Do not treat template verification as product
verification until the configured product command runs successfully.

After dependencies are provisioned, all Cargo checks run offline and locked:
provision the exact locked graph once with
`cargo +1.99.1 fetch --manifest-path 08_pkg/canary/Cargo.toml --locked --target x86_64-pc-windows-msvc`; record its
result. Every check below explicitly targets the selected Windows triple.

```powershell
cargo +1.99.1 fmt --manifest-path 08_pkg/canary/Cargo.toml -- --check
cargo +1.99.1 clippy --manifest-path 08_pkg/canary/Cargo.toml --all-targets --target x86_64-pc-windows-msvc --locked --offline -- -D warnings
cargo +1.99.1 build --manifest-path 08_pkg/canary/Cargo.toml --all-targets --target x86_64-pc-windows-msvc --locked --offline
cargo +1.99.1 test --manifest-path 08_pkg/canary/Cargo.toml --all-targets --target x86_64-pc-windows-msvc --locked --offline -- --list
cargo +1.99.1 test --manifest-path 08_pkg/canary/Cargo.toml --doc --target x86_64-pc-windows-msvc --locked --offline -- --list
```

Before coding acceptance, save the discovery output and exact expected unit and
integration test IDs. The launcher must fail on a missing ID, an empty intended
suite, or a skipped required test, then execute the discovered set and report
discovered/passed/failed/ignored counts. Include doctest discovery where doctests
are declared. Do not add `nextest` or another dependency for this guard.

The maintained full check is exactly:

```powershell
python -B scripts/hermetic_verification.py
```

Its configured product command runs the launcher, which runs formatting,
Clippy, all-target build, discovery/guard, all unit and integration tests,
doctests, schema/fixture checks, and runtime SQLite-version assertion. It must
return nonzero on any failed step and use no network after provisioning. Run it
from a clean checkout/candidate after the focused checks. Record its receipt and
the discovery/execution counts; a scaffold-only pass, zero tests, or silently
skipped tests fails the gate.

Configure its single product argv as
`["python", "-B", "scripts/canary_verify.py"]`. That tracked stdlib-only
launcher is the maintained exact Cargo orchestration and test-ID guard; it runs
the Cargo commands with `--manifest-path 08_pkg/canary/Cargo.toml` and the same
target/lock/offline flags. The outer verifier is invoked from the clean
  materialized candidate and records the candidate tree identity and launcher
  as the product check.

Crash/replay acceptance uses a dedicated database and these CLI process
boundaries (the canary implements these commands and JSON assertions):

```powershell
cargo +1.99.1 run --manifest-path 08_pkg/canary/Cargo.toml --locked --offline --target x86_64-pc-windows-msvc --bin relayforge -- canary seed --database "$db"
cargo +1.99.1 run --manifest-path 08_pkg/canary/Cargo.toml --locked --offline --target x86_64-pc-windows-msvc --bin relayforge -- canary crash-after-intent --database "$db"
cargo +1.99.1 run --manifest-path 08_pkg/canary/Cargo.toml --locked --offline --target x86_64-pc-windows-msvc --bin relayforge -- canary reconcile --database "$db" --json
cargo +1.99.1 run --manifest-path 08_pkg/canary/Cargo.toml --locked --offline --target x86_64-pc-windows-msvc --bin relayforge -- canary reconcile --database "$db" --json
```

`crash-after-intent` aborts after the transaction commits the pre-dispatch
intent but before any dispatch result exists; record its expected nonzero exit.
On fresh-process restart require state `interrupted`, unresolved
result/identity/cost `unknown`, and identical event IDs/row counts across both
reconciliations. Then append a synthetic failure receipt, verification, and
explicit acceptance through the same CLI, replay once more, and require the
original failure to remain, no duplicate events, and completion to require both
verification and acceptance. No real tool or model is called. Record each
process command, exit code, event count, and assertion result.

Canary pass requires exact toolchain/runtime evidence; successful format,
Clippy, build, discovered and executed nonempty tests; runtime SQLite `3.53.1`;
the abort/restart/replay assertions above; clean offline full verification; and
bounded disposable paths. Any unmet item remains open with its observed evidence
and actor/action. Passing this canary qualifies an implementation path only; it
does not accept G1-G6 or authorize release.

## Ordered release implementation slices

The architect assigns actual roadmap IDs and read/write envelopes when admitting
each slice. Order is dependency order; acceptance evidence belongs in the
existing ledger/review flow.

| Order | Gate and slice | Observable acceptance |
|---|---|---|
| 1 | **G1 — Reproducible native delivery.** Establish the Rust workspace, embeddable library, CLI, pinned toolchain/dependencies, clean-checkout build, test discovery and full check. Defer packaging and install/uninstall execution until D012 is clear. | A fresh Windows checkout builds offline with the locked toolchain; required tests are discovered and executed; maintained verification passes; local install/uninstall evidence is recorded when packaging is authorized. |
| 2 | **G2 — Durable local evidence.** Implement schema-versioned SQLite events, transactional pre-dispatch intent, idempotent ingestion/reconciliation, ownership, redaction, and restart safety. | Crash at each required persistence boundary and reopen in a fresh process; duplicate replay adds no rows; failures survive recovery; unknown stays unknown absent evidence; active sessions remain unclaimed; redaction checks pass. |
| 3 | **G3 — Inventory and ranking.** Add sourced exact identities/capability evidence, deterministic eligibility, local outcome summaries, uncertainty/missingness, and explainable comparison to a fixed baseline. | Contract tests show configured/unknown/unsupported distinctions, `no_eligible_candidate`, and `insufficient_evidence`; matched comparisons retain retries and repair with provenance. |
| 4 | **G4 — Codex and Claude adapters.** Build adapters separately against documented versioned coverage tables and supported observation paths. | Each adapter has a fresh-process local smoke receipt that records at least one real supported observation into the core; recommendation consumption/adoption is recorded; unavailable switching/hooks are explicit. Fixtures alone do not pass. |
| 5 | **G5 — Bounded JEV integration.** Add the provider boundary after eligible candidates and versioned questions exist; default to shadow mode. | Contract tests cover bounds, validation, abstention, timeout/error, caching/bypass, and shadow behavior. Live validation remains separately reported and blocked without credentials and explicit incremental spend authority. |
| 6 | **G6 — Evaluation and release readiness.** Run predeclared representative scenarios; account for total accepted work, failed attempts, recovery, usage completeness, and capture/routing overhead; finish docs and independent review. | Reproducible evidence covers success, wrong tools, retries/escalation, sparse evidence, interruption/restart, and unavailable capability; review has no open P0-P2 findings. Release claim includes limits and observed—not inferred—cost/usage. |

## D012 / Q001 distribution gate

Q001 remains open. Continue independent coding, local builds, tests, and
verification while it is open. Before creating package/install artifacts or
distributing them, the architect must either obtain and record an applicable
permission grant or review packaging inputs proving all imported and derived
template material is excluded. Inspect the resulting artifact contents before
distribution and retain notices for included third-party material. D012 blocks
G1 packaging/delivery and G6 release readiness; it does not block the canary or
independent implementation. The artifact allowlist currently described in Q001
is a guess until proven, not permission or closure evidence.
