# Product

<!-- impeccable:product-schema 1 -->

## Platform

web

<!-- No web or native UI exists or is planned (confirmed in init). `web` is the schema default; real surfaces are the CLI, its text/JSON output, reports, and docs. -->

## Users

The owner, and the coding agents (Codex, Claude Code) working in those harnesses. Owners need to choose models and thinking effort from observed results rather than names, prices, or public benchmarks. Agents consume routing recommendations and emit observations. No wider audience is confirmed for the first release.

## Product Purpose

RelayForge is an adaptive routing and performance layer for coding harnesses: an embeddable Rust core and CLI covering capability inventory, bounded JEV routing, session observation, adaptive performance records, and recovery. Success is reliable accepted work at the lowest measured total cost, with decisions grounded in observed model + effort performance, including retries, recovery, and parent repair.

## Positioning

It compares exact model/version + effort + task difficulty + harness/tool configuration on completed-work cost, not list price or benchmark rank. It keeps failure history (rescued tasks, wrong tool calls, abandoned work, unknown outcomes) instead of hiding it. Ordinary collection makes zero model calls, and routing never grants permissions.

## Operating Context

- Initial environment: Windows PowerShell; other environments need their own qualification.
- Harnesses in scope: Codex and Claude Code. OMP and Cursor are deferred.
- Manual-first workflow: architect, coder, and reviewer loop governed by `roadmap.yaml`, `00_brief/decisions.md`, and `05_governance/ledger.jsonl`.
- Evidence is local: SQLite event storage with crash/restart reconciliation.

## Capabilities and Constraints

- Product language Rust (pinned 1.99.1) with SQLite per D010; Python is excluded from the product and remains workflow tooling only.
- Distinguish configured capability, runtime-confirmed availability, and measured results. Unknown outcome, model, or cost stays unknown.
- Recommendations are advisory until adoption is recorded; they never masquerade as applied dispatch.
- JEV is bounded, shadow-mode by default, and never grants permissions. Live JEV effectiveness is not claimed.
- No savings percentage or model-effort crossover is promised; an honest negative or inconclusive result is valid.
- No secrets, resolved host paths, or raw private telemetry in tracked files.
- Distribution is blocked until Q001 (template distribution permission) is resolved.
- Release gates G1–G6 are defined in `00_brief/release-contract.md`.

## Brand Commitments

Name: RelayForge. Voice for human-read output (CLI text, reports, docs) is evidence-first and plain: say what was observed, how much evidence there is, and what remains unknown; never claim a saving without measured evidence. No other binding voice, visual, or accessibility commitments were set.

## Evidence on Hand

- Product intent, constraints, decisions, and release contract in `00_brief/`.
- Dated research and configuration findings in `02_analysis/`.
- Absent and not to be fabricated: any UI, screenshots, logos, customer testimonials, benchmarks, pricing, or measured routing results.

## Product Principles

1. Measured over named: decide from observed model + effort outcomes, not tier descriptions or public benchmarks.
2. Failure stays visible: preserve retries, wrong tool calls, rescue, and unknown outcomes; never label zero completed work a saving.
3. Cheap by default: ordinary observation makes no model calls; judgment calls are bounded and separate.
4. Bounded authority: a route never widens tools, permissions, or credentials.
5. Honest uncertainty: report sample size, provenance, and insufficient-evidence results explicitly.
