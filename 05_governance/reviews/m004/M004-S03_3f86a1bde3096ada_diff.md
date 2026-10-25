```diff
HEAD diff filtered to current-round paths; this is not a prior-round delta.
diff --git a/scripts/ci_contract_check.py b/scripts/ci_contract_check.py
index e11cf89..80c321e 100644
--- a/scripts/ci_contract_check.py
+++ b/scripts/ci_contract_check.py
@@ -21,6 +21,44 @@ LINUX_GATE = (
     "contains(fromJSON('[\"OWNER\",\"MEMBER\",\"COLLABORATOR\"]'), "
     "github.event.pull_request.author_association)))"
 )
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
 
 
 def require(condition: bool, message: str) -> None:
@@ -73,9 +111,22 @@ def validate(workflow: dict) -> None:
     }
     require(all(full_step.get("env", {}).get(key) == value for key, value in identity.items()), "scratch candidate Git identity changed")
     require('scripts/product_verify.py --stage "${{ matrix.stage }}"' in linux_runs, "Linux product stage missing")
+    require(len(linux["steps"]) == 3, "Linux job must have checkout, provision and verify steps")
+    provision, verify = linux["steps"][1:]
+    require(provision.get("env") == LINUX_HOMES and verify.get("env") == LINUX_HOMES, "Linux Rust homes escaped job storage")
+    require(provision.get("shell") == "bash" and verify.get("shell") == "bash", "Linux shell changed")
+    actual = [line.strip() for line in str(provision.get("run", "")).splitlines()]
+    expected = [line.strip() for line in LINUX_PROVISION.splitlines()]
+    require(actual == expected, "Linux pinned, checksum-gated bootstrap changed")
 
 
 def negative_fixtures(workflow: dict) -> int:
+    def late_checksum(candidate: dict) -> None:
+        step = candidate["jobs"]["linux"]["steps"][1]
+        checksum = "printf '%s  %s\\n' \"$expected_sha\" \"$installer\" | sha256sum --check --status\n"
+        execution = '"$installer" -y --no-modify-path --default-toolchain none --profile minimal\n'
+        step["run"] = step["run"].replace(checksum, "").replace(execution, execution + checksum)
+
     mutations = [
         ("fork trigger", lambda w: w["on"].update({"pull_request_target": {}})),
         ("write token", lambda w: w["permissions"].update({"contents": "write"})),
@@ -91,6 +142,16 @@ def negative_fixtures(workflow: dict) -> int:
         ("unbounded parallelism", lambda w: w["jobs"]["linux"]["strategy"].update({"max-parallel": "6"})),
         ("missing stage", lambda w: w["jobs"]["linux"]["strategy"]["matrix"].update({"stage": STAGES[:-1]})),
         ("new ungated job", lambda w: w["jobs"
[diff truncated; complete manifest remains authoritative]
diff --git a/.github/workflows/ci.yml b/.github/workflows/ci.yml
index bfa1ca1..ad79ae2 100644
--- a/.github/workflows/ci.yml
+++ b/.github/workflows/ci.yml
@@ -71,6 +71,33 @@ jobs:
         run: |
           set -euo pipefail
           python3 --version
+          getconf GNU_LIBC_VERSION >/dev/null
+          temp_root="$(realpath "$RUNNER_TEMP")"
+          mkdir -p "$RUSTUP_HOME" "$CARGO_HOME"
+          test "$(realpath "$RUSTUP_HOME")" = "$temp_root/relayforge-rustup"
+          test "$(realpath "$CARGO_HOME")" = "$temp_root/relayforge-cargo"
+          case "$(uname -m)" in
+            x86_64)
+              installer_target=x86_64-unknown-linux-gnu
+              expected_sha=20a06e644b0d9bd2fbdbfd52d42540bdde820ea7df86e92e533c073da0cdd43c
+              ;;
+            aarch64)
+              installer_target=aarch64-unknown-linux-gnu
+              expected_sha=e3853c5a252fca15252d07cb23a1bdd9377a8c6f3efa01531109281ae47f841c
+              ;;
+            *) echo "unsupported Rustup bootstrap architecture" >&2; exit 1 ;;
+          esac
+          installer="$CARGO_HOME/rustup-init"
+          url="https://static.rust-lang.org/rustup/archive/1.28.2/$installer_target/rustup-init"
+          python3 -c 'import sys, urllib.request; urllib.request.urlretrieve(sys.argv[1], sys.argv[2])' "$url" "$installer"
+          printf '%s  %s\n' "$expected_sha" "$installer" | sha256sum --check --status
+          chmod +x "$installer"
+          "$installer" -y --no-modify-path --default-toolchain none --profile minimal
+          export PATH="$CARGO_HOME/bin:$PATH"
+          test "$(command -v rustup)" = "$CARGO_HOME/bin/rustup"
+          test "$(rustup --version | cut -d' ' -f2)" = "1.28.2"
+          test -x "$CARGO_HOME/bin/cargo"
+          printf '%s\n' "$CARGO_HOME/bin" >> "$GITHUB_PATH"
           rustup toolchain install 1.99.1 --profile minimal --component rustfmt --component clippy
           native_target="$(rustc +1.99.1 -vV | sed -n 's/^host: //p')"
           test -n "$native_target"
diff --git a/02_analysis/ci-results.md b/02_analysis/ci-results.md
index 9cda32e..8fc71a5 100644
--- a/02_analysis/ci-results.md
+++ b/02_analysis/ci-results.md
@@ -28,3 +28,28 @@ Worker-local Windows observations for this candidate:
 Linux and hosted-Windows CI qualification is reserved for M004-S02. Runner
 availability, isolation and five simultaneous runner slots remain unobserved
 until real run evidence is recorded there.
+
+## M004-S03 Linux bootstrap correction
+
+The first actual CI run is recorded in `ci-qualification.md`: hosted Windows
+passed; five distinct fresh Linux runners accepted jobs, then failed
+provisioning because `rustup` was absent (observed exit 127 in the formatting
+job). No Linux product stage executed in that run.
+
+The official Rustup manual-installation guide specifies
+`static.rust-lang.org/rustup/archive/{version}/{target}/rustup-init` and its
+`.sha256` sidecar. On 2026-10-25, the official 1.28.2 sidecars and independently
+downloaded binaries both produced these SHA-256 values:
+
+- `x86_64-unknown-linux-gnu`:
+  `20a06e644b0d9bd2fbdbfd52d42540bdde820ea7df86e92e533c073da0cdd43c`
+- `aarch64-unknown-linux-gnu`:
+  `e3853c5a252fca15252d07cb23a1bdd9377a8c6f3efa01531109281ae47f841c`
+
+The corrected workflow checks the pinned digest before execution and confines
+installation to job-local homes. The worker-local
+`python -B scripts/ci_contract_check.py` passed with 24 unsafe fixtures
+rejected, including bootstrap, digest, shell-profile, default-toolchain,
+job-home and later-step PATH mutations. This is configuration evidence only.
+The architect-owned full verification and a new actual CI run remain pending;
+the frozen S02 report and failed-run history are unchanged.
diff --git a/docs/ci.md b/docs/ci.md
index 9aa359c..b144942 100644
--- a/docs/ci.md
+++ b/docs/ci.md
@@ -19,8 +19,22 @@ runner availability, absence of host credentials, or five-way capacity. Those
 properties need operator evidence and observed CI runs.
 
 The workflow provisions Rust 1.99.1 and downloaded Cargo sources under each
-job's temporary homes, then verifies with `--locked --offline`. Windows
-provisions Python 3.11 and PyYAML 6.0.3 in a job-owned virtual environment.
+job's temporary homes, then verifies with `--locked --offline`. Fresh Linux
+runners in [the first actual run](https://github.com/special-place-ai-heaven/RelayForge/actions/runs/40209605799)
+had Python 3.12.3 but no `rustup`; all five Linux jobs stopped at provisioning,
+before product checks. The Linux job now selects an official Rustup 1.28.2
+GNU/Linux installer for x86_64 or aarch64, checks its pinned SHA-256 before
+execution, and rejects other architectures or a non-glibc host. It verifies
+`RUSTUP_HOME` and `CARGO_HOME` resolve to children of the runner's temporary
+directory, disables shell-profile changes and default-toolchain installation,
+and adds only its job-local Cargo binary directory to current and later steps.
+The [official rustup guide](https://rust-lang.github.io/rustup/installation/other.html#manual-installation)
+documents the versioned archives and checksum sidecars. The pinned
+[x86_64 checksum](https://static.rust-lang.org/rustup/archive/1.28.2/x86_64-unknown-linux-gnu/rustup-init.sha256)
+and [aarch64 checksum](https://static.rust-lang.org/rustup/archive/1.28.2/aarch64-unknown-linux-gnu/rustup-init.sha256)
+were checked against their corresponding official installer binaries.
+
+Windows provisions Python 3.11 and PyYAML 6.0.3 in a job-owned virtual environment.
 Its verifier step supplies the existing owner Git identity solely for scratch
 candidate metadata; it creates no repository commit.
 No database, cache, build output or raw runtime data is uploaded.
@@ -36,6 +50,8 @@ python -B scripts/product_verify.py --stage integration
 The architect runs `python -B scripts/hermetic_verification.py` once after the
 slice is coded. It retains the clean-candidate Git ancestry, ledger check,
 canary checks, product checks, source/index/HEAD stability and 900-second
-verifier budget. Local Windows checks do not qualify Linux or GitHub-hosted
-Windows. S02 must record real run URLs, revision, native targets/tool versions,
-per-job conclusions and any runner access or queue blockers.
+verifier budget. The first actual run passed hosted Windows and proved five
+distinct fresh Linux runners accepted jobs; it did not qualify Linux product
+checks. S02 must record a new real run after the correction, with revision,
+native targets/tool versions and per-job conclusions. The `fresh` label does
+not prove host isolation or future availability.

Diff truncated at 32 KB or the smaller per-file evidence allowance.
```
