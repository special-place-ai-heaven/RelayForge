# Decision register

Accepted project decisions have one home here. Append new entries when a decision
changes; do not rewrite accepted history. Roadmap owns plan; ledger owns execution.

## D001 — Adopt the requested v4 manual scaffold

- Status: accepted
- Date: 2026-10-25
- Decision: Initialize RelayForge from jequihua/agentic-project-template-v4 at
  169a74c66140743008bef374f2c7d0389ee81cb9 using its git archive export.
  Preserve existing RelayForge Git identity and the template's numbered layout,
  scripts, AGENTS/CLAUDE entry, folder contexts, and manual /2 loop.
- Reason: Explicit owner request. Export excludes template qualification tests.
- Source: https://github.com/jequihua/agentic-project-template-v4
- Supersedes: none

## D002 — Define RelayForge around completed-work economics

- Status: accepted
- Date: 2026-10-25
- Decision: Build toward an embeddable routing/performance core and CLI that
  combines capability inventory, exact model/effort evidence, bounded JEV choices,
  tool-use/session tracking, recovery costs, and adaptive ranking. Codex and Claude
  are first; other harnesses are deferred.
- Reason: Owner requirements throughout project intake.
- Supersedes: none

## D003 — Separate workflow Python from the product language

- Status: accepted
- Date: 2026-10-25
- Decision: Retain the template's Python workflow scripts. RelayForge product
  language is not Python and remains undecided. Rust is a recommendation only.
  Existing external Python JEV code supplies reference behavior, not a mandate
  to reuse its language or install it as RelayForge.
- Reason: Explicit owner rejection of Python for the product while choosing this
  template. Early research assumed Python before that preference was stated.
- Supersedes: Python product-runtime assumptions in imported phase-one research

## D004 — Keep adaptation local, inexpensive, and bounded

- Status: accepted
- Date: 2026-10-25
- Decision: Observe small metadata events during work; compute failure/recovery
  statistics locally; consolidate on completion/end and reconcile on next start.
  JEV sees current evidence at unresolved decisions. Routine capture and reduction
  incur no model calls. Metrics do not change permission/capability facts.
- Reason: Owner wants ongoing performance feedback, low overhead, and survival
  of abrupt session endings.
- Supersedes: none

## D005 — Establish a scaffold baseline without claiming product execution

- Status: accepted
- Date: 2026-10-25
- Decision: This initialization populates project authority and research, configures
  scaffold checks, validates the manual tools and first prompt preview, and creates
  a local baseline commit required for a clean loop. Do not issue a real slice,
  fabricate ledger events, install harness hooks, run paid evaluations, or publish.
- Reason: Owner requests a ready project and will request the bootstrap/handoff
  prompt separately.
- Supersedes: none

## D006 — Admit a bounded foundation contract before native code

- Status: accepted
- Date: 2026-10-25
- Decision: M001-S01 is a docs slice producing a compact native foundation contract
  and at most two non-Python toolchain options. It cannot select an unapproved
  language or implement the runtime. After review, the architect records the
  toolchain decision and admits one disposable exact-toolchain product canary,
  including its real full verification and test-discovery gate.
- Reason: Follow initialization/architect.md without inventing product/toolchain
  authority during template setup.
- Supersedes: none

## D007 — Reject agent-jev

- Status: accepted
- Date: 2026-10-25
- Decision: Exclude malevrigns/agent-jev from the implementation plan and stop
  its evaluation. Preserve the completed source findings as optional reference;
  do not install, integrate or schedule further experiments unless the owner
  explicitly reopens this candidate.
- Reason: Owner instruction: "ok wee reject it then".
- Evidence: ../02_analysis/agent-jev-disposition-2026-10-25.md
- Supersedes: consideration of agent-jev as a possible backend; no accepted
  implementation or provider decision existed.

## D008 — Research broadly and adopt selectively

- Status: accepted
- Date: 2026-10-25
- Decision: Keep the implementation path limited to actionable mechanisms that
  solve a concrete RelayForge problem, improve a relevant outcome over a simpler
  baseline, and earn their complexity. Research availability alone admits no
  feature, abstraction or dependency. Keep detailed evidence optional; curate
  the strongest findings into the bounded planning context.
- Reason: Owner rejects forcing unsuitable components into the project and asks
  to separate grain from chaff, retaining only the most actionable ideas.
- Supersedes: article-by-article accumulation as the shape of the foundation brief.


## D009 — Authorize the first usable release campaign

- Status: accepted
- Date: 2026-10-25
- Decision: The owner authorizes continuous planning, delegated implementation,
  verification and review through release-contract.md. The architect admits
  successive bounded roadmap slices and continues after milestone acceptance,
  making routine technical choices and local commits without repeated approval.
  Preserve accepted evidence and the template ledger; external publication,
  global harness changes and incremental paid evaluation need separate authority.
- Reason: Owner requests completion of the whole usable product and explicitly
  delegates technical decisions rather than approving each internal milestone.
- Supersedes: D005 setup-only execution boundary and the foundation-only stop in
  the original bootstrap goal. D006 canary-before-product gate remains required.
  Earlier documents requesting owner selection of routine technical choices
  defer to this delegation and D010; historical reviewed artifacts remain intact.

## D010 — Select Rust stable 1.99.1 and SQLite

- Status: accepted under owner-delegated technical authority
- Date: 2026-10-25
- Decision: Use Rust 1.99.1, target x86_64-pc-windows-msvc initially, for the
  embeddable core and CLI. Pin the exact toolchain, rustfmt/clippy and dependency
  lockfile during canary admission. Use SQLite for transactional local events,
  deduplication and derived statistics. Select and lock a maintained Rust binding
  with bundled SQLite after inspecting compatibility; record its SQLite version.
  Prefer transactional append-only event history with rebuildable projections.
- Evidence: Official stable manifest retrieved 2026-10-25 reports
  date 2026-10-15, Rust 1.99.1 (58cf8f9c3 2026-10-13):
  https://static.rust-lang.org/dist/channel-rust-stable.toml
  SQLite documents atomic transactional crash behavior:
  https://www.sqlite.org/transactional.html
- Reason: Native embedding and distribution fit the product; SQLite avoids
  inventing durable multi-record storage/recovery. The canary must still prove
  our implementation, chosen settings and Windows build; upstream guarantees
  do not prove our application works. The observed shell default was 1.98.1,
  so explicitly provision/select 1.99.1 without changing the global default.
- Owner preference: Rust is strongly preferred for robustness and maintainability.
  Python is valued; dependency/environment upkeep is the frustration. Retain
  existing Python workflow scripts and avoid a Python product runtime.
- Supersedes: D003 undecided-language status and D006 owner-selection prerequisite.
  The architect may revise implementation details on evidence within D009; changing
  product language away from Rust requires owner agreement.

## D011 — License RelayForge under PolyForm Noncommercial 1.0.0

- Status: accepted
- Date: 2026-10-25
- Decision: License original RelayForge work under PolyForm Noncommercial
  License 1.0.0, using the same license text and owner notice as SymForge:
  `Required Notice: Copyright (c) 2026 special-place-administrator`.
  Keep project license metadata and README consistent with the root LICENSE.
  Preserve imported template materials and third-party dependencies under their
  original terms and attribution separately; this decision does not relicense
  them or supply missing upstream permissions. THIRD_PARTY_NOTICES.md records
  provenance and the separately retained notices available from upstream.
- Reason: Explicit owner licensing instruction.
- Supersedes: The unresolved project-license part of roadmap item N4.
  Future external-code reuse still requires provenance and license review.

## D012 — Gate distribution on template permission or proven exclusion

- Status: accepted
- Date: 2026-10-25
- Decision: Track the missing template license grant in
  ../questions/open/Q001-template-distribution-permission.md as a distribution
  blocker. Before packaging, establish applicable permission or review packaging
  inputs that exclude imported and derived template material from distributed
  artifacts. Inspect resulting artifact contents before distribution. Keep
  original licenses/notices for included third-party material. Continue
  independent implementation and verification while this blocker is open.
- Reason: Explicit owner instruction following D011.
- Scope: G1 packaging/delivery and G6 release readiness; no permission to publish
  or contact external parties is implied. The architect owns exclusion evidence;
  the human owner obtains upstream permission if that route is chosen.
- Supersedes: Any assumption that repository licensing alone clears distribution.

## D013 — Push reviewed milestones with the existing owner identity

- Status: accepted
- Date: 2026-10-25
- Decision: Push the current committed branch to origin without force and push
  after each reviewed, committed milestone for the remainder of this campaign.
  Use the existing Git identity attributed to special-place-administrator;
  invent no email and add no AI co-author trailers. Report authentication
  failures or remote divergence; never force-push. Keep source, tests, lockfiles,
  documentation and required workflow evidence tracked while excluding local
  stores, credentials, caches, build output and raw runtime data. Check already
  tracked files separately; ignore rules do not untrack files.
- Reason: Explicit owner instruction. The owner-requested .gitignore change was
  reviewed and committed separately as 89c0937; current branch push succeeded.
- Supersedes: D009 prohibition on repository pushes only. D012 packaging and
  distribution gate remains; no release publication or host reconfiguration is
  implied by this Git synchronization authority.

## D014 — Use fresh Linux capacity and GitHub-hosted Windows CI

- Status: accepted
- Date: 2026-10-25
- Decision: Run Linux CI jobs on the owner's five custom self-hosted runners
  labeled `fresh`; run Windows builds and tests on GitHub-hosted
  `windows-latest`. The repository is public. Use the Linux runners for extra
  parallel PR-check capacity, with explicit trust gates for public fork PRs,
  least-privilege job permissions and no privileged execution of untrusted PR
  code. The label does not itself prove ephemeral isolation. Qualify Linux
  separately before claiming Linux support; Windows remains the initial target.
- Reason: Explicit owner runner instruction during the release campaign.
- Supersedes: None; CI is to be admitted with tests and independent review.

## D015 — Incorporate material JEV source-review lessons at planning boundaries

- Status: accepted
- Date: 2026-10-25
- Authority: Owner requested review and inclusion of
  ../02_analysis/jev-harness-source-review-2026-10-25.md and its CONTEXT index
  entry after the frozen canary work, without new dependencies or paid experiments.
- Evidence compared: foundation-contract.md sections Inventory and routing,
  Observations and outcomes, and Durable events and reconciliation;
  phase1-spec.md snapshot/decision records, modes and R5-R15, attempt accounting
  and persistence; release-contract.md G2/G3/G5/G6; accepted M002 canary sources
  and verification. The canary proves synthetic durability/replay only. It does
  not implement product accounting, routing, provider decisions or adapters.

| Material finding | Disposition | Consequence for later admission |
|---|---|---|
| Every physical attempt, retry, failure and parent repair belongs to logical task cost; requested and observed identity remain distinct; withheld work is not accepted work. | Already covered | Preserve these foundation and phase1 requirements in G2/G6 tests. Canary failure preservation is partial evidence, not completed accounting. |
| Partial usage must expose a labelled known subtotal while leaving the full total unknown. | Adopt | G3 aggregation and G6 evaluation must test missing components, full retry/rescue chains and distinct cases versus repeated runs. G2 must retain the component observations needed for that accounting. |
| Versioned snapshots, relevant configuration fingerprints and stale-evidence rejection. | Already covered | Preserve phase1 snapshot/decision contracts and R10; historical evidence remains interpretable without becoming current execution authority. |
| Replay must compare with an expectation supplied independently by trusted current host state, including relevant dirty workspace state. | Adopt | G3/G5 decision/cache reuse tests change policy/question, catalog/candidates, configuration, model/effort and relevant workspace state. Persist bounded privacy-safe references/digests, not source contents; a receipt cannot provide its own trusted expected binding. Hash consistency is not authentication. |
| Eligibility precedes ranking/provider choice; recommendations, host adoption, request dispatch and provider confirmation are separate; existing shadow is zero-network and live shadow is separately enabled and non-influential. | Already covered | Preserve foundation gates and phase1 R5-R11. No configured-only fallback, promotion, tool narrowing or provider calls are authorized by this research. |
| Strict response validation and cancellation must not produce an applied decision or active handoff. | Adopt | G5 tests reject invented candidates, wrong identities, nonfinite/range-invalid values and, when a distribution is returned, missing/extra keys, invalid sum or inconsistent winner. Never silently normalize. Keep abstention, unavailable, invalid and cancelled outcomes distinct; shadow leaves baseline tool exposure unchanged. |
| Evaluator labels and experiment arms must not leak into worker input. | Adopt | G6 constructs runtime input from an explicit whitelist and tests exclusion. Report rejected/unavailable decisions separately from dispatched work and verified accepted completions. |
| Tool prerequisite bundles and whole-bundle accounting. | Defer | Revisit only when tool narrowing is admitted. Do not introduce a dependency framework in this campaign merely because the research describes one. |
| Adopt the upstream harness, pinned model, uncalibrated thresholds or reported savings as RelayForge evidence; persist full source contents in decision receipts. | Reject | Keep the note as attributed optional source research. No upstream tests or savings were reproduced; no dependency, paid experiment, egress/retention expansion or measured capability follows. |

- Execution: incorporate applicable adopted checks into future bounded roadmap
  slices through the existing v4 manual /2 ledger process. Do not reopen accepted
  work or alter a frozen envelope. G1-G6 and Q001 remain unchanged.
- Supersedes: none; these are material refinements of later product acceptance,
  not a replacement workflow or a new execution queue.

## D016 — Admit the native metadata core and CLI after canary qualification

- Status: accepted
- Date: 2026-10-25
- Decision: Use the separate root Cargo workspace and bounded contract in
  ../02_analysis/native-core-plan.md for M003. Keep the accepted disposable
  canary outside the product workspace. Reuse its exact Rust/SQLite versions
  and candidate-verification machinery; do not reuse its synthetic event model
  as the product. Persist closed typed metadata with transactional ownership,
  explicit recovery, immutable attempt history and verification/acceptance gates.
- Dependencies: serde =1.0.228 (derive) and serde_json =1.0.154 are admitted
  for strict JSON serialization/validation: stdlib has no JSON implementation,
  and hand-written JSON would add unnecessary correctness risk. Exact versions
  were resolved with cargo info from the package registry. Preserve upstream
  MIT/Apache-2.0 terms; third-party notice completeness remains a packaging gate.
  rusqlite =0.40.0 and locked libsqlite3-sys 0.38.0 retain SQLite 3.53.1.
- Bounds: no provider, harness hook, routing, network capture, service, package
  or install behavior in M003. No new direct dependencies beyond those named.
  Source, tests, locks and portable evidence are tracked; runtime stores and
  build output remain in owned scratch. Full verification has a 900-second limit.
- Reason: M002 qualification passed independent slice and holistic reviews.
  The next unmet release dependency is usable G1/G2 code, with D015 accounting
  observations preserved for later G3/G6 aggregation.
- Supersedes: roadmap N1's pending canary admission only. G1-G6 and Q001 remain.

## D017 — Preflight evidence and review coherent changes without duplicate work

- Status: accepted
- Date: 2026-10-25
- Authority: Owner requested streamlined verification/review at the next safe
  boundary while preserving correctness, recovery, privacy and release gates.
- Evidence: M002-S02 used four implementation rounds (three independent review
  reports). R1 review found missing negative-guard execution evidence; r2 full
  verification failed on the scratch Git baseline; r3 review found stale results
  wording; r4 closed the findings. M003 additionally exposed missing scratch
  commit ancestry. These are distinct from M003's two lifecycle product defects,
  which were collected together and closed with failing-first regressions.
- Amendment: Coders run final focused checks. After `coded`, the architect runs
  one authoritative full verification for the current candidate, then preflights
  baseline validity, witness stability, test/discovery coverage, snapshot identity,
  evidence completeness and stale references before requesting review. Avoid
  duplicate worker full runs that predictably fail pre-coded ledger drift.
- Review: one coherent implementation review collects all actionable findings;
  follow-up reviews cover fixes, regression evidence and affected behavior.
  Administrative formatting corrections do not trigger repeated full audits.
  Mandatory holistic review remains, using approved unchanged evidence to check
  milestone coverage/integration/closure. All P0-P2 closure and release gates stay.
- Scope: AGENTS.md, the coding prompt template and docs/operating.md now express
  this narrow procedure. Existing issued envelopes, historical reports and
  ledger bytes remain unchanged; no gate is waived and no new progress store is
  introduced. A milestone or paperwork pass is not release completion.
- Execution economy: use the lowest task-appropriate worker, compact bounded
  context, deterministic tooling and one active worker by default. Escalate on
  concrete complexity or failed evidence; do not add speculative research lanes.
- Supersedes: the requirement to duplicate the full command in every coder turn
  before the architect's mandatory authoritative run. No other authority changes.
