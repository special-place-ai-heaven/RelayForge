Added a pinned Rustup 1.28.2 bootstrap for Linux jobs lacking rustup, preserving the existing runner gates and product checks.

## Changed files

- .github/workflows/ci.yml
- scripts/ci_contract_check.py
- docs/ci.md
- 02_analysis/ci-results.md

## Commands run

- Official x86_64/aarch64 GNU checksum sidecars matched downloaded installer binaries.
- Focused workflow checker passed 24 negative fixtures.
- Git Bash shell syntax check passed.
- git diff --check passed.

## What could not be verified

- Linux installer execution and new remote CI results remain unobserved.
- Authoritative full verification follows coded registration.

## Deviations

- None. Installer homes are checked children of RUNNER_TEMP; checksum verification precedes execution; no profile/global-default change. No product, S02 history, commit or push changes by worker.
