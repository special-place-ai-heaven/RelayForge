Implemented original-content resolution-authority reading with same-path reachable
Git provenance and exact normalized-prefix checks. Integrity and prompt consumers
use the original grant; ordinary evidence remains strict.

## Changed files
- scripts/_protocol.py
- scripts/_integrity.py
- scripts/ledger.py
- scripts/prompt.py
- scripts/hermetic_verification.py
- tests/test_authority_history.py
- 02_analysis/authority-history-results.md

## Commands run
- Disposable Git fixture — reproduced prior append-only authority drift failure before fix.
- python -B -m unittest discover -s tests -p test_authority_history.py -v — PASS, 8 tests.
- python -B tests/test_authority_history.py — PASS, 8 tests; direct entrypoint rejects empty/skipped suite.
- git diff --check — PASS (worker and architect).

## What could not be verified
- Architect-owned authoritative full check pending at handoff.

## Deviations from the prompt
- None. No historical evidence, decisions or governance records were changed by the worker.
