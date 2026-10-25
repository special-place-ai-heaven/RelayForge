# RelayForge foundation brief

This is the bounded input for M001-S01. Read [accepted decisions](../00_brief/decisions.md)
and [roadmap](../roadmap.yaml) for authority. The [actionable findings](actionable-findings.md)
rank the research worth carrying forward; detailed source reviews are optional
lookups, not a feature checklist or required startup reading.

## Objective

Choose an eligible worker model and native thinking effort that completes work
reliably at the lowest total measured cost. Include parent reasoning, failed
attempts, wrong tool calls, tools, verification, repair, escalation and handoff.
Report completion rate and latency alongside cost per accepted task. No model's
100% reliability or Astra-effort versus Sol-max crossover is established.

## Minimum contracts to define

1. **Inventory:** exact available model/effort/profile IDs and native adapter
   capabilities, separate from performance estimates. Codex and Claude first.
2. **Observation:** sessions, tasks, attempts and execution spans with stable IDs,
   source/revision references and actual model/effort where exposed. Distinguish
   suggested, applied, sent and confirmed; unknown identity/usage stays unknown.
3. **Outcome:** technical checks, task acceptance and PR integration are separate.
   Preserve model-caused wrong calls and full recovery history after eventual
   success. Keep infrastructure faults, ambiguous causes and initial difficulty
   provenance distinct. The proposed three-wrong-call threshold is unapproved.
4. **Reconciliation:** persist incrementally before dispatch, record verification
   before completion, deduplicate and recover at the next start. Interrupted work
   is not automatically failure; do not abandon another active session's records.
5. **Routing:** deterministic eligibility and statistics first. JEV answers bounded
   unresolved choices over eligible IDs; version questions, candidates, policy and
   resolved judge model. Observe shadow recommendations before considering control
   of dispatch. Reuse a selection through tool continuations; switch deliberately
   at an evidence-backed boundary. Confidence is not task-success probability.

Routine event capture, counters and consolidation make zero model calls. Telemetry
cannot grant capabilities or permissions. Keep secrets/raw private state out of
tracked artifacts and minimize persisted/provider-bound content. Existing tools
such as SymForge remain the code-intelligence baseline.

## Next move and limits

**M001-S01 remains unstarted and docs-only.** Produce a compact foundation contract,
compare at most two non-Python toolchains, and name later exact-toolchain canary,
real verification and test-discovery gates. Product language and storage remain
undecided; Rust and SQLite are proposals. Python 3.11+/PyYAML run the retained
template workflow only. No runtime, hook, service or paid evaluation is claimed.

Native event/dispatch coverage and foreground model mapping remain unverified;
the historical audit found inconsistencies. Measure against a reasonable fixed
baseline before adding adaptation. Proxy integration, skills/tools/MCP selection,
compaction, review automation, local inference and other harnesses are deferred.
Agent-JEV is rejected by the owner (D007). Apply the complexity filter in D008.
