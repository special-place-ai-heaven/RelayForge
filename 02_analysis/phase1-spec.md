# RelayForge task-to-profile routing — Codex and Claude phase one

> Imported from the pre-project specification dated 2026-10-25 and adapted to the
> RelayForge repository. This is a design, not an implemented runtime. The
> [decision register](../00_brief/decisions.md) owns accepted project decisions and
> [roadmap](../roadmap.yaml) owns admitted work; this document does not admit every
> requirement as the next slice. A1 and the storage proposal in section 17 have
> been revised to reflect the owner's later direction: retain Python only for
> template workflow scripts and select a non-Python product language separately.
> Rust is a proposal, not an accepted choice. The external Python JEV inventory is
> compatibility evidence, not a mandate to extend that installation. Local
> assumption/decision IDs below are proposal identifiers, not the canonical
> project decision register.

**Status:** imported design specification; no RelayForge product runtime is implemented. Production promotion requires the owner decisions and evidence gates below.
**As of:** 2026-10-25
**Primary evidence:** [JEV and harness evidence brief](evidence-brief.md).
**Scope:** Codex and Claude Code. OMP and Cursor are deferred.

## 1. Purpose and success condition

Route a bounded task to a registered native worker profile that provides sufficient quality at the lowest measured total cost. A profile binds a role, model, thinking effort, instructions, tool permissions, and required skills within one harness.

JEV chooses among eligible profile IDs. It does not generate code, decompose a project, grant permissions, or invent model IDs. The foreground agent retains responsibility for task decomposition, acceptance criteria, reviewing results, and deciding when a task is complete.

Phase one is successful when both harnesses can demonstrate this complete chain:

1. Discover the profiles genuinely available in the current runtime.
2. Build a bounded candidate set from applicable policy and task requirements.
3. Obtain or bypass a JEV decision.
4. Validate the decision against the same registry snapshot.
5. Recommend or invoke the selected native profile through a supported harness mechanism.
6. Record the profile actually used and the completed task outcome.
7. Compare quality, total cost, and latency with the corresponding baseline.

A recommendation alone is not successful enactment. A configured profile file alone is not proof that the running harness can invoke it.

## 2. Evidence and design rationale

The [evidence brief, sections 2–4](evidence-brief.md) records these current gaps:

- Codex’s JEV policy selects personas; Claude’s selects personas and skills. Neither inspected policy provides the required profile contract.
- Claude has a JEV prompt hook. Codex currently relies on an instruction to call JEV.
- Existing integrations are advisory. Advice must be distinguished from actual worker invocation.
- Fifteen Codex agent files are pinned to Astra, while the written lane policy calls for cheaper workers. Their effective runtime registration is not yet verified.
- Claude already has native subagents with model and effort fields.
- Foreground settings conflict with the written chair policy: current Codex is Sol/medium and current Claude is Sonnet/medium; the written policy names Astra and Fable as chairs.
- Neither inspected policy supplies a validated Codex/Claude routing benchmark.

The [JevAgentHarness paper](https://arxiv.org/html/2610.21412v1), as summarized in the evidence brief, motivates separate confidence calibration for each decision and harness, fallback on uncertain decisions, and evaluation of completed work. Its findings do not establish expected savings here. The screenshot’s 0.87 threshold is illustrative, not a system default.

The [Boris Cherny interview](https://www.ycrootaccess.com/p/boris-cherny-building-claude-code) supports periodically removing guidance that no longer improves outcomes. Profile instructions, tool exposure, and routing guidance therefore belong in versioned evaluation inputs.

[HarnessRouter](https://github.com/HarnessRouter/harnessrouter) is an optional later execution adapter and evaluation surface, not a prerequisite for local routing. It implements the Unified Harness Protocol and an OpenAI Responses-compatible API; product-side code can select a configured harness for a newly launched task and use persistent sessions, streaming, files, cancellation, and structured failures. That could provide a common execution boundary for a later JEV-to-harness adapter. It is a separate Docker deployment with provider credentials and workspaces. Its public description does not establish that it imports the owner’s existing local agent profiles, MCP allowlists, plugins, or hooks unchanged, nor that it intercepts already-open local interactive sessions. Before adoption, verify that parity and compare setup burden, provider cost, latency, task quality, and actual dispatch. The published results cover one controlled task, so they motivate owner-specific evaluation rather than predict savings.

[awesome-jev-tools](https://github.com/v-modal/awesome-jev-tools) remains a discovery index; entries are not automatically approved dependencies. setup-gbrain does not supply this registry. Ponytail remains a separately versioned behavior layer whose effect must be measured.

## 3. External reference code and compatibility boundaries

SymForge was consulted first. It reported an unbound project and index_ready=false, so the following observations come from bounded literal reads. Symbol references, conventions, and impact have not been verified through an index. The paths below name the audited external installation; they are not files already present in RelayForge. Reuse of its behavior/contracts requires compatibility review and does not select RelayForge's language or authorize changes to that installation.

| External reference component | Observed responsibility | Intended phase-one treatment |
|---|---|---|
| .agents/jev/registry.py | md_agents, toml_agents, and skills expose name/description catalogues; fuller loaders add excerpts | Preserve compatibility for existing consumers if integrated. Define RelayForge's distinct typed profile inventory with availability and provenance. |
| .agents/jev/jev_router.py | Shared routing engine, policies, registry attachment, privacy handling, evaluation concepts | Evaluate the existing profile-family boundary and reusable behavior before implementing RelayForge's bounded profile results. |
| .agents/jev/jev_core/ | Existing shared decision transport and validation boundary | Evaluate reuse through an integration boundary or a compatible non-Python implementation. Preserve the frozen external interface unless a necessary gap is demonstrated. |
| .agents/jev/hooks/prompt_hook.py | Extracts prompt input, invokes routing, renders harness context | Use as a hook-contract reference for profile advice; context injection does not change model or effort. Preserve existing behavior when integrating. |
| .agents/jev/integrations/codex_route.py | Includes build_candidates(live_agent_types), privacy gates, and RouteResult separating proposed and selected outcomes | Determine whether it is active or experimental before choosing the integration entry point. Reuse relevant contracts; do not establish two competing Codex routers. |
| .agents/jev/policies/codex.yaml and claude.yaml | Harness-specific policy | Use policy behavior as a compatibility reference for explicit RelayForge profile bindings, modes, budgets, and calibration references. |
| Native Codex and Claude agent configuration | Actual worker identities and execution settings | Reconcile with approved registry bindings in a later implementation step; do not silently rewrite unrelated agents. |

**Compatibility requirement:** existing hook documentation defines shadow as log-only with zero network. Preserve that meaning. Introduce an explicitly named mode for live JEV decisions whose output cannot influence execution.

**Implementation prerequisite:** bind/index the actual source project when available, inspect callers and tests, and establish which Codex integration is active. File names above are inspection references, not authorized RelayForge write targets or a claim that their entire implementation has been reviewed.

## 4. Scope

Phase one includes:

- Read-only inventory of Codex and Claude profiles and the settings that affect their execution.
- A shared versioned profile contract and model fact records.
- Deterministic candidate filtering.
- JEV task-to-profile decisions.
- Native harness adapters for triggers, advice, dispatch acknowledgement, and outcome collection.
- Explicit overrides, bounded failure handling, and fallback.
- Separate calibration and evaluation for each harness.
- Measurement of routing overhead and total task cost.

Phase one excludes:

- Automatic foreground model switching.
- JEV-generated task decomposition or autonomous team creation.
- OMP and Cursor adapters.
- Dynamic skill recommendation as a new feature; existing explicitly selected skills continue to work.
- MCP tool proxies, automatic tool installation, or dynamic tool exposure.
- Automatic installation of Ponytail, gbrain, HarnessRouter, or third-party integrations.
- Changes to memory ownership or migration between memory systems.
- Automatic merge, publish, send, purchase, or destructive actions.
- Claims of savings derived solely from advertised token prices, model descriptions, or another benchmark.

A parent may submit each already-decomposed task separately. Each phase-one decision selects at most one worker profile or use_existing.

## 5. Assumptions and owner decisions

All numerical policies below are proposals, not measurements.

| ID | Assumption or decision | Consequence |
|---|---|---|
| A1 (superseded) | The earlier proposal to extend the Python JEV installation is superseded by the owner's non-Python product direction. The product language remains undecided; Rust is proposed only. | Preserve an embeddable core and a CLI boundary. Inspect existing JEV code for compatible contracts/integration. The template's Python scripts remain workflow tooling; an MCP facade is optional later packaging. |
| A2 | Native subagents are the intended execution mechanism in both harnesses. | If the current runtime cannot dispatch a registered profile, its adapter remains advisory. |
| A3 | Use a small initial set of logical worker profiles: scout, workhorse, frontier. | Specializations can retain existing native roles, but only explicitly registered bindings enter the candidate set. |
| A4 | Existing behavior is the baseline, including its current foreground model. | Comparisons must identify baseline settings precisely rather than assume the written lane policy is enacted. |
| A5 | Local structured metadata is sufficient for phase-one telemetry. | Raw requests, source code, and tool outputs are excluded from routine logs. |
| A6 | Unknown provider cost remains unknown. | Unknown cost cannot be treated as zero or used to certify monetary savings. |
| D1 | **Owner must choose the foreground chair/workhorse model mapping for Codex and Claude.** | Keep current foreground configuration unchanged; mark the canonical chair mapping unresolved. Production promotion must not silently resolve this conflict. |
| D2 | Owner approves concrete native worker model/effort bindings. | Implement registry validation and shadow collection beforehand; do not substitute price alone for measured capability. |
| D3 | Owner approves quality, latency, privacy, and promotion budgets. | Suggested starting values below remain proposals until accepted. |

JEV is a decision service, not one of the selectable generative worker profiles.

## 6. Registry contract

Use a versioned local snapshot. Native runtime configuration is authoritative for what can run; owner policy is authoritative for what may run; sourced model facts and local evaluations inform ranking. The registry records their intersection and conflicts.

### Snapshot

Required fields:

- schema_version, snapshot_id, created_at.
- Harness identity, version, session scope, and adapter version.
- Policy version and configuration fingerprint.
- Discovery method and evidence: configured_only, runtime_confirmed, or unavailable.
- Profile records, model facts, and evaluation references.
- Explicit unresolved conflicts and excluded-profile reasons.

A snapshot ID identifies immutable contents. It changes when relevant instructions, tools, permissions, models, efforts, skills, or bindings change.

### Profile

Required fields:

| Field | Meaning |
|---|---|
| profile_id, revision | Stable logical identity and version |
| lane, task_classes | Intended work and limits |
| harness, native_role | Exact native invocation binding |
| model.provider, configured_id, resolved_id | Provider and model identity; unresolved alias is explicit |
| effort, supported_efforts | Effective setting and supported values |
| instructions_fingerprint | Version of behavior applied to the worker |
| tools.allowed, tools.required, tools.effective | Declared permissions, task requirements, and observed availability |
| skills.required, skills.versions | Explicit dependencies and versions |
| permissions, restrictions | Execution boundaries, including read-only roles |
| availability, observed_at | Configured versus runtime-confirmed state |
| model_fact_refs, evaluation_refs | Evidence used for eligibility and ranking |
| fallback_profile_id | Optional approved fallback; no cycles |

use_existing is an explicit routing outcome, not a fabricated model profile. It means retain the current authorized execution path and make no JEV-directed change.

### Model facts

Every mutable fact carries source_url, retrieved_at, as_of, expires_at, and evidence_kind.

Record model identifiers, provider, modality support, context limits, supported effort levels, and pricing units. Prices distinguish uncached input, cached input, output, service tier, and currency when applicable. Subscription or credit costs must not be converted to dollar savings without a documented method.

External capability scores record benchmark name, version, methodology, and task category. They are supporting evidence; they cannot substitute for local profile outcomes.

**Proposed freshness policy:** resolve availability at session start and immediately before dispatch; invalidate snapshots on relevant configuration change. Refresh price and capability facts through a separate maintenance action, never by browsing on every prompt. Expired facts exclude a profile from cost-optimized automatic selection, while its availability can still be reported.

## 7. Routing input and output

### Input: RouteRequest

- Schema version and opaque request ID.
- Harness/session identity and registry snapshot ID.
- Decision type: task_profile.
- Trigger: prompt submission or parent-subtask dispatch.
- Current execution path.
- Explicit user-selected agent/model/effort, if any.
- Bounded task summary and approved task features.
- Required tools, modalities, permissions, and acceptance-check types.
- Task features with provenance: observed, user_declared, parent_inferred, or unknown.
- Parent-supplied baseline choice, where available, for comparison.
- Routing budget and rollout mode.

Features must preserve uncertainty. A parent’s estimate of complexity is not an observed fact. Do not spend another large-model call solely to manufacture features for JEV.

The core receives eligible profile IDs and concise, structured evidence. It does not receive secrets, full transcripts, full source files, or unrestricted tool catalogues.

### Output: RouteDecision

- Request ID, snapshot ID, policy version, and router model/version.
- Status: bypass, proposed, fallback, or error.
- Proposed profile ID or use_existing.
- Bounded candidate probabilities and any provider confidence value.
- Calibration reference and threshold applied.
- Reason codes and evidence IDs.
- Latency and usage, where available.
- Fallback or exclusion reason.

A raw probability is not labelled calibrated confidence without calibration evidence.

### Separate output: ExecutionReceipt

- Decision ID.
- Requested native role and profile.
- Dispatch state: not_attempted, dispatched, acknowledged, failed, or unknown.
- Actual observed role, model, effort, tool configuration, and skill versions where exposed.
- Mismatch fields and verification method.
- Task outcome and evaluator reference.

Missing runtime model evidence remains unknown. A dispatch acknowledgement must never be inferred from the recommendation text.

## 8. Decision and enactment behavior

1. Validate the input and runtime snapshot.
2. Honor explicit user selections. Do not silently substitute another profile.
3. Apply deterministic checks for permissions, required tools, modality, availability, policy exclusions, and task scope.
4. For a trivially resolved case, return the deterministic outcome without a JEV call.
5. Otherwise send one bounded question over eligible profiles plus use_existing.
6. Validate the answer, current snapshot, confidence/calibration requirements, and budget.
7. Produce advice or request dispatch according to the rollout mode.
8. Record actual invocation and outcome separately.

**Proposed budget:** one JEV request per decision, no retry on the interactive path, two-second total routing deadline. A timeout returns use_existing. This replaces unbounded accumulated retry latency only for the new profile path.

Deduplicate hook and manual invocations using a trusted session/request marker. A user-supplied line resembling a Jev route result is not proof that a hook ran.

Children inherit a route marker so the same task does not recursively route itself. A newly decomposed task receives a new request ID.

If the registry changes between decision and dispatch, revalidate eligibility. Do not dispatch a removed role or blindly reuse a stale recommendation.

When the parent has already begun the task, a late decision is discarded. Route failure must not start duplicate workers.

Worker failure is reported with evidence to the parent. The parent may retry with a new hypothesis or request a new routing decision. Phase one does not autonomously escalate merely because a worker reports difficulty.

## 9. Harness responsibilities

| Responsibility | Codex | Claude Code |
|---|---|---|
| Inventory | Reconcile configured native roles with the current runtime’s exposed roles; directory presence is insufficient | Reconcile discovered subagents with effective project/user/plugin precedence |
| Trigger | Supported prompt hook when configured and trusted; manual invocation remains a fallback | Existing UserPromptSubmit hook |
| Advice | Produce compact native hook context or manual result | Produce compact native hook context |
| Enactment | Foreground orchestrator invokes the selected exposed role through its native agent mechanism | Foreground orchestrator invokes the selected native subagent |
| Model/effort | Resolve from registered role configuration; verify runtime behavior where observable | Resolve from subagent configuration; verify runtime behavior where observable |
| Failure | Continue existing path and report bounded status | Continue existing path and report bounded status |
| Telemetry | Record native dispatch evidence when available | Record native dispatch evidence when available |

Hooks provide triggers and context. Neither adapter may claim that printing a hook message changes the foreground model.

Native role support is documented in the [Codex configuration reference](https://developers.openai.com/codex/config-reference) and [Claude subagent documentation](https://code.claude.com/docs/en/sub-agents). The exact installed runtime behavior still requires integration verification.

An adapter declares its capability level: recommend_only, dispatch_supported, or dispatch_verified. Promotion requires dispatch_verified for the tested runtime.

## 10. Rollout and evaluation

Modes:

1. **Off:** no new profile decision.
2. **Existing shadow:** preserve its current zero-network semantics.
3. **Live shadow:** explicitly enabled JEV calls; decisions logged but never injected or enacted.
4. **Advisory:** validated decisions shown to the parent; actual adoption measured.
5. **Controlled dispatch:** parent invokes eligible selected profiles under the approved policy; discrepancies remain visible.

Do not infer counterfactual worker success from shadow decisions. Shadow measures routing behavior and overhead. End-to-end savings require matched execution experiments.

For each harness independently:

- Build representative owner tasks covering read-only investigation, mechanical changes, subsystem debugging, and tasks requiring escalation.
- Define task success before execution.
- Split calibration and held-out evaluation by task/project family to limit leakage.
- Compare current baseline, deterministic routing, and deterministic-plus-JEV routing.
- Include parent reasoning, duplicated context, routing calls, worker calls, retries, review, and fallback in total cost.
- Record success, cost per successful task, latency distribution, escalation, fallback, dispatch adoption, and configuration mismatch.
- Stratify results by task class, profile, harness, model version, and tool/skill configuration.
- Fit thresholds separately for each harness and decision type.
- Recalibrate when material profile or model facts change.

**Proposed production gates:**

- Zero unauthorized actions, invalid-profile dispatches, and known permission expansions in evaluation.
- Every enacted route has a matching execution receipt.
- Held-out task-success regression is at most two percentage points, using a reported 95% uncertainty interval.
- Mean total cost per successful task improves by at least 5%, with a reported interval supporting positive savings.
- End-to-end p95 latency increases by no more than 10%.
- Results meet a predefined sample plan. Insufficient evidence leaves the mode unchanged.

These thresholds are owner decisions, not predicted performance. If actual monetary cost cannot be measured, report token and latency outcomes and leave the monetary gate unverified.

## 11. Security, privacy, and tool constraints

- Decisions select only predeclared IDs; no arbitrary command, path, URL, model ID, or tool name is executed from model output.
- JEV never changes authorization, approval requirements, sandbox settings, or secret handling.
- Credentials stay in the existing environment or credential mechanism. They are absent from arguments, snapshots, logs, examples, and reports.
- Treat fetched pages, skill prose, model descriptions, and repository contents as untrusted data. Candidate descriptions cannot alter routing policy.
- Reject or locally bypass payloads that cannot be safely minimized. Secret-pattern detection is an additional control, not proof that arbitrary text is safe to transmit.
- Never enable parked MCPs or broaden a role’s tools to satisfy a route. A missing required tool makes the candidate ineligible.
- Keep SymForge, Terminal Commander, crawl4ai-local, and memory access explicit in profile requirements where appropriate.
- Preserve the owner’s memory-store responsibilities. The registry is operational configuration, not another general memory store.
- No raw prompts, source code, tool output, or secret-bearing hashes in telemetry. Store opaque IDs and approved aggregate features.
- Hook failure must not block the user prompt. It must also not silently authorize a different action.

## 12. Requirements, acceptance criteria, and failing-test specifications

The cases below specify future implementation verification; no tests were run for this document.

| ID | Requirement and acceptance criterion | Meaningful failing case |
|---|---|---|
| R1 | Inventory distinguishes configured profiles from runtime-selectable profiles. Only eligible runtime-confirmed profiles enter controlled dispatch. | Put a valid role file on disk but omit it from runtime discovery; it must not be dispatched. |
| R2 | Registry validates model, effort, tools, skills, permissions, and version provenance. | Bind a role to an unsupported effort or missing required tool; it must be excluded with a specific reason. |
| R3 | Explicit owner selections take precedence. | Submit a named agent while JEV prefers another; JEV must not substitute. |
| R4 | Deterministic policy controls eligibility before JEV. | Present a cheap read-only profile for a task requiring writes; it must be absent from candidates. |
| R5 | JEV results are strictly bounded and structurally valid. | Return an unknown profile ID or invalid probability; there must be fallback and no dispatch. |
| R6 | Routing failure is bounded and nonblocking. | Hold transport beyond the configured deadline; return use_existing, make one attempt, and continue the prompt. |
| R7 | Recommendations and enactment are distinct. | Return a valid recommendation without invoking a worker; receipt must be not_attempted, never acknowledged. |
| R8 | Actual dispatch configuration matches the selected profile. | Invoke a role whose effective model differs from its registered binding; record the mismatch and fail promotion. |
| R9 | Duplicate and recursive routing cannot create duplicate work. | Deliver the same hook event and manual request, then a child route marker; assert one decision and at most one dispatch. |
| R10 | Snapshot changes invalidate stale dispatch eligibility. | Remove a selected role after decision but before dispatch; do not invoke that role. |
| R11 | Existing zero-network shadow semantics remain intact; live shadow cannot influence execution. | Assert zero calls in existing shadow and zero dispatch/advice in live shadow. |
| R12 | Privacy and permission boundaries survive adversarial input. | Include a synthetic secret marker and a candidate description requesting tool enablement; assert no secret transmission/logging and no permission change. |
| R13 | Registry facts distinguish unknown, expired, advertised, and measured evidence. | Expire pricing or leave an alias unresolved; do not fabricate cost or measured-quality claims. |
| R14 | Calibration and evaluation are harness-specific. | Supply only Claude calibration for Codex; Codex must not enter controlled dispatch. |
| R15 | Evaluation measures full task cost and real success. | Give a cheap worker repeated failures with expensive parent repair; include repair cost and do not count the first attempt as success. |
| R16 | Promotion requires all approved gates and adequate evidence. | Produce high route accuracy but worse held-out task success or missing dispatch receipts; refuse promotion. |
| R17 | Integration does not silently alter unrelated configuration or legacy routing. | Compare configuration scope and legacy persona/skill behavior before and after; allow only approved changes and no competing worker dispatch. |
| R18 | Foreground model conflict remains explicit until the owner resolves it. | Attempt canonical foreground bindings with D1 unresolved; preserve current foreground settings and report the conflict. |

## 13. Implementation sequence

1. Resolve active source entry points and document existing tests and policy consumers.
2. Implement read-only inventory and the typed registry; expose discrepancies without applying them.
3. Add task/profile contracts, deterministic filtering, bounded results, and compatibility handling.
4. Add separate live-shadow mode and metadata-only telemetry.
5. Implement Codex and Claude adapter capability checks and dispatch receipts.
6. Prepare owner-reviewable native role mappings, including the unresolved foreground decision.
7. Establish calibration and held-out evaluation for each harness.
8. Enable advisory behavior after privacy and adapter checks.
9. Promote controlled dispatch only for evaluated task classes and profile versions.
10. Revisit skill routing and MCP selection only after phase-one outcomes justify the next decision family.

## 14. Completion and verification boundaries

**Done for this specification:** organized the supplied research and local configuration evidence into requirements, contracts, control behavior, implementation boundaries, and proposed evaluation gates. Inspected the evidence brief and relevant local JEV source excerpts.

**Skipped:** implementation code, harness edits, test execution, external issues, dependency installation, and OMP/Cursor implementation.

**Could not verify:** Codex effective runtime registration and invocation; end-to-end hook-to-worker enactment; actual model costs and task savings; complete source references/impact because SymForge was unbound; the owner’s intended foreground model mapping. These remain explicit implementation or owner-decision gates.

## 15. Required refinement: measured model/effort crossovers and adaptive routing

**Owner clarification, 2026-10-25:** apply this to every eligible model and supported thinking level, beginning with the question: which Astra effort can equal or exceed GPT-6.1 Sol at max while costing less? Compare completed coding workflows and tool use, not just public benchmark scores. Count failed attempts, repeated recovery, abandoned work, and parent repair. Dynamically adapt routing from observed performance rather than prose.

This section refines the registry and evaluation requirements above. Logical lanes describe responsibilities; they do not prove a fixed ordering of model capability or cost.

### Unit of comparison

A measured candidate is an exact model/version + native effort/thinking configuration + harness/version + worker instructions + tools/MCPs + skill/plugin configuration. Record service tier, cache conditions, context size, task class, and orchestration configuration when they affect results. Identical effort names across vendors or generations are not equivalent. Ultra/subagent orchestration is a distinct execution configuration, not a scalar effort interchangeable with max.

Inventory every accessible, eligible model and its supported settings. Distinguish unavailable, configured-only, runtime-confirmed, and measured combinations. Publicly documented newer models are discovery leads until access and native harness support are confirmed.

Measure both:

1. Pairwise crossovers: can a candidate at lower effort replace another candidate at its supported maximum while meeting quality/reliability requirements and costing less?
2. The best completion policy among all eligible candidates: a direct stronger attempt can beat a cheap-first retry chain; a cheap-first chain can also win where failures are reliably detected and rare.

Do not limit comparison to adjacent model tiers. Do not assume that a crossover exists for every workload.

### Crossover evidence record

Each pairwise record contains candidate IDs and revisions, reference effort, workload slice, harness/tool configuration fingerprint, evaluator version, sample size, observation window, quality and completion-rate differences with uncertainty, total cost ratio, latency differences, and evidence provenance.

Record one of: verified advantageous replacement, quality/cost tradeoff, worse, or unknown. Equivalence or non-inferiority requires an explicit tolerance and sufficient paired evidence; a statistically inconclusive difference does not establish equivalence. Leave the Astra-versus-Sol-max crossover unknown until evidence supports a particular effort and workload. Never derive it from the words flagship, workhorse, near-Astra, or the names low/medium/high.

### Outcomes and failure attribution

For every attempt, record model + effort and a bounded outcome: verified complete, partial, failed, abandoned, externally blocked, or unknown. Acceptance is based on task-specific checks and review, not the worker declaring success.

Track first-attempt completion, eventual completion, repeated failures, failed recovery attempts, cost of all attempts, parent intervention/repair, escalation outcomes, and time to accepted result. Zero accepted results cannot be reported as cheap successful work. Keep completion rate alongside cost per accepted result so abandoning hard tasks cannot improve the apparent ranking.

Attribute failure where evidence permits: model reasoning/edit error, wrong tool or invalid tool arguments, configuration/permission mismatch, infrastructure/MCP outage, evaluator failure, changed user requirements, or unknown. Infrastructure failures count toward real workflow cost but must not automatically lower the model's reasoning score. Preserve attribution uncertainty.

### Tool-use performance

Controlled comparisons start from the same task/repository snapshot and comparable context, permissions, tools, skills, and instruction versions. Keep SymForge, Terminal Commander, crawl4ai-local, and relevant memory availability consistent. Use isolated sessions/workspaces so one candidate does not inherit another candidate's edits or answers. Separate cold-context and cache-reuse experiments rather than mixing their costs.

Measure successful appropriate tool use, invalid/failed calls, repeated calls without new evidence, redundant reads/searches, bytes/tokens returned and re-ingested, tool latency and charges, tool fallbacks, unnecessary delegation, and missed verification. Measure whether the model actually uses the supplied code-intelligence/terminal tools effectively.

Fewer calls is not inherently better. Necessary investigation and verification can improve accepted results. Optimize the completed workflow rather than a raw call-count target.

### Cost and escalation objective

Measure total billed cost across the entire task chain: foreground/worker input, cache reads/writes, output and reasoning tokens, JEV, tools, delegated context, failed attempts, verification, recovery, and repair. Report usage separately from monetary cost where subscription billing cannot be resolved.

Primary aggregate: total spend across the cohort, including failed and abandoned work, divided by accepted completions; also report completion rate and latency distributions. Compare difficult/failure-prone task slices and expensive tails as well as averages.

At a failure, compare the expected remaining cost and probability of completion for another attempt, higher effort, or another model, including handoff/redo overhead and latency constraints. A lower token price is not grounds to repeat a failing route. Repeating the same failed approach without a new hypothesis or evidence is not a valid recovery strategy. Apply explicit progress/retry budgets and recommend escalation when recovery is unlikely to outperform the next eligible profile.

Do not confuse sunk spend with the incremental decision: record all past waste for evaluation, but choose the next action from expected remaining outcomes. No route claims 100% certainty from finite observations.

### Adaptive performance ledger

Separate three layers:

- **Capability/configuration facts:** exact IDs, supported efforts, native roles, availability, permissions, tool requirements, and sourced prices. Outcome statistics do not rewrite these facts.
- **Observed performance:** append-only metadata events and computed completion/failure rates, recovery costs, tool-use metrics, latency, and uncertainty per candidate/workload/configuration.
- **Routing policy:** JEV receives compact current metrics over eligible candidates and selects within approved policy bounds.

A deterministic updater recomputes metrics after validated outcomes. JEV does not self-train or author its own success scores. Candidate descriptions can help explain scope, but cannot override measured failures or certify quality.

Use sample counts and uncertainty-aware estimates rather than trusting 1/1 success or assigning zero failures to an untested profile. Record both recent and historical evidence; detect drift when failure rates or costs change. Configuration/model revisions create a new evidence version, with explicitly justified reuse of older observations rather than silent carryover.

Routine updates to observed metrics can adjust ranking within the already approved candidate set; they must not install models/tools, broaden permissions, or change foreground settings. Missing or stale evidence reduces confidence. Sparse or unfamiliar tasks use the approved fallback or an explicitly budgeted evaluation path. Do not silently spend production work exploring unknown candidates.

Keep proposal, actual invocation, outcome, and recovery chain joined by IDs. A recommendation adopted by the parent is not a success observation. Keep task sampling/composition visible so routing only easy work to a candidate does not falsely establish that it is generally reliable.

### Added acceptance criteria

- R19: Every observed failure can be grouped by exact model + effort + workload/configuration, with attempt count, attribution, and uncertainty.
- R20: A cheap model that repeatedly fails and requires expensive repair must lose its apparent cost advantage when all chain costs are counted.
- R21: New verified failures update the relevant performance metrics and can change eligible ranking; unrelated model/tool configurations do not inherit the penalty.
- R22: Sparse data, unknown costs, missing max-reference runs, or inconclusive quality differences produce unknown crossover, never an invented equivalence.
- R23: A broken MCP is recorded as infrastructure/configuration failure when evidence supports that attribution, while its workflow cost remains accounted for.
- R24: A model/version or material tool/instruction change invalidates unqualified transfer of prior crossover evidence.
- R25: A repeated failed approach with no new evidence cannot trigger unlimited cheap retries; the parent receives a bounded escalation/fallback recommendation.

**Implementation implication:** build the attempt/outcome ledger and reliable attribution before claiming dynamic optimization. Start from existing metadata and passive outcome collection. No paid model-comparison campaign or harness changes were performed to write this refinement.

Sources: [OpenAI model selection](https://learn.chatgpt.com/docs/model-selection), [OpenAI model and effort guidance](https://learn.chatgpt.com/docs/models), [OpenAI deployment checklist](https://developers.openai.com/api/docs/guides/deployment-checklist), [Anthropic cost and intelligence guidance](https://platform.claude.com/docs/en/about-claude/models/optimizing-for-cost-and-intelligence), [Claude effort](https://platform.claude.com/docs/en/build-with-claude/effort).

## 16. Required refinement: session tracking and wrong-tool-call failures

**Owner clarification, 2026-10-25:** track sessions by LLM, thinking effort, task difficulty, and success rate. Several wrong tool calls constitute failure.

### Accounting hierarchy

Use session_id → task_id → attempt_id → execution_span_id. Each execution span binds the actual caller model/version, native effort/thinking configuration, role, harness/configuration snapshot, and start/end timestamps. A model or effort change starts a new span; a worker has its own linked span. Keep the parent/recovery chain intact so its cost cannot disappear behind a successful final worker.

A session may contain multiple tasks of different difficulty. Aggregate task and attempt outcomes within sessions; do not treat a long session as one homogeneous sample. Difficulty has an initial label, provenance, classifier/policy version, and any later independently supported revision. Preserve the initial estimate so observed failure cannot automatically relabel an easy task as difficult and excuse poor performance.

Record outcome counts and rates by model + effort + task class/difficulty + harness/tool configuration, with sample size and uncertainty. Report first-attempt completion, eventual accepted completion, operational failure, recovery, and abandonment separately.

### Tool-call outcome contract

Store bounded metadata per call: session/task/attempt/span IDs, native tool ID, call index, tool/schema version when known, result category, latency, output token/byte counts, correction/retry link, and attributed cause. Exclude raw secret-bearing arguments and outputs.

A model-caused wrong call includes selecting an unsuitable tool, issuing invalid arguments against the available schema, inventing a tool/capability, violating a known tool precondition, or repeating a known-invalid call without new evidence. A server outage or transport failure is not automatically a wrong model call. Ambiguous cases remain unknown rather than being assigned a fabricated cause.

**Proposed starting threshold:** three confirmed model-caused wrong calls within one task attempt set operational_failure=true. Track total and consecutive wrong-call counts. The threshold is configurable policy, recorded with every result, and must be approved/calibrated before automatic dispatch promotion. It is a concrete initial interpretation of “several,” not an observed optimum.

Operational failure and deliverable acceptance are distinct labels. An attempt can produce an accepted result after three wrong calls and still fail the owner's tool-use criterion. Preserve both labels and all recovery cost; never rewrite the attempt as clean success after a stronger model rescues it. Ranking must include the operational-failure rate and the applicable tool-use acceptance constraint.

On reaching the threshold, stop repeating the same approach and reassess. The parent receives the evidence and a bounded recommendation to change strategy, effort, model, or report a genuine tool/configuration problem. The threshold never grants new permissions or forces a tool that is unavailable.

### Added acceptance criteria

- R26: Tool calls are attributable to their actual model/effort execution span; unknown runtime identity is explicitly unknown.
- R27: Reaching the configured wrong-call threshold marks operational failure even when a later span completes the deliverable.
- R28: Parent repair, child attempts, effort changes, and model switches remain joined in session/task cost and recovery reports.
- R29: Difficulty labels preserve provenance and initial estimates; difficulty cannot be silently revised after failure to improve reported success.
- R30: Aggregates expose both tool-use operational failures and deliverable acceptance, with denominators and sample sizes.

## 17. Required refinement: inexpensive continuous observation and crash recovery

**Owner clarification, 2026-10-25:** observe events during a session, write and rank continuously, consolidate on session end or next start because sessions can end abruptly, and keep operation very cheap.

### Recommended phase-one implementation

Use an embedded local event collector/reducer shared by the harness adapters. Select durable storage alongside the non-Python product runtime; SQLite is a candidate, not an installed dependency or accepted storage choice. The earlier Python-standard-library proposal is superseded. Keep core observation/reduction callable as ordinary code. A shared background daemon is an optional optimization only if measured hook/process overhead justifies its operational burden.

The continuous observer is software processing events, not a continuously running LLM conversation. No model call is required to count observed errors, tool outcomes, token usage, accepted checks, attempts, or elapsed time. Persist compact metadata locally; do not routinely send session transcripts or tool outputs to JEV or memory systems.

Path: native event → bounded normalization/redaction → durable event record → incremental counters → updated performance view. The next route reads a small cached view over the eligible profiles.

### Trigger and coverage responsibilities

- During work: record observable tool/attempt/span/usage events and update active counters. Reassess routing at a new task or an evidenced recovery decision, not every tool event.
- On turn/worker completion: reconcile the attempt and child receipts; a stopped response is not proof the task succeeded.
- On graceful session end: consolidate pending observations.
- On session start/resume: reconcile outstanding work from earlier sessions and import only unseen events.
- Provide the same reconciliation as an explicit local command for adapters/environments without the relevant hooks.

Current official documentation lists Codex SessionStart, SessionEnd, PostToolUse, Stop, SubagentStart, and SubagentStop; SessionEnd does not run for subagents. Claude documents SessionStart, SessionEnd, PostToolUse, PostToolUseFailure, Stop, SubagentStart, and SubagentStop. These documentation facts do not prove the installed local runtimes expose every field needed for actual-model/effort attribution or failed-call collection. Discover and verify that payload coverage in each adapter. Do not invent Codex PostToolUseFailure parity.

A missing hook event or lack of observable billing/model identity produces a coverage gap. Import available structured harness records locally where supported. Do not treat silence as success.

### Durability and idempotence

Persist events during work so correctness does not depend on a final hook. Give each event a stable source/session/event identity; duplicate delivery has one effect. Commit metric updates and the processing cursor atomically. Use a single-writer/transaction strategy that tolerates simultaneous harnesses; avoid racing full-registry rewrites.

Track pending/running/finalized state separately from accepted/failed outcome. After a crash, retain known costs and wrong calls; mark the attempt interrupted or unknown unless other evidence establishes failure/completion. Startup must not mark another currently active session as abandoned; use trustworthy process/session liveness and bounded reconciliation.

On a reducer crash, reprocess pending events safely. A stale registry view identifies its age and uses the approved fallback if its freshness is insufficient. SessionEnd is a useful flush opportunity, not a durability guarantee.

### Cost controls

1. Increment counters from new events only. Do not reread all historical transcripts on every prompt or start.
2. Store structured IDs, error categories, counts, costs, timing, fingerprints, and evidence references. Exclude full prompt/code/tool payloads by default.
3. Keep versioned session-level configuration/model metadata once and reference it from events; avoid recomputing full tool/skill inventories per call.
4. Use deterministic parsing and checks first. An ambiguous semantic tool error stays unknown or is assessed in an explicitly budgeted batch; do not call a large model for every tool event.
5. Refresh rankings at completed-attempt/failure boundaries and by bounded batches. Do not rebuild the whole registry for every successful tool call.
6. Run the local routing gate for each task, but bypass JEV network calls when approved rules or measured dominance already resolve the choice. Invoke JEV when uncertainty or task-specific tradeoffs justify its overhead.
7. Begin with passive observations from work already being performed. Dedicated paired model/effort experiments require a separate explicit spend plan; passive evidence is useful but does not alone establish controlled equivalence.
8. Reconcile only unfinished or newly changed records at session start, with a cursor. Keep the prompt path bounded and hand longer cleanup to background execution.
9. Cache sourced model/pricing facts and refresh them independently of task routing.
10. Measure collector CPU/wall time, event count/storage, routing-call frequency, routing tokens/cost, and consolidation cost. Their full overhead belongs in the savings comparison. Introduce a persistent service or further abstraction only after this overhead is shown to matter.

No universal numeric overhead target is established yet. The default target is zero additional LLM calls for ordinary event capture/counting and zero transcript analysis calls at ordinary session boundaries. Explicit evaluation/judgment calls are separately metered and budgeted.

### Registry mutation boundary

The observer appends evidence and updates derived performance statistics and rankings within approved bounds. It does not rewrite model identity, supported effort, permissions, installed MCPs, worker instructions, or foreground configuration. Build each published performance snapshot atomically and retain its evidence/version so changed routing is explainable and reversible.

### Added acceptance criteria

- R31: Abrupt termination before SessionEnd retains already committed event/cost data; next start reconciles the unfinished attempt without inventing an outcome.
- R32: Duplicate hooks, repeated reconciliation, and reducer restart do not double-count attempts, failures, or cost.
- R33: Multiple simultaneous Codex/Claude sessions retain correct ownership and cannot overwrite each other's metrics or mark active work abandoned.
- R34: Routine event handling and start/end consolidation make zero model calls by default.
- R35: Routing and reconciliation consume incremental records/cached views rather than replaying all history or injecting previous transcripts.
- R36: Observer and routing overhead are included in measured task cost/latency and reported separately.
- R37: Runtime hook/metadata gaps remain explicit; stopped turns and missing events cannot certify completion.

Sources checked 2026-10-25: [Codex hooks](https://learn.chatgpt.com/docs/hooks), [Claude hooks](https://code.claude.com/docs/en/hooks).
