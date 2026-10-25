Corrected scratch candidate ancestry: commit-tree now parents the real baseline HEAD, asserts exact parent and ancestry, then checks the candidate ledger before Rust builds. No product behavior changed.

## Changed files

- scripts/canary_verify.py
- 02_analysis/native-core-results.md

## Commands run

- Materializer self-test passed, job job_01a173a60212762b841b93ef5b86a58b: ledger ok, clean candidate, source/index/HEAD unchanged.
- Product code unchanged since focused 17/17 unit/integration and 1/1 doctest pass.
- Historical r1 full verification failed at candidate ledger check: more than one pending commit operation. Diagnosis: private orphan baseline hid completed commits from ancestry-path lookup.

## What could not be verified

- Authoritative r2 full verification follows coded registration.

## Deviations

- Architect directed cheap materialization diagnosis before authoritative full execution; no duplicate worker full run. No governance changes or commits by worker.
