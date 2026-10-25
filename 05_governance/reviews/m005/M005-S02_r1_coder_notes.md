Implemented immutable pre-attempt task context, complete attempt/repair accounting,
conservative descriptive comparisons, and fresh-process CLI paths. A failing
regression exposed late UsageCost receipts being treated as later unresolved work;
the narrow correction distinguishes accounting receipts from new work.

## Changed files
- 08_pkg/relayforge-core/src/accounting.rs
- 08_pkg/relayforge-core/src/lib.rs
- 08_pkg/relayforge-core/tests/storage_contract.rs
- 07_app/relayforge-cli/src/main.rs
- scripts/product_verify.py
- 02_analysis/accounting-results.md

## Commands run
- Targeted accounting regression — FAIL before late-receipt correction; PASS in final focused run.
- cargo +1.99.1 fmt --manifest-path Cargo.toml --all — PASS.
- python -B scripts/product_verify.py --focused — initial formatting, Clippy and discovery failures corrected; final PASS: 14 unit, 15 integration, 1 doctest.
- Architect git diff --check — PASS.

## What could not be verified
- Native adapters, production calibration and trusted runtime binding; standalone CLI reports binding_unavailable.
- Architect-owned full verification pending at handoff.

## Deviations from the prompt
- None reported.
