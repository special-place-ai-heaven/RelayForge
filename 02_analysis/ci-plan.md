# CI admission

M004 adds and qualifies CI for the accepted core, without reopening its domain
contract. User policy D014 selects `[self-hosted, linux, fresh]` for Linux and
`windows-latest` for Windows. Five Linux workers provide parallel capacity;
maximum parallelism is five, not a requirement to duplicate the same full suite.

Use five useful product checks: formatting, Clippy, unit tests, integration/CLI
tests, and doctests. Maintain them as explicit stages of product_verify.py so
local and CI test-discovery rules cannot silently diverge. Test stages reject
missing, zero, failed or skipped required tests. Integration builds the actual
CLI and supplies its exact executable path. On Windows, run the maintained full
hermetic command, including canary, product and candidate/ledger checks.

Generalize only verifier/test platform assumptions needed to run native Rust on
Windows MSVC and Linux. Pin Rust 1.99.1 and locked dependencies; record actual
host/target and SQLite version, use the host's executable suffix, and retain
Windows crash/recovery/privacy assertions. Linux is unqualified until observed
CI evidence passes. The product implementation and schema remain unchanged.

## Workflow boundary

- Trigger pushes on main and campaign branches, and pull requests. Windows
  hosted checks run on public fork PRs with a read-only token and no secrets.
- Custom Linux jobs run only for repository pushes and same-repository PRs by
  OWNER, MEMBER or COLLABORATOR authors. Fork PRs never execute on custom runners.
  Do not use pull_request_target or execute a fork head through a privileged job.
  This is a contributor trust gate, not proof that a runner is ephemeral or that
  privileged host data is inaccessible. Document the remaining host boundary.
- Set contents: read, persist-credentials: false, bounded job timeouts and
  max-parallel: 5. Avoid shared credentialed caches and privilege escalation.
  Checkout full Git ancestry (fetch-depth: 0): the ledger resolves completed
  commit operations through ancestry, so a shallow/orphan snapshot is invalid.
- Pin actions to reviewed full commit SHAs. Registry observations on admission:
  actions/checkout v4 resolves to 11d5960a326750d5838078e36cf38b85af677262;
  actions/setup-python v5 resolves to a26af69be951a213d495a4c3e4e4022e16d87065.
  Use these or justify another verified immutable official release in evidence.
- Provision Python workflow requirements and Rust into job-owned environments
  using available tooling. Do not change global Rust defaults, install services
  or read host credentials. Fetch Cargo dependencies explicitly, then verify
  locked/offline. Use the existing verified owner identity only for temporary
  candidate Git metadata; no AI co-author trailers or real CI commits.
- Keep generated databases, dependency caches, build output and raw runtime data
  out of Git and uploaded artifacts. Portable check summaries are sufficient.

Add a small deterministic workflow-contract check covering runner labels,
permissions, immutable action pins, full ancestry, fork trust gates and timeouts.
Its negative fixtures must prove that unsafe/missing configuration is rejected;
do not merely assert strings in the implementation. Use existing PyYAML and
stdlib, not a new tool dependency. Run this check in the maintained full verifier
through product_verify.py, leaving the architect-owned outer argv unchanged.

## Delivery and qualification

S01 accepts portable check mechanisms and reviewed workflow configuration after
local Windows full verification. Push the reviewed, committed slice to exercise
CI; this uses the owner's existing non-force push authority. S02 records actual
run URLs, source revision, target/tool versions and per-job conclusions. It must
not substitute local mocks for Linux or hosted-Windows results. If runner access,
authentication or queue availability blocks execution, retain exact evidence and
record a typed blocker; continue independent release work. Do not manufacture
Linux success from the label or silently switch to hosted Linux.

Earlier API observations listed no repository runners and denied organization
runner enumeration with 403. Availability, isolation and five-way concurrency
are unverified. A workflow file alone does not establish any of them.
