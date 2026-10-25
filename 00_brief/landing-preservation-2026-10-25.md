# RelayForge main landing and draft preservation, 2026-10-25

This dated report records the owner-authorized landing housekeeping. The first
release goal remains paused; this report does not admit a new product slice or
change the ledger. Read actual Git, ledger and CI state when work resumes.

## Git and CI observation

- Before landing, `origin/main` was `89c0937d2fa33dceedc59ca217e3ba805c7e3f03`,
  primary `main` was `184b710c96497a0ba6101195216e3e31f51bbf7c`, and
  `campaign/delivery` was `577e2764df4e77cd09faefe0dc7025a2c9237a87`.
  Primary `main` was an ancestor, 28 commits behind the campaign head. Fetch
  found no remote divergence.
- After preserving the primary drafts, `git merge --ff-only campaign/delivery`
  moved primary `main` from `184b710` to `577e276`. The original campaign
  commits and review history were retained. No product or ledger file was edited
  for landing.
- The campaign head's [push CI run](https://github.com/special-place-ai-heaven/RelayForge/actions/runs/40223231330)
  completed successfully: hosted Windows and all five Linux jobs passed. The
  preceding M006 closure [run](https://github.com/special-place-ai-heaven/RelayForge/actions/runs/40223019954)
  also completed successfully. At this observation, no PR existed for the
  campaign branch, and GitHub returned no `main` branch protection or ruleset.
- The landing housekeeping commit `ce84f0031f005fd0eebae9142e0a5727984053d2`
  was pushed to `main`. Its [CI run](https://github.com/special-place-ai-heaven/RelayForge/actions/runs/40225708817)
  completed successfully: hosted Windows and all five Linux jobs passed. The
  run emitted a nonblocking Node 20 checkout warning for later CI maintenance;
  no workflow change was made in this cleanup.

The maintained `python -B scripts/hermetic_verification.py` completed locally
after the fast-forward and this housekeeping draft: exit 0 in 134.050 seconds,
Terminal Commander job `job_01a1747fda1972129cd94f9a92563825`. It reported
roadmap and ledger checks, clean candidate and unchanged source/index/HEAD;
canary tests, the CI contract, 17 product unit tests, 15 integration tests and
one doctest passed. The scoped `git diff --check` on the edited tracked brief
files also passed. The pushed `main` run above supplies separate CI evidence.

## Primary drafts saved before fast-forward

The primary checkout had exactly 13 dirty tracked or untracked paths. A named
`git stash push --include-untracked` saved them at
`f88157109526c39ccb51f6b016cabe47bad4ec13`; its untracked-file parent is
`e18f8a04b6444708ac3f9dc6b93b1a423181186e`. The durable local recovery
reference `refs/backup/relayforge-primary-prelanding-2026-10-25` points to the
stash commit. Do not drop the stash or apply it over accepted campaign files.

An external sibling `RelayForge-landing-preservation-20261025` holds
`primary-dirty-manifest.json` (SHA-256
`bc2a4b8b4e0bc0ccf18473fb306c8fbb54fca02e1f6a0669703090ac00877d48`).
The manifest names all 13 repository-relative paths, their original Git blob
IDs and raw-byte SHA-256 digests. Each saved stash blob matched both identities,
13 of 13. The original versions are recoverable through the named ref and blob
IDs without restoring the older files to `main`.

| Primary draft paths | Comparison with accepted campaign head |
|---|---|
| `05_governance/ledger.jsonl` | Primary's 32 lines are an exact prefix of the campaign's 190 lines. The original prompt event is retained in accepted history. |
| `02_analysis/CONTEXT.md` | The new JEV research link is present in the campaign. The primary version still says no product exists; the campaign has later product state. |
| `02_analysis/canary-results.md` | Primary describes the first candidate snapshot; campaign retains it as historical and adds the reviewed later rounds. |
| `08_pkg/canary/rust-toolchain.toml`, `scripts/canary_verify.py` | Primary pins a Windows-only target and uses the earlier candidate verifier. Campaign has the reviewed native-target and private-baseline corrections. |
| Other eight untracked paths | Byte-identical to campaign: the JEV research note, M002 r1 envelope, five other canary files, and the M002 r1 coding prompt. |

These comparisons describe content relationships, not permission to discard the
original bytes. The recovery ref and manifest preserve every pre-landing variant.

## Campaign checkout cleanup

After the pushed `main` run passed, live remote `main` and local `main` both
resolved to `ce84f0031f005fd0eebae9142e0a5727984053d2`. The reviewed
campaign commit `577e276` remained an ancestor; both worktrees were clean.
The campaign's only ignored top-level directories were `.symforge/` index data
and `target/` Rust build output. Shallow inspection found only tracked
`local_state/README.md` under `local_state/`; no local database or raw runtime
store was observed there. The primary checkout's ignored state was left intact.

Both ignored directories were moved natively into the existing external sibling
`RelayForge-landing-preservation-20261025` as `campaign-symforge/` and
`campaign-target/`. The original paths were absent afterward, both destinations
were directories, their shallow entries matched, and selected shallow file
SHA-256 digests matched before and after. This was a directory move, not a
recursive content audit of the build cache.

With no dirty or ignored campaign paths remaining, `git worktree remove`
completed without force. `git branch -d campaign/delivery` then deleted the
fully merged local branch. Only the primary `main` worktree remains registered.
The remote campaign branch was left untouched. The pre-landing stash, recovery
reference and 13-file manifest remain intact; all 13 saved blobs were verified
again before cleanup.
