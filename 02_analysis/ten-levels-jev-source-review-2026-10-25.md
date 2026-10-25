# Ten Levels of JEV: source investigation for RelayForge

Date: 2026-10-25
Status: architect research intake; proposals, not admitted implementation or accepted policy.

## Source and evidence boundary

- Repository: https://github.com/disler/ten-levels-of-jev
- Inspected revision: `777adaf47d37ae0553220d35b2f15b3a3a063305`.
- Read the public README with crawl4ai and inspected the supplied local checkout
  with SymForge, including client, routing, hooks, file tools, and state assembly.
- This is source inspection. No repository scripts, model calls, installs, live
  extensions, or evaluation suites were executed. Bundled tests are specifications,
  not observed passing evidence or proof of model accuracy.
- Code citations below are relative to `apps/ten-levels/` unless that prefix is
  already present. `.claude/` and `LICENSE` references are repository-root-relative.
  Line numbers refer to the inspected revision.
  [Pinned source tree](https://github.com/disler/ten-levels-of-jev/tree/777adaf47d37ae0553220d35b2f15b3a3a063305).
- The fifteen supplied diagrams illustrate intentions. Executable code exposes
  additional cost, error, truncation, and host-integration behavior.
- License is MIT, copyright 2026 IndyDevDan / AgenticEngineer.com
  ([license](https://github.com/disler/ten-levels-of-jev/blob/777adaf47d37ae0553220d35b2f15b3a3a063305/LICENSE)).
  Copying substantial code requires retaining its notice. No source was copied
  into RelayForge and no product licensing decision was made.

Read alongside [the earlier registry source review](registry-source-review-2026-10-25.md)
for primary TypeSafe documentation, confidence semantics, model versions, and
coding-agent outcome taxonomy. This note adds implementation evidence.

## Assessment

This is a useful implementation-pattern library and Pi integration demonstration.
It is not an adaptive registry for measured model/effort performance.

The strongest transferable pieces are typed questions and response validation,
explicit uncertainty outcomes, batching independent questions over bounded state,
separating decisions from execution, and explicit file/concurrency limits.
The largest gaps for RelayForge are evidence-based candidate selection, full
attempt/cost accounting, private-data handling, and verified native adapters.

### What the ten levels contribute

| Level | Observed pattern | RelayForge disposition |
|---|---|---|
| 1: single decisions | Noul or bounded category over supplied state | Useful for unresolved semantic flags; deterministic facts stay in code |
| 2: multiple choice | Multiple independent questions share a request | Reuse batching where answers need the same state |
| 3: composite scoring | Code combines rubric scores with weights | Possible ranking feature; weighted ordinal scores require validation |
| 4: confidence gating | Code maps answers/confidence to actions | Reuse explicit uncertainty path; calibrate each decision family |
| 5: routing | Two model lanes plus low/high effort | Useful shape; replace prose tiers with eligible measured profiles |
| 6: guardrail hooks | Pi tool-call checks and result warning banners | Later optional adapter feature; not a portable enforcement guarantee |
| 7: compaction | Pi context observations, semantic signals, advice and tool | Later host-specific experiment; retain native token budgeting |
| 8: cheap reads | File content becomes JEV state, answer enters main context | Potential semantic filter after code-intelligence retrieval |
| 9: files at scale | Prune first, one call per retained file, bounded parallelism | Later measured experiment; count every attempt and skipped path |
| 10: agentic JEV | Agent authors questions, supplies files/state/command | Experimental diagnostic aid; never registry outcome authority |

## 1. Client contracts worth adapting

The client validates the request before transport and again after JSON
serialization. Response validation checks model/usage shape, requested question
IDs and types, unit-range probabilities, candidate-key correspondence, approximate
distribution sum, selected candidate membership, score range, and score legend.
These are real defenses against malformed responses, beyond a diagram promising
a closed choice set.

Evidence: `apps/ten-levels/src/core/client.ts:154-173,322-362`;
`apps/ten-levels/src/core/types.ts:127-189`.

A closed set limits the returned identifier. It does not establish that the
selected identifier is correct, available, appropriately permissioned, or useful.
RelayForge should deterministically shortlist valid native profiles, include a
no-fit/fallback outcome, then check the returned ID against the actual snapshot.
A probability/confidence value is not permission to execute a destructive action.

The demo exposes request and response events, preserves raw exchanges, and returns
provider, requested model, resolved model, elapsed time, attempts, and cost source.
Those metadata distinctions fit RelayForge. Its cost helper separates reported,
estimated, unknown, and mock costs.

Evidence: `src/core/client.ts:64-97,143-152,250-282` under `apps/ten-levels/`.

**Adaptation required:** raw retention is inappropriate as a default RelayForge
telemetry policy. Full requests can contain code, file paths, user requests, and
command output. Keep minimized metadata, hashes/revisions and local evidence
references; separately authorize any provider-bound sensitive state. Never commit
raw exchanges or secrets. A listener receiving full state is a data boundary.

### Timeout, retries, and identity

- Defaults use `jev-latest` or OpenRouter's `~typesafe/jev-latest`.
- Live calls have a 30-second shared deadline and at most three HTTP attempts.
  Retryable statuses are 429, 502, 503, and 529, with jittered exponential backoff
  and bounded Retry-After handling.
- Provider selection is explicit or environment-derived; missing credentials
  throw rather than silently using the mock. Test-context isolation defaults to
  mock unless explicitly opted into live behavior.
- Successful results include attempt count. The logical `calls` counter increments
  once before transport, not once per retry. Observers receive request and
  successful response events; the shown event union has no matching error event.

Evidence: `src/core/client.ts:18-35,95-97,112-136,154-248,261-282`.

RelayForge needs event records for failed, timed-out, cancelled, retried, and
successful attempts, including unreported charges as unknown. One successful
response's reported usage is not proof of total retry cost. Record stable request
and attempt IDs, adapter/provider/model identities, latency and retry causes.
A changing alias must not pool distinct models into one calibrated population.

The declared 64,000-token budget is a library constant, not a general tokenizer
enforcement in `validateRequest`. File/state helpers use character estimates.
Do not mistake the exported constant for a guarantee that every request fits.
Evidence: `src/core/types.ts:107-136`.

## 2. The routing example does not solve the registry problem

The model router asks for `fast` versus `powerful` based on descriptions and an
ordinal effort score. A pure function maps scores above 1 to high effort and
everything else to low. Confidence is printed in a rationale; this function does
not use it to abstain or look up local success rates.

Evidence: `src/levels/level05/model-router.ts:9-37`.

Missing from this route are exact native IDs, supported effort enumerations,
availability, model/effort-specific outcomes, task difficulty provenance,
tool competence, recovery costs, uncertainty, and verified dispatch receipts.
The example also does not establish a crossover between Astra and Sol max.

Proposed RelayForge flow, consistent with existing requirements:

1. Code assembles eligible native profiles and observed statistics, using the
   current harness, tools, task requirements, and policy.
2. Code selects directly if existing rules/evidence settle the route.
3. Otherwise JEV answers bounded questions over that minimized snapshot.
4. Code validates the recommendation, applies uncertainty/fallback policy, and
   sends an advisory result or uses an explicitly supported dispatch adapter.
5. Observation records the actual worker and effort, acceptance, failures and
   recovery; consolidation updates derived evidence rather than capability facts.

This keeps semantic judgment useful without asking JEV to invent benchmarks or
discover capability facts from prose.

## 3. Cheap reads save main context, not necessarily total work

Level 8 reads the entire eligible UTF-8 file and sends it as JEV state. A NUL byte
within the first 8 KiB is its binary heuristic. Missing, non-file, binary, and
oversize inputs fail. The implementation enforces a 240,000-byte file-size cap
using `stat().size`, despite naming the constant `MAX_FILE_CHARS`. The constant
comes from a 64k-token budget minus a 4k question margin at four characters per
token; this is not an exact UTF-8 token calculation. Files are not silently
truncated.

Evidence: `src/levels/level08/read-state.ts:9-43`;
`ask-file-bool.ts:20-31`, `ask-file-choice.ts:20-32`,
`ask-file-score.ts:22-33` in that directory.

The main model receives a compact judgment rather than file text. The file still
must be read and transmitted to a model. That adds input cost, privacy exposure,
latency, and the risk of a wrong semantic answer.

For this owner's workflow, use SymForge to shortlist files/symbols and retrieve
bounded evidence first. Exact symbols, string presence, types, references,
syntax, and tests have deterministic or indexed answers. A JEV file question
could help rank an unresolved semantic candidate; it cannot certify that token
validation is correct or a security property holds.

The upstream tool descriptions themselves reserve read/grep for exact lookup,
quoting and editing:
`extensions/ask-jev-file.ts:15-18`;
`extensions/ask-jev-files.ts:29-32`.

Proposed result additions: content revision/hash, scope of evaluated evidence,
question/rubric version, typed answer, uncertainty, skipped reasons and an evidence
reference for targeted follow-up. An answer without source text is a triage hint,
not independently checkable proof. Preserve an unknown outcome and a path back to
the relevant source, especially after files change.

## 4. Files at scale: bounded parallelism is not free batching

Level 9 validates questions, expands paths/patterns, prunes candidates, then sends
one request per retained file. Every request carries the same question block.
Default concurrency is 16. The default candidate cap is 255; lock/binary extension
lists, generated/dependency directories, empty/oversize and outside-root paths
are pruned. Default directory expansion is nonrecursive.

Evidence: `src/levels/level09/ask-files.ts:24-75`;
`src/levels/level09/prune.ts:10-55`.

Per-file exceptions become skipped records. The returned `calls` count is the
number of successful results, so it can understate attempted requests and costs.
A later pick-first request can choose from at most 254 paths plus `none`;
confidence below 0.3 produces no pick. This adds another call when candidates exist.

Evidence: `src/levels/level09/ask-files.ts:62-75`;
`src/levels/level09/pick-first.ts:23-36`.

For RelayForge, prefer deterministic/indexed narrowing, deduplicate by file
revision and question version, then consider bounded semantic ranking. Bound total
files, estimated input tokens, attempts, concurrency, latency and spend separately.
Count errors/retries and selection calls. Cache only when evidence and policy
versions match. A character estimate is a preflight heuristic, not metered usage.

Compare against SymForge and direct targeted reads on realistic tasks before
admission. Measure relevant-file recall, false exclusion, extra reads after a
wrong answer, main-model context saved, total charges, latency and accepted work.
A missed file that causes a failed implementation can erase apparent savings.

## 5. Guard hooks: Pi implementation, partial coverage, fail-open

The guard extension subscribes to Pi `tool_call` and `tool_result`.
It checks bash before execution, write/edit path/content before mutation, and
screens text from read/bash results. A positive screen appends a warning banner;
it does not remove the suspicious payload or establish injection immunity.

Evidence: `extensions/jev-guard.ts:20-51`.

Important limits:

- Hook errors are caught and reported, then execution falls through without a
  block. This is fail-open, including a client timeout.
- Result screening evaluates only the first 6,000 characters, skipping empty text.
- Exact tool names determine coverage. Different tools, execution paths, or a
  session without the extension are not shown to be intercepted.
- Level 6 implements allow/block. The earlier Level 4 example has a separate
  human-review branch; the diagram does not mean every hook supports it.
- A command label such as `npm test` does not prove harmlessness: scripts and
  dependencies determine its actual behavior.

Evidence: `extensions/jev-guard.ts:22-51`;
`src/levels/level06/result-screen.ts:41-47`;
`src/levels/level06/bash-gate.ts:32-52`;
`src/levels/level04/shell-command-gate.ts:35-54`.

The lab launches Pi RPC with explicit extension/tools and suppresses ambient
extensions, skills, prompts and context files. Those controlled demo sessions do
not reproduce the owner's full installed harness configuration.
Evidence: `server/agent-session.mjs:91-100`;
`server/server.mjs:87-103`.

Native permissions remain authoritative. RelayForge should declare each adapter's
actual observation, suggestion, dispatch, pre-call and post-result capabilities.
MCP exposure or a skill hint alone does not prove interception of built-in tools.
Any later gate needs an explicit policy for timeout/unknown decisions while
retaining existing host protections. No new approval flow is adopted by this note.

## 6. Compaction: useful observations, host-specific action

The extension builds state from user requests, prior summary, latest turn and Pi's
context-usage API. It asks semantic questions at turn end after a user request.
Eligibility depends on the host's keep-recent setting (sandbox: 4,000 tokens);
demo notice/recommend/request tiers use 6k/10k/14k. Mid-operation/same-task signals
can suppress guidance.

Evidence: `extensions/jev-compact.ts:21-24,40-67,85-112`;
`src/levels/level07/should-compact.ts:19-104`;
`sandbox/.pi/settings.json:1-6`.

At 14k it injects a request for the agent to compact. The separate `compact_now`
tool invokes Pi `ctx.compact`. A pre-compaction hook asks which user turn begins
live work, then changes compaction instructions when supported; a low-confidence
or no-cut answer falls back to generic guidance. Pi produces the summary.

Evidence: `extensions/jev-compact.ts:114-170`;
`src/levels/level07/pick-cut-point.ts:31-43`.

The diagram's arrow to COMPACT compresses a recommendation, agent action, and
host summarization into one step. There is no JEV-generated summary or guaranteed
automatic compaction at the drawn threshold.

For RelayForge, token counting and hard limits stay deterministic. Semantic
boundary/cut-point advice is an optional later experiment only where the adapter
can request or influence compaction. Thresholds must use the actual context
window, recent-context policy, caching and expected next operation; the demo's
absolute numbers are not portable defaults. Include compaction/rehydration cost
and lost-detail recovery in outcome accounting. Durable event recording still
needs to survive abrupt termination independently of compaction.

## 7. Agent-authored questions: useful diagnostics, weak outcome authority

Level 10 lets the agent supply state, file paths/globs, questions and an optional
command. Assembly caps own state at 8,000 characters, files at 20, and aggregate
state at an estimated 60k tokens with a 4k question margin. File globs recurse.
Overflow errors explain size contributors rather than silently trimming the
assembled state.

Evidence: `src/levels/level10/assemble.ts:10-17,60-145`;
`src/levels/level10/ask.ts:18-45`.

The extension's command adapter runs shell `exec` with a 60-second timeout and
800,000-byte maxBuffer. It separately slices stdout/stderr to 200,000 characters
each before assembly. Therefore the assembler's no-truncation rule does not mean
the original command output is complete. A nonzero exit can be classified as data.

The command is checked by the Level 6 bash gate before execution, adding a possible
JEV call ahead of the final classification. A model judgment is not an OS sandbox.
Prefer existing approved command execution and bounded receipts; avoid adding a
second arbitrary-shell lane through the routing core.

Evidence: `extensions/ask-jev.ts:28-35,57-80`;
`src/levels/level10/assemble.ts:136-145`.

The tool can suggest whether a failure relates to code, tests, environment, or
other causes. That suggestion alone cannot amend success/failure statistics.
Agent-authored alternatives can omit the real cause, lead the answer, or excuse
the agent's own errors. Preserve an unknown option, original deterministic
receipts and independent acceptance evidence. Version approved evaluation
questions; keep exploratory diagnostic questions out of calibrated outcome labels.

Possible later controls: bounded question count/state/attempt budget, fixed
candidate IDs where relevant, question provenance, no recursive unlimited JEV
loops, and a requirement for new evidence before retry. These are proposals,
not new product scope or permission machinery.

## 8. Cost display and accounting caveats

The web cost comparison tokenizes sent/received JSON and applies a static table
of model prices. JEV uses reported cost when present or a hardcoded input rate.
It scales that per-call estimate to volumes. This is an illustrative equal-payload
comparison, not a measured model-capability benchmark or full coding task cost.

Evidence: `web/src/lib/cost.ts:11-19,24,46-64`.

It cannot establish current prices, equivalent decision quality, thinking-token
cost, cache effects, latency, tool-use quality, retries, recovery, or accepted
coding output. Do not import that price table into the registry as live facts.

The Level 10 spend helper records its ask calls, not every Level 8/9 call or all
guard decisions. Per-file returned usage is useful but not a complete session
ledger. Evidence: `src/levels/level10/spend.ts:25-34`;
`extensions/ask-jev.ts:67-80`;
`extensions/ask-jev-files.ts:38-45`.

RelayForge's counterfactual is the complete alternative workflow. Compare total
cost per accepted task alongside completion rate, first-attempt success,
wrong-tool-call rate, parent repair and wall time. Report unknown costs explicitly.
Neither 300 ms nor a quoted million-call price is a local service-level guarantee.

## 9. Bundled skill: reference design, not an installation recommendation

The repository's `.claude/skills/hyper-jev/SKILL.md` describes a portable
TypeScript client and cookbook, provider selection, bounded questions, explicit
mock behavior, calibration, and scope limits. It explicitly distinguishes offline
contract testing from model accuracy. These are useful development instructions.

It also mandates reusing its TypeScript client and preserving full raw payloads.
Those instructions are source material, not RelayForge authority. Product language
is undecided; automatic raw-state retention conflicts with this project's default
telemetry/privacy boundary. A coding skill helps an agent build an integration;
it does not continuously observe sessions, intercept calls, or dynamically dispatch
workers on its own. Do not install the whole skill/starter merely to obtain
these patterns.

If a RelayForge skill is later warranted, keep it thin: discover the installed
core/adapter, ask for a bounded route when necessary, interpret the typed result,
and report actual execution. Runtime logic and statistics belong in software,
with progressive reference loading rather than a large permanent prompt.

## 10. Suggested admission order

These are analysis recommendations, not changes to roadmap or accepted decisions.

1. Keep the existing foundation scope: inventory, route, observation, outcome and
   reconciliation contracts. Add missing uncertainty, identity and attempt/cost
   boundaries learned here to their definitions.
2. After toolchain admission, qualify native Codex/Claude adapter capabilities and
   actual execution identity. Establish cheap local observation and measured
   model/effort routing before adding more JEV decision families.
3. Consider semantic file ranking, compaction advice, dynamic diagnostics or
   optional tool gates only as separate, budgeted experiments with a deterministic
   baseline and acceptance metric. Retain only demonstrated improvements.

The first foundation slice remains unstarted. This investigation does not claim
a RelayForge runtime, install a hook, change any harness configuration, select a
product language, or establish model performance.
