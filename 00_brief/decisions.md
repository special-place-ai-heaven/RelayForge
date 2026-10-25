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
