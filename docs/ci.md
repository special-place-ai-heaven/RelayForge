# CI checks

The workflow in `.github/workflows/ci.yml` runs the maintained clean-candidate
full verifier on GitHub-hosted `windows-latest`. It also runs five separate
product stages (`fmt`, `clippy`, `unit`, `integration`, `doc`) on runners
labeled `[self-hosted, linux, fresh]`, with at most five parallel matrix jobs.
Each stage uses the host's native Rust target and locked, offline dependencies.
The integration stage builds the actual CLI and gives its executable path to the
fresh-process tests. Discovery and execution must both contain nonzero tests;
required tests cannot be absent, ignored, skipped or failed.

The custom-runner job is limited to the canonical repository's pushes and
same-repository pull requests authored by an OWNER, MEMBER or COLLABORATOR.
Public fork pull requests still receive the hosted Windows check. The workflow
uses a read-only contents token, credential-free full-history checkout, pinned
action commits and job timeouts. This gate limits who can submit code to the
custom runners. The `fresh` label does not establish ephemeral isolation,
runner availability, absence of host credentials, or five-way capacity. Those
properties need operator evidence and observed CI runs.

The workflow provisions Rust 1.99.1 and downloaded Cargo sources under each
job's temporary homes, then verifies with `--locked --offline`. Windows
provisions Python 3.11 and PyYAML 6.0.3 in a job-owned virtual environment.
Its verifier step supplies the existing owner Git identity solely for scratch
candidate metadata; it creates no repository commit.
No database, cache, build output or raw runtime data is uploaded.

Local focused checks:

```text
python -B scripts/ci_contract_check.py
python -B scripts/product_verify.py --focused
python -B scripts/product_verify.py --stage integration
```

The architect runs `python -B scripts/hermetic_verification.py` once after the
slice is coded. It retains the clean-candidate Git ancestry, ledger check,
canary checks, product checks, source/index/HEAD stability and 900-second
verifier budget. Local Windows checks do not qualify Linux or GitHub-hosted
Windows. S02 must record real run URLs, revision, native targets/tool versions,
per-job conclusions and any runner access or queue blockers.
