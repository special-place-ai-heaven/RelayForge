# RelayForge delivery handover, 2026-10-25

## Stop state and access

The owner requested a pause after the reviewed checkpoint, commit, push and
handover. That handover reached `577e276`; the delivery goal is paused. Subsequent
landing housekeeping does not resume it. Resume only on the owner's instruction.

This is a dated navigation and evidence summary, not a replacement for the
roadmap, decision register or ledger.

- Use the primary RelayForge checkout on `main`. The dated landing and original
  draft recovery details are in [landing-preservation-2026-10-25.md](landing-preservation-2026-10-25.md).
- Remote: https://github.com/special-place-ai-heaven/RelayForge
- Last reviewed code checkpoint: M006-S01 accepted at 995ab1396eb3a5e789c4ae523be21924a7f18303; milestone closed and pushed at 09f169c8d1c2c336c0b2e893d43cb65fe78ddc9e.
- The campaign handover was committed at `577e276`; landing housekeeping may
  follow it. Read actual HEAD, branch, status and ledger before resuming.
- The older primary canary, research, ledger and prompt drafts were preserved in
  a named local stash/recovery reference before `main` was fast-forwarded. They
  remain historical drafts; do not apply them over accepted campaign files.
- No release package, source archive, installer, release publication or PR was
  created by this campaign. Branch pushes were explicitly authorized.

## Goal and authority

Deliver the full first usable release in 00_brief/release-contract.md, G1 through
G6. A completed milestone or empty admitted queue is not completion. The owner has
now explicitly requested that this session pause because of quota consumption; the next context
must retain the full objective and remaining gates.

Read AGENTS.md and follow its architect entry order:
initialization/architect.md; 00_brief/CONTEXT.md and its named brief files;
roadmap.yaml; docs/operating.md; docs/workflow-graph.md;
prompts/GOAL_RELAYFORGE_BOOTSTRAP.md; then the bounded analysis entry files.
This handover supplies navigation, not new authority.

Authority has one home per fact:
- roadmap.yaml: admission, slice boundaries and checks.
- 00_brief/decisions.md: accepted decisions.
- 05_governance/ledger.jsonl: actual workflow history.
- questions/open/Q001-template-distribution-permission.md: open distribution gate.

The handoff skill also references the owner's external AI-OS-Wiki/00-Index/CLAUDE.md
and its named operating files. Those are optional environment context if supplied;
the repository's explicit startup/read boundaries take precedence. Do not ingest
unrelated company/brand files or all research to resume this project.

The owner requires an architect/orchestrator who does not implement code.
Delegate code to model-pinned workers. Existing AGENTS.md uses Luna for bounded
work and Sol for evidence-backed escalation or sensitive cross-system work.
Do not let a chair-model full-history fork silently become the implementation lane.
One worker at a time was adopted to control this session's quota use.
A new context's larger weekly allowance is not a project inference/spend budget.

## What is successfully implemented and accepted

| Area | Completed behavior | Principal evidence |
|---|---|---|
| Foundation | Native metadata, capability and routing contracts; configured, unsupported and unknown evidence distinguished; toolchain/storage admission | M001 accepted review and 02_analysis/foundation-contract.md |
| Licensing and housekeeping | First-party PolyForm Noncommercial 1.0.0; owner's required notice; consistent metadata/README; separate imported/dependency attribution; reviewed ignore policy and non-force pushes | D011-D014; commits e93c4e4 and 89c0937; LICENSE; THIRD_PARTY_NOTICES.md |
| M002 canary | Rust 1.99.1/SQLite qualification; transactional durability, crash/replay behavior; discovered/executed tests; clean-candidate verifier with original Git ancestry | bd0df83, milestone 69e2f70; 05_governance/reviews/m002/ |
| M003 native core/CLI | Versioned bounded events; immutable SQLite append, source/event deduplication and conflict refusal; committed intents; explicit ownership/recovery; reconciliation; preserved failures/unknowns; verification plus explicit acceptance; actual CLI processes | 4d6126f, milestone de05ed4; native-core-plan.md and native-core-results.md |
| M004 CI | Hosted Windows full checks and five Linux stages on self-hosted linux/fresh runners; public-fork exclusion from custom runners; pinned actions, job-local pinned/checksummed Rust bootstrap, empty compiler-wrapper overrides and negative fixtures | milestone 65a092c; docs/ci.md; 05_governance/reviews/m004/ |
| M005 inventory | Immutable bounded registry snapshots; exact profile/model/effort/harness metadata; capability source/scope/freshness checks; separate host-supplied current state; selection without substitution; CLI import/show/eligibility | cecf822; inventory-plan.md and inventory-results.md |
| M005 accounting/comparison | Immutable pre-attempt context; all physical attempts, retries and parent repair; explicit acceptance; known subtotals versus unknown totals; ambiguous/mixed-unit/overflow handling; exact cohorts, Wilson intervals, fixed baseline and honest insufficient-evidence results; actual CLI reports | 379d9c2, milestone 579e7e8; accounting-plan.md and accounting-results.md |
| M006 workflow repair | Resolution-authority reads accept verified append-only register changes while preserving the original reachable Git content; ordinary evidence remains strict | 02_analysis/authority-history-results.md; 05_governance/reviews/m006/ |

The existing product remains a Rust library plus CLI. Python is repository workflow
tooling, not the product runtime. Direct product dependencies are pinned:
serde 1.0.228, serde_json 1.0.154 and rusqlite 0.40.0; bundled SQLite runtime
3.53.1 through locked libsqlite3-sys 0.38.0. Preserve Cargo.lock and Rust 1.99.1.

Current CLI functionality includes init, append, show, reconcile and sqlite-version,
plus inventory-import, inventory-show, eligibility, context-import, accounting
and compare. Older core commands use --database; newer inventory/accounting paths
use --db. Consult their implemented parser and results examples before invoking.
No command currently implements native harness observation/adoption or JEV transport.

The standalone eligibility and comparison commands deliberately report
binding_unavailable. The positive eligibility/comparison paths exist in the
library and are tested with separately supplied current state. Fixtures are not
evidence of native runtime capture or production routing effectiveness.

## Checks that actually passed

- M005 final focused and full clean-candidate verification: 17 product unit tests,
  15 integration tests, and one doctest; retained canary, formatting, Clippy,
  build, SQLite runtime and source/index/HEAD invariance checks also passed.
- M006 current full receipt: 05_governance/reviews/m006/M006-S01_r3_verification.json: ok=true, full command exit 0, 128.063 seconds, stable before/after witness at baseline 7d3818a.
- Dedicated authority-history suite: nine focused tests passed; the same required suite is included in the passing full clean-candidate command.
- The full command is python -B scripts/hermetic_verification.py. It now includes
  scripts/test_authority_history.py as well as roadmap/ledger/Git and product checks.
- The focused product command is python -B scripts/product_verify.py --focused.
  CI has explicit fmt, clippy, unit, integration and doc stages.
- Required discovery gates reject absent required product tests. The authority
  test entrypoint rejects an empty or skipped suite; a missing file fails execution.

Observed GitHub run evidence:
| Source revision | Run | Observed result |
|---|---|---|
| e3e30a1 | https://github.com/special-place-ai-heaven/RelayForge/actions/runs/40213004003 | Windows and all five Linux stages passed; first successful Linux qualification |
| 65a092c | https://github.com/special-place-ai-heaven/RelayForge/actions/runs/40214780155 | M004 closure passed |
| cecf822 | https://github.com/special-place-ai-heaven/RelayForge/actions/runs/40216604672 | Inventory slice passed |
| 579e7e8 | https://github.com/special-place-ai-heaven/RelayForge/actions/runs/40219234590 | Accounting/M005 closure passed |
| 09f169c | https://github.com/special-place-ai-heaven/RelayForge/actions/runs/40223019954 | Windows and all five Linux stages passed; observed after handover preparation |
| 577e276 | https://github.com/special-place-ai-heaven/RelayForge/actions/runs/40223231330 | Handover commit: Windows and all five Linux stages passed |

Successful qualification observed Windows x86_64-pc-windows-msvc and Linux
x86_64-unknown-linux-gnu with rustc/Cargo 1.99.1. Windows used CPython 3.11.9;
the custom Linux jobs reported Python 3.12.3. The fresh label does not establish
runner ephemerality, host isolation or future availability. Fork safety is
configuration and negative-fixture evidence, not a performed live-fork experiment.

## Issues encountered and disposition

| Issue | Disposition and evidence |
|---|---|
| Missing imported-template license grant | UNRESOLVED, Q001. Independent implementation is allowed; packaging/distribution is blocked pending permission or reviewed exclusion inputs. |
| Canary negative guards were not all executed by the maintained full path | RESOLVED during M002 rounds; retained full/discovery checks and accepted r4 evidence. |
| Scratch Git whitespace/staging behavior and later lost commit ancestry | RESOLVED: private scratch Git metadata/index/objects with original HEAD as parent; ancestry and ledger checks precede builds; original checkout/index remain protected. |
| Native verification helper stalls/abrupt-exit behavior | RESOLVED: explicit CLI path, bounded child waits and controlled abrupt-exit helper; crash/recovery tests passed. |
| Latest Unknown/NotRun after Success could still allow task acceptance | RESOLVED, M003-S01-001; acceptance requires the latest successful result plus required checks. |
| Session close could strand unresolved intents | RESOLVED, M003-S01-002; unresolved/missing results prevent close; recovery and later-result regressions passed. |
| Linux runners lacked rustup | RESOLVED, D019/S03. First failed run 40209605799 is preserved; job-local pinned/checksummed bootstrap added. |
| Linux inherited unavailable sccache wrapper | RESOLVED, D020/S04. Failed run 40211448552 is preserved; both compiler-wrapper variables cleared, including step-override guards. |
| One CI checksum-fixture review diagnosis was incorrect | CORRECTED by reviewer in S03 r2. Original review was retained and the mistaken diagnosis explicitly withdrawn, not presented as a product defect. |
| Current CI guide still claimed Linux was pending | RESOLVED by independently reviewed architect documentation hotfix 80d4ebe, explicitly outside S02's frozen envelope; S02 r3/holistic closure cite it. |
| Editing historical blocker evidence broke its pinned hash | RESOLVED by restoring original ci-qualification.md and placing subsequent observations in new S02 r2 handoff notes. That old document remains historical evidence, not current CI status. |
| Appending the decision register broke historical resolution-authority checks | RESOLVED in accepted M006: narrow same-path reachable-history proof returns original cited content, with exact current prefix; ordinary artifacts stay strict |
| New authority tests disappeared from the clean candidate | RESOLVED, M006 r2: moved required suite from export-ignored /tests into scripts; retained imported export policy |
| Authority fixture Git commands inherited the verifier's private Git environment | RESOLVED, M006 r3: isolate six inherited repository selectors in disposable fixtures; regression plus full clean-candidate pass, parent HEAD/index preserved |
| Unsupported/configured capability facts were classified before scope/freshness applicability | RESOLVED, M005-S01-001; checks now precede all four statuses; failing-first regression and full tests passed. |
| Late UsageCost receipts incorrectly looked like later unresolved work | RESOLVED during M005-S02 implementation; accounting receipts are distinguished from later work. |
| Missing cohort/profile mismatch and overlapping-interval tests | RESOLVED, M005-S02-001; added table-driven boundaries, discovered by verifier; no production correction was needed. |
| Formatting, Clippy and new-test discovery failures during development | RESOLVED before final accepted receipts. These were verification/development failures, not silently counted as passing checks. |
| SymForge connection could select the wrong checkout | OPERATIONAL RISK: earlier wrong-checkout edit was restored. Each worker must select the primary `main` index and confirm status; do not trust another connection's selection. |
| Native startup metadata is insufficient for the current resolved-identity gate | OPEN DESIGN/INTEGRATION GAP, explained below. No adapter smoke or policy correction has been implemented. |
| Runtime notices absent at checked minimal-toolchain documentation paths | OPEN DISTRIBUTION INPUT TASK. Obtain verified runtime/standard-library notice inputs; Cargo package metadata alone is not enough. |

Historical failures and superseded notes are intentionally retained. Do not rewrite
their text or hashes to make the history appear clean.

## Native adapter findings, verified scope and open decision

Installed versions were checked locally on 2026-10-25: Codex CLI 0.164.0 and
Claude Code 2.1.306. No real fresh-process adapter observation smoke was launched.

Codex's generated v2 schema in the temporary relayforge-g4-codex-schema directory
was inspected. Revalidate or regenerate through the installed CLI help if that
temporary directory is gone. Relevant files are the v2 aggregate schema plus
ClientRequest.json and ClientNotification.json.

- Client methods include initialize, model/list, thread/start and turn/start;
  initialized is a notification.
- ModelListResponse.data entries expose id, model, defaultReasoningEffort and
  supportedReasoningEfforts entries with reasoningEffort.
- ThreadStartParams accepts ephemeral and optional model/modelProvider.
  ThreadStartResponse returns model, modelProvider, reasoningEffort and thread.
- Thread.model/reasoningEffort are configured/persisted settings, not per-turn
  execution telemetry. No resolved backend revision is established by these fields.
- Candidate zero-inference probe: initialize, initialized, model/list, ephemeral
  thread/start, then orderly shutdown. Never send turn/start in that probe.

Claude's local help exposes model, effort, settings and setting-sources options.
The official [hooks reference](https://code.claude.com/docs/en/hooks) describes
SessionStart source, optional active model and common session/workspace fields.
Its model identifier does not establish a backend version or per-turn fallback.
An isolated startup hook is a candidate observation path, not a completed smoke.

The current evaluator rejects a profile with unknown resolved_model_id or
resolved_model_version as unknown_execution_identity. Therefore startup metadata
alone cannot currently produce an eligible model candidate. Do not invent a
backend revision from an alias or copy configured selection into observed execution.

The next architect must decide and record a truthful, versioned contract before
implementation. Possible directions needing evaluation are retaining the strict
gate until an approved genuine execution receipt exists, or separating verified
capability/configuration eligibility from execution-identity requirements for
performance comparison. The latter must never allow configured-only required
capabilities to qualify, unknown execution identity to support a performance win,
or advisory/manual adoption to masquerade as actual dispatch. No such native-policy correction
has yet been accepted. D021 records only the owner-requested pause and handover. Do not treat this proposal as authority or quietly reduce
G4 to printed advice.

G4 still requires each adapter's versioned coverage table and a real fresh-process
observation path into the core, plus supported native or explicit CLI consumption
with adoption recorded. There is currently no adoption-recording path. A manually
typed --model option is not, by itself, evidence that a recommendation was consumed.

Use isolated owned profiles/workspaces, bounded processes and allowlisted metadata.
Preserve unknowns, redact/drop private raw fields, and clean up only owned children.
Do not inspect credentials, configure global hooks or assume a paid inference
allowance. These limits remain even if the next agent has no weekly model quota.

## Release gaps that remain

| Gate | Current assessment | Work still required |
|---|---|---|
| G1 | Native build/core/CLI/lockfile/CI substantially implemented | Reviewed distribution inputs, actual artifacts, Windows local install/uninstall and final reproducibility evidence. |
| G2 | Core durability/recovery/ownership/privacy behavior implemented and tested | Final release-level coverage audit and native adapter integration evidence; metadata validation is not a universal secret detector. |
| G3 | Inventory/accounting/descriptive comparison implemented and tested | Resolve native current-state/identity boundary and demonstrate useful live integration honestly; current standalone commands cannot supply trusted binding. |
| G4 | Not implemented; bounded interface investigation only | Codex and Claude adapters, native smoke receipts, coverage tables, current-state capture and recommendation consumption/adoption. |
| G5 | Not implemented | Bounded versioned provider boundary, strict eligible-set validation, timeout/error/abstention, cache/bypass and zero-influence shadow semantics. Live provider validation remains unapproved without credentials and explicit incremental budget. |
| G6 | Not completed | Representative reproducible evaluation, observed usage completeness/accepted-work/overhead versus fixed baseline, onboarding/privacy/recovery/troubleshooting documentation, independent release review with no open P0-P2 findings. |

G5 permits a first local release with contract-tested but live-unvalidated JEV only
if prominently disclosed and shadow-only. It does not permit live effectiveness
claims. There is no required positive savings result; negative or inconclusive
evidence is legitimate. Automatic effective routing needs separate policy/evidence.

Tool prerequisite bundles remain deferred until narrowing is admitted. Do not
import the reviewed community harness or add dependencies/paid experiments on the
authority of research notes.

## Distribution, licensing and preservation

Q001 names imported agentic-project-template-v4 revision
169a74c66140743008bef374f2c7d0389ee81cb9. No standalone grant was found in the
checked upstream material. First-party PolyForm licensing does not grant rights
to that imported material.

02_analysis/distribution-input-observations.md records the read-only audit:
root LICENSE exactly matched the SymForge reference and owner notice; native
core/CLI addition history starts in first-party commit 4d6126f; locked metadata
was inspected. It is not exclusion approval.

The proposed route is a binary-only explicit allowlist containing the first-party
CLI, its LICENSE/notice, original product instructions, and separately preserved
dependency/runtime notices. Independently review provenance and exact inputs
before packaging, then inspect every artifact. Exclude all imported/derived
template scripts/docs/prompts/governance, canary, stores, credentials, caches,
intermediates and raw data. Source archives need their own review if created.

Keep source, tests, Cargo.lock, docs and required ledger/review evidence tracked.
Artifact exclusion is separate from .gitignore, which never untracks existing
files. Preserve template/dependency licenses and attribution separately.

## Operational lessons for the next context

- Use the actual ledger step, not a prior chat or this summary, to resume.
- Preview each real generated prompt; frozen blocked envelopes do not widen just
  because roadmap prefixes changed. Admit an explicit correction when needed.
- Record coded work before the one architect-owned full verification run.
- Before independent review, check receipt success, stable matching baseline,
  required evidence, diff scope and stale current references.
- First coherent review collects all findings. Re-review fixes and affected
  behavior; required holistic closure reuses accepted code evidence.
- Historical outcome evidence remains immutable. Only resolution-authority references may use the narrow original-content reader; outcome/artifact evidence cannot use it.
- Root /tests is export-ignored by the imported template. Required project
  verification tests must survive clean-candidate materialization.
- Disposable Git fixtures must isolate inherited Git selectors and preserve their
  parent environment/HEAD/index. git -C alone is not sufficient isolation.
- Keep long/noisy commands in Terminal Commander. Do not restart a live command
  because observation timed out; poll its exact handle and observe terminal state.
- On Windows, PowerShell exec uses tty=true, login=false. No visible helper windows.
- Do not kill host services/processes. No force-push. Report authentication failure
  or remote divergence rather than hiding it.
- Verified commit identity: special-place-administrator,
  administrator@special-place.online. No AI co-author trailers.
- GitHub Linux jobs use self-hosted, linux, fresh. Windows uses windows-latest.
  Preserve the public-fork trust gate and read-only token/checkout policy.
- Review prompts/report table source/hash cells should be raw values, not
  backtick-decorated strings that confuse the parser.
- Source/raw runtime paths and secrets do not belong in tracked evidence.

## Exact resumption steps

1. Obtain the owner's resume instruction, enter the primary `main` checkout and
   verify branch/HEAD/status. Keep the saved pre-landing drafts under their
   recovery reference; do not reapply them over accepted files.
2. Follow AGENTS.md architect startup. Run:
   python -B scripts/roadmap.py check
   python -B scripts/ledger.py check
   python -B scripts/ledger.py status
   git status --short
3. Confirm M001-M006 closure and any pending commit intent from the ledger.
   Verify https://github.com/special-place-ai-heaven/RelayForge/actions/runs/40223231330
   and the newest `main` run; do not infer success from a push.
4. Read the native adapter findings above and the exact G4 contract. Resolve and
   record the identity/adoption boundary without weakening required evidence.
   D021 is the first real append after the repair; its ledger check is recorded
   in the handover review. No new adapter policy was appended here.
5. Admit the next bounded native slice in roadmap.yaml. Preview and issue its
   prompt; delegate implementation, verify, independently review, commit and push.
6. Continue G4, then G5 and representative evaluation. Resolve Q001 through actual
   permission or independently reviewed exclusions before any packaging.
7. Finish all G1-G6 evidence, installation/uninstallation and independent release
   review. Only then mark the full delivery goal complete.

There is no authorization here to claim a full release, run paid experiments,
configure the owner's global harnesses, or publish artifacts.

## First prompt for a fresh context

On the owner's instruction, resume the paused RelayForge full delivery campaign
using this handover and the primary `main` checkout. Act as architect/orchestrator
and delegate code to task-appropriate model-pinned workers. Read AGENTS.md and its ordered startup
files, verify the actual ledger and Git/CI state, then address the native
identity/adoption boundary and continue the remaining G1-G6 work. Preserve accepted
evidence and the saved pre-landing drafts. Keep Q001 as a packaging/distribution
blocker until permission or reviewed exclusion is established. Use the verified
owner identity, no AI trailers, and non-force pushes after reviewed commits.
No weekly model quota does not authorize a project provider-spend experiment.
Do not restart the foundation, waive missing gates, or stop at an empty queue.
