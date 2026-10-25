# Environment

RelayForge currently contains design documents and the retained template's
manual development workflow. There is no RelayForge product runtime to install.

## Workflow requirements

The upstream manual template reports Windows PowerShell qualification; POSIX
qualification is pending. Its workflow scripts require Python 3.11+, PyYAML, and
Git. They are local and offline after installation and use no service or
credential. An upstream qualification is not evidence that RelayForge's future
product has passed any runtime checks.

## Product runtime decision

The owner selected Rust 1.99.1 and SQLite for the non-Python product runtime
(D010). Existing external Python JEV code is a compatibility reference, not
RelayForge's product runtime. The disposable canary qualification is pending;
its maintained verification command is part of the full check and will pass
once `scripts/canary_verify.py` is implemented and the canary qualifies.

## Setup

Create and activate a local virtual environment for the workflow scripts, then
install the template dependency metadata:

```powershell
python -m venv .venv
.\.venv\Scripts\Activate.ps1
python -m pip install -e .
```

POSIX activation is `source .venv/bin/activate`. A controlled-offline install
may use a reviewed wheelhouse:

```powershell
python -m pip install --no-index --find-links <wheelhouse> setuptools wheel PyYAML
python -m pip install --no-index --find-links <wheelhouse> --no-build-isolation -e .
```

Do not commit venvs, wheels, caches, machine constraints, or executable paths.
An optional ignored `project.local.toml` selects the workflow's Python executable;
otherwise scripts use their invoking interpreter. A `/2` roadmap may pin that
portable runtime version. This setting does not select RelayForge's product
language. See [runtime and coverage](docs/project_checks.md).

## Commands

```powershell
python scripts/roadmap.py check
python scripts/roadmap.py render
python scripts/ledger.py check
python scripts/ledger.py status
```

The commands above inspect or render workflow state. Exported projects do not
contain the template's framework qualification tests; their authoritative full command is
`verification.full` in `roadmap.yaml` and runs through
`python scripts/verify.py <SLICE>`. The retained entry point is
`scripts/hermetic_verification.py`; its project-owned `COMMANDS` must describe
the currently admitted checks and fail closed when unconfigured. Bare argv lists
run from repository root; mappings such as
`{"argv": ["npm", "test"], "cwd": "07_app/web"}` run from a safe project
subdirectory. `TEMP`, `TMP`, and `VERIFICATION_SCRATCH` identify external
scratch space so tools can keep caches and build residue outside the project.

Scaffold checks cannot verify an unimplemented router or collector. Future product
slices must add checks for their actual contracts and a live integration path
before reporting product readiness or measured savings.

frutlups is optional and unnecessary for manual work. Version 0.3.2 cannot run
the new `/2` contract. Keep it disabled until an exact compatible pair passes
the gates in [the upgrade guide](docs/upgrading.md). Future runner selection must
consume the same project runtime convention. The committed `frutlups.toml`
contains portable behavior only; machine executable paths stay ignored.
