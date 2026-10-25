# Review prompt: M004 — CI configuration and observed platform qualification (round holistic)

Read `AGENTS.md` first. Do not change product files. Use the receipt as execution
evidence; do not rerun verification unless this prompt explicitly says so.

## Objective and acceptance

Judge holistic closure of M004 across its accepted slices.

### M004-S01

- Implement the workflow and verifier contracts in 02_analysis/ci-plan.md with no product schema or domain-logic change.
- Windows full checks use windows-latest; five useful Linux stages use self-hosted linux/fresh with max-parallel 5 and native target evidence.
- Reject public fork execution on custom runners; preserve read-only permissions, immutable action pins, full Git ancestry and job timeouts.
- Maintain explicit fmt, Clippy, unit, integration/CLI and doc stages with actual CLI builds and nonempty discovered/executed required tests; retain all Windows assertions.
- Deterministic workflow-contract checks and negative fixtures reject unsafe runner, permission, action-pin, ancestry and trust-gate changes; include them in full verification.
- Local Windows full verification remains clean-candidate, pinned, offline and source/index/HEAD-stable; record portable evidence and documented CI commands/limits.
- Do not claim Linux or GitHub-hosted qualification before actual CI runs; preserve missing runner/access evidence for S02.

### M004-S03

- Use an official version-pinned Rustup installer with verified pinned checksum and job-local homes; reject checksum failure before execution.
- Preserve shell profiles and global defaults, fork exclusion, read-only permissions and all existing stage/test gates.
- Provision Rust 1.99.1 and expose only job-local tool binaries to subsequent steps; keep locked fetch followed by offline checks.
- Extend deterministic workflow checks with missing, mutable or unchecked bootstrap and escaped job-home negative fixtures.
- Local focused and full Windows verification pass; document the observed Linux failure and correction without preclaiming remote success.

### M004-S04

- CI job environments explicitly clear RUSTC_WRAPPER and RUSTC_WORKSPACE_WRAPPER without modifying host settings or installing wrappers.
- Preserve all runner trust, bootstrap checksum, pinned/offline toolchain and existing test gates.
- Negative fixtures reject absent or nonempty compiler-wrapper overrides; a bounded probe demonstrates missing-wrapper failure and cleared-wrapper success.
- Focused and authoritative full checks pass locally; document run 40211448552 without claiming unobserved Linux success.

### M004-S02

- Record run URLs, exact source revision, host/target/tool versions and per-job conclusions for Windows full and all five Linux stages.
- Verify public fork versus trusted-source runner gating from actual workflow configuration and executed event context.
- Report unavailable runners, authentication, divergence or failures honestly with actor/action evidence; never infer Linux success from configuration.
- Use only sanitized results; keep raw logs, stores, caches and build output untracked and retain Q001 packaging gate.

## Non-goals

### M004-S01

- Product domain changes, new product dependencies, packaging, provider calls or host/service reconfiguration.

### M004-S03

- Product changes, global software/service installation or changing the frozen S02 qualification envelope.

### M004-S04

- Product changes, dependency installation, host configuration changes or edits to the frozen S02 evidence envelope.

### M004-S02

- Modifying workflows or product code in this evidence-only slice; hiding failed jobs or changing runner policy.

## Read first

### M004-S01

- `02_analysis/CONTEXT.md`
- `00_brief/decisions.md`
- `00_brief/release-contract.md`
- `02_analysis/ci-plan.md`
- `scripts/product_verify.py`
- `scripts/canary_verify.py`
- `scripts/hermetic_verification.py`
- `08_pkg/relayforge-core/tests/storage_contract.rs`
- `rust-toolchain.toml`
- `08_pkg/canary/rust-toolchain.toml`
- `pyproject.toml`

### M004-S03

- `02_analysis/CONTEXT.md`
- `00_brief/decisions.md`
- `02_analysis/ci-plan.md`
- `02_analysis/ci-qualification.md`
- `.github/workflows/ci.yml`
- `scripts/ci_contract_check.py`
- `docs/ci.md`

### M004-S04

- `02_analysis/CONTEXT.md`
- `00_brief/decisions.md`
- `.github/workflows/ci.yml`
- `scripts/ci_contract_check.py`
- `docs/ci.md`

### M004-S02

- `02_analysis/CONTEXT.md`
- `00_brief/decisions.md`
- `02_analysis/ci-plan.md`
- `docs/ci.md`
- `.github/workflows/ci.yml`

## Implementation boundary

Allowed prefixes: ### M004-S01

`.github/workflows/`, `scripts/product_verify.py`, `scripts/canary_verify.py`, `scripts/ci_contract_check.py`, `08_pkg/relayforge-core/tests/storage_contract.rs`, `rust-toolchain.toml`, `08_pkg/canary/rust-toolchain.toml`, `docs/ci.md`, `02_analysis/ci-results.md`

### M004-S03

`.github/workflows/ci.yml`, `scripts/ci_contract_check.py`, `docs/ci.md`, `02_analysis/ci-results.md`

### M004-S04

`.github/workflows/ci.yml`, `scripts/ci_contract_check.py`, `docs/ci.md`, `02_analysis/ci-results.md`

### M004-S02

`02_analysis/ci-qualification.md`

Forbidden: ### M004-S01

`00_brief/`, `05_governance/`, `prompts/`, `questions/answered/`, `roadmap.yaml`, `frutlups.toml`, `AGENTS.md`, `CLAUDE.md`

### M004-S03

`00_brief/`, `05_governance/`, `prompts/`, `questions/answered/`, `roadmap.yaml`, `frutlups.toml`, `AGENTS.md`, `CLAUDE.md`

### M004-S04

`00_brief/`, `05_governance/`, `prompts/`, `questions/answered/`, `roadmap.yaml`, `frutlups.toml`, `AGENTS.md`, `CLAUDE.md`

### M004-S02

`00_brief/`, `05_governance/`, `prompts/`, `questions/answered/`, `roadmap.yaml`, `frutlups.toml`, `AGENTS.md`, `CLAUDE.md` and all other paths. These describe the coder's scope;
review remains product-read-only. Read the evidence artifacts named below using
your file-reading tools. Their categories describe volume, not authority.

## Declared verification

Focused:

### M004-S01

```text
python -B scripts/ci_contract_check.py
```

```text
python -B scripts/product_verify.py --focused
```

### M004-S03

```text
python -B scripts/ci_contract_check.py
```

### M004-S04

```text
python -B scripts/ci_contract_check.py
```

### M004-S02

```text
python -B scripts/ci_contract_check.py
```

Full:

### M004-S01

```text
python -B scripts/hermetic_verification.py
```

Execution policy: runtime {}; timeout 900 seconds; observation process.

### M004-S03

```text
python -B scripts/hermetic_verification.py
```

Execution policy: runtime {}; timeout 900 seconds; observation process.

### M004-S04

```text
python -B scripts/hermetic_verification.py
```

Execution policy: runtime {}; timeout 900 seconds; observation process.

### M004-S02

```text
python -B scripts/hermetic_verification.py
```

Execution policy: runtime {}; timeout 900 seconds; observation process.

## Changed files

17 cumulative paths. Complete manifest: `05_governance/reviews/m004/M004_85bf134ba7ccf7d2_paths.json` (sha256 `85bf134ba7ccf7d288011a43b82aac7aee793d823a9754496a17dc42f99f3a6d`; read as a file, starting at line 1)

## Code diff

Bounded diff page: `05_governance/reviews/m004/M004_44048d9f16e8b7a0_diff.md` (sha256 `44048d9f16e8b7a03887967e347141032378f4d48c649a01d00864377ccc4510`; read as a file, starting at line 1)

```diff
git diff 82c380775aac7a771001f8f970dcf74f20fd1f8c..HEAD --stat (selected paths)
 .github/workflows/ci.yml          | 116 +++++++++++++++++++++
 02_analysis/ci-qualification.md   |  33 ++++++
 02_analysis/ci-results.md         |  73 ++++++++++++++
 08_pkg/canary/rust-toolchain.toml |   1 -
 docs/ci.md                        |  70 +++++++++++++
 rust-toolchain.toml               |   1 -
 scripts/canary_verify.py          |  38 ++++---
 scripts/ci_contract_check.py      | 206 ++++++++++++++++++++++++++++++++++++++
 scripts/product_verify.py         |  88 ++++++++++------
 9 files changed, 577 insertions(+), 49 deletions(-)

Per-slice diffs: current working tree against accepted base (bounded; complete manifests remain authoritative).
### M004-S01

diff --git a/scripts/canary_verify.py b/scripts/canary_verify.py
index 318a5ea..1433ace 100644
--- a/scripts/canary_verify.py
+++ b/scripts/canary_verify.py
@@ -16,7 +16,6 @@ from pathlib import Path, PurePosixPath
 
 ROOT = Path(__file__).resolve().parents[1]
 MANIFEST = Path("08_pkg/canary/Cargo.toml")
-TARGET = "x86_64-pc-windows-msvc"
 TIMEOUT_SECONDS = 900
 INNER_GUARD = "RELAYFORGE_CANARY_INNER"
 REQUIRED_TESTS = {
@@ -187,7 +186,7 @@ def self_test() -> str:
 
 def _rust_commands(root: Path, env: dict[str, str], deadline: float, scratch: Path) -> None:
     version_checks = [
-        ["rustc", "+1.99.1", "--version"],
+        ["rustc", "+1.99.1", "-vV"],
         _cargo("--version"),
         ["rustfmt", "+1.99.1", "--version"],
         ["cargo", "+1.99.1", "clippy", "--version"],
@@ -201,19 +200,24 @@ def _rust_commands(root: Path, env: dict[str, str], deadline: float, scratch: Pa
     cargo_version = results[1].stdout.strip()
     if not rustc_version.startswith("rustc 1.99.1 ") or not cargo_version.startswith("cargo 1.99.1 "):
         raise VerificationError("rustc and cargo must both report the pinned 1.99.1 version")
-    revision = re.search(r"\(([0-9a-f]{8})", rustc_version)
+    revision = re.search(r"^commit-hash: ([0-9a-f]{8})", rustc_version, re.MULTILINE)
     if revision is None or any(
         revision.group(1) not in result.stdout
         for result in results[2:4]
     ):
         raise VerificationError("rustfmt and Clippy do not match the pinned rustc toolchain build")
-    if TARGET not in results[4].stdout.split():
-        raise VerificationError(f"required Rust target is not installed: {TARGET}")
+    hosts = re.findall(r"^host: (\S+)$", rustc_version, re.MULTILINE)
+    if len(hosts) != 1:
+        raise VerificationError("pinned rustc did not report exactly one host target")
+    target = hosts[0]
+    if target not in results[4].stdout.split():
+        raise VerificationError(f"required Rust target is not installed: {target}")
+    print(f"CANARY NATIVE TARGET {target}", flush=True)
 
     graph = _run(
         _cargo(
             "tree", "--manifest-path", str(MANIFEST), "--locked", "--offline",
-            "--target", TARGET, "-i", "libsqlite3-sys",
+            "--target", target, "-i", "libsqlite3-sys",
         ),
         cwd=root,
         env=env,
@@ -224,7 +228,7 @@ def _rust_commands(root: Path, env: dict[str, str], deadline: float, scratch: Pa
         raise VerificationError("locked dependency graph does not match rusqlite 0.40.0/libsqlite3-sys 0.38.0")
     features = _run(
         _cargo(
-            "tree", "--locked", "--offline", "--target", TARGET,
+            "tree", "--locked", "--offline", "--target", target,
             "--manifest-path", str(MANIFEST), "--edges", "features", "-i", "libsqlite3-sys",
         ),
         cwd=root,
@@ -244,7 +248,7 @@ def _rust_commands(root: Path, env: dict[str, str], deadline: float, scratch: Pa
     )
     _run(
         _cargo(
-            "clippy", "--manifest-path", str(MANIFEST), "--all-targets", "--target", TARGET,
+            "clippy", "--manifest-path", str(MANIFEST), "--all-targets", "--target", target,
             "--locked", "--offline", "--", "-D", "warnings",
         ),
         cwd=root,
@@ -254,7 +258,7 @@ def _rust_commands(root: Path, env: dict[str, str], deadline: float, scratch: Pa
     )
     _run(
         _cargo(
-            "build", "--manifest-path", str(MANIFEST), "--all-targets", "--target", TARGET,
+            "build", "--manifest-path", str(MANIFEST), "--all-targets", "--target", target,
             "--locked", "--offline",
         ),
         cwd=root,
@@ -265,7 +269,7 @@ def _rust_commands(root: Path, env: dict[str, str], deadline: float, scratch: Pa
 
     discovered = _run(
         _cargo(
-            "test", "--manifest-path", str(MANIFEST), "--all-targets", "--target", TARGET,
+            "test", "--manifest-path", str(MANIFEST), "--all-targets", "--target", target,
             "--locked", "--offline", "--", "--list",

[diff truncated; complete manifest remains authoritative]
### M004-S01

diff --git a/scripts/ci_contract_check.py b/scripts/ci_contract_check.py
new file mode 100644
index 0000000..c803558
--- /dev/null
+++ b/scripts/ci_contract_check.py
@@ -0,0 +1,206 @@
+"""Check the public-repository CI trust boundary and exercise negative fixtures."""
+
+from __future__ import annotations
+
+import copy
+import re
+from pathlib import Path
+
+import yaml
+
+ROOT = Path(__file__).resolve().parents[1]
+WORKFLOW = ROOT / ".github/workflows/ci.yml"
+CHECKOUT = "actions/checkout@11d5960a326750d5838078e36cf38b85af677262"
+SETUP_PYTHON = "actions/setup-python@a26af69be951a213d495a4c3e4e4022e16d87065"
+STAGES = ["fmt", "clippy", "unit", "integration", "doc"]
+LINUX_GATE = (
+    "github.repository == 'special-place-ai-heaven/RelayForge' && "
+    "(github.event_name == 'push' || "
+    "(github.event_name == 'pull_request' && "
+    "github.event.pull_request.head.repo.full_name == github.repository && "
+    "contains(fromJSON('[\"OWNER\",\"MEMBER\",\"COLLABORATOR\"]'), "
+    "github.event.pull_request.author_association)))"
+)
+LINUX_HOMES = {
+    "RUSTUP_HOME": "${{ runner.temp }}/relayforge-rustup",
+    "CARGO_HOME": "${{ runner.temp }}/relayforge-cargo",
+}
+LINUX_PROVISION = """set -euo pipefail
+python3 --version
+getconf GNU_LIBC_VERSION >/dev/null
+temp_root="$(realpath "$RUNNER_TEMP")"
+mkdir -p "$RUSTUP_HOME" "$CARGO_HOME"
+test "$(realpath "$RUSTUP_HOME")" = "$temp_root/relayforge-rustup"
+test "$(realpath "$CARGO_HOME")" = "$temp_root/relayforge-cargo"
+case "$(uname -m)" in
+x86_64)
+installer_target=x86_64-unknown-linux-gnu
+expected_sha=20a06e644b0d9bd2fbdbfd52d42540bdde820ea7df86e92e533c073da0cdd43c
+;;
+aarch64)
+installer_target=aarch64-unknown-linux-gnu
+expected_sha=e3853c5a252fca15252d07cb23a1bdd9377a8c6f3efa01531109281ae47f841c
+;;
+*) echo "unsupported Rustup bootstrap architecture" >&2; exit 1 ;;
+esac
+installer="$CARGO_HOME/rustup-init"
+url="https://static.rust-lang.org/rustup/archive/1.28.2/$installer_target/rustup-init"
+python3 -c 'import sys, urllib.request; urllib.request.urlretrieve(sys.argv[1], sys.argv[2])' "$url" "$installer"
+printf '%s  %s\\n' "$expected_sha" "$installer" | sha256sum --check --status
+chmod +x "$installer"
+"$installer" -y --no-modify-path --default-toolchain none --profile minimal
+export PATH="$CARGO_HOME/bin:$PATH"
+test "$(command -v rustup)" = "$CARGO_HOME/bin/rustup"
+test "$(rustup --version | cut -d' ' -f2)" = "1.28.2"
+test -x "$CARGO_HOME/bin/cargo"
+printf '%s\\n' "$CARGO_HOME/bin" >> "$GITHUB_PATH"
+rustup toolchain install 1.99.1 --profile minimal --component rustfmt --component clippy
+native_target="$(rustc +1.99.1 -vV | sed -n 's/^host: //p')"
+test -n "$native_target"
+cargo +1.99.1 fetch --manifest-path Cargo.toml --locked --target "$native_target"
+"""
+
+
+def require(condition: bool, message: str) -> None:
+    if not condition:
+        raise ValueError(message)
+
+
+def validate(workflow: dict) -> None:
+    require(isinstance(workflow, dict), "workflow must be a mapping")
+    triggers = workflow.get("on")
+    require(isinstance(triggers, dict) and set(triggers) == {"push", "pull
```

[Inline excerpt bounded; read the full diff page above.]

## Verification receipt

### M004-S01

Verification passed. Complete receipt: `05_governance/reviews/m004/M004-S01_r1_verification.json` (sha256 `1657a4eb343f29d3ba9377ebf80084f199427ea28433261c090cc65d27191d8e`)

Review: `05_governance/reviews/m004/M004-S01_r1_review.md` (sha256 `c87975663a12e91a4e6e72b86ea7385cc80563ee5413bd5365fb413884d3ef34`)

### M004-S03

Verification passed. Complete receipt: `05_governance/reviews/m004/M004-S03_r2_verification.json` (sha256 `d098b3d4c91f371cd8859f27f206c8aa8c2d1111d0853b90c364066b3b3b48ec`)

Review: `05_governance/reviews/m004/M004-S03_r2_review.md` (sha256 `3ad38643642b64cd5eb7435a49bad9b57b9f5cdd265dc6cce60404177123b666`)

### M004-S04

Verification passed. Complete receipt: `05_governance/reviews/m004/M004-S04_r2_verification.json` (sha256 `18a1f13decb05a9b3e5ed428a7baed7ec0e0a35515260d246069e2b9d725dc7a`)

Review: `05_governance/reviews/m004/M004-S04_r2_review.md` (sha256 `20bacc4e5b6a962ed1b242c7a72e9952443aea8661c122ab4667b7c8884ecefa`)

### M004-S02

Verification passed. Complete receipt: `05_governance/reviews/m004/M004-S02_r3_verification.json` (sha256 `8a6f6d6fbe2f86bd1511ecd56a0440c824b5e4020d1ce8eb5b6b514984ad9824`)

Review: `05_governance/reviews/m004/M004-S02_r3_review.md` (sha256 `b4738b812e9104d210eee166dd19d3b84bc165ee1fe2267a214eeaaf2fb39aa3`)

## Prior findings

Complete findings and recovery context: `05_governance/reviews/m004/M004_1bfd9ea69d6db05b_context.md` (sha256 `1bfd9ea69d6db05b096107f347e7bef228dc99f71d873d7e9e2396e6869f9985`; read as a file, starting at line 1)

Source review: `05_governance/reviews/m004/M004-S03_r1_review.md` (sha256 `9e807b3a0836d2920e5306779c98a093575a2e8deaed2f0d986f72494b852b29`)
| M004-S03-BOOT-001 | P2 | open | `scripts/ci_contract_check.py:126-128`: `late_checksum`’s pattern is over-escaped and does not remove the checksum before execution. It appends another checksum after execution, so the exact-script comparison rejects a changed script without exercising the reordered-checksum case. Build the fixture by moving the actual checksum line after installer execution. |

Source review: `05_governance/reviews/m004/M004-S04_r1_review.md` (sha256 `b38d809d3aea89a5ffc555115c0869fecbfb96c7223d16e9276fc9e1ca37c7f5`)
| M004-S04-ENV-001 | P2 | open | `scripts/ci_contract_check.py:79-81, 167-171`: validation checks only job-level wrapper values. A read-only probe added `RUSTC_WRAPPER=sccache` to the Windows full-verification step's environment; `validate` accepted it, allowing a step-level override to restore the observed failure. Reject nonempty wrapper values in every step environment and add a fixture. |

Resolution authority: `00_brief/decisions.md` (sha256 `46cd9399b2f3e71fd6dcb15e2be06acc808e75e91e00dd44d7a245f658216851`)

Unblock reason: Holistic findings: M004-S02-DOC-001

Source review: `05_governance/reviews/m004/M004_holistic_review.md` (sha256 `965eaae0731c26700691e0380680a225be2569362a740ff05d0e17b5db0b858f`)
| M004-S02-DOC-001 | P2 | open | `docs/ci.md:53-66` still says S02 must record a new run and remote Linux qualification is pending. The S02 handoff records run 40213004003 passing Windows and all five Linux stages. Update the current CI status documentation with that qualification while retaining the failed-run history and Q001 gate. |

Original blocker: `05_governance/reviews/m004/M004-S02_r1_outcome_12fed74cb06b40de.json` (sha256 `12fed74cb06b40de9f74813b6960b900b6099119b1397c0e5749c236c774b447`)
{"action":"Admit and review job-local Rust bootstrap correction, push, then resume actual-run qualification","actor":"architect","authority":[

[Context excerpt bounded; read the complete context file above.]

## Output

Autonomous seats return the complete report for the runner to save. In manual
mode, write only `05_governance/reviews/m004/M004_holistic_2_review.md` when that tool is granted, or return it for
the architect to save.

Use this exact contract:

```markdown
# Review: M004 round holistic

## Findings
| id | severity | disposition | summary |
| --- | --- | --- | --- |

## Closure Decision
Objective status: achieved | not_achieved | indeterminate
Objective evidence: one sentence tied to acceptance and the receipt

## Verdict
Verdict: pass|needs_work|blocked - next: one move
```

Return the report as the plain text of your final message, in exactly this shape, not enclosed in a code fence. Use one allowed value on each choice line. A pass requires zero open P0-P2.
Every P0-P2 finding ID must start with the affected slice ID, for example `M001-S02-H1-F1`.

To change an older finding, add `## Finding updates` before Closure Decision with columns `source | sha | id | disposition | related`. Name the original report path, its SHA-256, exact finding ID and explicit disposition; related is linked IDs or `-`. An unrelated pass closes nothing. Only a human may waive findings.
