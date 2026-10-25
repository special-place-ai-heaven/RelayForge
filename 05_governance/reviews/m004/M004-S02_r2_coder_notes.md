The architect recorded sanitized actual CI observations, preserving both failed runs and the successful reviewed source revision.

## Changed files
- No product files; original qualification evidence restored unchanged.

## Commands run
- GitHub run/job APIs and individual completed job logs — success; all six jobs in run 40213004003 passed.
- `python -B scripts/ci_contract_check.py` — PASS, 31 rejected unsafe fixtures.

## What could not be verified
- No live fork experiment or runner-isolation audit; explicitly limited to configuration and actual push evidence.
- Full release and Q001 distribution gate remain incomplete.

## Deviations from the prompt
- Architect authored the evidence-only document; no product changes.

## Evidence preservation

The r1 qualification document is immutable evidence pinned by the original blocker. Its bytes are restored unchanged. This architect-recorded handoff carries the subsequent qualification observations below; the product write envelope and historical hashes are unchanged. The first coded/verify attempts refused before recording because the historical evidence path had been edited. No product test failed in those refused attempts.

# CI qualification observations

Observed run: https://github.com/special-place-ai-heaven/RelayForge/actions/runs/40209605799

Source revision: d24d064166520089a9019e8ba60b1693b63455ce. Event: repository
push on campaign/delivery. This is actual GitHub execution, not a local mock.

| Job | GitHub job ID | Runner labels | Conclusion |
|---|---|---|---|
| windows | 121217824782 | windows-latest | success |
| linux (integration) | 121217824940 | self-hosted, linux, fresh | failure |
| linux (clippy) | 121217824982 | self-hosted, linux, fresh | failure |
| linux (doc) | 121217825022 | self-hosted, linux, fresh | failure |
| linux (fmt) | 121217825047 | self-hosted, linux, fresh | failure |
| linux (unit) | 121217825056 | self-hosted, linux, fresh | failure |

Five distinct custom runners accepted the Linux jobs. Availability is established
for this run; host isolation and general future capacity are not inferred.
The Linux formatting job's failed-step log shows Python 3.12.3 and
`rustup: command not found` (exit 127) in the provisioning step. Its formatting
verification step was skipped. The other Linux conclusions are retained above;
none is a product-test pass. The pinned Windows full workflow passed.

The source workflow binds custom-runner execution to the canonical repository
and trusted events; the observed event was a repository push. Fork exclusion is
contract-tested configuration evidence, not an executed public-fork experiment.

This first run did not qualify Linux. The subsequent observations below preserve
the failure and qualify the reviewed corrections. No raw logs, machine-local
paths, caches or runtime data are retained. Q001 still blocks packaging/distribution.

## Second run: inherited wrapper failure

Run https://github.com/special-place-ai-heaven/RelayForge/actions/runs/40211448552
tested fccb02eceb8d951b4c8ca99aec59b5d47216275b. Hosted Windows passed. All five
Linux jobs failed provisioning after the new pinned bootstrap installed Rust
1.99.1. The sampled formatting job (121223343078) failed because inherited
`RUSTC_WRAPPER=sccache` referenced an unavailable executable (exit 101).
No Linux product-stage pass is attributed to this run. D020 and accepted S04
clear both compiler-wrapper variables in job environments and reject nonempty
step-level overrides; they do not modify runner hosts.

## Qualified run

Run https://github.com/special-place-ai-heaven/RelayForge/actions/runs/40213004003
completed successfully, attempt 1, on 2026-10-25. Source revision:
`e3e30a1d2ad3bdec2b3a76dddaa4f510ce7a2b23`. Event: canonical repository push to
`campaign/delivery`. GitHub run/job APIs and the named job logs supply these
observations; this is actual execution, not a fixture or inferred result.

| Job | GitHub job ID | Runner labels | Conclusion |
|---|---|---|---|
| windows | 121228078630 | windows-latest | success |
| linux (integration) | 121228078888 | self-hosted, linux, fresh | success |
| linux (fmt) | 121228078892 | self-hosted, linux, fresh | success |
| linux (unit) | 121228078909 | self-hosted, linux, fresh | success |
| linux (clippy) | 121228078919 | self-hosted, linux, fresh | success |
| linux (doc) | 121228078933 | self-hosted, linux, fresh | success |

All jobs used rustc 1.99.1 (58cf8f9c3, 2026-10-13) and Cargo 1.99.1
(5f94df478, 2026-08-27). Windows used host/target
`x86_64-pc-windows-msvc`, Microsoft Windows Server 2025, hosted image
`windows-2025-vs2026` version `20261021.278.1`, and CPython 3.11.9. Linux used host/target
`x86_64-unknown-linux-gnu` and Python 3.12.3. No custom-runner OS distribution,
ephemerality or future availability is inferred from those observations.

Windows ran `python -B scripts/hermetic_verification.py`: clean-candidate
workflow checks, canary, product fmt/Clippy/build, 5 product unit tests, 13
integration/CLI tests and 1 doctest passed with zero ignored tests. The runtime
SQLite report was 3.53.1. Linux ran the five explicit `product_verify.py`
stages; logs report 5 unit, 13 integration and 1 doctest passed, zero failed or
ignored. Each stage uses the pinned toolchain and locked, offline dependencies
after job-local provisioning. These results qualify this source revision, not
future edits or the still-incomplete release contract.

At this revision `.github/workflows/ci.yml` permits custom Linux execution only
in the canonical repository for pushes or same-repository pull requests from
OWNER/MEMBER/COLLABORATOR authors. Public fork PRs remain hosted-Windows-only;
there is no `pull_request_target` trigger. Both jobs have contents-read token
permissions, nonpersistent checkout credentials and bounded timeouts; Linux
parallelism is capped at five. The observed event matches the permitted push
branch. `python -B scripts/ci_contract_check.py` passed with 31 unsafe fixtures
rejected. Fork exclusion is configuration/fixture evidence, not a claimed live
fork experiment. Host isolation is not established by the `fresh` label.

Q001 remains open: no packaging or distribution is authorized by this CI result.
