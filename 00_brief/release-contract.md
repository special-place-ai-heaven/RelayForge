# First usable release contract

This is the campaign completion contract authorized by D009. The architect owns
decomposition and admission in roadmap.yaml; the ledger owns execution progress.
A completed milestone is an internal checkpoint, not the campaign's finish.

## Required outcomes

| Gate | Required observable evidence |
|---|---|
| G1 Reproducible native delivery | Rust 1.99.1 pinned, dependency lockfile, embeddable core and CLI; documented clean-checkout build, test discovery, full verification and local install/uninstall exercised on Windows. |
| G2 Durable local evidence | SQLite event storage, versioned records, transactional pre-dispatch intent, duplicate-safe ingestion/reconciliation, crash/restart tests, preserved failures and unknown outcomes, concurrent-session ownership checks, secret-redaction checks. |
| G3 Useful inventory and ranking | Exact model/effort/profile/harness identities and sourced capability evidence; deterministic eligibility; local outcome statistics, missingness and uncertainty; explicit insufficient-evidence result; fixed baseline and explainable comparisons including retries and parent repair. |
| G4 Codex and Claude integration | Each adapter has a versioned coverage table and a real fresh-process local smoke receipt demonstrating at least one supported observation path into the core. Routing recommendations are consumed through a supported native or explicit CLI path with adoption recorded. Unavailable native switching/hooks are explicit; advisory recommendations cannot masquerade as applied dispatch. Fixtures alone cannot pass this gate. |
| G5 Bounded JEV integration | Provider boundary accepts only eligible candidates and versioned questions, validates responses, handles timeout/error/abstention, caches or bypasses unnecessary calls, and defaults to shadow mode. Contract tests pass; live provider validation is reported separately and remains blocked if credentials or an explicit incremental spend budget are missing. |
| G6 Evaluation and release readiness | Reproducible representative scenarios cover success, wrong tools, retries/escalation, sparse evidence, interruption, restart and unavailable harness capability. Report observed cost/usage completeness, accepted-work accounting and capture/routing overhead against a fixed baseline. Document onboarding, limitations, privacy, recovery and troubleshooting. Independent release review has no unresolved P0-P2 findings. |

Every gate needs evidence paths, revision/tool versions, commands and results in
the final release review. Reuse the template's review artifacts and ledger rather
than creating a second progress database. A missing required gate is incomplete;
do not quietly replace live integration evidence with mocks or waive requirements.

G5 permits a first local release with provider integration contract-tested but
live JEV unvalidated, if prominently disclosed and shadow-only. It does not permit
claims of live JEV effectiveness. No savings percentage or model-effort crossover
is required to be positive: an honest negative or inconclusive result is valid.
Do not tune evaluations to manufacture a win. No automatic promotion to effective
routing until a separately recorded policy and comparative evidence justify it.

## Delivery order and admission

1. Preserve accepted foundation evidence and commit attributable baseline work.
2. Select/pin the toolchain and storage per D010, admit the disposable canary and
   prove build, discovered/executed tests, transactional durability and recovery.
3. Admit durable core/CLI and inventory/observation slices.
4. Admit deterministic statistics, eligibility and baseline ranking.
5. Admit native adapter coverage and real local integration paths.
6. Admit bounded JEV shadow and evaluation, then packaging/docs/release review.

This is dependency order, not a fixed number of slices. The architect expands,
splits and refines roadmap slices at stable boundaries with narrow read/write
envelopes, checks and acceptance. It may do independent work while another area
is externally blocked. A milestone with no next slice triggers planning the next
eligible slice; it does not finish the campaign.

## Delegated decisions and limits

The architect may choose repository-local libraries, schemas, internal APIs,
CLI commands, test tools, and adapter design, recording consequential decisions.
Use standard library/existing dependencies first; pin justified additions.
It may provision the explicitly selected Rust toolchain through existing rustup
without changing the global default, fetch locked build dependencies, create
isolated test profiles and run local build/integration checks. Local commits of
reviewed work and architect-owned plan updates are authorized.

It must preserve the non-Python product, cheap deterministic capture, bounded
routing authority, existing template and rejection of agent-jev. No paid benchmark
campaign, unbounded JEV inference, global harness reconfiguration, live service
installation, publication, push, or credential disclosure is authorized here.
Use isolated harness profiles/workspaces for validation. When a real dependency
needs outside authority, record the exact blocker and continue independent work.

Before repeated execution, define bounded attempts/timeouts in the admitted plan
and enforce them where supported; track usage across sessions. Prose limits are
not enforcement. Retry only with a changed hypothesis; after two failed attempts
on the same hypothesis, stop that approach and investigate/escalate. This is a
workflow stop rule, not an approved product failure-rate threshold.
