Corrected BOOT-001 in the checker only: move the actual checksum line after installer execution, and assert exactly one checksum remains in that order.

## Changed files

- scripts/ci_contract_check.py

## Commands run

- Focused checker passed: 24 negative fixtures.
- git diff --check passed.

## What could not be verified

- Authoritative r2 full run follows coded registration; Linux execution remains for subsequent qualification.

## Deviations

- None. Bootstrap workflow, product code and documentation unchanged.
