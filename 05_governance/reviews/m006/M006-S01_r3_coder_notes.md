Isolated disposable Git fixtures from inherited private repository selectors, restoring the test process environment after each case. Production authority logic is unchanged in this round.

## Changed files
- scripts/test_authority_history.py
- 02_analysis/authority-history-results.md

## Commands run
- python -B scripts/test_authority_history.py — PASS, 9 tests.
- git diff --check — PASS.
- Parent HEAD and index byte hash comparison — unchanged before and after focused tests.

## What could not be verified
- Architect-owned authoritative full clean-candidate verification pending at handoff.

## Deviations from the prompt
- None.
