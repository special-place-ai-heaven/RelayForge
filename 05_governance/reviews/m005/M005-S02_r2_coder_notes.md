Added table-driven cohort mismatch tests (label provenance, policy, harness and
adapter IDs/versions, configuration), profile-revision mismatch and overlapping
Wilson intervals. Each asserts an explicit reason and no preferred alternative.
No production implementation change was needed.

## Changed files
- 08_pkg/relayforge-core/src/accounting.rs (tests)
- scripts/product_verify.py (required discovery)

## Commands run
- cargo +1.99.1 test --manifest-path Cargo.toml -p relayforge-core --lib comparison_ --locked --offline — PASS, 4 tests.
- cargo +1.99.1 fmt --manifest-path Cargo.toml --all — PASS.
- python -B scripts/product_verify.py --focused — new-test Clippy issue corrected; final PASS: 17 unit, 15 integration, 1 doctest.
- Architect git diff --check — PASS.

## What could not be verified
- Authoritative full check pending at handoff.

## Deviations from the prompt
- None.
