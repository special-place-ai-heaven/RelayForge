# Distribution input observations — 2026-10-25

Planning evidence only; Q001 remains open. No package was created and no
permission grant or exclusion approval is claimed by these observations.

Observed after M005 implementation:

- SHA-256 comparison found root LICENSE byte-identical to the owner's SymForge
  reference. Its owner notice is `Required Notice: Copyright (c) 2026 special-place-administrator`.
- Git addition history places the native core, CLI and root Cargo workspace in
  first-party implementation commit 4d6126f, not the imported template baseline.
  This is provenance input; the eventual independent input review must also
  inspect code/build inclusion and any copied or derived material.
- `cargo +1.99.1 metadata --locked --offline --format-version 1` reports both
  first-party packages as PolyForm-Noncommercial-1.0.0. The lock closure contains
  third-party MIT/Apache-2.0 alternatives, foldhash Zlib, unicode-ident with an
  additional Unicode-3.0 term, MIT-only packages and bundled SQLite through
  libsqlite3-sys. Preserve original texts/notices; SPDX metadata alone is not a
  complete notice bundle. The unfiltered metadata includes non-Windows targets;
  determine the actual Windows dependency closure before finalizing notices.
- The minimal Rust 1.99.1 installation did not contain the explicitly checked
  share/doc/rust/COPYRIGHT, LICENSE-APACHE or LICENSE-MIT paths. Obtain verified
  standard-library/runtime attribution inputs before packaging rather than
  assuming Cargo metadata covers the Rust runtime.

## Proposed exclusion route for later admission

Use a Windows binary archive with an explicit input allowlist: the built CLI,
first-party LICENSE/owner notice, independently authored product instructions,
and release-scoped third-party/runtime notices. Do not create a source archive
unless a separate source input review establishes its exclusions too.

Exclude imported/derived template material, workflow scripts, prompts,
initialization/governance, original template docs, canary, stores/databases,
credentials, caches, build intermediates and raw runtime data. Keep required
source/tests/locks/ledger evidence tracked in Git; exclusion from an artifact is
separate from .gitignore and does not mean removing tracked evidence.

Before packaging: independently review the exact build and allowlist inputs
against provenance and included third-party requirements, and record Q001's
exclusion disposition. After packaging: inspect every actual artifact against
that approved list. Exercise local installation/uninstallation in an owned
sandbox, preserving unrelated sentinel files and making no global PATH,
registry, service or host-tool changes. These remain future release checks.
