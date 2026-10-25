# Actionable findings: grain, chaff, and a lean path

Research curation, not a roadmap amendment. The evidence below is design input; it does not prove RelayForge runtime coverage or savings. Current scope is an embeddable core and CLI for Codex and Claude first; product language remains undecided and Python is excluded. See [accepted decisions](../00_brief/decisions.md), [foundation brief](foundation-brief.md), and the [repository roundup](jev-repository-roundup-2026-10-25.md).

Admission filter: retain an idea only when it solves a concrete RelayForge problem, improves an agreed outcome over a simpler baseline, and earns its complexity; research breadth is not feature scope.

## Five mechanisms worth carrying forward

These constrain one small core; they are not five new subsystems to implement.

1. **Account for the whole accepted task.** Link task, attempts, execution spans, tool outcomes, verification, repair/recovery, and acceptance. Count parent work, failed calls and retries; keep unknown cost/identity unknown, never zero. Separate task identity from initial difficulty and readiness, and retain each label's provenance. This reveals whether expensive starts avoid more expensive recovery. Compare completion rate, cost per accepted result, and latency on matched work. [Hermes documented lane evidence](https://github.com/kerpopule/hermes-jev-skills/blob/02f0461cfda2dda7353e916e38d9e97ef87c68c2/docs/lanes.md) is a reference, not calibration.

2. **Make execution identity and adoption observable.** Record recommendation as suggested, applied, request-sent, or provider-confirmed; retain actual model/effort only when exposed. Distinguish a new user decision from tool continuation and pin the chosen model/effort across continuations, with an explicit later decision boundary for switching. This prevents crediting a suggestion that never ran or misattributing a continuation. Check against native receipts and real Codex/Claude sessions. [Jev Router proxy boundaries](https://github.com/gargpratyush/jev-router/tree/38da6b84ea01241bfc41fbddc0928d0f40a703f0) illustrate continuation handling; they are not a qualified adapter.

3. **Persist evidence before action; verify before completion.** Stable event/tool IDs and recovery links, bounded evidence references, and durable incremental observations let interrupted work reconcile without losing earlier failures. Persist dispatch state before starting work; do not mark complete until required deterministic checks and acceptance evidence are recorded. Validate by restart/reconciliation scenarios and event coverage on both native harnesses. [Foreman runtime](https://github.com/thruwire/foreman/tree/c3142ef636617f32f772140125aa310d81ceece9) shows this ordering; restart behavior still needs proof.

4. **Use deterministic capability gates and fixed baselines first.** Inventory eligible models, native efforts, tools, and adapter coverage separately from measured performance. Code computes counts, prices, and failure rates; compare a simple fixed reasonable baseline before adaptive policy. Telemetry may update observed performance, but cannot grant capabilities. This gives a reproducible answer to whether adaptation improves accepted-task outcomes. The [registry source review](registry-source-review-2026-10-25.md) separates deterministic calculations from semantic choice; use it as a design reference, not a threshold source.

5. **Put JEV in bounded shadow mode only after measurement works.** Ask it only at unresolved choices over registered eligible IDs; log versioned question, candidates, recommendation, and later observed outcome without affecting dispatch. Replay against fixed baselines; promote only if complete-task results improve with acceptable cost and no authority expansion. [Hermes replay design](https://github.com/kerpopule/hermes-jev-skills/tree/02f0461cfda2dda7353e916e38d9e97ef87c68c2) is a reference. Promotion criteria and budgets remain to be approved.

## Lean sequence and scope

Proposed future sequence: **foundation contract → exact-toolchain canary → native capability proof plus cheap durable observation → deterministic statistics and fixed-baseline comparison → bounded JEV shadow → measured promotion**. Routine capture, counters, and consolidation make no model calls. The contract must preserve uncertainty, wrong tool calls, recovery cost, task identity/difficulty/readiness, adoption stages, and continuation pinning.

This sequence is not admitted execution. **M001-S01 is still unstarted and is docs-only:** define the compact native contract, compare at most two non-Python toolchains, and name later canary verification gates. No runtime, hook, install, paid evaluation, new threshold, or roadmap change is authorized by this curation.

**Defer:** proxy adapters, skill/tool retrieval, context pruning, review automation, local logits, more harnesses, and broad orchestration. **Reject:** Python product runtime and agent-JEV; the owner explicitly rejected agent-JEV, so no further experiment. Detailed repository research is optional reference material, not required reading or a feature checklist.
