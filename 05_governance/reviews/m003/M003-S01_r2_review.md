# Review: M003-S01 round 2

## Findings

| id | severity | disposition | summary |
| --- | --- | --- | --- |
| M003-S01-001 | P1 | open | `08_pkg/relayforge-core/src/lib.rs:1028-1053` records only the latest success and failure. After a success and passing verification, a later `AttemptResult::Unknown` or `NotRun` leaves `latest_success > latest_failure`, so `TaskAccepted` can still be appended. Validate the latest attempt result’s outcome before accepting; add regression coverage for later unknown and not-run results. |
| M003-S01-002 | P1 | open | `08_pkg/relayforge-core/src/lib.rs:979-993, 1091-1116` allows `SessionClosed` with unresolved intents and marks the session closed. `reconcile` rejects closed sessions (`383-388`), recovery rejects them (`867-870`), and closed sessions cannot accept late results (`893-895`). Reject closing with unresolved attempts, or record a recoverable resolution atomically so those intents remain resolvable. Add a regression covering close followed by recovery. |

## Closure Decision

Objective status: not_achieved
Objective evidence: The r2 receipt reports exit 0 with 17/17 tests and 1/1 doctest, but the implementation permits acceptance after an unknown or not-run result and can strand unresolved intents on session close.

## Verdict

Verdict: needs_work - next: fix both P1 findings and add regression coverage.
