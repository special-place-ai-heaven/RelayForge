# Review: M005-S01 round 1

## Findings

| id | severity | disposition | summary |
| --- | --- | --- | --- |
| M005-S01-001 | P2 | open | evaluate_eligibility classifies Configured and Unsupported facts before checking whether their profile revision, harness/configuration, and runtime scope match current state. A fact from another scope can therefore report unresolved_configured_only or excluded_unsupported for the current scope. Check applicability first and cover both cases in tests. |

## Closure Decision

Objective status: not_achieved
Objective evidence: The receipt records successful full verification, and the reviewed diff covers immutable inventory storage and fresh-process CLI diagnostics; however, the evaluator can report status reasons from facts whose scope does not match current state.

## Verdict

Verdict: needs_work - next: enforce capability fact scope before classifying configured and unsupported evidence, with tests for mismatches.
