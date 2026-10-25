# JEV harness: selective source review

Status: optional planning evidence; no new work admitted.
Reviewed: 2026-10-25. Audience: architect and owners of release gates G2/G3/G5/G6.

## Provenance and verdict

Repository: [TypeSafeAI/jev-harness](https://github.com/TypeSafeAI/jev-harness), inspected at
`1463609d433715eae303a0eebe70c371c88b60cc`.
The README identifies TypeSafeAI as an independent community organization, not the
official TypeSafe AI team. This is not evidence of an endorsed production SDK.
The repository license is MIT, copyright 2026 jev-harness contributors.

Method: temporary clone, bounded reads of routing, context preparation, dependency
bundles, receipt auditing, evaluation, host transport and related tests/reports.
No dependencies installed, tests executed, paid calls made or savings reproduced.
Test code is evidence of intended checks, not a passing verification receipt.

**Verdict: retain the following design lessons; do not adopt this harness as a
dependency or replace RelayForge's Rust core, SQLite event log or v4 workflow.**
This is a separate project from the rejected `malevrigns/agent-jev` candidate.

## 1. Count physical attempts and distinguish outcomes

The host example records each provider attempt, applies one deadline to the whole
logical request, and permits only explicitly bounded probability-sum recovery.
It never repairs a malformed probability distribution by silently normalizing it.
Measurement distinguishes reported token subtotals from complete totals: missing
usage keeps the total unknown. These are useful patterns; the example's ledger is
in memory and is **not durable-before-dispatch intent recording**.

Evaluation separates validation rejection, unavailable decisions, provider
failures, legitimate clarification and semantic outcomes. Repeated runs do not
increase the count of distinct cases. Its proposer-input constructor copies an
explicit runtime whitelist, excluding expected labels and experiment arms.

RelayForge application, G2/G6: preserve the existing durable log and record every
physical attempt, including retries, failures and rescue work. Separate routing
opportunities, dispatched work and verified accepted completions. Track requested
and observed model/effort independently; unknown identity or usage stays unknown.
Do not score withheld work as cheaper successful work.

Acceptance checks to reuse when applicable:

- Multiple attempts remain one logical task; all attempt costs count.
- Missing usage produces an unknown total plus a clearly labelled known subtotal.
- Duplicate observations do not inflate evidence; evaluator labels cannot enter worker input.

Sources: [host transport](https://github.com/TypeSafeAI/jev-harness/blob/1463609d433715eae303a0eebe70c371c88b60cc/examples/host/jev-choice.ts),
[measurement](https://github.com/TypeSafeAI/jev-harness/blob/1463609d433715eae303a0eebe70c371c88b60cc/examples/routing/measurement.ts),
[evaluation](https://github.com/TypeSafeAI/jev-harness/blob/1463609d433715eae303a0eebe70c371c88b60cc/src/benchmark/evaluation.ts).

## 2. Bind a decision to the exact state it judged

Receipt auditing snapshots evidence, canonicalizes bounded plain data, hashes it,
and recomputes the verdict. Replay compares against an expected binding supplied
by trusted current host state. A receipt cannot supply its own trusted expectation.
Bindings include policy/question versions, model, source and exact request/workspace.

RelayForge application, G2/G3/G5: use versioned fingerprints for relevant policy,
question set, catalog, available candidates, configuration and model/effort. A git
commit alone cannot identify dirty workspace state. Store privacy-safe references
and digests in the existing event log; do not copy the upstream practice of binding
full file contents into persisted receipts without explicit retention/egress rules.
A SHA-256 digest detects inconsistency, not an attacker who can replace and rehash
the record. Historical versioned evidence must remain interpretable even when it
is no longer eligible for current execution.

Acceptance checks: changing relevant dirty state, policy or model invalidates
current reuse; replay obtains its expectation independently; stale evidence is
retained as historical evidence rather than silently upgraded.

Source: [receipt audit and replay](https://github.com/TypeSafeAI/jev-harness/blob/1463609d433715eae303a0eebe70c371c88b60cc/src/audit/receipt.ts).

## 3. Make bounded routing and shadow mode literal

Routing validates model identity, candidate membership, exact probability keys,
finite ranges, distribution sum and agreement between winner and probabilities.
Explicit clarification and unavailable results remain separate from selection.
Receipts say execution was not applied. Shadow preparation preserves the full
available tool set, including on router failure; lean mode exposes selected tools.
Observed cancellation empties active context while retaining comparison evidence.

RelayForge application, G3/G5: validate against the actual eligible inventory and
keep recommendation, host adoption and execution as distinct events. Shadow mode
must not quietly narrow tools. Cancellation and invalid outputs need typed outcomes.
The upstream confidence threshold is explicitly uncalibrated; neither that number
nor its pinned model belongs in our registry as measured capability.

Acceptance checks: invented candidates and malformed distributions are rejected;
unavailable never becomes an applied success; shadow preserves baseline exposure;
cancelled preparation cannot hand off an active tool set.

Sources: [router](https://github.com/TypeSafeAI/jev-harness/blob/1463609d433715eae303a0eebe70c371c88b60cc/src/routing/route.ts),
[preparation](https://github.com/TypeSafeAI/jev-harness/blob/1463609d433715eae303a0eebe70c371c88b60cc/src/routing/prepare.ts).

## Deferred: preserve prerequisites when narrowing tools

The bundle assembler expands selected roots through host-declared dependencies,
deduplicates the closure and rejects cycles. Missing/unavailable prerequisites
withhold the entire handoff. Root selection stays separate from the exposed bundle.
This addresses a concrete failure: selecting a patch tool while hiding its read tool.

Keep this for a later admitted tool-narrowing slice. Dependencies must also satisfy
host permissions. The upstream ceiling is per tool, **not a whole-bundle budget**;
estimated descriptor costs are not observed accepted-task economics. Any RelayForge
adaptation needs whole-bundle accounting and a missing-prerequisite check. No generic
dependency framework is justified before that feature is admitted.

Source: [bundle assembler](https://github.com/TypeSafeAI/jev-harness/blob/1463609d433715eae303a0eebe70c371c88b60cc/src/routing/bundle.ts).

## What the experiments establish

The [September 23 report](https://github.com/TypeSafeAI/jev-harness/blob/1463609d433715eae303a0eebe70c371c88b60cc/docs/routing-evaluation/2026-09-23.md)
reports 61% less input, but 27 of 57 routed cases never started the proposer;
in the selected subset, acceptable outputs fell from 27/30 to 15/30. Tool prerequisites,
clarifications and unequal cache behavior prevent a clean savings conclusion.

The later [September 26 report](https://github.com/TypeSafeAI/jev-harness/blob/1463609d433715eae303a0eebe70c371c88b60cc/docs/routing-evaluation/2026-09-26-host-recovery.md)
reports improvements: 171/171 expected routes and 39/39 assessable routed outputs
meeting its rubric, with another 18 clarification cases producing no authored output.
All first responses were valid, so it does not demonstrate recovery benefit.
Assessment used fresh agents, not human gold labels; proposed patches/tests were not
executed. Requested model identity was pinned, not provider-attested. These are
upstream synthetic results, not RelayForge benchmarks or proof of production savings.

## Integration boundary

At the next normal planning boundary, compare these findings with existing contracts
and implementation. Record only material dispositions: already covered, adopt in an
appropriate future slice, defer or reject. Use the existing decisions/roadmap/ledger
process; this note has no admission authority. Do not reopen accepted canary work or
alter a frozen active envelope just to incorporate research. Continue the delivery
campaign. No new dependency, paid experiment or additional framework follows from
this note.
