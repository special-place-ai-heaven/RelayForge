"""Check the public-repository CI trust boundary and exercise negative fixtures."""

from __future__ import annotations

import copy
import re
from pathlib import Path

import yaml

ROOT = Path(__file__).resolve().parents[1]
WORKFLOW = ROOT / ".github/workflows/ci.yml"
CHECKOUT = "actions/checkout@11d5960a326750d5838078e36cf38b85af677262"
SETUP_PYTHON = "actions/setup-python@a26af69be951a213d495a4c3e4e4022e16d87065"
STAGES = ["fmt", "clippy", "unit", "integration", "doc"]
LINUX_GATE = (
    "github.repository == 'special-place-ai-heaven/RelayForge' && "
    "(github.event_name == 'push' || "
    "(github.event_name == 'pull_request' && "
    "github.event.pull_request.head.repo.full_name == github.repository && "
    "contains(fromJSON('[\"OWNER\",\"MEMBER\",\"COLLABORATOR\"]'), "
    "github.event.pull_request.author_association)))"
)
LINUX_HOMES = {
    "RUSTUP_HOME": "${{ runner.temp }}/relayforge-rustup",
    "CARGO_HOME": "${{ runner.temp }}/relayforge-cargo",
}
LINUX_PROVISION = """set -euo pipefail
python3 --version
getconf GNU_LIBC_VERSION >/dev/null
temp_root="$(realpath "$RUNNER_TEMP")"
mkdir -p "$RUSTUP_HOME" "$CARGO_HOME"
test "$(realpath "$RUSTUP_HOME")" = "$temp_root/relayforge-rustup"
test "$(realpath "$CARGO_HOME")" = "$temp_root/relayforge-cargo"
case "$(uname -m)" in
x86_64)
installer_target=x86_64-unknown-linux-gnu
expected_sha=20a06e644b0d9bd2fbdbfd52d42540bdde820ea7df86e92e533c073da0cdd43c
;;
aarch64)
installer_target=aarch64-unknown-linux-gnu
expected_sha=e3853c5a252fca15252d07cb23a1bdd9377a8c6f3efa01531109281ae47f841c
;;
*) echo "unsupported Rustup bootstrap architecture" >&2; exit 1 ;;
esac
installer="$CARGO_HOME/rustup-init"
url="https://static.rust-lang.org/rustup/archive/1.28.2/$installer_target/rustup-init"
python3 -c 'import sys, urllib.request; urllib.request.urlretrieve(sys.argv[1], sys.argv[2])' "$url" "$installer"
printf '%s  %s\\n' "$expected_sha" "$installer" | sha256sum --check --status
chmod +x "$installer"
"$installer" -y --no-modify-path --default-toolchain none --profile minimal
export PATH="$CARGO_HOME/bin:$PATH"
test "$(command -v rustup)" = "$CARGO_HOME/bin/rustup"
test "$(rustup --version | cut -d' ' -f2)" = "1.28.2"
test -x "$CARGO_HOME/bin/cargo"
printf '%s\\n' "$CARGO_HOME/bin" >> "$GITHUB_PATH"
rustup toolchain install 1.99.1 --profile minimal --component rustfmt --component clippy
native_target="$(rustc +1.99.1 -vV | sed -n 's/^host: //p')"
test -n "$native_target"
cargo +1.99.1 fetch --manifest-path Cargo.toml --locked --target "$native_target"
"""


def require(condition: bool, message: str) -> None:
    if not condition:
        raise ValueError(message)


def validate(workflow: dict) -> None:
    require(isinstance(workflow, dict), "workflow must be a mapping")
    triggers = workflow.get("on")
    require(isinstance(triggers, dict) and set(triggers) == {"push", "pull_request"}, "unsafe or missing event trigger")
    require(triggers["push"] == {"branches": ["main", "campaign/**"]}, "push branches changed")
    require(workflow.get("permissions") == {"contents": "read"}, "workflow token must be contents: read only")
    require("env" not in workflow, "workflow-wide environment is prohibited")
    jobs = workflow.get("jobs")
    require(isinstance(jobs, dict) and set(jobs) == {"windows", "linux"}, "expected exactly Windows and gated Linux jobs")
    windows, linux = jobs["windows"], jobs["linux"]
    require(windows.get("runs-on") == "windows-latest" and "if" not in windows, "fork PRs need hosted Windows verification")
    require(linux.get("runs-on") == ["self-hosted", "linux", "fresh"], "custom runner labels changed")
    require(" ".join(str(linux.get("if", "")).split()) == LINUX_GATE, "custom runner trust gate changed")
    require(linux.get("strategy") == {
        "fail-fast": "false",
        "max-parallel": "5",
        "matrix": {"stage": STAGES},
    }, "five Linux stages or concurrency changed")
    for name, job in jobs.items():
        require("permissions" not in job and "secrets" not in job, f"{name} overrides job trust boundary")
        timeout = job.get("timeout-minutes")
        require(timeout is not None and timeout.isdigit() and 0 < int(timeout) <= 30, f"{name} timeout missing or excessive")
        steps = job.get("steps")
        require(isinstance(steps, list) and steps, f"{name} steps missing")
        actions = [step["uses"] for step in steps if "uses" in step]
        allowed = {CHECKOUT, SETUP_PYTHON} if name == "windows" else {CHECKOUT}
        require(set(actions) == allowed and len(actions) == len(allowed), f"{name} action pin changed")
        require(all(re.fullmatch(r"actions/[\w-]+@[0-9a-f]{40}", action) for action in actions), f"{name} action is mutable")
        checkout = next(step for step in steps if step.get("uses") == CHECKOUT)
        require(checkout.get("with") == {"persist-credentials": "false", "fetch-depth": "0"}, f"{name} checkout credentials or ancestry changed")
        runs = "\n".join(str(step.get("run", "")) for step in steps)
        require("rustup toolchain install 1.99.1" in runs and "cargo +1.99.1 fetch" in runs and "--locked" in runs, f"{name} pinned provisioning changed")
        require("secrets." not in str(job), f"{name} reads workflow secrets")
    windows_runs = "\n".join(str(step.get("run", "")) for step in windows["steps"])
    linux_runs = "\n".join(str(step.get("run", "")) for step in linux["steps"])
    require("scripts/hermetic_verification.py" in windows_runs, "Windows full verifier missing")
    full_step = next(step for step in windows["steps"] if "scripts/hermetic_verification.py" in str(step.get("run", "")))
    identity = {
        "GIT_AUTHOR_NAME": "special-place-administrator",
        "GIT_AUTHOR_EMAIL": "administrator@special-place.online",
        "GIT_COMMITTER_NAME": "special-place-administrator",
        "GIT_COMMITTER_EMAIL": "administrator@special-place.online",
    }
    require(all(full_step.get("env", {}).get(key) == value for key, value in identity.items()), "scratch candidate Git identity changed")
    require('scripts/product_verify.py --stage "${{ matrix.stage }}"' in linux_runs, "Linux product stage missing")
    require(len(linux["steps"]) == 3, "Linux job must have checkout, provision and verify steps")
    provision, verify = linux["steps"][1:]
    require(provision.get("env") == LINUX_HOMES and verify.get("env") == LINUX_HOMES, "Linux Rust homes escaped job storage")
    require(provision.get("shell") == "bash" and verify.get("shell") == "bash", "Linux shell changed")
    actual = [line.strip() for line in str(provision.get("run", "")).splitlines()]
    expected = [line.strip() for line in LINUX_PROVISION.splitlines()]
    require(actual == expected, "Linux pinned, checksum-gated bootstrap changed")


def negative_fixtures(workflow: dict) -> int:
    def late_checksum(candidate: dict) -> None:
        step = candidate["jobs"]["linux"]["steps"][1]
        checksum_lines = [
            line for line in step["run"].splitlines(keepends=True)
            if "sha256sum --check --status" in line
        ]
        require(len(checksum_lines) == 1, "late-checksum fixture source must contain one checksum")
        checksum = checksum_lines[0]
        execution = '"$installer" -y --no-modify-path --default-toolchain none --profile minimal\n'
        require(step["run"].count(execution) == 1, "late-checksum fixture source must contain one installer execution")
        step["run"] = step["run"].replace(checksum, "", 1).replace(execution, execution + checksum, 1)
        require(step["run"].count(checksum) == 1, "late-checksum fixture must retain one checksum line")
        require(step["run"].count("sha256sum --check --status") == 1, "late-checksum fixture must contain one checksum")
        require(step["run"].index(execution) < step["run"].index(checksum), "late-checksum fixture must execute first")

    mutations = [
        ("fork trigger", lambda w: w["on"].update({"pull_request_target": {}})),
        ("write token", lambda w: w["permissions"].update({"contents": "write"})),
        ("custom runner labels", lambda w: w["jobs"]["linux"].update({"runs-on": ["self-hosted", "linux"]})),
        ("fork trust gate", lambda w: w["jobs"]["linux"].pop("if")),
        ("copied fork push", lambda w: w["jobs"]["linux"].update({"if": LINUX_GATE.split(" && ", 1)[1]})),
        ("author association", lambda w: w["jobs"]["linux"].update({"if": "github.event_name == 'pull_request'"})),
        ("mutable action", lambda w: w["jobs"]["windows"]["steps"][0].update({"uses": "actions/checkout@v4"})),
        ("checkout credentials", lambda w: w["jobs"]["linux"]["steps"][0]["with"].update({"persist-credentials": "true"})),
        ("shallow ancestry", lambda w: w["jobs"]["windows"]["steps"][0]["with"].update({"fetch-depth": "1"})),
        ("missing candidate identity", lambda w: w["jobs"]["windows"]["steps"][-1]["env"].pop("GIT_AUTHOR_EMAIL")),
        ("missing timeout", lambda w: w["jobs"]["linux"].pop("timeout-minutes")),
        ("unbounded parallelism", lambda w: w["jobs"]["linux"]["strategy"].update({"max-parallel": "6"})),
        ("missing stage", lambda w: w["jobs"]["linux"]["strategy"]["matrix"].update({"stage": STAGES[:-1]})),
        ("new ungated job", lambda w: w["jobs"].update({"extra": {"runs-on": ["self-hosted", "linux", "fresh"]}})),
        ("missing bootstrap", lambda w: w["jobs"]["linux"]["steps"][1].update({"run": w["jobs"]["linux"]["steps"][1]["run"].replace('"$installer" -y ', 'true # "$installer" -y ')})),
        ("mutable installer", lambda w: w["jobs"]["linux"]["steps"][1].update({"run": w["jobs"]["linux"]["steps"][1]["run"].replace("archive/1.28.2", "dist")})),
        ("wrong checksum", lambda w: w["jobs"]["linux"]["steps"][1].update({"run": w["jobs"]["linux"]["steps"][1]["run"].replace("20a06e644b0d", "00a06e644b0d")})),
        ("unchecked installer", lambda w: w["jobs"]["linux"]["steps"][1].update({"run": w["jobs"]["linux"]["steps"][1]["run"].replace("sha256sum --check --status", "true")})),
        ("late checksum", late_checksum),
        ("shell profile edit", lambda w: w["jobs"]["linux"]["steps"][1].update({"run": w["jobs"]["linux"]["steps"][1]["run"].replace("--no-modify-path", "--modify-path")})),
        ("global default", lambda w: w["jobs"]["linux"]["steps"][1].update({"run": w["jobs"]["linux"]["steps"][1]["run"].replace("--default-toolchain none", "--default-toolchain stable")})),
        ("escaped Rust home", lambda w: w["jobs"]["linux"]["steps"][1]["env"].update({"RUSTUP_HOME": "/tmp/shared-rustup"})),
        ("escaped Cargo home", lambda w: w["jobs"]["linux"]["steps"][2]["env"].update({"CARGO_HOME": "/tmp/shared-cargo"})),
        ("missing later path", lambda w: w["jobs"]["linux"]["steps"][1].update({"run": w["jobs"]["linux"]["steps"][1]["run"].replace('>> "$GITHUB_PATH"', '> /dev/null')})),
    ]
    for label, mutate in mutations:
        fixture = copy.deepcopy(workflow)
        mutate(fixture)
        try:
            validate(fixture)
        except ValueError:
            continue
        raise ValueError(f"negative fixture accepted: {label}")
    return len(mutations)


def main() -> int:
    try:
        workflow = yaml.load(WORKFLOW.read_text(encoding="utf-8"), Loader=yaml.BaseLoader)
        validate(workflow)
        count = negative_fixtures(workflow)
    except (OSError, ValueError, yaml.YAMLError) as exc:
        print(f"CI CONTRACT FAIL: {exc}")
        return 1
    print(f"CI CONTRACT PASS: workflow trust boundary; {count} unsafe fixtures rejected")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
