# JEV repository roundup: seven source investigations

Date: 2026-10-25
Status: architect research intake; recommendations, not admitted implementation.

## Scope and method

Discovery source: [Analytics Vidhya's ten-project roundup](https://www.analyticsvidhya.com/blog/2026/10/jev-projects-on-github/),
read with crawl4ai. The roundup is a discovery list, not evidence of runtime
correctness, savings, or model competence.

Selected seven projects for their relevance to RelayForge's Codex/Claude routing,
observation, evidence and future decision adapters. Shallow-cloned each into one
dedicated disposable folder. Inspected indexed source and repository prose using
SymForge and bounded literal reads. Delegated distinct repo pairs; source
citations below identify the code behind each finding.

No dependencies were installed, cloned-project scripts/tests executed, services started,
hooks installed, model weights downloaded, paid evaluations run, or harness
configuration changed. These findings establish source behavior and documented
claims, not live integration or quality. No upstream code was copied into RelayForge.

SymForge generated local indexes and a `.gitignore` hygiene line in each temp
checkout. Final tracked-diff inspection found only `.gitignore` changed in all
seven; cited source files match the recorded HEADs. A local
`RESEARCH-MANIFEST.json` records origins, revisions, cleanup purpose and absolute
temporary location. The disposable clones remain available for later deletion.
Machine-local paths are excluded from this tracked note.

Explicit project-name selectors were unavailable on this single-project SymForge
transport. Investigators used their active bound indexes and checked source paths;
this is not evidence of daemon multi-project routing support.

## Selection and snapshots

Each link is pinned to the inspected revision. All seven repositories carry MIT
licenses at these snapshots. Retain the relevant notices if substantial code is
later reused; this research makes no product licensing decision.

| Project | Inspected revision | Why selected |
|---|---|---|
| [Hermes Jev Skills](https://github.com/kerpopule/hermes-jev-skills/tree/02f0461cfda2dda7353e916e38d9e97ef87c68c2) | `02f0461cfda2dda7353e916e38d9e97ef87c68c2` | Shadow routing, model/effort decisions, replay evidence, skills |
| [Jev Router](https://github.com/gargpratyush/jev-router/tree/38da6b84ea01241bfc41fbddc0928d0f40a703f0) | `38da6b84ea01241bfc41fbddc0928d0f40a703f0` | Native CLI proxy boundary and continuation pinning |
| [Foreman](https://github.com/thruwire/foreman/tree/c3142ef636617f32f772140125aa310d81ceece9) | `c3142ef636617f32f772140125aa310d81ceece9` | Worker supervision, durable state and deterministic checks |
| [Fast Jev Compaction](https://github.com/tamaratran/fast-jev-compaction/tree/e3f262a7f4d42bd8dd32ced30d26176f7cb545b0) | `e3f262a7f4d42bd8dd32ced30d26176f7cb545b0` | Tool-pair-safe pruning and hook failure behavior |
| [Jev Review](https://github.com/devagrawal09/jev-review/tree/31f89602797fb7bea007f8a480bf368bf564954e) | `31f89602797fb7bea007f8a480bf368bf564954e` | Bounded evidence selection and auditable findings |
| [SemIf](https://github.com/TheoLeeCJ/SemIf-OpenJev/tree/23cf1f39fc9534fe81437200959b6dfc7106e45a) | `23cf1f39fc9534fe81437200959b6dfc7106e45a` | Local decision backend, logits, cache and calibration contracts |
| [Jev Search](https://github.com/superagents-lab/jev-search/tree/1bfe835e8b4320444a5304043243d4e96ed278b7) | `1bfe835e8b4320444a5304043243d4e96ed278b7` | Decision-provider adapters, batched ranking and partial failure |

Deferred the roundup's Jev Ultrafast, Mobile Jev and Jev Trader. Their described
browser, Android and trading execution domains are less direct matches for the
initial Codex/Claude registry. They were not cloned or source-investigated;
deferral is a scope judgment, not a finding that they lack useful ideas.

## Highest-value findings

| Mechanism | Best source | Proposed value to RelayForge |
|---|---|---|
| Distinguish recommendation from effective request | Hermes | Avoid recording a suggested model as the model that actually worked |
| Reuse one difficulty judgment for model and effort | Hermes | Save duplicate classification while respecting native effort support |
| Fresh-turn detection and continuation pinning | Jev Router | Avoid rerouting every tool continuation and rebuilding context/cache |
| Persist lifecycle state before dispatch; verify before completion | Foreman | Crash-safe attribution and honest completion records |
| Select evidence using concrete handles plus no-match | Jev Review | Explain decisions without treating free-form model claims as proof |
| Preserve tool-call/result relationships during pruning | Fast Jev Compaction | Prevent malformed histories and orphaned execution evidence |
| Return prompt/model identity, raw scoring and cache metadata | SemIf | Keep decision evidence reproducible and calibration provider-specific |
| Batch snippet judgments; isolate partial failures | Jev Search | Potential later tool/skill retrieval experiment with bounded state |

These are design candidates. None supplies a complete verified adaptive
model × effort × task-difficulty registry with all tool/recovery costs.

## 1. Hermes Jev Skills: closest reference for routing evidence

### Useful mechanisms

- Once-per-turn decision reuse. The plugin keys state by turn ID, rejects
  mismatched request/turn state, and invalidates a decision when provider, base
  URL or model binding changes. Explicit user model selections remain pinned.
  Evidence: `hermes/plugin/hermes-jev/__init__.py:217-227,449-532`.
- One decision requests difficulty distribution, work kind and costly-mistake
  judgment. Local policy maps those signals to routing; effort can reuse the
  same answers. Effort injection is conditional on exact provider/model capability,
  supported request format, mode and explicit user settings.
  Evidence: `jevkit/route.py:555-566,622-697`;
  `jevkit/effort.py:14-42,76-147`;
  `hermes/plugin/hermes-jev/__init__.py:575-614`.
- Shadow mode separates preference, effective request model and applied status.
  The integration explicitly does not equate those fields with provider usage.
  This is a valuable distinction between suggested, applied, sent and confirmed.
  Evidence: `hermes/plugin/hermes-jev/__init__.py:451-453,560-573`.
- Deterministic lane/loop rules handle repeated failures, failing checks, empty
  diffs and other observable facts before semantic decisions. Privacy-aware
  routing sends redacted/capped text or coarse features; merged route/skill
  calls are disabled for sensitive profiles.
  Evidence: `jevkit/policies/lane.json:7-18`;
  `jevkit/policies/loop-step.json:7-23,54-71`;
  `jevkit/turn.py:29-46,68-90`; `jevkit/route.py:569-574,606-625`.
- Route and first-stage skill questions can share one request when they need the
  same state. Second-stage skill selection remains separate because its candidates
  depend on the first answer.
  Evidence: `jevkit/turn.py:1-19,68-90`.

[Routing source](https://github.com/kerpopule/hermes-jev-skills/blob/02f0461cfda2dda7353e916e38d9e97ef87c68c2/jevkit/route.py),
[plugin](https://github.com/kerpopule/hermes-jev-skills/blob/02f0461cfda2dda7353e916e38d9e97ef87c68c2/hermes/plugin/hermes-jev/__init__.py).

### Empirical material worth learning from, with strict limits

The repository documents observational fleet history and a controlled replay of
nine commits. In the latter, all three arms pass 9/9 task-specific checks:
Sonnet 5 medium reports $1.60, Opus 5.5 high $3.27, and Haiku 4.5 low with escalation
armed $2.00. The smallest model used more turns/tokens and took longer than the
middle arm. This is directly relevant to the owner's cost-of-recovery concern.

The replay restores source while retaining tests and uses commit messages as
tasks; success means the retained tests pass without editing tests. That is a
bounded coding evaluation, not proof of complete user acceptance. All nine lane
choices were medium, so the result does not show that an adaptive router beats
a fixed medium baseline on a varied task mix.

Observational effort comparisons and smaller-model runs have profile/task
selection confounds. Some summaries exclude cache-read tokens. The Claude fleet
paragraph prices lanes on the same tokens (docs/lanes.md:64); inference: this
repricing does not measure a counterfactual execution with another model/effort.

Evidence: `docs/lanes.md:50-78`;
`jevkit/lane_replay.py:1-20`; replay lead `evals/lanes/replay_git.py`.
[Documented measurements](https://github.com/kerpopule/hermes-jev-skills/blob/02f0461cfda2dda7353e916e38d9e97ef87c68c2/docs/lanes.md#L50-L78).

### Disposition

Adapt shadow/effective-request distinction, bounded shared judgments and
controlled replay methodology. Keep actual usage/acceptance/tool-failure evidence
separate. Do not copy its language, broad plugin suite, promotion thresholds,
model hierarchy, fleet figures or effort mappings as RelayForge facts. Its
Hermes hooks do not establish native Codex/Claude parity. Python remains excluded
from RelayForge's product runtime.

## 2. Jev Router: concrete native proxy boundary

### Useful mechanisms

- The Claude proxy distinguishes a fresh user turn from tool-result continuations
  and removes reminder noise for routing. A per-conversation state caches the
  chosen tier/model; follow-ups keep that selection. The Codex proxy has its own
  continuation/title-generation checks and conversation-key extraction.
  Evidence: `src/proxy.mjs:55-71,120-180,213-263`;
  `src/codex-proxy.mjs:80-102`.
- With routing enabled and its key configured, wrappers launch the real CLIs
  through a loopback API proxy; Codex launches unrouted without a routing key.
  The proxy rewrites
  model/capability fields while forwarding native traffic. A sentinel model is
  the routing opt-in; explicit concrete models bypass routing. Catalog-derived
  candidates are used when available, with static cold-start fallback.
  Evidence: `src/config.mjs:27-39`;
  `src/codex-cli.mjs:94-105`;
  `src/proxy.mjs:100-118,258-347`;
  `src/codex-proxy.mjs:38-59,104-134,244-323`.
- Local policy keeps the current choice on failures or uncertainty, constrains
  downgrade/escalation, and accounts for large-context switching with a heuristic
  token estimate. Codex effort is normalized to declared supported defaults;
  this is not evidence-based effort optimization.
  Evidence: `src/policy.mjs:4-61`; `src/config.mjs:48-67`;
  `src/proxy.mjs:226-227`; `src/codex-proxy.mjs:126-134`.

[Claude proxy](https://github.com/gargpratyush/jev-router/blob/38da6b84ea01241bfc41fbddc0928d0f40a703f0/src/proxy.mjs),
[Codex proxy](https://github.com/gargpratyush/jev-router/blob/38da6b84ea01241bfc41fbddc0928d0f40a703f0/src/codex-proxy.mjs).

### Limits and disposition

This demonstrates an architectural route around unequal hook surfaces, but not
a qualified RelayForge adapter. It depends on CLI wire formats, credentials,
stream forwarding, model catalog semantics and auxiliary requests. Version drift
must be tested through real native sessions before adoption.

The policy is static/prose-driven. It does not learn from subsequent wrong tool
calls, task acceptance or repair costs. Commented cache/latency figures are not a
replicated local benchmark. Conversation pinning must also allow controlled
escalation at a legitimate new decision boundary.

Diagnostics retain raw prompts and JEV exchanges in bounded temp files with
restrictive modes (`src/status.mjs:5-81`). Do not adopt raw retention by default;
Windows access controls also need native qualification rather than assuming Unix
mode bits establish equivalent protection.

The investigator's SymForge read of `src/router.mjs` was withheld under its
secret-context admission policy; no raw-read bypass was attempted. Accordingly,
client transport/retry/schema handling inside that file remains unverified.
Inspectable callers and policies establish only the boundaries described above.

Adapt continuation identity, opt-in/manual-override semantics and a possible
proxy adapter experiment. Do not install the wrapper or change existing harness
connections during research.

## 3. Foreman: evidence ordering and supervisor control

### Useful mechanisms

A narrow worker protocol offers run, steer and terminate with capability
declarations (`src/foreman/workers/base.py:30`). Runtime creates a worker record
with attempt number, persists state, then starts the worker
(`src/foreman/runtime.py:349-370`).

Configured command checks run after the coding worker completes and before the
worker is removed from the active set. Command evidence is persisted before the
supervisor can consider the run finished. This ordering can prevent a false
completion while verification is still pending.
Evidence: `src/foreman/runtime.py:274-315`;
`src/foreman/config.py:23`.

The runtime distinguishes assessment failures from worker retries. It caps
transient judgment failures, retries, workers, iterations, concurrency and
timeouts. Model judgments inform permitted interventions rather than directly
executing arbitrary actions.
Evidence: `src/foreman/runtime.py:402-431,500-520`;
`src/foreman/policy.py:47-112`; `src/foreman/config.py:55`;
`docs/decision-models.md`.

Normalized hook events include session/work/event identity and tool/continuation
context. Bounded evidence groups let different judgment questions receive their
relevant evidence rather than an entire transcript.
Evidence: `src/foreman/hooks.py:57-89`; `docs/decision-models.md`.

[Runtime](https://github.com/thruwire/foreman/blob/c3142ef636617f32f772140125aa310d81ceece9/src/foreman/runtime.py),
[decision-model boundary](https://github.com/thruwire/foreman/blob/c3142ef636617f32f772140125aa310d81ceece9/docs/decision-models.md).

### Limits and disposition

Adapt persist-before-dispatch and verify-before-complete ordering, distinct error
types and explicit budgets. A check pass still needs the actual revision,
acceptance scope and required command set; a generic completed event is not proof
of user acceptance.

Do not import the broad Python framework or its roster. Exact crash/restart
recovery, cross-process idempotence, actual model/effort attribution and complete
provider/tool/parent-repair costs were not established by this inspection.
Durable state code alone does not prove restart behavior. Provider substitution
also does not imply equal judgment quality or interchangeable thresholds.

## 4. Fast Jev Compaction: safe pairing, incomplete recovery

### Useful mechanisms

The bridge targets a Claude Code function-hook API: `session.compact` and optional
`turn.complete` usage-triggered compaction. Documentation scopes this to an
early-access function-hook surface. It is not generic Codex/Claude middleware,
and host support was not live-qualified here.
Evidence: `hooks/fast-jev.ts:259-299`;
`hooks/README.md:1-6,26-28`; `README.md:128-139`.

Tool calls/results are matched by `tool_use_id`. Removing a call removes its
result; truncating a result retains the call and updates mirrored text copies.
Unchanged host-message handles are retained; rebuilt messages do not reuse stale
engine handles. Initial/recent messages protect some calls from pruning.
Evidence: `src/state.ts:59-90`; `src/compact.ts:48-67,135-224`;
`hooks/fast-jev.ts:105-156`.

Transcript transformation begins only after `Promise.all` resolves successfully.
If one batch rejects, the hook returns the original event for native summarization,
so this path returns no partial pruning edit. Other concurrent requests are not
cancelled and may still complete or incur cost; no durable decision checkpoint
is written.
Evidence: `src/compact.ts:271-288`; `hooks/fast-jev.ts:263-289`.

[Compaction implementation](https://github.com/tamaratran/fast-jev-compaction/blob/e3f262a7f4d42bd8dd32ced30d26176f7cb545b0/src/compact.ts),
[hook bridge](https://github.com/tamaratran/fast-jev-compaction/blob/e3f262a7f4d42bd8dd32ced30d26176f7cb545b0/hooks/fast-jev.ts).

### Limits and disposition

The fitted history is itself lossy and repeated in each concurrent batch.
Heuristic token estimates do not meter provider charges. If one request fails,
other successful requests may already have incurred costs; the shown Promise.all
path does not cancel them or durably checkpoint decisions.

The code describes removed results as permanent and suggests rerunning for
recovery (`src/state.ts:4-5`; `src/compact.ts:135-145`). Rerunning a
side-effectful, expensive or nonrepeatable tool is not a safe general recovery
strategy. Protecting only initial/recent content does not protect every old
constraint, important failure or receipt.

Adapt pairing, stable identity and this hook's all-or-fallback transcript return
behavior as invariants; this is not durable commit/rollback.
For RelayForge, keep durable minimized outcome records and evidence references
outside compacted working context. Require preservation/retrieval tests before
a later pruning experiment. Checked-in fake-HTTP/hook tests are not evidence of a
live hook session; no tests were run for this review.

## 5. Jev Review: use concrete evidence handles

### Useful mechanisms

A shared workflow screens files, profiles the most relevant candidates, performs
bounded follow-ups, then sorts findings by severity. Current constants limit
follow-ups to eight, profiles to five, concurrency to three and screening threshold
to 0.7; these are project policy, not transferable calibration.
Evidence: `src/review/workflow.ts:37-91`; `src/domain/config.ts:1`.

Change review supplies patch/tests, then asks for a concrete parsed hunk or
`noMatch`. Invalid or low-confidence selections are rejected before classifying
mechanism and severity. Broader review uses bounded source regions and at most
four related test snippets capped at 1,800 characters.
Evidence: `src/review/changes.ts`; `src/review/judgments.ts:163-220`;
`src/review/codebase-judgments.ts:24-28`.

Reports preserve screening matrices, profiles, selected location, mechanism and
severity confidence, ownership and workflow counts.
Evidence: `src/review/workflow.ts:87-106`; `src/domain/types.ts:31-64`.

[Workflow](https://github.com/devagrawal09/jev-review/blob/31f89602797fb7bea007f8a480bf368bf564954e/src/review/workflow.ts),
[evidence judgments](https://github.com/devagrawal09/jev-review/blob/31f89602797fb7bea007f8a480bf368bf564954e/src/review/judgments.ts).

### Limits and disposition

A correctly selected hunk supports inspectability, not bug existence or complete
context. Screening can miss the only relevant file; bounded snippets can hide a
counterexample. The README calls findings review prompts rather than proof.
No complete per-call cost/model/effort ledger or validated review-quality corpus
was established in the inspected code.

Adapt the evidence funnel and no-match outcome for future tool/skill/candidate
selection, with SymForge supplying indexed evidence first. Do not use severity
or classifier confidence as a worker-success label. Deterministic check outcomes
and independently accepted deliverables remain separate.

## 6. SemIf: local decision evidence, not local JEV

### Useful mechanisms

Direct mode renders state/criterion/options, runs a forward pass, reads logits
at uppercase answer slots and softmaxes over those slots. It checks that labels
round-trip as one token and do not retokenize the prompt boundary. Requests support
2–16 choices and reject oversize prompts rather than silently truncating.
Evidence: `src/semif_phase1/core.py:18-46`;
`src/semif_phase1/direct.py:13-77`.

Results include option IDs, logits/probabilities, input-token counts, prompt hash
and version, model metadata and timing. Serial/shared paths add exact-prefix cache
statistics. Serial keeps one state cache and copies it for suffix evaluation;
shared batching requires identical state and unique IDs, with device-specific
CUDA/MPS handling.
Evidence: `src/semif_phase1/serial.py:43-131`;
`src/semif_phase1/shared.py:54-181`.

Offline temperature calibration uses labeled saved logits and leaves argmax
unchanged. It can change gate behavior without changing selected options.
Evidence: `docs/CALIBRATION.md:11-48`.

[Direct scorer](https://github.com/TheoLeeCJ/SemIf-OpenJev/blob/23cf1f39fc9534fe81437200959b6dfc7106e45a/src/semif_phase1/direct.py),
[method](https://github.com/TheoLeeCJ/SemIf-OpenJev/blob/23cf1f39fc9534fe81437200959b6dfc7106e45a/docs/METHOD.md),
[calibration](https://github.com/TheoLeeCJ/SemIf-OpenJev/blob/23cf1f39fc9534fe81437200959b6dfc7106e45a/docs/CALIBRATION.md).

### Limits and disposition

This is independent open-model scoring, not TypeSafe's weights or a demonstrated
equivalent to JEV. Probabilities are conditional on selected vocabulary slots,
excluding other vocabulary mass. Calibration and option/order robustness on
RelayForge workloads need their own validation; the repository's calibration and
perturbation experiments do not transfer automatically. Valid tokenization is
necessary but not semantic correctness.

Prefix reuse requires exact matching state and supported cache operations;
copying cache objects can raise memory pressure, a risk to measure alongside device
behavior. Documented
systems timings use a warm-loaded RTX 3090 and exclude model loading/result writes.
They do not establish end-to-end latency, accuracy or cheaper deployment on this
owner's machine. CLI/backend tests include mocks.

Adapt audit/result fields and provider-specific calibration concepts. Consider a
local backend only as a separately budgeted later experiment including hardware,
memory, load time, idle cost and task quality. Its Python modules are not an
accepted RelayForge runtime or ready cross-language embedding boundary.

## 7. Jev Search: batch ranking and preserve partial results

### Useful mechanisms

- Provider adapters translate a bounded request/answer contract. Outage fallback
  tries each configured provider once and stops on non-outage errors or abort.
  An explicit model selection filters the provider chain to that model instead
  of silently substituting another model.
  Evidence: `src/lib/typesafe.ts:378-401`;
  `src/lib/judge-config.ts:149-154`.
- Relevance ranking batches up to 40 snippet questions over shared request/results
  state, avoiding one request per result. Empty input makes no call.
  Evidence: `src/lib/typesafe.ts:515-567`.
- Search lanes stream independently; a failed engine yields a typed lane error,
  and a ranking failure can leave items available but unranked.
  Evidence: `src/lib/pipeline.ts:194-302`.
- A versioned key includes engine, normalized query, time window and result count.
  Only nonempty successful retrieval is cached; cache errors do not prevent live
  retrieval. HTTP validation/rate limiting precede provider work, with a shared
  30-second deadline and cancellation.
  Evidence: `src/lib/cache.ts:17-49`; `src/routes/api/ask.ts:32-101`.

[Decision adapter](https://github.com/superagents-lab/jev-search/blob/1bfe835e8b4320444a5304043243d4e96ed278b7/src/lib/typesafe.ts),
[pipeline](https://github.com/superagents-lab/jev-search/blob/1bfe835e8b4320444a5304043243d4e96ed278b7/src/lib/pipeline.ts).

### Limits and disposition

The normalizer defaults absent usage to zero and absent answers to an empty map;
missing/refused relevance answers can become zero relevance.
Evidence: `src/lib/typesafe.ts:116-130,561-564`. RelayForge must preserve unknown
usage and unknown judgment separately from actual zero cost or irrelevance.

Ranking happens per lane before merged deduplication. Cached raw search results
still pass through ranking, so a retrieval cache hit does not prove a model-cost
saving. Successful input-token totals do not capture every failed provider attempt
or full cost, and the streaming summary is not a complete model-execution ledger.

Provider-specific translations are useful examples, not proof of equal output
semantics or calibration. Source ranking percentages do not verify source claims.
Queries/snippets reach external providers and raw query text is present in cache
keys; privacy/retention choices need independent review.

Adapt bounded batching, failure isolation and explicit-model fallback boundaries.
A later skill/tool retrieval experiment should compare against current deterministic
inventory/SymForge retrieval. Do not add Search1API, Cloudflare hosting, a search
UI or new browsing machinery merely to reuse these patterns. Owner crawl4ai/web
search workflows remain the relevant baseline.

## What changes in RelayForge's design thinking

1. **Record route adoption in stages.** Suggested, applied, request-sent and
   provider-confirmed identities differ. Attribute outcome to actual execution
   spans; missing identity/usage remains unknown.
2. **Respect the unit of work.** A new user turn can warrant routing; a tool
   continuation usually should reuse the current selection. Keep a controlled
   evidence-driven escalation boundary. Record model-switch/cache/rehydration
   costs rather than assuming every switch saves money.
3. **Keep independent authorities.** Capability inventory limits candidates,
   statistical evidence estimates performance, JEV handles unresolved semantic
   decisions, and deterministic code controls permitted execution.
4. **Make evidence selectable and recoverable.** Stable event/hunk/tool IDs,
   revision/prompt/model hashes, explicit no-fit and coverage flags matter more
   than a persuasive rationale.
5. **Separate history reduction from performance accounting.** Working-context
   pruning must not erase failures, spend, authoritative constraints or receipt
   provenance. Pairing and all-or-fallback updates are useful invariants.
6. **Evaluate the complete alternative.** Compare fixed reasonable baselines,
   deterministic routing and JEV-assisted routing on matched tasks; include
   all attempts, verification, escalation, parent repair, caching and latency.
   A small all-medium replay does not validate a general adaptive router.
7. **Keep provider interchangeability explicit.** A common JSON shape does not
   imply common probability calibration, capacity, privacy or failure behavior.
   A local logits backend is an optional provider experiment, not a JEV clone.

## Recommended admission order

This does not amend roadmap or accepted decisions.

- Foundation contract: make execution identity, uncertainty, evidence references,
  attempt/cost boundaries and actual adapter capabilities explicit.
- After toolchain admission: implement cheap local observations and deterministic
  accounting, then qualify native Codex/Claude capabilities on real sessions.
- Add shadow recommendations and replay comparison against fixed baselines before
  allowing evidence-backed worker routing.
- Only then consider a proxy adapter, semantic tool/skill retrieval, context
  pruning, review assistance or local decision provider as separate experiments
  with data, cost and success criteria.

No inspected project proves the desired Astra-effort versus Sol-max crossover.
The initial foundation slice remains unstarted. The research provides reusable
mechanisms and better experiment designs, without justifying a broad framework
dependency or more product scope in the current slice.
