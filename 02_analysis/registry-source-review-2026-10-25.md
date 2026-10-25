# Registry source review: coding-agent failures and JEV decision patterns

**Reviewed:** 2026-10-25

**Status:** architect intake and research proposals. These notes inform the existing
foundation-contract work. Accepted decisions remain in
[the decision register](../00_brief/decisions.md), admitted work in
[the roadmap](../roadmap.yaml), and execution history in the ledger.
No feature, dependency, hook, threshold, experiment, or product implementation
is admitted merely by appearing here.

## Sources and access

| ID | Source | Evidence type and boundary |
| --- | --- | --- |
| P1 | [ACM DOI 10.1145/3793302.3793579](https://dl.acm.org/doi/full/10.1145/3793302.3793579) | Publisher page blocked by bot verification; publisher revisions and definitive publication date were not verified. |
| P2 | [Where Do AI Coding Agents Fail?](https://arxiv.org/html/2601.15195v1) | Public author preprint corresponding to P1; empirical observational research. |
| P3 | [Preprint record](https://arxiv.org/abs/2601.15195) | Submission metadata: 21 January 2026; lists MSR 2026 acceptance. |
| P4 | [Author replication repository](https://github.com/SOAR-Lab/MSR2026_AIDev) | Replication lead identified by the researcher; its analysis was not executed here. |
| P5 | [Underlying AIDev dataset paper](https://arxiv.org/html/2602.09185v1) | Dataset sampling and annotation provenance. |
| B1 | [ByteByteGo EP227](https://blog.bytebytego.com/p/ep227-top-9-places-to-use-jev) | Secondary overview, 26 September 2026; use-case suggestions and supplied explanatory infographics, not a controlled RelayForge benchmark. |
| F1 | [Flavio Copes: Jev](https://flaviocopes.com/jev/) | Secondary integration guide; distinguishes illustrative examples, builder/vendor reports, and the author's proposed integrations. |

All supplied article reads used crawl4ai. No subscription/access controls were
bypassed. User-supplied images are interpretation inputs; they are not copied into
the repository or treated as normative API/harness specifications.

## What the coding-agent paper establishes

P2 is by Ramtin Ehsani, Sakshi Pathak, Shriya Rawal, Abdullah Al Mujahid,
Mia Mohammad Imran, and Preetha Chatterjee. It examines 33,596 agentic PRs;
24,014 were merged (71.48%). Product populations are unequal, and product labels
do not identify exact worker models or effort.

The underlying public-repository dataset has a 1 August 2025 cutoff and a
greater-than-100-stars boundary. Task-purpose labels are automated. These
observations predate RelayForge's current model candidates.

Qualitative analysis sampled 600 rejected PRs; 38 were inaccessible and 562
were categorized. Counts include abandonment 228, duplicates 142, and CI/test
failures 99. Abandonment is a plurality. The reported 38% uses 600; among
accessible cases, 228/562 is approximately 40.6% (our calculation).
The paper associates outcomes with task type, change size, final-commit CI,
and review activity. Those associations do not establish causal model ability.

Do not reuse the paper's questionable per-line odds interpretation or treat
Cliff's delta as a percentage increase. Its evidence does not establish exact
model/effort failure rates, tool-call competence, recovery cost, or routing gains.

### Transfer to the registry — our inference

Keep these observations distinct:

| Observation | What it can establish | What it cannot establish alone |
| --- | --- | --- |
| Command/tool returned successfully | An operation executed | It was the appropriate operation or advanced the task |
| Required checks passed | Those checks passed on that revision | The user's entire task was accepted |
| Task accepted | The defined acceptance process accepted it | Every attempt was efficient or operationally successful |
| PR merged | Repository workflow integrated the change | A model/effort is universally reliable |
| PR closed/unmerged | A workflow outcome needing interpretation | Technical model failure |
| Session interrupted | Observed execution ended or evidence is incomplete | Wrong answer or deliberate abandonment |

Retain initial attempts, intermediate check results, feedback, wrong calls,
handoffs, repair, and final outcome. Recording only final CI loses the cost of
getting there. Mark duplicate, superseded, unwanted, externally blocked,
interrupted, and technically incorrect work separately.

Maintain denominators and outcome maturity. Open/pending work is not silently
failed. A late accepted/merged outcome is a new observation, not an overwrite of
earlier recovery costs. Failures-only samples cannot estimate the failure rate
of the whole workload. Product-level public merge rates are research priors,
not local model-plus-effort scores.

## ByteByteGo: which use cases fit RelayForge?

B1 presents nine candidate applications. Its broad speed/cost headline and the
infographic's dollar figures, 300 ms loop, and 0.5/0.9 confidence thresholds are
not verified defaults for this project.

| Application | Potential RelayForge use | Scope/cost constraint |
| --- | --- | --- |
| Model routing | Choose a registered worker/model/effort | Existing initial routing goal; measure full accepted-work economics |
| Guardrails | Optional semantic warning/classification | Deterministic authority remains outside the model |
| Tool-call gating | Suitability/scope signals | Native enforcement and actual event coverage must be verified |
| Inbox triage | Analogy for event/failure annotation | No email product or external communication is admitted |
| Reranking | Shortlist context, skills, or tools | Compare with current retrieval and require a fallback |
| LLM evaluations | Bounded rubric annotations | Do not replace compiler/tests or task acceptance with a score |
| Bulk labeling | Batch ambiguous historical annotations | Use minimized evidence and a separately bounded budget |
| Real-time control | React to observed task events | Local counters are event driven; no perpetual API polling |
| Confidence gate | Select an approved fallback when uncertain | Calibrate per decision and workload; no universal cutoff |

## Flavio Copes and primary TypeSafe documentation

F1's strongest design patterns are narrow typed questions, small structured state,
local composition, explicit fallback, and counting the whole pipeline cost. Its
author says the described personal rollout has not reached production. Builder
demos and vendor cost/latency claims remain reported examples, not verified local
results.

The primary docs below establish API behavior and limitations. The design
transfers remain proposals to qualify against RelayForge's own workload.

### Batch independent questions over shared state

[TypeSafe state](https://docs.typesafe.ai/concepts/state.md) says questions share
one state and are evaluated independently. [Fan-out](https://docs.typesafe.ai/patterns/fan-out.md)
documents bundling them in one request.

For a routing decision, task category, data needs, ambiguity, and other independent
judgments can share a bounded request. Code combines the answers. One answer
does not become another question's input in the same request. A second request
is justified when new data or a changed candidate set is genuinely needed.

Batch only questions with expected value: extra questions still consume input
tokens. Independent evaluation is not proof of statistically independent errors;
do not multiply their probabilities into a joint success guarantee.

### Separate probability, provider confidence, and observed reliability

[TypeSafe confidence](https://docs.typesafe.ai/confidence.md) defines Choice
confidence as:

    (p_max - 1/n) / (1 - 1/n)

Here n is the number of options. At p_max = 0.8, confidence is 0.6 for two options
and 0.75 for five options. These values are calculations from the documented
formula, not API experiment results.

A provider confidence of 0.9 is not automatically a measured 90% task-completion
rate. Store its type/definition and calibration reference separately from actual
worker success, label confidence, or expected completion probability. Noul returns
a yes probability without a separate confidence field.

Version the candidate set, option count/order, question, rubric, thresholds,
decision model, and calibration together. A changed roster can change confidence
even without a new JEV release.

### Keep arithmetic and factual comparisons in code

The version-specific [jev-1.13 jaggedness note](https://docs.typesafe.ai/model-jaggedness/jev-1.13.md)
warns about counting, numeric precision, dates, irrelevant state, adversarial
content, and option-order sensitivity. It recommends explicit criteria, state
filtering, and testing reordered choices. This is documented for 1.13, not proof
that every later version shares the same errors.

Compute prices, counts, failure rates, freshness, deadlines, and cost crossovers
locally. JEV can help classify ambiguous evidence. Score levels are ordinal
rubrics, not precise numerical measurements; do not interpolate dollars or
failure probabilities from a severity/ambiguity score.

Validate order sensitivity in qualification. Keep production order stable and
versioned; calling repeatedly with shuffled options on every turn would add cost
without establishing a calibrated ensemble.

### Resolve moving aliases

The [primary model reference](https://docs.typesafe.ai/models.md), read on
2026-10-25, lists jev-latest and jev-preview pointing to jev-1.14.0. F1 describes
an earlier period when latest still pointed to 1.13.0. This is a concrete reason
to retain source date and log the response's resolved model ID.

Use account discovery for availability and response metadata for the model that
actually answered. Pin a version when qualifying thresholds. Do not silently
transfer an old rubric's calibration to a moved alias.
The reference also says model discovery currently lists aliases while versioned
IDs can be accepted even when absent from that list. Do not treat discovery
omission alone as proof that a documented version pin is invalid.

### Skill selection already has a directly relevant worked example

The primary [skill-suggestion cookbook](https://docs.typesafe.ai/cookbooks/skill_suggestion.md)
ranks a Hermes roster of 182 skills, then rereads three shortlisted candidates.
Either stage can return no suggestion. The agent retains its roster; the hint
does not force adoption, and stable prefix caching is preserved.

The vendor reports 488 requests with jev-1.12 and a pinned Haiku 4.5:
wrong skill loads fell from 16.8% to 7.3%, and unnecessary loads from 9.8% to 4.0%.
These measure that setup's skill selection, not completed coding-task quality or
a Codex/Claude token saving. Even a supplied correct suggestion did not guarantee
the agent loaded it correctly.

Transfer to RelayForge: measure suggestion quality, actual adoption, unnecessary
loads, cache effects, latency, and full task outcome separately. Compare a cheap
local shortlist against a full-catalog call. Two requests are an experiment
candidate, not a mandatory per-turn expense or an admitted new feature.

## Reading the supplied router and firewall diagrams

The task router separates task kind, freshness needs, consequence, and ambiguity.
Use those as distinct, provenance-labelled features. Current-data needs affect
retrieval/tool eligibility, not automatically the LLM size. Consequence severity
and the probability of a bad outcome are different quantities.

The route may be an existing deterministic operation or a registered worker.
A RAG label is not a quality guarantee. Avoid forcing every task into a model
hierarchy or a human-confirmation path when existing authority and evidence
already resolve it.

The firewall image usefully places deterministic permission checks and a hard
policy engine around JEV. Proposed policy ordering:

1. Apply native deterministic permissions and schema/precondition checks.
2. Short-circuit denied calls and cases already settled by approved policy.
3. Where justified, obtain bounded semantic goal/scope/risk signals.
4. Apply existing hard policy and authorization; uncertainty grants no permission.
5. Record the decision and actual execution outcome.

Log allowed, denied, deferred, confirmation-requested, and failed-gate outcomes.
The diagram shows audit only after execution; that would omit important operational
failures and user-friction costs.

A classifier can inform enforcement only through a verified adapter. A prompt
hint or generic MCP server does not by itself intercept every native call.
Missing hook/tool coverage remains a capability gap, not a claimed firewall.
No new confirmation requirement is adopted from the example diagrams.

## Correcting the MCP and context simplifications

The [MCP architecture](https://modelcontextprotocol.io/docs/learn/architecture)
supports local STDIO and remote transports. Direct function calling can also
wrap remote services; local-versus-remote is not the essential distinction.
The [tools specification](https://modelcontextprotocol.io/specification/2025-11-25/server/tools)
defines discovery and invocation, including paginated listing. It does not
require the host to dump every schema into every model prompt or supply a
semantic best-tool router.

The host manages exposure and execution. RelayForge therefore needs adapter
capabilities and actual catalog exposure, rather than assuming that MCP adds
routing or saves context automatically.

The context diagram is useful for identifying experimental variables, not for
asserting identical internals or a fixed nine-layer assembly in every harness.
[Claude's primary memory guide](https://code.claude.com/docs/en/memory) documents
instruction loading at launch and on-demand subdirectory loading.

Track instruction/skill/tool-schema revisions, context-selection strategy,
cache/compaction configuration, and available token evidence. Exact per-source
token attribution may be estimated or unknown; label that instead of inventing it.
Store sanitized configuration references and counts, not private context contents.

## Proposed registry additions to evaluate

These refine existing contracts, not the current admitted feature scope.

| Record | Proposed evidence fields | Why |
| --- | --- | --- |
| Decision definition | decision kind, primitive, question/rubric revision, criteria, candidate-set ID/count/order, JEV resolved version, thresholds, calibration ID | Make choices reproducible and confidence comparable only where justified |
| Context profile | instruction/skill/tool revisions, exposure/retrieval strategy, cache/compaction mode, measured/estimated token provenance | Avoid assigning context-induced cost or failure solely to the model |
| Outcome observation | technical verification, task acceptance, integration state, failure attribution, source, maturity, observed time | Avoid treating PR rejection or interrupted sessions as technical failure |
| Recommendation receipt | proposed candidate, actual loaded skill/worker/tool, adoption/mismatch, execution evidence | A correct recommendation is not successful enactment |
| Operational economics | attempts, recovery/handoff cost, tool/gate failures, confirmation friction, latency and observer overhead | Optimize accepted work rather than a cheap isolated decision |

Keep hard constraints, sourced model facts, observed performance, and routing
policy separate. Deterministic calculations update the performance ledger; JEV
selects among eligible options where a semantic decision remains.

## What remains unverified

- Exact model-effort crossovers, including Astra versus Sol max.
- Benefits of these patterns with the owner's current harness/tool configurations.
- Native interception/event/usage coverage and actual advice adoption.
- Publisher changes relative to the examined preprint.
- Local accuracy/calibration of guardrails, annotation, skill reranking, or routing.
- Any claimed universal speedup, saving, or default confidence threshold.

No model benchmarks, paid experiments, dependency installs, product code, hooks,
permissions, foreground settings, or slice-state transitions were performed
for this research intake.
