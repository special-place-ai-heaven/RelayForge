"""Hermetic product checks for the native RelayForge core and CLI."""

from __future__ import annotations

import os
import subprocess
import sys
import tempfile
import time
from pathlib import Path

import canary_verify as canary

ROOT = Path(__file__).resolve().parents[1]
TARGET = "x86_64-pc-windows-msvc"
MANIFEST = ROOT / "Cargo.toml"
REQUIRED = {
    "tests::unknown_fields_and_schema_versions_are_rejected",
    "tests::event_size_and_depth_are_bounded",
    "tests::session_open_is_a_durable_owner_claim",
    "tests::unknown_identity_and_cost_are_explicit_values",
    "acceptance_requires_latest_success_and_current_passing_checks",
    "acceptance_rejects_unknown_and_not_run_after_success",
    "actual_cli_fresh_processes_init_append_show_reconcile_accept",
    "committed_intent_survives_abrupt_child_exit_and_precommit_does_not",
    "explicit_recovery_protects_active_owner_and_reconcile_is_repeatable",
    "identical_replay_is_noop_and_conflicting_duplicate_writes_nothing",
    "immutable_rows_reject_mutation_and_sources_can_reuse_event_ids",
    "parent_scope_mismatch_and_failed_append_are_atomic",
    "synthetic_sensitive_input_is_rejected_without_echo_or_storage",
    "two_process_claim_race_has_exactly_one_owner",
    "repair_attempts_preserve_predecessors_and_observed_identity",
}


def cargo(*args: str) -> list[str]:
    return ["cargo", "+1.99.1", *args]


def run_product(root: Path, scratch: Path) -> None:
    root = root.resolve()
    scratch = scratch.resolve()
    if scratch == root or root in scratch.parents:
        raise canary.VerificationError("product verification scratch must be outside source tree")
    env = os.environ.copy()
    env.update({"CARGO_TARGET_DIR": str(scratch / "cargo-target"), "CARGO_NET_OFFLINE": "true", "CARGO_BUILD_JOBS": "2"})
    env["RELAYFORGE_CLI_PATH"] = str(Path(env["CARGO_TARGET_DIR"]) / TARGET / "debug" / "relayforge.exe")
    deadline = time.monotonic() + canary.TIMEOUT_SECONDS
    for command in (
        ["rustc", "+1.99.1", "--version"],
        cargo("--version"),
        ["rustfmt", "+1.99.1", "--version"],
        cargo("clippy", "--version"),
    ):
        canary._run(command, cwd=root, env=env, deadline=deadline, scratch=scratch)

    graph = canary._run(cargo("tree", "--manifest-path", str(root / "Cargo.toml"), "--workspace", "--locked", "--offline", "--target", TARGET), cwd=root, env=env, deadline=deadline, scratch=scratch)
    if "rusqlite v0.40.0" not in graph.stdout or "libsqlite3-sys v0.38.0" not in graph.stdout:
        raise canary.VerificationError("product dependency graph does not lock rusqlite 0.40.0 and libsqlite3-sys 0.38.0")
    features = canary._run(cargo("tree", "--manifest-path", str(root / "Cargo.toml"), "--workspace", "--locked", "--offline", "--target", TARGET, "--edges", "features", "-i", "libsqlite3-sys"), cwd=root, env=env, deadline=deadline, scratch=scratch)
    if 'feature "bundled"' not in features.stdout:
        raise canary.VerificationError("product SQLite dependency does not enable bundled")
    canary._run(cargo("fmt", "--manifest-path", str(root / "Cargo.toml"), "--all", "--", "--check"), cwd=root, env=env, deadline=deadline, scratch=scratch)
    canary._run(cargo("clippy", "--manifest-path", str(root / "Cargo.toml"), "--workspace", "--all-targets", "--target", TARGET, "--locked", "--offline", "--", "-D", "warnings"), cwd=root, env=env, deadline=deadline, scratch=scratch)
    canary._run(cargo("build", "--manifest-path", str(root / "Cargo.toml"), "--workspace", "--all-targets", "--target", TARGET, "--locked", "--offline"), cwd=root, env=env, deadline=deadline, scratch=scratch)
    sqlite = canary._run(cargo("run", "--manifest-path", str(root / "Cargo.toml"), "--locked", "--offline", "--target", TARGET, "--bin", "relayforge", "--", "sqlite-version"), cwd=root, env=env, deadline=deadline, scratch=scratch)
    if '"sqlite_version":"3.53.1"' not in sqlite.stdout.replace(" ", ""):
        raise canary.VerificationError("bundled runtime does not report SQLite 3.53.1")

    discovered = canary._run(cargo("test", "--manifest-path", str(root / "Cargo.toml"), "--workspace", "--all-targets", "--target", TARGET, "--locked", "--offline", "--", "--list"), cwd=root, env=env, deadline=deadline, scratch=scratch)
    found = canary.guard_discovery(discovered.stdout + discovered.stderr, REQUIRED, "product")
    print(f"PRODUCT TEST DISCOVERY discovered={len(found)} required={len(REQUIRED)} ignored=0", flush=True)
    doctest_list = canary._run(cargo("test", "--manifest-path", str(root / "Cargo.toml"), "--workspace", "--doc", "--target", TARGET, "--locked", "--offline", "--", "--list"), cwd=root, env=env, deadline=deadline, scratch=scratch)
    doctests = canary.guard_discovery(doctest_list.stdout + doctest_list.stderr, set(), "product doctest")
    if not doctests:
        raise canary.VerificationError("product doctest discovery is empty")

    executed = canary._run(cargo("test", "--manifest-path", str(root / "Cargo.toml"), "--workspace", "--all-targets", "--target", TARGET, "--locked", "--offline", "--", "--nocapture", "--test-threads=1"), cwd=root, env=env, deadline=deadline, scratch=scratch)
    counts = canary.parse_execution(executed.stdout + executed.stderr, found, "product")
    print(f"PRODUCT TEST EXECUTION discovered={len(found)} passed={counts[0]} failed={counts[1]} ignored={counts[2]}", flush=True)
    docs = canary._run(cargo("test", "--manifest-path", str(root / "Cargo.toml"), "--workspace", "--doc", "--target", TARGET, "--locked", "--offline"), cwd=root, env=env, deadline=deadline, scratch=scratch)
    doc_counts = canary.parse_execution(docs.stdout + docs.stderr, doctests, "product doctest")
    print(f"PRODUCT DOCTEST EXECUTION discovered={len(doctests)} passed={doc_counts[0]} failed={doc_counts[1]} ignored={doc_counts[2]}", flush=True)


def main() -> int:
    try:
        if "--focused" in sys.argv[1:]:
            with tempfile.TemporaryDirectory(prefix="relayforge-product-focused-") as name:
                scratch = Path(name).resolve()
                if not scratch.is_relative_to(Path(tempfile.gettempdir()).resolve()) or scratch == Path(tempfile.gettempdir()).resolve():
                    raise canary.VerificationError("focused scratch escaped system temp")
                run_product(ROOT, scratch)
            print("FOCUSED PRODUCT VERIFICATION PASS", flush=True)
            return 0
        if os.environ.get(canary.INNER_GUARD) == "1":
            canary.verify_inner()
            scratch = Path(os.environ["RELAYFORGE_CANARY_SCRATCH"]).resolve()
            product_scratch = scratch / "product"
            product_scratch.mkdir()
            canary._owned_scratch(product_scratch, scratch, ROOT)
            run_product(ROOT, product_scratch)
            print("INNER CANARY AND PRODUCT VERIFICATION PASS", flush=True)
            return 0
        with tempfile.TemporaryDirectory(prefix="relayforge-product-full-") as name:
            canary._candidate_run(Path(name).resolve(), report_blobs=False)
        print("CLEAN-CANDIDATE PRODUCT VERIFICATION PASS", flush=True)
        return 0
    except (OSError, ValueError, canary.VerificationError, subprocess.SubprocessError) as exc:
        print(f"product verification failed: {exc}", file=sys.stderr)
        return 1


if __name__ == "__main__":
    raise SystemExit(main())
