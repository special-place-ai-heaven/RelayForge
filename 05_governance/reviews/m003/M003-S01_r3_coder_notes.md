Remediated both lifecycle findings together: task acceptance now requires the latest attempt result to be success; session close rejects missing/unknown results without writes while explicit recovery, late results and resolved closure remain available.

## Changed files

- Core implementation and storage contract regressions.
- Product verifier required-test discovery list.
- Native core results note with current r3 and historical r1/r2 receipt references.

## Commands run

- Regression-first: acceptance_rejects_unknown_and_not_run_after_success failed before the core fix because acceptance was appended.
- Regression-first: required recovery test failed before the core fix because premature close was appended.
- Final focused job job_01a173b077cf7389b6d40db693d9f9fc passed: 18/18 unit/integration, 15 required discovered, zero ignored, 1/1 doctest, fmt, Clippy, build, dependency and SQLite checks.
- Coverage includes refused close/no writes, recovery/reconcile/late result/close, Unknown close refusal and NotRun close allowed.

## What could not be verified

- Authoritative full r3 verification follows coded registration.

## Deviations

- Architect directed focused regression validation then one authoritative full run, avoiding a duplicate pre-coded full invocation. Worker made no governance edits or commits.
