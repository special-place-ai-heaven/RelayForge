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
job's temporary homes, then verifies with `--locked --offline`. Fresh Linux
runners in [the first actual run](https://github.com/special-place-ai-heaven/RelayForge/actions/runs/40209605799)
had Python 3.12.3 but no `rustup`; all five Linux jobs stopped at provisioning,
before product checks. The Linux job now selects an official Rustup 1.28.2
GNU/Linux installer for x86_64 or aarch64, checks its pinned SHA-256 before
execution, and rejects other architectures or a non-glibc host. It verifies
`RUSTUP_HOME` and `CARGO_HOME` resolve to children of the runner's temporary
directory, disables shell-profile changes and default-toolchain installation,
and adds only its job-local Cargo binary directory to current and later steps.
The [official rustup guide](https://rust-lang.github.io/rustup/installation/other.html#manual-installation)
documents the versioned archives and checksum sidecars. The pinned
[x86_64 checksum](https://static.rust-lang.org/rustup/archive/1.28.2/x86_64-unknown-linux-gnu/rustup-init.sha256)
and [aarch64 checksum](https://static.rust-lang.org/rustup/archive/1.28.2/aarch64-unknown-linux-gnu/rustup-init.sha256)
were checked against their corresponding official installer binaries.

Windows provisions Python 3.11 and PyYAML 6.0.3 in a job-owned virtual environment.
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
verifier budget. The first actual run passed hosted Windows and proved five
distinct fresh Linux runners accepted jobs; it did not qualify Linux product
checks. The corrected source revision `e3e30a1` subsequently passed Windows
and all five Linux stages in [run 40213004003](https://github.com/special-place-ai-heaven/RelayForge/actions/runs/40213004003).
Exact native targets, tool versions and per-job conclusions are retained in
[the accepted S02 qualification evidence](../05_governance/reviews/m004/M004-S02_r2_coder_notes.md).
The `fresh` label does
not prove host isolation or future availability.

Both jobs set empty `RUSTC_WRAPPER` and `RUSTC_WORKSPACE_WRAPPER` values at
the job level. This prevents inherited runner wrapper settings from changing
Cargo's compiler invocation while leaving host configuration untouched. Run
40211448552 passed Windows; Linux installed Rust 1.99.1, then Cargo failed when
the inherited `sccache` wrapper was unavailable. That run did not execute Linux
product stages. The S04 worker probe and configuration evidence are recorded in
`02_analysis/ci-results.md`; the successful run above qualifies the corrected
revision on Linux as well as Windows. Earlier failed-run evidence is historical.
Q001 remains open and blocks packaging/distribution; CI success does not close it.
