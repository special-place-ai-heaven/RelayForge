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

Qualification remains incomplete. Required next action: architect admits and
reviews job-local Rust bootstrap for runners without rustup, pushes the reviewed
correction, then resumes S02 against the new actual run. Record exact target/tool
versions and successful job conclusions when observed. Preserve this failed run
as history. No raw logs, machine-local paths, caches or runtime data are retained.
Q001 still blocks packaging/distribution.
