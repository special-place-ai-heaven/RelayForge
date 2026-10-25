Implemented the requested runner workflow, native-target verifier support and five explicit product stages. Product domain logic and schema are unchanged.

## Changed files

- .github/workflows/ci.yml
- scripts/ci_contract_check.py, scripts/product_verify.py, scripts/canary_verify.py
- rust-toolchain.toml, 08_pkg/canary/rust-toolchain.toml
- docs/ci.md, 02_analysis/ci-results.md

## Commands run

- Workflow contract check: passed, 14 unsafe fixtures rejected.
- Final focused product: passed, 5 unit, 13 integration/CLI and 1 doctest; fmt, Clippy, build and SQLite checks passed.
- Focused canary: passed; candidate materialization self-test passed ledger ancestry and source/index/HEAD stability.
- git diff --check: passed.

## What could not be verified

- Linux and GitHub-hosted Windows execution remain for S02.
- Authoritative full verification is architect-owned after coded registration.

## Deviations

- None; no product domain, governance, commit or push changes by worker.
