"""Deterministic release tests; no network calls, tokens, or third-party modules."""

from contextlib import contextmanager
import hashlib
import json
import os
from pathlib import Path
import subprocess
import sys
import tempfile
import unittest
from unittest.mock import patch

import audit
import release


@contextmanager
def directory(path):
    previous = Path.cwd()
    os.chdir(path)
    try:
        yield
    finally:
        os.chdir(previous)


class VersionTests(unittest.TestCase):
    def test_bumps(self):
        for kind, expected in [("patch", "1.2.4"), ("minor", "1.3.0"), ("major", "2.0.0")]:
            self.assertEqual(release.next_version("1.2.3", kind), expected)
        self.assertEqual(release.next_version("0.1.0", "current"), "0.1.0")
        self.assertEqual(release.next_version("0.1.0", "manual", "1.0.0-rc.1"), "1.0.0-rc.1")
        self.assertEqual(release.next_version("0.2.0", "resume", "0.1.0"), "0.1.0")

    def test_semver_precedence(self):
        values = ["1.0.0-alpha", "1.0.0-alpha.1", "1.0.0-alpha.beta", "1.0.0-beta",
                  "1.0.0-beta.2", "1.0.0-beta.11", "1.0.0-rc.1", "1.0.0"]
        self.assertEqual(sorted(reversed(values), key=release.semver), values)
        self.assertEqual(release.semver("1.0.0+build.1"), release.semver("1.0.0+build.2"))

    def test_reject_invalid_versions(self):
        for value in ["", "v1.2.3", "01.2.3", "1.02.3", "1.2.03", "1.2", "1.2.3.4",
                      "1.2.3-01", "1.2.3-a..b", "1.2.3+", "1.2.3\n", " 1.2.3",
                      "1.2.3;echo bad", "1.2.3-雪", f"{2**64}.0.0"]:
            with self.subTest(value=value), self.assertRaises(RuntimeError):
                release.semver(value)

    def test_reject_rollback_equal_or_wrong_input(self):
        for kind, value in [("manual", "1.2.3"), ("manual", "1.2.3+metadata"),
                            ("manual", "1.2.2"), ("manual", "1.2.3-rc.1"),
                            ("manual", ""), ("resume", ""), ("patch", "1.3.0")]:
            with self.subTest(kind=kind, value=value), self.assertRaises(RuntimeError):
                release.next_version("1.2.3", kind, value)

    def test_metadata_edit_is_precise(self):
        with tempfile.TemporaryDirectory() as temporary, directory(temporary):
            Path("Cargo.toml").write_text('[package]\nname = "zclap"\nversion = "0.1.0"\n\n'
                                        '[package.metadata.other]\nversion = "9.9.9"\n')
            lock = 'version = 3\n\n[[package]]\nname = "zclap"\nversion = "0.1.0"\n'
            Path("Cargo.lock").write_text(lock)
            Path("comparison").mkdir()
            Path("comparison/Cargo.lock").write_text(
                lock + '\n[[package]]\nname = "competitor"\nversion = "0.1.0"\n')
            Path("README.md").write_text('zclap = "0.1.0"\n'
                                       'zclap = { version = "0.1.0", default-features = false }\n'
                                       'Historical benchmark: zclap 0.1.0\n')
            release.set_version("0.2.0")
            self.assertEqual(release.package_version(), "0.2.0")
            self.assertIn('version = "9.9.9"', Path("Cargo.toml").read_text())
            self.assertIn('name = "competitor"\nversion = "0.1.0"', Path("comparison/Cargo.lock").read_text())
            self.assertIn('version = "0.2.0", default-features', Path("README.md").read_text())
            self.assertIn('Historical benchmark: zclap 0.1.0', Path("README.md").read_text())
            before = Path("Cargo.toml").read_text()
            release.set_version("0.2.0")
            self.assertEqual(Path("Cargo.toml").read_text(), before)

    def test_missing_secret_fails_explicitly(self):
        with patch.dict(os.environ, {"CARGO_REGISTRY_TOKEN": ""}):
            with self.assertRaisesRegex(RuntimeError, "secrets.CARGO_REGISTRY_TOKEN"):
                release.require_token()

    def test_request_schema_accepts_supported_modes(self):
        with tempfile.TemporaryDirectory() as temporary:
            path = Path(temporary) / "request.json"
            for kind in release.RELEASE_KINDS:
                version = "1.2.3" if kind in {"manual", "resume"} else ""
                path.write_text(json.dumps({"bump": kind, "version": version}))
                self.assertEqual(release.request_settings(path), (kind, version))

    def test_request_schema_rejects_unknown_duplicate_or_nonstring_fields(self):
        invalid = ["[]", "{}", '{"bump":"current"}',
                   '{"bump":"current","version":"","extra":true}',
                   '{"bump":"current","bump":"patch","version":""}',
                   '{"bump":"unknown","version":""}',
                   '{"bump":true,"version":""}',
                   '{"bump":"manual","version":123}', "not JSON"]
        with tempfile.TemporaryDirectory() as temporary:
            path = Path(temporary) / "request.json"
            for text in invalid:
                path.write_text(text)
                with self.subTest(text=text), self.assertRaises((RuntimeError, ValueError)):
                    release.request_settings(path)

    def test_request_cli_passes_data_to_the_existing_plan(self):
        with tempfile.TemporaryDirectory() as temporary:
            path = Path(temporary) / "request.json"
            path.write_text('{"bump":"current","version":""}')
            with patch.object(sys, "argv", ["release.py", "plan", "--request", str(path)]), \
                    patch.object(release, "plan") as plan:
                release.main()
            plan.assert_called_once_with("current", "")
            path.write_text('{"bump":"manual","version":"$(touch injected)"}')
            with self.assertRaises(RuntimeError):
                release.next_version("0.1.0", *release.request_settings(path))
            self.assertFalse((Path(temporary) / "injected").exists())

    def test_dependency_audit_covers_all_tables(self):
        for table in ["dependencies", "dev-dependencies", "build-dependencies"]:
            for value in [{table: {"a": "1"}}, {"target": {"cfg(windows)": {table: {"a": "1"}}}}]:
                with self.assertRaises(AssertionError):
                    audit.check_manifest(value)
        audit.check_manifest({"dependencies": {}, "features": {"default": ["std"], "std": []}})

    def test_registry_checks_do_not_adopt_foreign_or_yanked_versions(self):
        for data in [{"checksum": "wrong"}, {"checksum": "correct", "yanked": True}]:
            with patch.object(release, "api_version", return_value=data):
                with self.assertRaises(RuntimeError):
                    release.verify_registry("0.1.0", "correct")
        with patch.object(release, "api_version", return_value=None):
            self.assertFalse(release.verify_registry("0.1.0", "correct"))


class TransactionTests(unittest.TestCase):
    """Exercise real Git pushes and interrupted retries against a local bare remote."""

    def setUp(self):
        self.temporary = tempfile.TemporaryDirectory()
        self.base = Path(self.temporary.name)
        self.previous = Path.cwd()
        self.repo = self.base / "repo"
        self.remote = self.base / "remote.git"
        subprocess.run(["git", "init", "--bare", "--quiet", str(self.remote)], check=True)
        subprocess.run(["git", "init", "-b", "main", "--quiet", str(self.repo)], check=True)
        os.chdir(self.repo)
        release.git("config", "user.name", "Release Test")
        release.git("config", "user.email", "test@example.invalid")
        Path("Cargo.toml").write_text('[package]\nname = "zclap"\nversion = "0.1.0"\n')
        Path("Cargo.lock").write_text('version = 3\n\n[[package]]\nname = "zclap"\nversion = "0.1.0"\n')
        Path("README.md").write_text('zclap = "0.1.0"\n')
        Path(".gitignore").write_text("/target/\n")
        release.git("add", ".")
        release.git("commit", "--quiet", "-m", "initial")
        self.source = release.git("rev-parse", "HEAD")
        release.git("remote", "add", "origin", str(self.remote))
        release.git("push", "--quiet", "-u", "origin", "main")
        self.registry = None
        self.has_release = False
        self.uploads = 0
        self.fail_release = False
        self.original_run = release.run
        self.environment = patch.dict(os.environ, {
            "CARGO_REGISTRY_TOKEN": "test-placeholder", "GITHUB_RUN_ID": "123",
            "GITHUB_REF": "refs/heads/main", "GITHUB_REPOSITORY": "example/zclap",
        })
        self.environment.start()
        self.runner = patch.object(release, "run", side_effect=self.fake_run)
        self.runner.start()
        self.api = patch.object(release, "api_version", side_effect=lambda _: self.registry)
        self.api.start()

    def tearDown(self):
        self.api.stop()
        self.runner.stop()
        self.environment.stop()
        os.chdir(self.previous)
        self.temporary.cleanup()

    def fake_run(self, *args, capture=True, check=True):
        if args[0] == "git":
            return self.original_run(*args, capture=capture, check=check)
        if args[:2] == ("cargo", "package"):
            version = release.package_version()
            package = Path(f"target/package/zclap-{version}.crate")
            package.parent.mkdir(parents=True, exist_ok=True)
            package.write_bytes((release.git("rev-parse", "HEAD") + version).encode())
            return None
        if args[:2] == ("cargo", "metadata"):
            return json.dumps({"target_directory": str(self.repo / "target")})
        if args[:2] == ("cargo", "publish"):
            if "--dry-run" not in args:
                self.uploads += 1
                version = release.package_version()
                package = Path(f"target/package/zclap-{version}.crate")
                self.registry = {"checksum": hashlib.sha256(package.read_bytes()).hexdigest(), "yanked": False}
            return subprocess.CompletedProcess(args, 0) if not check else None
        if args[:2] == ("python", "scripts/audit.py"):
            return None
        if args[:2] == ("gh", "api"):
            return "main"
        if args[:3] == ("gh", "release", "view"):
            if check:
                return f"https://github.com/example/zclap/releases/tag/{args[3]}"
            data = {"tagName": args[3], "targetCommitish": "main", "isDraft": False}
            return subprocess.CompletedProcess(args, 0 if self.has_release else 1, stdout=json.dumps(data))
        if args[:3] == ("gh", "release", "create"):
            if self.fail_release:
                raise RuntimeError("simulated GitHub outage")
            self.has_release = True
            return None
        if args[:3] == ("gh", "release", "upload"):
            return None
        self.fail(f"unexpected external command: {args}")

    def test_first_current_release_reuses_the_validated_commit(self):
        with patch.object(release, "write_output") as output:
            release.plan("current", "")
        output.assert_called_once_with({"version": "0.1.0", "source": self.source,
                                       "tag": "v0.1.0", "kind": "current"})
        self.assertFalse(release.tag_exists("v0.1.0"))
        release.publish(self.source, "0.1.0", "current")
        self.assertEqual(release.git("rev-parse", "HEAD"), self.source)
        self.assertEqual(release.git("rev-parse", "v0.1.0^{}"), self.source)
        self.assertEqual(self.uploads, 1)

    def test_plan_rejects_duplicate_dispatch_and_allows_explicit_resume(self):
        release.publish(self.source, "0.1.1", "patch")
        tagged = release.git("rev-parse", "HEAD")
        release.git("checkout", "--detach", self.source)
        with patch.dict(os.environ, {"GITHUB_RUN_ID": "124"}):
            with self.assertRaisesRegex(RuntimeError, "duplicate version/tag"):
                release.plan("patch", "")
            with patch.object(release, "write_output") as output:
                release.plan("resume", "0.1.1")
            output.assert_called_once_with({"version": "0.1.1", "source": tagged,
                                           "tag": "v0.1.1", "kind": "resume"})

    def test_committed_request_retains_secret_branch_and_source_gates(self):
        Path(".github").mkdir()
        path = Path(".github/release-request.json")
        path.write_text('{"bump":"current","version":""}')
        release.git("add", str(path))
        release.git("commit", "--quiet", "-m", "request first release")
        release.git("push", "origin", "main")
        source = release.git("rev-parse", "HEAD")
        settings = release.request_settings(path)
        with patch.dict(os.environ, {"CARGO_REGISTRY_TOKEN": ""}):
            with self.assertRaisesRegex(RuntimeError, "Missing secrets.CARGO_REGISTRY_TOKEN"):
                release.plan(*settings)
        with patch.dict(os.environ, {"GITHUB_REF": "refs/heads/feature"}):
            with self.assertRaisesRegex(RuntimeError, "default branch"):
                release.plan(*settings)
        with patch.object(release, "write_output") as output:
            release.plan(*settings)
        output.assert_called_once_with({"version": "0.1.0", "source": source,
                                       "tag": "v0.1.0", "kind": "current"})
        self.assertEqual(release.git("status", "--porcelain"), "")
        self.assertEqual(release.git("rev-parse", "origin/main"), source)
        self.assertFalse(release.tag_exists("v0.1.0"))
        self.assertEqual(self.uploads, 0)

    def test_complete_release_updates_version_tag_registry_and_release(self):
        release.publish(self.source, "0.1.1", "patch")
        tagged = release.git("rev-parse", "v0.1.1^{}")
        self.assertNotEqual(tagged, self.source)
        self.assertEqual(release.git("rev-parse", "origin/main"), tagged)
        self.assertEqual(release.package_version(), "0.1.1")
        self.assertEqual(self.uploads, 1)
        self.assertTrue(self.has_release)
        self.assertEqual(json.loads(Path("target/release-state.json").read_text())["stage"], "complete")

    def test_resume_after_registry_success_does_not_publish_twice(self):
        self.fail_release = True
        with self.assertRaisesRegex(RuntimeError, "simulated GitHub outage"):
            release.publish(self.source, "0.1.1", "patch")
        tagged = release.git("rev-parse", "HEAD")
        self.assertEqual(self.uploads, 1)
        self.fail_release = False
        with patch.dict(os.environ, {"GITHUB_RUN_ID": "124"}):
            release.publish(tagged, "0.1.1", "resume")
        self.assertEqual(self.uploads, 1)
        self.assertEqual(release.git("rev-parse", "HEAD"), tagged)
        self.assertTrue(self.has_release)

    def test_rerun_same_dispatch_reuses_original_source(self):
        self.fail_release = True
        with self.assertRaises(RuntimeError):
            release.publish(self.source, "0.1.1", "patch")
        tagged = release.git("rev-parse", "HEAD")
        release.git("checkout", "--detach", self.source)
        self.fail_release = False
        release.publish(self.source, "0.1.1", "patch")
        self.assertEqual(release.git("rev-parse", "HEAD"), tagged)
        self.assertEqual(self.uploads, 1)

    def test_abort_if_branch_changed_during_validation(self):
        Path("README.md").write_text("new work\n")
        release.git("commit", "-am", "concurrent work")
        release.git("push", "origin", "main")
        release.git("checkout", "--detach", self.source)
        with self.assertRaisesRegex(RuntimeError, "advanced during checks"):
            release.publish(self.source, "0.1.1", "patch")
        self.assertFalse(release.tag_exists("v0.1.1"))
        self.assertEqual(self.uploads, 0)

    def test_no_changes_when_secret_is_missing(self):
        with patch.dict(os.environ, {"CARGO_REGISTRY_TOKEN": ""}):
            with self.assertRaisesRegex(RuntimeError, "Missing secrets"):
                release.publish(self.source, "0.1.1", "patch")
        self.assertEqual(release.git("status", "--porcelain"), "")
        self.assertEqual(release.package_version(), "0.1.0")


if __name__ == "__main__":
    unittest.main()
