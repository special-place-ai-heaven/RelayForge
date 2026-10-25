# Review: M002-S02 round 1

## Findings
| id | severity | disposition | summary |
| --- | --- | --- | --- |
| M002-S02-P2-1 | P2 | open | The receipt records only the full command. That command invokes `canary_verify.py` without `--self-test`, so the receipt does not verify the required negative self-checks; the notes claim they passed, but the receipt is the execution evidence. |

## Closure Decision
Objective status: indeterminate
Objective evidence: The receipt shows the full verifier passed, including clean-candidate and crash/replay assertions, but does not record the declared focused self-test run.

## Verdict
Verdict: needs_work - next: obtain receipt evidence for `python -B scripts/canary_verify.py --self-test`
