"""Hermetic runner and clean-Git-candidate check for the native canary."""

from __future__ import annotations

import hashlib
import io
import json
import os
import re
import subprocess
import sys
import tarfile
import tempfile
import time
from pathlib import Path, PurePosixPath

ROOT = Path(__file__).resolve().parents[1]
MANIFEST = Path("08_pkg/canary/Cargo.toml")
TARGET = "x86_64-pc-windows-msvc"
TIMEOUT_SECONDS = 900
INNER_GUARD = "RELAYFORGE_CANARY_INNER"
REQUIRED_TESTS = {
    "unit": {
        "tests::completion_requires_verification_and_acceptance",
        "tests::capability_facts_are_distinct_from_observations",
        "tests::duplicate_events_do_not_inflate_counts",
        "tests::completion_state_is_false_until_both_events_exist",
        "tests::event_identity_is_unique_even_when_replayed",
    },
    "integration": {
        "bundled_runtime_reports_sqlite_3531",
        "committed_seed_survives_reopen_with_stable_event_ids",
    },
}


class VerificationError(RuntimeError):
    pass


def _display(argv: list[str], scratch: Path | None = None) -> str:
    parts = [str(item) for item in argv]
    if scratch is not None:
        parts = [part.replace(str(scratch), "<scratch>") for part in parts]
    return " ".join(parts)


def _run(
    argv: list[str],
    *,
    cwd: Path,
    env: dict[str, str],
    deadline: float,
    scratch: Path | None = None,
    allow_failure: bool = False,
) -> subprocess.CompletedProcess[str]:
    remaining = deadline - time.monotonic()
    if remaining <= 0:
        raise VerificationError("900-second canary verification budget exhausted")
    command = _display(argv, scratch)
    print(f"COMMAND {command}", flush=True)
    try:
        result = subprocess.run(
            argv,
            cwd=cwd,
            env=env,
            shell=False,
            check=False,
            capture_output=True,
            text=True,
            timeout=remaining,
        )
    except subprocess.TimeoutExpired as exc:
        raise VerificationError(f"timed out: {command}") from exc
    print(f"EXIT {result.returncode}: {command}", flush=True)
    if result.stdout:
        print(result.stdout, end="" if result.stdout.endswith("\n") else "\n", flush=True)
    if result.stderr:
        print(result.stderr, file=sys.stderr, end="" if result.stderr.endswith("\n") else "\n", flush=True)
    if result.returncode and not allow_failure:
        raise VerificationError(f"command failed with exit {result.returncode}: {command}")
    return result


def _cargo(*args: str) -> list[str]:
    return ["cargo", "+1.99.1", *args]


def parse_discovery(output: str) -> tuple[set[str], set[str]]:
    found: set[str] = set()
    ignored: set[str] = set()
    for line in output.splitlines():
        match = re.match(r"^(.+): (test|benchmark|ignored)$", line.strip())
        if match:
            name, state = match.groups()
            (ignored if state == "ignored" else found).add(name)
    return found, ignored


def guard_discovery(
    output: str, expected: set[str], suite: str
) -> set[str]:
    found, ignored = parse_discovery(output)
    if not found:
        raise VerificationError(f"{suite} discovery is empty")
    missing = expected - found
    if missing:
        raise VerificationError(f"{suite} discovery is missing required tests: {sorted(missing)}")
    skipped = expected & ignored
    if skipped:
        raise VerificationError(f"{suite} marks required tests ignored: {sorted(skipped)}")
    return found


def parse_execution(output: str, expected: set[str], suite: str) -> tuple[int, int, int]:
    passed = failed = ignored = 0
    seen: set[str] = set()
    for line in output.splitlines():
        match = re.match(r"^test (.+) \.\.\. (ok|FAILED|ignored)(?: .*)?$", line.strip())
        if match:
            name, state = match.groups()
            seen.add(name)
            if state == "ok":
                passed += 1
            elif state == "ignored":
                ignored += 1
            else:
                failed += 1
    missing = expected - seen
    if missing:
        raise VerificationError(f"{suite} execution omitted required tests: {sorted(missing)}")
    if not passed or failed or ignored:
        raise VerificationError(
            f"{suite} execution must pass nonzero tests with no failures or skips; "
            f"passed={passed}, failed={failed}, ignored={ignored}"
        )
    return passed, failed, ignored


def self_test() -> str:
    expected = {"unit::one", "unit::two"}
    normal = "unit::one: test\nunit::two: test\n"
    assert guard_discovery(normal, expected, "unit") == expected
    negative_cases = [
        ("", expected, "empty discovery"),
        ("unit::one: test\n", expected, "missing test"),
        ("unit::one: test\nunit::two: ignored\n", expected, "ignored test"),
    ]
    for output, required, label in negative_cases:
        try:
            guard_discovery(output, required, "unit")
        except VerificationError:
            pass
        else:
            raise VerificationError(f"negative self-check failed to reject {label}")
    executed = "test unit::one ... ok\ntest unit::two ... ok\n"
    assert parse_execution(executed, expected, "unit") == (2, 0, 0)
    for output, label in [
        ("", "zero execution"),
        ("test unit::one ... ok\n", "missing executed test"),
        ("test unit::one ... ok\ntest unit::two ... ignored\n", "skipped execution"),
        ("test unit::one ... ok\ntest unit::two ... FAILED\n", "failed execution"),
    ]:
        try:
            parse_execution(output, expected, "unit")
        except VerificationError:
            pass
        else:
            raise VerificationError(f"negative self-check failed to reject {label}")
    with tempfile.TemporaryDirectory(prefix="relayforge-scratch-self-test-") as name:
        boundary = Path(name).resolve()
        nested = boundary / "project-verification"
        nested.mkdir()
        assert _owned_scratch(nested, boundary, ROOT) == nested
        for rejected in (boundary.parent / f"{boundary.name}-sibling", ROOT / "verification-output"):
            try:
                _owned_scratch(rejected, boundary, ROOT)
            except VerificationError:
                pass
            else:
                raise VerificationError("negative self-check accepted scratch outside its owned boundary")
    return (
        "discovery rejects empty/missing/ignored; execution rejects zero/missing/skipped/failed; "
        "scratch accepts nested and rejects sibling/source-tree paths"
    )


def _rust_commands(root: Path, env: dict[str, str], deadline: float, scratch: Path) -> None:
    version_checks = [
        ["rustc", "+1.99.1", "--version"],
        _cargo("--version"),
        ["rustfmt", "+1.99.1", "--version"],
        ["cargo", "+1.99.1", "clippy", "--version"],
        ["rustup", "target", "list", "--installed", "--toolchain", "1.99.1"],
    ]
    results = [
        _run(command, cwd=root, env=env, deadline=deadline, scratch=scratch)
        for command in version_checks
    ]
    rustc_version = results[0].stdout.strip()
    cargo_version = results[1].stdout.strip()
    if not rustc_version.startswith("rustc 1.99.1 ") or not cargo_version.startswith("cargo 1.99.1 "):
        raise VerificationError("rustc and cargo must both report the pinned 1.99.1 version")
    revision = re.search(r"\(([0-9a-f]{8})", rustc_version)
    if revision is None or any(
        revision.group(1) not in result.stdout
        for result in results[2:4]
    ):
        raise VerificationError("rustfmt and Clippy do not match the pinned rustc toolchain build")
    if TARGET not in results[4].stdout.split():
        raise VerificationError(f"required Rust target is not installed: {TARGET}")

    graph = _run(
        _cargo(
            "tree", "--manifest-path", str(MANIFEST), "--locked", "--offline",
            "--target", TARGET, "-i", "libsqlite3-sys",
        ),
        cwd=root,
        env=env,
        deadline=deadline,
        scratch=scratch,
    )
    if "rusqlite v0.40.0" not in graph.stdout or "libsqlite3-sys v0.38.0" not in graph.stdout:
        raise VerificationError("locked dependency graph does not match rusqlite 0.40.0/libsqlite3-sys 0.38.0")
    features = _run(
        _cargo(
            "tree", "--locked", "--offline", "--target", TARGET,
            "--manifest-path", str(MANIFEST), "--edges", "features", "-i", "libsqlite3-sys",
        ),
        cwd=root,
        env=env,
        deadline=deadline,
        scratch=scratch,
    )
    if 'feature "bundled"' not in features.stdout:
        raise VerificationError("locked libsqlite3-sys graph lacks bundled feature evidence")

    _run(
        _cargo("fmt", "--manifest-path", str(MANIFEST), "--", "--check"),
        cwd=root,
        env=env,
        deadline=deadline,
        scratch=scratch,
    )
    _run(
        _cargo(
            "clippy", "--manifest-path", str(MANIFEST), "--all-targets", "--target", TARGET,
            "--locked", "--offline", "--", "-D", "warnings",
        ),
        cwd=root,
        env=env,
        deadline=deadline,
        scratch=scratch,
    )
    _run(
        _cargo(
            "build", "--manifest-path", str(MANIFEST), "--all-targets", "--target", TARGET,
            "--locked", "--offline",
        ),
        cwd=root,
        env=env,
        deadline=deadline,
        scratch=scratch,
    )

    discovered = _run(
        _cargo(
            "test", "--manifest-path", str(MANIFEST), "--all-targets", "--target", TARGET,
            "--locked", "--offline", "--", "--list",
        ),
        cwd=root,
        env=env,
        deadline=deadline,
        scratch=scratch,
    )
    found, ignored = parse_discovery(discovered.stdout + discovered.stderr)
    expected = REQUIRED_TESTS["unit"] | REQUIRED_TESTS["integration"]
    if not found:
        raise VerificationError("unit/integration test discovery is empty")
    missing = expected - found
    if missing:
        raise VerificationError(f"required test IDs were not discovered: {sorted(missing)}")
    skipped = expected & ignored
    if skipped:
        raise VerificationError(f"required tests are marked ignored: {sorted(skipped)}")
    print(f"TEST DISCOVERY unit+integration discovered={len(found)} ignored={len(ignored)}", flush=True)

    doctest_list = _run(
        _cargo(
            "test", "--manifest-path", str(MANIFEST), "--doc", "--target", TARGET,
            "--locked", "--offline", "--", "--list",
        ),
        cwd=root,
        env=env,
        deadline=deadline,
        scratch=scratch,
    )
    doctests, ignored_doctests = parse_discovery(doctest_list.stdout + doctest_list.stderr)
    if not doctests or ignored_doctests:
        raise VerificationError(
            f"declared doctests must be nonempty and nonignored; found={len(doctests)}, "
            f"ignored={len(ignored_doctests)}"
        )

    executed = _run(
        _cargo(
            "test", "--manifest-path", str(MANIFEST), "--all-targets", "--target", TARGET,
            "--locked", "--offline",
        ),
        cwd=root,
        env=env,
        deadline=deadline,
        scratch=scratch,
    )
    execution_counts = parse_execution(executed.stdout + executed.stderr, found, "unit/integration")
    print(
        "TEST EXECUTION unit+integration "
        f"passed={execution_counts[0]} failed={execution_counts[1]} ignored={execution_counts[2]}",
        flush=True,
    )

    doctests_run = _run(
        _cargo(
            "test", "--manifest-path", str(MANIFEST), "--doc", "--target", TARGET,
            "--locked", "--offline",
        ),
        cwd=root,
        env=env,
        deadline=deadline,
        scratch=scratch,
    )
    doc_counts = parse_execution(doctests_run.stdout + doctests_run.stderr, doctests, "doctest")
    print(
        f"TEST EXECUTION doctest passed={doc_counts[0]} failed={doc_counts[1]} ignored={doc_counts[2]}",
        flush=True,
    )

    version = _run(
        _cargo(
            "run", "--manifest-path", str(MANIFEST), "--locked", "--offline", "--target", TARGET,
            "--bin", "relayforge", "--", "sqlite-version",
        ),
        cwd=root,
        env=env,
        deadline=deadline,
        scratch=scratch,
    )
    if version.stdout.strip().splitlines()[-1:] != ["3.53.1"]:
        raise VerificationError("runtime SQLite version is not exactly 3.53.1")
    print("SQLITE RUNTIME PASS: 3.53.1; locked libsqlite3-sys bundled feature present", flush=True)

    database = scratch / "runtime" / "canary.db"
    database.parent.mkdir(parents=True, exist_ok=True)
    common = [
        "run", "--manifest-path", str(MANIFEST), "--locked", "--offline", "--target", TARGET,
        "--bin", "relayforge", "--",
    ]

    def cli(name: str, *args: str, allow_failure: bool = False):
        result = _run(
            _cargo(*common, "canary", name, "--database", str(database), *args),
            cwd=root,
            env=env,
            deadline=deadline,
            scratch=scratch,
            allow_failure=allow_failure,
        )
        try:
            output = json.loads(result.stdout.splitlines()[-1]) if result.stdout.strip() else None
        except json.JSONDecodeError as exc:
            raise VerificationError(f"{name} did not return valid JSON") from exc
        return result, output

    _, seeded = cli("seed")
    if seeded is None or seeded["event_count"] != 2:
        raise VerificationError("seed did not write exactly the session and task records")
    crash, _ = cli("crash-after-intent", allow_failure=True)
    if crash.returncode == 0:
        raise VerificationError("crash-after-intent failpoint unexpectedly exited successfully")
    print(f"CRASH ASSERTION PASS: committed intent, process exited {crash.returncode}", flush=True)

    _, first = cli("reconcile", "--json")
    _, second = cli("reconcile", "--json")
    if not first or not second:
        raise VerificationError("reconcile did not return snapshots")
    for snapshot in (first, second):
        if (
            snapshot["state"] != "interrupted"
            or snapshot["result"] != "unknown"
            or snapshot["identity"] != "unknown"
            or snapshot["cost"] != "unknown"
        ):
            raise VerificationError("restart reconciliation did not preserve unknown interrupted outcome")
    if first["event_ids"] != second["event_ids"] or first["event_count"] != second["event_count"]:
        raise VerificationError("second reconciliation changed event IDs or row count")
    if first["event_count"] != 4:
        raise VerificationError("first reconciliation did not append exactly one interrupted event")
    print(
        f"REPLAY ASSERTION PASS: interrupted unknown; stable events={first['event_count']}; "
        "IDs identical on second reconcile",
        flush=True,
    )

    for command in ("record-failure", "verify", "accept"):
        cli(command)
    _, final = cli("reconcile", "--json")
    if not final or final["event_count"] != 7 or not final["complete"]:
        raise VerificationError("failure/verification/acceptance replay did not complete exactly once")
    if final["result"] != "failure" or not {
        "tool.failure", "task.verified", "task.accepted"
    }.issubset(final["event_ids"]):
        raise VerificationError("acceptance did not preserve the original failure history")
    print(
        f"ACCEPTANCE ASSERTION PASS: complete requires verification+acceptance; "
        f"failure retained; final events={final['event_count']}",
        flush=True,
    )


def _git(
    args: list[str],
    *,
    root: Path,
    env: dict[str, str],
    deadline: float,
    binary: bool = False,
) -> subprocess.CompletedProcess:
    remaining = deadline - time.monotonic()
    if remaining <= 0:
        raise VerificationError("900-second canary verification budget exhausted")
    try:
        result = subprocess.run(
            ["git", *args],
            cwd=root,
            env=env,
            shell=False,
            capture_output=True,
            check=False,
            timeout=remaining,
        )
    except subprocess.TimeoutExpired as exc:
        raise VerificationError(f"timed out: git {' '.join(args)}") from exc
    if result.returncode:
        error = result.stderr.decode("utf-8", "replace")
        raise VerificationError(f"git {' '.join(args)} failed: {error.strip()}")
    if binary:
        return result
    return subprocess.CompletedProcess(
        result.args,
        result.returncode,
        result.stdout.decode("utf-8", "replace"),
        result.stderr.decode("utf-8", "replace"),
    )


def _index_snapshot(root: Path, deadline: float) -> tuple[str, bytes | None]:
    env = os.environ.copy()
    git_dir = _git(
        ["rev-parse", "--absolute-git-dir"], root=root, env=env, deadline=deadline
    ).stdout.strip()
    index_path = Path(
        _git(["rev-parse", "--git-path", "index"], root=root, env=env, deadline=deadline)
        .stdout.strip()
    )
    if not index_path.is_absolute():
        index_path = root / index_path
    data = index_path.read_bytes() if index_path.is_file() else None
    head = _git(["rev-parse", "HEAD"], root=root, env=env, deadline=deadline).stdout.strip()
    digest = hashlib.sha256(data).hexdigest() if data is not None else "missing"
    return f"{git_dir}|{head}|{digest}", data


def _materialize(
    tree: str, repo: Path, destination: Path, env: dict[str, str], deadline: float
) -> None:
    archive = _git(
        ["archive", "--format=tar", tree], root=repo, env=env, deadline=deadline, binary=True
    ).stdout
    destination.mkdir(parents=True)
    with tarfile.open(fileobj=io.BytesIO(archive), mode="r:") as bundle:
        for member in bundle.getmembers():
            relative = PurePosixPath(member.name)
            if relative.is_absolute() or any(part in ("", ".", "..") for part in relative.parts):
                raise VerificationError("candidate archive contains an unsafe path")
            target = destination.joinpath(*relative.parts)
            if not target.resolve().is_relative_to(destination.resolve()):
                raise VerificationError("candidate archive path escapes owned scratch")
            if member.issym() or member.islnk():
                raise VerificationError("candidate archive links are unsupported by this canary")
            if member.isdir():
                target.mkdir(parents=True, exist_ok=True)
            elif member.isfile():
                target.parent.mkdir(parents=True, exist_ok=True)
                source = bundle.extractfile(member)
                if source is None:
                    raise VerificationError("candidate archive contains an unreadable file")
                target.write_bytes(source.read())
                target.chmod(member.mode & 0o777)


def _worktree_snapshot(root: Path, deadline: float) -> tuple[bytes, str]:
    env = os.environ.copy()
    status = _git(
        ["status", "--porcelain=v1", "--untracked-files=all", "-z"],
        root=root,
        env=env,
        deadline=deadline,
        binary=True,
    ).stdout
    diff = _git(
        ["diff", "--binary", "HEAD"], root=root, env=env, deadline=deadline, binary=True
    ).stdout
    digest = hashlib.sha256(diff)
    untracked = _git(
        ["ls-files", "--others", "--exclude-standard", "-z"],
        root=root,
        env=env,
        deadline=deadline,
        binary=True,
    ).stdout
    root = root.resolve()
    for raw_path in untracked.split(b"\0"):
        if not raw_path:
            continue
        path = root.joinpath(*Path(os.fsdecode(raw_path)).parts)
        if not path.resolve(strict=False).is_relative_to(root):
            raise VerificationError("untracked source escapes the repository root")
        digest.update(raw_path)
        digest.update(b"\0")
        if path.is_symlink():
            digest.update(os.readlink(path).encode("utf-8", "surrogateescape"))
        elif path.is_file():
            digest.update(path.read_bytes())
        else:
            raise VerificationError("untracked candidate entry is not a regular file")
    return status, digest.hexdigest()


def _owned_scratch(path: Path, boundary: Path, project_root: Path) -> Path:
    resolved = path.resolve()
    owned = boundary.resolve()
    root = project_root.resolve()
    if resolved == owned or not resolved.is_relative_to(owned) or resolved.is_relative_to(root):
        raise VerificationError("verification scratch is outside the canary-owned temporary tree")
    if not resolved.is_dir():
        raise VerificationError("verification scratch directory does not exist")
    return resolved


def _candidate_run(
    scratch: Path, *, execute_verifier: bool = True, report_blobs: bool = True
) -> None:
    repo = ROOT
    deadline = time.monotonic() + TIMEOUT_SECONDS
    temp_root = Path(tempfile.gettempdir()).resolve()
    scratch = scratch.resolve()
    if not scratch.is_relative_to(temp_root) or scratch == temp_root:
        raise VerificationError("scratch directory is outside the owned system temp root")
    before_source = _worktree_snapshot(repo, deadline)
    before_index, before_bytes = _index_snapshot(repo, deadline)
    before_head = _git(
        ["rev-parse", "HEAD"], root=repo, env=os.environ.copy(), deadline=deadline
    ).stdout.strip()

    source = scratch / "candidate-source"
    alternate_index = scratch / "candidate.index"
    candidate_objects = scratch / "candidate-objects"
    candidate_git_dir = scratch / "candidate.git"
    repo_env = os.environ.copy()
    for key in (
        "GIT_DIR",
        "GIT_WORK_TREE",
        "GIT_COMMON_DIR",
        "GIT_INDEX_FILE",
        "GIT_OBJECT_DIRECTORY",
        "GIT_ALTERNATE_OBJECT_DIRECTORIES",
    ):
        repo_env.pop(key, None)
    original_objects = Path(
        _git(
            ["rev-parse", "--path-format=absolute", "--git-path", "objects"],
            root=repo,
            env=repo_env,
            deadline=deadline,
        ).stdout.strip()
    ).resolve()
    object_format = _git(
        ["rev-parse", "--show-object-format"],
        root=repo,
        env=repo_env,
        deadline=deadline,
    ).stdout.strip()
    if object_format not in {"sha1", "sha256"}:
        raise VerificationError(f"unsupported Git object format: {object_format}")
    if not original_objects.is_dir():
        raise VerificationError("source Git object directory does not exist")

    candidate_objects.mkdir()
    build_env = repo_env.copy()
    build_env.update(
        {
            "GIT_INDEX_FILE": str(alternate_index),
            "GIT_OBJECT_DIRECTORY": str(candidate_objects),
            "GIT_ALTERNATE_OBJECT_DIRECTORIES": str(original_objects),
        }
    )
    _git(["read-tree", before_head], root=repo, env=build_env, deadline=deadline)
    _git(["add", "--all", "--", "."], root=repo, env=build_env, deadline=deadline)
    tree = _git(["write-tree"], root=repo, env=build_env, deadline=deadline).stdout.strip()
    entries = _git(
        ["ls-tree", "-r", "-z", tree],
        root=repo,
        env=build_env,
        deadline=deadline,
        binary=True,
    ).stdout
    manifest: list[tuple[str, str]] = []
    for record in entries.split(b"\0"):
        if not record:
            continue
        header, raw_path = record.split(b"\t", 1)
        oid = header.decode("ascii").split()[2]
        manifest.append((raw_path.decode("utf-8", "surrogateescape"), oid))
    if not manifest:
        raise VerificationError("candidate tree has no files")
    product_paths = (
        "Cargo.toml",
        "Cargo.lock",
        "rust-toolchain.toml",
        "08_pkg/relayforge-core/Cargo.toml",
        "08_pkg/relayforge-core/src/lib.rs",
        "08_pkg/relayforge-core/tests/storage_contract.rs",
        "07_app/relayforge-cli/Cargo.toml",
        "07_app/relayforge-cli/src/main.rs",
        "scripts/product_verify.py",
        "02_analysis/native-core-results.md",
        "02_analysis/canary-results.md",
        "08_pkg/canary/Cargo.lock",
        "08_pkg/canary/Cargo.toml",
        "08_pkg/canary/rust-toolchain.toml",
        "08_pkg/canary/src/lib.rs",
        "08_pkg/canary/src/main.rs",
        "08_pkg/canary/tests/storage_contract.rs",
        "scripts/canary_verify.py",
    )
    blob_ids = dict(manifest)
    missing = [path for path in product_paths if path not in blob_ids]
    if missing:
        raise VerificationError(f"candidate is missing slice product files: {missing}")
    snapshot_summary = "; ".join(f"{path}={blob_ids[path]}" for path in product_paths)
    print(f"CANDIDATE TREE {tree} files={len(manifest)}", flush=True)
    if report_blobs:
        for path, oid in manifest:
            print(f"CANDIDATE BLOB {oid} {path}", flush=True)

    _materialize(tree, repo, source, build_env, deadline)
    _git(
        [
            "init",
            "--quiet",
            "--bare",
            "--template=",
            f"--object-format={object_format}",
            str(candidate_git_dir),
        ],
        root=scratch,
        env=repo_env,
        deadline=deadline,
    )
    private_objects = candidate_git_dir / "objects" / "info"
    private_objects.mkdir(parents=True, exist_ok=True)
    alternate_paths = (candidate_objects.resolve(), original_objects)
    if any("\n" in path.as_posix() or "\r" in path.as_posix() for path in alternate_paths):
        raise VerificationError("Git object alternate path contains a line break")
    (private_objects / "alternates").write_text(
        "".join(f"{path.as_posix()}\n" for path in alternate_paths),
        encoding="utf-8",
        newline="\n",
    )
    _git(
        ["config", "--file", str(candidate_git_dir / "config"), "core.bare", "false"],
        root=scratch,
        env=repo_env,
        deadline=deadline,
    )
    candidate_env = repo_env.copy()
    candidate_env.update(
        {
            "GIT_DIR": str(candidate_git_dir),
            "GIT_WORK_TREE": str(source),
            "GIT_INDEX_FILE": str(candidate_git_dir / "index"),
            "GIT_AUTHOR_IDENT": _git(
                ["var", "GIT_AUTHOR_IDENT"], root=repo, env=repo_env, deadline=deadline
            ).stdout.rstrip("\r\n"),
            "GIT_COMMITTER_IDENT": _git(
                ["var", "GIT_COMMITTER_IDENT"], root=repo, env=repo_env, deadline=deadline
            ).stdout.rstrip("\r\n"),
        }
    )
    baseline_commit = _git(
        ["commit-tree", tree, "-p", before_head, "-m", "RelayForge isolated canary baseline"],
        root=source,
        env=candidate_env,
        deadline=deadline,
    ).stdout.strip()
    baseline_parents = _git(
        ["rev-list", "--parents", "-n", "1", baseline_commit],
        root=source,
        env=candidate_env,
        deadline=deadline,
    ).stdout.split()
    if baseline_parents != [baseline_commit, before_head]:
        raise VerificationError("private Git baseline does not preserve the original HEAD parent")
    _git(
        ["merge-base", "--is-ancestor", before_head, baseline_commit],
        root=source,
        env=candidate_env,
        deadline=deadline,
    )
    _git(["update-ref", "HEAD", baseline_commit], root=source, env=candidate_env, deadline=deadline)
    _git(["read-tree", tree], root=source, env=candidate_env, deadline=deadline)
    baseline_tree = _git(
        ["rev-parse", "HEAD^{tree}"], root=source, env=candidate_env, deadline=deadline
    ).stdout.strip()
    if baseline_tree != tree:
        raise VerificationError("private Git baseline does not match the candidate tree")

    candidate_env.update(
        {
            "RELAYFORGE_CANDIDATE_TREE": tree,
            "RELAYFORGE_CANARY_INNER": "1",
            "RELAYFORGE_CANARY_SCRATCH": str(scratch),
            "TEMP": str(scratch),
            "TMP": str(scratch),
            "VERIFICATION_SCRATCH": str(scratch),
            "CARGO_TARGET_DIR": str(scratch / "cargo-target"),
            "PYTHONDONTWRITEBYTECODE": "1",
        }
    )
    _run(
        [sys.executable, "-B", "scripts/ledger.py", "check"],
        cwd=source,
        env=candidate_env,
        deadline=deadline,
        scratch=scratch,
    )
    print("PRIVATE CANDIDATE LEDGER CHECK PASS: original Git ancestry is visible", flush=True)
    untracked = _git(
        ["ls-files", "--others", "--exclude-standard", "-z"],
        root=source,
        env=candidate_env,
        deadline=deadline,
        binary=True,
    )
    diff = _git(
        ["diff", "--exit-code"], root=source, env=candidate_env, deadline=deadline
    )
    cached = _git(
        ["diff", "--cached", "--exit-code", tree],
        root=source,
        env=candidate_env,
        deadline=deadline,
    )
    _git(["diff", "--check"], root=source, env=candidate_env, deadline=deadline)
    _git(["diff", "--cached", "--check"], root=source, env=candidate_env, deadline=deadline)
    if untracked.stdout or diff.stdout or cached.stdout:
        raise VerificationError("materialized candidate is not clean against its private baseline")
    print(
        f"CANDIDATE CLEAN PASS: tree={tree}; private baseline={baseline_commit}; "
        "scratch-only metadata/index/objects",
        flush=True,
    )
    if not execute_verifier:
        after_source = _worktree_snapshot(repo, deadline)
        after_index, after_bytes = _index_snapshot(repo, deadline)
        after_head = _git(
            ["rev-parse", "HEAD"], root=repo, env=os.environ.copy(), deadline=deadline
        ).stdout.strip()
        if before_source != after_source or before_index != after_index or before_bytes != after_bytes:
            raise VerificationError("candidate self-test changed the worktree or real Git index")
        if before_head != after_head:
            raise VerificationError("candidate self-test changed HEAD")
        print("CANDIDATE MATERIALIZATION SELF-TEST PASS: source/index/HEAD unchanged", flush=True)
        print(f"CANDIDATE SNAPSHOT SUMMARY: tree={tree}; blobs={snapshot_summary}", flush=True)
        return

    command = [sys.executable, "-B", "scripts/hermetic_verification.py"]
    result = _run(
        command,
        cwd=source,
        env=candidate_env,
        deadline=deadline,
        scratch=scratch,
    )
    if result.returncode:
        raise VerificationError(f"clean-candidate full verifier failed: {result.returncode}")

    after_source = _worktree_snapshot(repo, deadline)
    after_index, after_bytes = _index_snapshot(repo, deadline)
    after_head = _git(
        ["rev-parse", "HEAD"], root=repo, env=os.environ.copy(), deadline=deadline
    ).stdout.strip()
    if before_source != after_source or before_index != after_index or before_bytes != after_bytes:
        raise VerificationError("full verification changed the real worktree or Git index")
    if before_head != after_head:
        raise VerificationError("full verification changed HEAD")
    print("SOURCE/INDEX/HEAD ASSERTION PASS: unchanged by clean-candidate full verifier", flush=True)
    print(f"CANDIDATE SNAPSHOT SUMMARY: tree={tree}; blobs={snapshot_summary}", flush=True)
    product_blobs = "; ".join(
        f"{path}={blob_ids[path]}"
        for path in product_paths
        if path.startswith(("08_pkg/relayforge-core/", "07_app/relayforge-cli/", "scripts/product_verify.py"))
    )
    print(f"PRODUCT SNAPSHOT SUMMARY: {product_blobs}", flush=True)



def verify_inner() -> str:
    negative_check_summary = self_test()
    root = ROOT
    temp = Path(os.environ.get("VERIFICATION_SCRATCH", tempfile.gettempdir())).resolve()
    owned_scratch_value = os.environ.get("RELAYFORGE_CANARY_SCRATCH")
    if not owned_scratch_value:
        raise VerificationError("inner verifier is missing its owned scratch boundary")
    _owned_scratch(temp, Path(owned_scratch_value), root)
    cargo_target = temp / "cargo-target"
    env = os.environ.copy()
    env.update(
        {
            "CARGO_TARGET_DIR": str(cargo_target),
            "CARGO_NET_OFFLINE": "true",
        }
    )
    deadline = time.monotonic() + TIMEOUT_SECONDS
    _rust_commands(root, env, deadline, temp)
    return negative_check_summary


def main() -> int:
    try:
        if "--self-test" in sys.argv[1:]:
            summary = self_test()
            print(f"SELF-TEST PASS: {summary}", flush=True)
            with tempfile.TemporaryDirectory(prefix="relayforge-canary-self-test-") as name:
                _candidate_run(Path(name), execute_verifier=False, report_blobs=False)
            return 0
        if "--focused" in sys.argv[1:]:
            with tempfile.TemporaryDirectory(prefix="relayforge-canary-focused-") as name:
                scratch = Path(name).resolve()
                temp_root = Path(tempfile.gettempdir()).resolve()
                if not scratch.is_relative_to(temp_root) or scratch == temp_root:
                    raise VerificationError("focused scratch escaped the system temp root")
                verification_scratch = scratch / "verification"
                verification_scratch.mkdir()
                os.environ.update(
                    {
                        "TEMP": str(scratch),
                        "TMP": str(scratch),
                        "VERIFICATION_SCRATCH": str(verification_scratch),
                        "CARGO_TARGET_DIR": str(verification_scratch / "cargo-target"),
                        "RELAYFORGE_CANARY_SCRATCH": str(scratch),
                    }
                )
                summary = verify_inner()
            print(f"FOCUSED CANARY VERIFICATION PASS; {summary}", flush=True)
            return 0
        if os.environ.get(INNER_GUARD) == "1":
            summary = verify_inner()
            print(f"INNER CANARY VERIFICATION PASS; {summary}", flush=True)
            return 0
        with tempfile.TemporaryDirectory(prefix="relayforge-canary-") as name:
            owned = Path(name).resolve()
            _candidate_run(owned)
        print("CLEAN-CANDIDATE FULL VERIFICATION PASS", flush=True)
        return 0
    except (OSError, ValueError, VerificationError, subprocess.SubprocessError) as exc:
        print(f"canary verification failed: {exc}", file=sys.stderr)
        return 1


if __name__ == "__main__":
    raise SystemExit(main())
