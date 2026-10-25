Capability evaluation now checks exact scope and temporal applicability before
classifying status. A four-status matrix covers revision, harness, configuration,
runtime, future and expired mismatches; the test is required by verifier discovery.

## Changed files
- 08_pkg/relayforge-core/src/lib.rs
- scripts/product_verify.py

## Commands run
- Targeted regression before fix — FAIL as expected for Configured with mismatched profile revision, exit 101.
- cargo +1.99.1 fmt --all — PASS.
- python -B scripts/product_verify.py --focused — PASS; 11 unit, 14 integration, 1 doctest.
- git diff --check — PASS.

## What could not be verified
- Architect-owned full check pending at handoff.

## Deviations from the prompt
- None.
