/goal Establish RelayForge's reviewed native foundation contract until M001-S01 is accepted with verification and independent review evidence and the next toolchain decision is explicit, without expanding the admitted product scope.

# RelayForge bootstrap goal

## Context and authority

This is a reusable instruction file for a fresh Codex or Claude Code session.
Reading it does not activate a harness goal automatically. The invoking user
must ask the harness to pursue this goal using its available goal mechanism,
or follow the same procedure in an ordinary session.

Enter as architect under ../AGENTS.md. The repository is already initialized.
roadmap.yaml owns admitted work, 00_brief/decisions.md owns decisions, and
05_governance/ledger.jsonl owns execution history. This file is a procedure,
not a second progress log; do not tick it off, rewrite history, or delete it.
Resolve paths below from the repository root.

## Success criteria

1. M001-S01 has its actual foundation-contract artifact within the frozen
   acceptance envelope, a successful verification receipt, independent review
   with no unresolved P0-P2 findings, and ledger acceptance for that evidence.
2. The contract covers inventory, routing, observation, outcomes and restart
   reconciliation; distinguishes capabilities from measured statistics; and
   accounts for tool failures, retries, recovery cost and uncertainty.
3. The contract presents at most two non-Python toolchain options and defines
   the exact next canary evidence, full verification and test-discovery gate.
   It accurately records unresolved owner decisions; it does not invent approval.
4. The final handoff reports current ledger state, evidence paths, remaining
   decisions and the next authorized action. Accepted foundation work is never
   described as a working RelayForge product or proven cost saving.
   An unresolved toolchain decision explicitly naming its owner, options and
   required next action satisfies the decision handoff; approval is not assumed.

## Required reading

Follow the root AGENTS.md session-entry chain:
- initialization/architect.md
- 00_brief/CONTEXT.md and the brief files it names
- roadmap.yaml and docs/operating.md
- 02_analysis/CONTEXT.md, foundation-brief.md and actionable-findings.md
- docs/workflow-graph.md for the existing control flow and bounded gap analysis

Use detailed research only to resolve a named question. Articles and source
examples do not admit new features. Coders and reviewers read only their issued
prompt and its required evidence, not this entire architect bootstrap.

## Procedure

1. Run the roadmap check, ledger check, ledger status and Git status commands
   named in AGENTS.md. Inspect any existing in-progress round before dispatch.
   Resume its reported step; never issue duplicate work after an interruption.
2. Compare the current roadmap and decisions with this goal. If M001-S01 is
   already accepted, verify its recorded evidence and produce the handoff.
   If its identity or scope has changed, reconcile with current authority before
   acting; do not reset the project to match this historical slice identifier.
3. Read docs/workflow-graph.md. Use the existing state machine and manual /2
   commands. Treat remaining graph questions as bounded planning evidence.
   Adding an engine, graph store or alternate execution ledger is out of scope.
4. Preview the admitted prompt with:
   python -B scripts/prompt.py M001-S01 --preview
   Check its read list, acceptance and writable envelope before actual issuance.
5. Issue with scripts/prompt.py, then delegate the bounded contract to an
   available documentation/specification worker. Discover the current harness's
   real roles and models; do not assume Claude agent names exist in Codex.
   Pick the lowest sufficient available lane and record actual execution identity.
6. Inspect the actual diff and record coded work with scripts/ledger.py coded.
   Run scripts/verify.py for the slice. On failure, use its evidence to form a
   new corrective hypothesis; do not repeat the same attempt unchanged.
7. After successful verification, issue the review prompt with --review.
   Assign a separate read-only reviewer with the original acceptance criteria,
   candidate diff and receipt. Record its report through scripts/ledger.py record.
   A model's claim of completion cannot replace verification or review.
8. Follow needs-work and blocker transitions exactly as docs/operating.md defines.
   Accept only a passing reviewed candidate with scripts/ledger.py accept.
   Local commits require current authorization; never push or publish.
9. Report the accepted contract and remaining toolchain decision. Admit a later
   canary only when its decision and execution authority exist. This goal ends
   at the foundation boundary and does not silently become a full-product build.

## Stop, recovery and limits

Use ledger status and docs/operating.md recovery after any interruption. Preserve
unknown work, receipts and pending invocations; unknown completion is not success
or automatic model failure. A malformed ledger or unexplained artifact drift
requires attribution before proceeding.

Before any retry, identify new evidence or a changed hypothesis. If none exists,
stop and report the precise blocker and needed next action. Honor the frozen
verification timeout and any separately admitted, enforced run allowances.
The current manual loop has no controller-enforced attempt or cost budget.
A session restart must not erase recorded consumption. Do not describe a prose
limit as a controller-enforced budget.

The user has not authorized paid benchmark campaigns, dependencies, services,
harness hooks, global configuration changes or external publication under this
goal. Product language and storage remain undecided. Agent-JEV remains rejected.
Routine observation and accounting should require no model calls. JEV advice
cannot grant permission or accept a candidate. Proposed numeric failure thresholds
must remain proposals until their policy and measurement are admitted.
