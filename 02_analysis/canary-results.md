# M002-S02 native canary evidence

**Observed:** 2026-10-25. The disposable Windows canary passed its maintained
full verifier. This qualifies the selected Rust/SQLite implementation path; it
does not accept G1–G6 or authorize packaging, installation, or distribution.

## Toolchain and locked graph

- Target: `x86_64-pc-windows-msvc`.
- `rustc 1.99.1 (58cf8f9c3 2026-10-13)`; `cargo 1.99.1 (5f94df478 2026-08-27)`.
- Pinned tool components: `rustfmt 1.10.0-stable` and `clippy 0.1.99`, both
  from the Rust 1.99.1 build.
- `rusqlite 0.40.0`, exact manifest requirement, with `bundled`; final lock
  selects `libsqlite3-sys 0.38.0`. Runtime `SELECT sqlite_version()` returned
  `3.53.1`, and the locked feature graph included `bundled`.
- During dependency qualification the first lock selected `libsqlite3-sys
  0.38.2`, whose bundled runtime returned `3.53.2`. The lock was corrected to
  `0.38.0` and provisioned explicitly; the final locked runtime and feature
  assertions passed. No binding or product dependency was added for the pin.

## Commands and results

Dependency provisioning, before offline checks:

```text
cargo +1.99.1 generate-lockfile --manifest-path 08_pkg/canary/Cargo.toml
cargo +1.99.1 update --manifest-path 08_pkg/canary/Cargo.toml -p libsqlite3-sys --precise 0.38.0
cargo +1.99.1 fetch --manifest-path 08_pkg/canary/Cargo.toml --locked --target x86_64-pc-windows-msvc
```

The fetch completed successfully. The maintained self-test passed, including
negative checks for empty discovery, missing tests, ignored/skipped tests,
failed execution, and candidate scratch boundaries. The final prescribed full
command passed:

```text
python -B scripts/hermetic_verification.py
```

The launcher ran format check, warning-denying Clippy, all-target build, locked
offline test discovery/execution, doctest discovery/execution, dependency
feature and runtime SQLite assertions, and fresh-process crash/replay checks.
All seven discovered unit/integration tests passed (five unit and two
integration); no required tests were ignored. The declared doctest was
discovered and passed (1/1).

The first full-command attempt materialized its clean candidate but stopped
because the inner verifier compared its owned nested scratch directory with its
reset `TEMP` root. The launcher was changed to require the nested scratch path
to be strictly inside its explicit temporary boundary and outside the source
tree. The self-test then passed positive nested-path and negative sibling/source
path checks. The corrected prescribed full command passed in 19.653 seconds.

## Crash and replay assertions

All CLI operations used a disposable database and separate fresh processes.

| Step | Observed result |
|---|---|
| Seed session and task | Exit 0; 2 event rows |
| Commit intent, then abort at failpoint | Process exit `3221226505` (nonzero); the committed intent remained |
| First restart reconciliation | 4 rows; attempt `interrupted`; result, identity, and cost `unknown` |
| Second reconciliation | Same 4 event IDs and row count; no duplicate event |
| Append synthetic failure, verification, acceptance; reconcile again | 7 rows; completion true; original failure retained |

The tests also confirmed that completion requires both verification and
acceptance, capability facts live in a separate table from performance
observations, duplicate event IDs do not increase row counts, and event rows
carry schema version and creation time.

## Historical candidate snapshot (round 1)

The following tree and file blobs are historical identities from the r1 full
verification snapshot. They are not the candidate identities for authoritative
r3. The r1 snapshot contained 137 files and was materialized before this evidence
document was created.

Tree: `14f84de21388fbf986c53f410e59c7561ccc1759`

| Candidate file | Blob ID |
|---|---|
| `08_pkg/canary/Cargo.lock` | `a48c354eca4b07d08fc82adb694e46927424061d` |
| `08_pkg/canary/Cargo.toml` | `133e75864a245a71ce8307b7370764345e39bdcf` |
| `08_pkg/canary/rust-toolchain.toml` | `778a406b4f2d6cc8be195074edc66cdfce49f0a2` |
| `08_pkg/canary/src/lib.rs` | `f15a5835db813c63d67e2bfd58dd709dba8c4f9d` |
| `08_pkg/canary/src/main.rs` | `c1b9f0dc3e4f66a380bbf7e757f2e1d8483089fb` |
| `08_pkg/canary/tests/storage_contract.rs` | `6ca90079d7d8042c95e5d1121a106d6c636038f8` |
| `scripts/canary_verify.py` | `73bc74c1aa14b969a07e04b997782c85e75ad731` |

The historical r1 run emitted every candidate path/blob pair in Terminal
Commander job `job_01a1736adeaa7537ad102a33189f663e`, passed the clean-candidate
check, and left the real worktree, index bytes, and `HEAD` unchanged. The
evidence document's r1 blob is omitted to avoid self-reference.


## Round 2 negative-guard receipt correction (historical)

- The maintained `canary_verify.py` full inner-candidate path runs the existing negative guard self-check suite once before Rust checks. After successful checks it emits an `INNER CANARY VERIFICATION PASS` line summarizing discovery rejections (empty, missing, ignored), execution rejections (zero, missing, skipped, failed), and scratch boundary checks (nested accepted; sibling/source-tree rejected). The `--self-test` mode retains the same guards and reports the same summary. No recursive verifier invocation was added.
- Focused `python -B scripts/canary_verify.py --self-test` passed for this correction. Its receipt reported the self-test summary and candidate materialization/source-index/HEAD self-test pass.
- The pre-coded worker full attempt stopped before reaching the canary: `roadmap: ok`, then `scripts/ledger.py check` reported drift in `scripts/canary_verify.py`. Terminal Commander job `job_01a17372a7fd764aa135fc791fdd8182` exited 2; no Rust/Cargo checks ran.
- The later r2 authoritative receipt, `05_governance/reviews/m002/M002-S02_r2_verification.json`, reached the candidate but its cached whitespace check compared the candidate index with real `HEAD` and rejected two immutable generated review snippets. The r3 private-baseline correction addressed that check; authoritative r3 passed, as recorded below.


## Round 3 authoritative verification (2026-10-25)

- The r2 authoritative receipt (`05_governance/reviews/m002/M002-S02_r2_verification.json`) reached a candidate, then command 4 `git diff --cached --check` compared the alternate index with real `HEAD` and rejected trailing whitespace in immutable generated review snippets: `05_governance/reviews/m002/M002-S02_e324fb79181feb0e_diff.md:163` and `prompts/for_review_agent/011_M002-S02_r1.md:239`. Those files and the whitespace gates remain unchanged.
- Candidate construction writes new snapshot objects to owned scratch, reads original Git objects through a read-only alternates list, and creates private Git metadata, index and a baseline commit whose tree is the exact candidate tree. The private commit uses the repository’s resolved author/committer identities; candidate verification points `GIT_DIR`, `GIT_INDEX_FILE` and `GIT_WORK_TREE` only at scratch metadata and the materialized source.
- Focused worker self-test passed (Terminal Commander job `job_01a1737bac93739596840eddd725129f`). Candidate tree `7c64db7688c761fb0c9ea8e5a273f390f9034678` and private baseline `b967612e765409425c0996ca096b2762c8de619d` are historical IDs from that focused snapshot, not identities for the authoritative r3 run.
- The pre-coded worker full attempt stopped at ledger drift in `scripts/canary_verify.py` (job `job_01a1737bd3ef7614911f95ed0f036cf0`, exit 2); no Cargo checks ran in that attempt. After the architect recorded the correction, the authoritative r3 full verification passed: exit 0 in 23.156 seconds. Immutable receipt: `05_governance/reviews/m002/M002-S02_r3_verification.json`, SHA-256 `2f00e96b77c04c23b5ead052f67fbfbcee31c09c6124d89430dc9db9a8461ee2`.
- The r3 receipt tail records the negative self-check summary, replay and acceptance assertions, unchanged source/index/HEAD, and clean-candidate full-verification pass. Its bounded output does not retain the earlier candidate tree/blob lines, so the tree and blobs in the preceding section remain explicitly historical r1 identities; no r3 IDs are reconstructed from unrelated hashes.


## Round 4 receipt-tail summary (2026-10-25)

- After candidate verification and the real source/index/HEAD invariance checks pass, the launcher now emits one final `CANDIDATE SNAPSHOT SUMMARY` line containing the candidate tree ID and Git blob IDs for the eight slice product paths: this evidence document, Cargo lock/manifest/toolchain, library, CLI, integration test, and verifier launcher. The full early candidate path/blob output remains intact. Missing any required product path fails closed. Current snapshot IDs are kept in the verification receipt rather than copied into this document.
- Focused `python -B scripts/canary_verify.py --self-test` passed (Terminal Commander job `job_01a17382d16f7545bf98aaa23e07d181`). Its receipt tail retained the candidate snapshot summary with all eight product paths and blob IDs, plus the source/index/HEAD self-test pass. These are focused-run identities and are not asserted as a later full candidate snapshot.
- The prescribed full command was attempted once before the r4 correction was recorded. `roadmap: ok`; `scripts/ledger.py check` reported pre-coded drift in `02_analysis/canary-results.md` and `scripts/canary_verify.py`. Terminal Commander job `job_01a173830673750fa386a6f0560f9c7b` exited 2 before candidate materialization, so no Cargo checks ran in this attempt. The architect must record the r4 files and run authoritative verification.
