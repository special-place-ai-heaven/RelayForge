# JEV issue triage: article review

Date: 2026-10-25
Status: architect research intake; no feature admission or accepted policy.

## Source and reported observations

[Using Jev to triage GitHub issues](https://www.curtiscode.dev/post/using-jev-to-triage-github-issues),
published 2026-10-09; read with crawl4ai on 2026-10-25.
These are author-reported observations, not a reproduced evaluation.

- Each issue supplies title, repository description and body capped at 4,000
  characters. Five questions share one request. Skill options come from 37 actual
  folders/descriptions, plus a general category; descriptions are capped at 300
  characters.
- Across 862 issues the author reports $0.097, 330 ms median and roughly one-second
  p95 latency; 838 issues ran in under 26 seconds at concurrency 20. Most request
  tokens came from the skill catalog.
- Removing identifying title prefixes reduced skill accuracy from 98.6% to 86.5%
  on 148 examples, and category accuracy from 98.8% to 92.7% on 82. The category
  sample contains 72 enhancements, five bugs and five questions.
- Rejection prediction was weak; readiness predictions could not be validated.
  The author proposes dropping both questions. Borderline answers varied across
  repeated calls, and the article warns that issue text can manipulate results.

The linked [implementation repository](https://github.com/curtiscde/jev-issue-triage)
received a limited public-source inspection of request, batch, blind-evaluation
and scoring code. No commit was pinned; links below refer to the branch read on
2026-10-25. This is not a full code audit or a selected dependency. No installs,
live model calls or dataset reproduction were performed for this review.

The [request code](https://github.com/curtiscde/jev-issue-triage/blob/main/lib/jev.ts)
disables SDK retries. The [batch script](https://github.com/curtiscde/jev-issue-triage/blob/main/pipeline/classify.ts)
persists successful results and supports rerunning failures; the
[scorer](https://github.com/curtiscde/jev-issue-triage/blob/main/pipeline/score.ts)
aggregates stored classifications. Inference: these summaries do not capture
complete failure-and-recovery cost or elapsed time. The repository does not commit
the generated input/results, so source availability does not independently
reproduce its reported measurements.

## Implications for RelayForge

### Separate identification, readiness, difficulty and worker suitability

Issue category and skill identity describe what a request concerns. They do not
establish whether its contract is complete, implementation is safe, or a particular
model/effort profile can complete it. Maintain distinct question families and
validation evidence. Do not transfer one classifier's accuracy to another.

Readiness needs independently assessable criteria, such as missing acceptance
conditions, unresolved choices, required access and known dependencies. A readiness
label is an advisory signal within the existing task authority; it does not grant
permission or bypass the template's admission rules.

Difficulty estimates should retain their original question version, observed
input scope and uncertainty. Later observed effort/recovery can improve derived
estimates without retroactively changing the original difficulty label to excuse
a failure. Worker suitability comes from exact model/effort/harness outcomes and
capability facts, not from predicted category alone.

### Use real catalog entries, but budget their descriptions

The useful pattern is to build candidates from the installed inventory rather
than model-invented names. First filter deterministically for actual harness
availability, capabilities and scope. Version the surviving IDs, descriptions,
order and snapshot so changes are not silently pooled into the same calibration.

Avoid shipping the full installed skill/tool catalog on every route. Retrieve a
bounded candidate set when appropriate and measure both selection cost and false
exclusion. Shortening descriptions is not automatically harmless; record what was
omitted. Distinguish no-fit, ambiguous, multi-component and insufficient-evidence
outcomes where the decision requires them instead of conflating all with one
catch-all label.

### Evaluate against realistic baselines and held-out outcomes

Prefix removal is a useful leakage check, but title-derived labels form a selected
subset. Remaining text can repeat the name, and similar skill names complicate the
label ontology. Neither headline accuracy is a representative local-routing result.

The article's category mix implies an always-enhancement baseline of about 87.8%
(72/82, calculated). Report per-class errors, denominators and a majority baseline,
not just aggregate accuracy. Rare classes here are too small for strong claims.
For RelayForge, distinguish deterministic rules, inventory retrieval, JEV selection
and actual dispatch/acceptance. Use held-out tasks and independent outcome evidence;
uncertain or unavailable labels stay unknown.

A predicted rejection depends on maintainer policy and future information. It is
not a reliable proxy for complexity, value, or probability of worker success.
Likewise, a predicted high impact does not establish the number of affected users.
Measure each decision family before allowing it to influence an adaptive ranking.

### Record input coverage and keep decision authority outside untrusted text

A fixed character cap can omit acceptance conditions or a late correction.
RelayForge should record truncation, source length/revision and omitted scope;
retrieve missing evidence or abstain when it could change the route. Comments,
linked code and later clarifications may matter even if an issue's title/body
appear complete.

Issue descriptions and imported tool results remain untrusted evidence. Typed
options prevent invented output IDs but do not prevent a malicious selection.
Existing authorization and deterministic eligibility checks remain authoritative;
this research does not introduce an extra approval flow for ordinary work.

### Measure end-to-end reliability and cost

The reported pricing and throughput describe one workload/provider setup. They do
not establish current provider prices, service guarantees or local coding savings.
Record failed attempts, retries, rate limits, unknown charges and recovery, as well
as successful responses. Separate cold and warm latency and use bounded concurrency
appropriate to the configured provider.

Repeated calls to resolve an ambiguous label cost money and do not independently
verify correctness. Preserve the original distribution, request/model identity,
rubric and calibration; use a bounded fallback or obtain new evidence. Evaluate
calibration and abstention separately from label accuracy and task completion.

## Effect on project planning

This strengthens the planned split between capability inventory, semantic task
signals, observed outcomes and routing policy. It does not establish the
Astra-versus-Sol-max crossover or justify auto-dispatch from a generic readiness
label. The foundation contract can proceed; classification experiments can be
admitted later with explicit datasets, baselines and cost budgets.
