Cleared RUSTC_WRAPPER and RUSTC_WORKSPACE_WRAPPER at Windows and Linux job scope. The checker requires both empty values and rejects missing/nonempty overrides.

## Changed files

- .github/workflows/ci.yml
- scripts/ci_contract_check.py
- docs/ci.md
- 02_analysis/ci-results.md

## Commands run

- Focused workflow checker passed with 29 unsafe fixtures rejected.
- Child-only pinned, locked, offline Cargo fetch probe: unavailable wrapper exited 101; empty overrides exited 0.
- Scoped Git diff/status inspected; unrelated paths preserved.

## What could not be verified

- New remote Linux qualification and architect-owned full receipt.

## Deviations

- None; no host settings or product code changed.
