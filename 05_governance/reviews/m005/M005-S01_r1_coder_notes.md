Implemented immutable inventory storage, conservative eligibility evaluation with
separate host-supplied current state, and fresh-process CLI diagnostics. The
standalone CLI reports binding_unavailable and does not promote imported JSON to
runtime confirmation. Final focused verification followed expiry and bounded-read
hardening plus added state/selection assertions.

## Changed files
- 08_pkg/relayforge-core/src/lib.rs
- 07_app/relayforge-cli/src/main.rs
- 08_pkg/relayforge-core/tests/storage_contract.rs
- scripts/product_verify.py
- 02_analysis/inventory-results.md

## Commands run
- cargo +1.99.1 fmt --all — PASS.
- python -B scripts/product_verify.py --focused — PASS; 10 unit, 14 integration, 1 doctest.
- git diff --check — PASS.

## What could not be verified
- Live G4 runtime binding/observations; standalone CLI reports binding_unavailable.
- Authoritative full check remains architect-owned at handoff.

## Deviations from the prompt
- None reported by worker.
