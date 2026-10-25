/goal Deliver RelayForge's first usable local release until every required gate in 00_brief/release-contract.md has evidence and independent release acceptance, without weakening scope, evidence, privacy or authority constraints.

# Continuous delivery campaign

Use this file in the existing or a fresh Codex/Claude session. The user invokes
the harness's available goal mechanism or asks the agent to pursue this contract.
A file does not itself start or keep a process alive. Do not self-delete this file.

## Read and resume

Enter as architect under AGENTS.md and follow its session-entry chain. Read
00_brief/release-contract.md, decisions D009/D010, the accepted foundation contract,
roadmap.yaml, docs/operating.md and docs/workflow-graph.md. Load detailed research
only for a named unresolved question. Current decisions supersede earlier
foundation-only restrictions; preserve historical acceptance artifacts unchanged.

Check roadmap, ledger and Git state. Resume actual in-progress work, reconcile
pending invocations, and preserve unknown changes. If the foundation is already
accepted, do not repeat it. Inspect its uncommitted artifacts against recorded
evidence and commit attributable accepted work before new prompt issuance.
Do not sweep unrelated changes into the baseline or alter hashes to silence drift.

## Completion contract

The campaign succeeds only when release gates G1-G6 in the release contract have
the required evidence, the independent release review has no unresolved P0-P2
findings, and the ledger records acceptance for all admitted release work.
Supply build/install instructions, the actual coverage/limitations, evaluation
results, evidence paths and final revision. Do not require a fabricated positive
saving or claim mock, shadow or advisory behavior is live effective routing.

A finished slice, finished milestone, clean working tree, passing scaffold checks,
empty queue, or plan document is not campaign completion. When no next slice is
admitted and a required release gate is missing, plan and admit the next eligible
slice yourself under D009. Continue immediately after each accepted slice.

## Execution loop

1. Reconcile state and choose the next unmet release dependency. Make routine
   technical decisions under D009/D010; record reasoning and exact versions.
   Rust 1.99.1 and SQLite are selected. Start with the disposable canary.
2. Maintain roadmap.yaml with bounded slices, acceptance, non-goals, required
   reads, allowed writes, full verifier, focused checks and timeouts. Validate it.
   Product work must have real product tests; scaffold checks cannot substitute.
   Architect owns verifier/plan setup outside coder write envelopes.
3. Preview scripts/prompt.py <slice> --preview. After a clean attributable
   baseline, issue the prompt and delegate to the lowest sufficient available
   worker. Use actual harness roles; record actual model and effort when exposed.
   Partition independent work, avoid overlapping writes, and bound concurrency.
4. Inspect the actual diff; record coded work with scripts/ledger.py coded.
   Run scripts/verify.py <slice>, retain the receipt and diagnose any failure.
5. Issue the review prompt only after passing verification. Use a separate
   read-only reviewer against original requirements, candidate and receipt.
   Record with scripts/ledger.py record. Fix findings through corrective rounds;
   accept only eligible passing work using scripts/ledger.py accept.
6. Commit accepted work and its evidence. Complete required holistic review and
   milestone closure through documented commands. Then continue to the next
   release dependency, admitting new work when necessary.
7. At integration, exercise real fresh-process Codex and Claude paths in isolated
   profiles; document unsupported control surfaces. Before final acceptance,
   verify the packaged CLI/core path from a clean checkout and perform independent
   release review against every gate, including privacy and recovery behavior.

## Recovery, blockers and stopping

The ledger is the sole execution history; roadmap is the plan. Do not add a
hand-maintained parallel checklist or rewrite old review records. Derive the
next action from current evidence and docs/operating.md after interruption.
A new session resumes this same campaign; it does not reset attempt consumption.

Apply admitted timeout/retry rules. Change the hypothesis before another retry.
Escalate on evidence, preserve failed attempts and unknown usage. Stop only for
completed release criteria, an explicit user pause, a harness-enforced limit, or
a precise external blocker with no independent work left. Report incompletion
honestly; never mark the goal complete merely because the context or budget ends.

Routine language/storage/library/API choices are delegated. Ask the owner only
for requirements that cannot be inferred or authority outside D009/D010. Explicit
spend approval is still required for incremental paid evaluation/provider calls.
A missing live JEV allowance follows the disclosed G5 release rule; missing real
Codex/Claude evidence does not pass G4. No push or publication under this campaign.
