# Review: M004-S03 round 1

## Findings

| id | severity | disposition | summary |
| --- | --- | --- | --- |
| M004-S03-BOOT-001 | P2 | open | `scripts/ci_contract_check.py:126-128`: `late_checksum`’s pattern is over-escaped and does not remove the checksum before execution. It appends another checksum after execution, so the exact-script comparison rejects a changed script without exercising the reordered-checksum case. Build the fixture by moving the actual checksum line after installer execution. |

## Closure Decision

Objective status: not_achieved
Objective evidence: The S03 receipt records full verification passing with a stable source/index/HEAD witness, but the late-checksum fixture does not test checksum ordering; the new Linux bootstrap has not yet run on Linux.

## Verdict

Verdict: needs_work - next: correct the late-checksum fixture and rerun verification
