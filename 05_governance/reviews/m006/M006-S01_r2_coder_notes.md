Moved the unchanged authority-history suite into scripts so the clean Git
candidate includes it. The original /tests export-ignore policy is unchanged.
Updated the full command and current results note; authority logic is unchanged.

## Changed files
- scripts/test_authority_history.py
- scripts/hermetic_verification.py
- 02_analysis/authority-history-results.md
- Removed the original untracked tests/test_authority_history.py path.

## Commands run
- python -B scripts/test_authority_history.py — PASS, 8 tests.
- git diff --check — PASS.

## What could not be verified
- Authoritative full check pending at handoff.

## Deviations from the prompt
- None. User requested pause after this reviewed, committed and pushed checkpoint.
