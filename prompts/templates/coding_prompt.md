# Coding prompt: {{slice_id}} — {{title}} (round {{round}})

Read `AGENTS.md` first. This prompt is the whole task; do not read the roadmap or
ledger.

## Objective

{{objective}}

## Acceptance

{{acceptance}}

## Non-goals

{{non_goals}}

## Read first

{{read_first}}

## Write boundary

Allowed prefixes: {{allowed_prefixes}}

Forbidden: everything else, and always {{forbidden}}. Do not commit.

## Verification

Focused, run while working:

{{focused}}

Full, run by the architect after recording coded work:

{{full}}

Report your final focused results. The architect owns the authoritative full
receipt; do not duplicate that run just to encounter pre-coded ledger drift.

## Advisory notes

{{notes}}

## Findings to resolve

{{open_findings}}

## Memory

{{memory}}

## Finish

Begin with one short paragraph explaining the implemented approach. Then give
exactly four lists: changed files; commands run with pass or fail; what could
not be verified; deviations from this prompt. Facts only.
For a blocker, use the last two lists to name the affected requirement, observed
reason and evidence paths, who must act and the required action. Preserve work
and identify what remains; do not invent authority or claim unobserved success.

## Autonomous outcome

{{outcome_contract}}
