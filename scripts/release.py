#!/usr/bin/env python3
"""Release transaction: validate, commit/tag atomically, publish, then verify.

The Git tag records the source commit and workflow run. A failed upload can be
resumed without making another version or moving an existing tag. No credentials
are accepted as command arguments or written into diagnostics.
"""

import argparse
import hashlib
import json
import os
from pathlib import Path
import re
import subprocess
import time
import tomllib
import urllib.error
import urllib.request


SEMVER = re.compile(
    r"(0|[1-9][0-9]*)\.(0|[1-9][0-9]*)\.(0|[1-9][0-9]*)"
    r"(?:-([0-9A-Za-z-]+(?:\.[0-9A-Za-z-]+)*))?"
    r"(?:\+([0-9A-Za-z-]+(?:\.[0-9A-Za-z-]+)*))?\Z"
)
METADATA_FILES = ("Cargo.toml", "Cargo.lock", "comparison/Cargo.lock", "README.md")
RELEASE_KINDS = {"patch", "minor", "major", "manual", "current", "resume"}


def require(condition, message):
    if not condition:
        raise RuntimeError(message)


def semver(value):
    match = SEMVER.fullmatch(value)
    require(match is not None, f"invalid SemVer: {value!r} (use X.Y.Z, no v prefix)")
    numbers = tuple(int(n) for n in match.groups()[:3])
    require(all(n < 2**64 for n in numbers), "SemVer number exceeds u64")
    prerelease = match.group(4)
    parts = prerelease.split(".") if prerelease else []
    require(all(not (p.isdigit() and len(p) > 1 and p[0] == "0") for p in parts),
            "numeric prerelease identifiers must not have leading zeroes")
    order = tuple((0, int(p)) if p.isdigit() else (1, p) for p in parts)
    return (*numbers, not bool(prerelease), order)


def next_version(current, kind, requested=""):
    old = semver(current)
    require(not requested or kind in {"manual", "resume"},
            "version input is only accepted with manual or resume")
    if kind in {"manual", "resume"}:
        require(bool(requested), f"{kind} requires a full version")
        candidate = requested
    elif kind == "current":
        candidate = current
    else:
        require(kind in {"patch", "minor", "major"}, f"unknown bump: {kind}")
        major, minor, patch = old[:3]
        candidate = {"patch": f"{major}.{minor}.{patch + 1}",
                     "minor": f"{major}.{minor + 1}.0",
                     "major": f"{major + 1}.0.0"}[kind]
    new = semver(candidate)
    if kind not in {"current", "resume"}:
        require(new > old, f"version must advance: {current} -> {candidate}")
    return candidate


def request_settings(path):
    """Read an explicit release request as data, never as shell arguments."""
    def unique_fields(pairs):
        data = dict(pairs)
        require(len(data) == len(pairs), "release request contains duplicate fields")
        return data

    data = json.loads(Path(path).read_text(encoding="utf-8"), object_pairs_hook=unique_fields)
    require(isinstance(data, dict) and set(data) == {"bump", "version"},
            "release request must contain exactly the fields bump and version")
    require(isinstance(data["bump"], str) and data["bump"] in RELEASE_KINDS,
            "release request bump must be patch, minor, major, manual, current, or resume")
    require(isinstance(data["version"], str), "release request version must be a string")
    return data["bump"], data["version"]


def run(*args, capture=True, check=True):
    result = subprocess.run(args, check=False, text=True, encoding="utf-8",
                            stdout=subprocess.PIPE if capture else None)
    if check:
        require(result.returncode == 0, f"command failed ({result.returncode}): {' '.join(args)}")
    return result.stdout.strip() if capture and check else result


def git(*args):
    return run("git", *args)


def package_version(ref=None):
    content = git("show", f"{ref}:Cargo.toml") if ref else Path("Cargo.toml").read_text(encoding="utf-8")
    return tomllib.loads(content)["package"]["version"]


def set_version(version):
    semver(version)
    old = package_version()
    text = Path("Cargo.toml").read_text(encoding="utf-8")
    section = re.search(r"(?ms)^\[package\]\s*\n(.*?)(?=^\[|\Z)", text)
    require(section is not None, "Cargo.toml lacks a [package] section")
    body, count = re.subn(r'(?m)^version\s*=\s*"[^"]+"', f'version = "{version}"', section[1])
    require(count == 1, "expected one literal package.version in Cargo.toml")
    Path("Cargo.toml").write_text(text[:section.start(1)] + body + text[section.end(1):], encoding="utf-8")
    for name in ("Cargo.lock", "comparison/Cargo.lock"):
        path = Path(name)
        if not path.exists():
            continue
        text = path.read_text(encoding="utf-8")
        blocks = text.split("[[package]]")
        for index in range(1, len(blocks)):
            if re.search(r'(?m)^name = "zclap"$', blocks[index]):
                blocks[index], count = re.subn(r'(?m)^version = "[^"]+"$',
                                               f'version = "{version}"', blocks[index], count=1)
                require(count == 1, f"missing zclap version in {name}")
        path.write_text("[[package]]".join(blocks), encoding="utf-8")
    readme = Path("README.md")
    if readme.exists():
        text = readme.read_text(encoding="utf-8")
        # Update installation snippets only; historical benchmark versions stay intact.
        text = re.sub(r'(?m)(^zclap\s*=\s*(?:\{\s*version\s*=\s*)?")' +
                      re.escape(old) + r'(")', lambda m: m[1] + version + m[2], text)
        readme.write_text(text, encoding="utf-8")


def api_version(version):
    request = urllib.request.Request(f"https://crates.io/api/v1/crates/zclap/{version}",
                                     headers={"User-Agent": "zclap-release (github.com/LIghtJUNction/zclap)"})
    try:
        with urllib.request.urlopen(request, timeout=30) as response:
            return json.load(response)["version"]
    except urllib.error.HTTPError as error:
        if error.code == 404:
            return None
        raise RuntimeError(f"crates.io version check failed with HTTP {error.code}") from error


def default_branch():
    repo = os.environ.get("GITHUB_REPOSITORY", "LIghtJUNction/zclap")
    return run("gh", "api", f"repos/{repo}", "--jq", ".default_branch")


def tag_exists(tag):
    return run("git", "rev-parse", "--verify", "--quiet", f"refs/tags/{tag}", check=False).returncode == 0


def tag_message(tag):
    require(git("cat-file", "-t", f"refs/tags/{tag}") == "tag", "release tag must be annotated")
    return git("for-each-ref", "--format=%(contents)", f"refs/tags/{tag}")


def require_token():
    require(bool(os.environ.get("CARGO_REGISTRY_TOKEN", "").strip()),
            "Missing secrets.CARGO_REGISTRY_TOKEN. Add the crates.io publish token in "
            "Settings > Secrets and variables > Actions; local environment variables are not inherited.")


def write_output(data):
    print(json.dumps(data, indent=2))
    if path := os.environ.get("GITHUB_OUTPUT"):
        with open(path, "a", encoding="utf-8") as output:
            for key, value in data.items():
                output.write(f"{key}={value}\n")


def plan(kind, requested):
    require_token()
    branch = default_branch()
    require(os.environ.get("GITHUB_REF") == f"refs/heads/{branch}",
            f"dispatch releases from the default branch ({branch})")
    source = git("rev-parse", "HEAD")
    git("fetch", "origin", branch, "--tags")
    version = next_version(package_version(), kind, requested)
    tag = f"v{version}"
    if kind == "resume":
        require(tag_exists(tag), f"cannot resume: {tag} does not exist")
        message = tag_message(tag)
        require("Source-Commit: " in message and "Release-Run: " in message,
                "tag lacks release provenance; refusing to adopt an unrelated tag")
        source = git("rev-parse", f"{tag}^{{}}")
        require(package_version(source) == version, "tag and Cargo.toml version disagree")
        run("git", "merge-base", "--is-ancestor", source, f"origin/{branch}")
    elif tag_exists(tag):
        message = tag_message(tag)
        require(f"Release-Run: {os.environ.get('GITHUB_RUN_ID')}\n" in message + "\n"
                and f"Source-Commit: {source}\n" in message + "\n",
                f"duplicate version/tag {tag}; use resume to finish its existing release")
    else:
        require(git("rev-parse", f"origin/{branch}") == source,
                "default branch advanced since dispatch; start a new release run")
        require(api_version(version) is None, f"zclap {version} already exists on crates.io")
        if kind == "current":
            require(not git("tag", "--list", "v*"), "current is only for the first release; choose a bump")
    write_output({"version": version, "source": source, "tag": tag, "kind": kind})


def state(**values):
    path = Path("target/release-state.json")
    path.parent.mkdir(exist_ok=True)
    data = json.loads(path.read_text(encoding="utf-8")) if path.exists() else {}
    data.update(values)
    path.write_text(json.dumps(data, indent=2) + "\n", encoding="utf-8")


def verify_registry(version, checksum, attempts=1):
    for attempt in range(attempts):
        data = api_version(version)
        if data is not None:
            require(not data.get("yanked"), "published version is yanked; refusing to mark release complete")
            require(data.get("checksum") == checksum,
                    "registry checksum differs from the tagged package; refusing duplicate/foreign publication")
            return True
        if attempt + 1 < attempts:
            time.sleep(5)
    return False


def publish(source, version, kind):
    require_token()
    semver(version)
    require(re.fullmatch(r"[0-9a-f]{40}", source), "invalid source commit")
    require(not git("status", "--porcelain"), "release needs a clean checkout")
    require(git("rev-parse", "HEAD") == source, "checkout differs from the validated source")
    branch = default_branch()
    git("fetch", "origin", branch, "--tags")
    tag = f"v{version}"
    state(version=version, source=source, stage="preflight")
    set_version(version)
    for name in METADATA_FILES:
        if Path(name).exists():
            git("add", "--", name)
    expected_tree = git("write-tree")
    if tag_exists(tag):
        message = tag_message(tag)
        require(kind == "resume" or
                (f"Release-Run: {os.environ.get('GITHUB_RUN_ID')}\n" in message + "\n" and
                 f"Source-Commit: {source}\n" in message + "\n"),
                "existing tag belongs to another release run; use resume")
        require(git("rev-parse", f"{tag}^{{tree}}") == expected_tree,
                "existing release tag does not match the validated source and version")
        git("checkout", "--detach", tag)
    else:
        require(kind != "resume", "resume requires an existing tag")
        require(git("rev-parse", f"origin/{branch}") == source,
                "default branch advanced during checks; refusing to overwrite new work")
        require(api_version(version) is None, "version appeared on crates.io during checks; refusing overwrite")
        run("cargo", "publish", "--locked", "--dry-run", "--allow-dirty", capture=False)
        git("config", "user.name", "github-actions[bot]")
        git("config", "user.email", "41898282+github-actions[bot]@users.noreply.github.com")
        if git("diff", "--cached", "--name-only"):
            git("commit", "-m", f"chore: release {tag}")
        message = f"zclap {version}\n\nSource-Commit: {source}\nRelease-Run: {os.environ.get('GITHUB_RUN_ID', 'local')}"
        git("tag", "-a", tag, "-m", message)
        # Branch and tag are one transaction; never force-push either reference.
        run("git", "push", "--atomic", "origin", f"HEAD:refs/heads/{branch}", f"refs/tags/{tag}", capture=False)
    commit = git("rev-parse", "HEAD")
    state(stage="tagged", commit=commit, tag=tag)
    run("cargo", "package", "--locked", capture=False)
    run("python", "scripts/audit.py", capture=False)
    metadata = json.loads(run("cargo", "metadata", "--locked", "--offline", "--no-deps",
                              "--format-version", "1"))
    package = Path(metadata["target_directory"]) / "package" / f"zclap-{version}.crate"
    checksum = hashlib.sha256(package.read_bytes()).hexdigest()
    state(checksum=checksum)
    if not verify_registry(version, checksum):
        result = run("cargo", "publish", "--locked", capture=False, check=False)
        # An upload can succeed even if Cargo's subsequent index polling times out.
        require(verify_registry(version, checksum, attempts=12),
                f"cargo publish exited {result.returncode}; version not verified on crates.io. "
                f"Inspect logs, then rerun this job or choose resume / {version}.")
    state(stage="published")
    repo = os.environ.get("GITHUB_REPOSITORY", "LIghtJUNction/zclap")
    release = run("gh", "release", "view", tag, "--repo", repo,
                  "--json", "tagName,targetCommitish,isDraft,url", check=False)
    if release.returncode == 0:
        data = json.loads(release.stdout)
        require(data["tagName"] == tag, "GitHub release tag mismatch")
        if data["isDraft"]:
            run("gh", "release", "edit", tag, "--repo", repo, "--draft=false", capture=False)
    else:
        notes = Path("target/release-notes.md")
        notes.write_text(f"zclap {version}\n\n"
                         f"- Crate: https://crates.io/crates/zclap/{version}\n"
                         f"- Validated commit: {commit}\n"
                         f"- Package SHA-256: `{checksum}`\n"
                         "- Linux, macOS, Windows, MSRV, no_std and zero-dependency checks passed.\n", encoding="utf-8")
        prerelease = [] if semver(version)[3] else ["--prerelease", "--latest=false"]
        run("gh", "release", "create", tag, "--repo", repo, "--verify-tag",
            "--title", f"zclap {version}", "--notes-file", str(notes), *prerelease, capture=False)
    run("gh", "release", "upload", tag, str(package), "--repo", repo, "--clobber", capture=False)
    url = run("gh", "release", "view", tag, "--repo", repo, "--json", "url", "--jq", ".url")
    state(stage="complete", release_url=url)
    print(f"Verified zclap {version}: {url}; crates.io SHA-256 {checksum}")
    if path := os.environ.get("GITHUB_STEP_SUMMARY"):
        with open(path, "a", encoding="utf-8") as output:
            output.write(f"## zclap {version}\n\n[GitHub Release]({url}) · "
                         f"[crates.io](https://crates.io/crates/zclap/{version})\n\n"
                         f"Commit: `{commit}`\n\nSHA-256: `{checksum}`\n")


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    commands = parser.add_subparsers(dest="command", required=True)
    edit = commands.add_parser("set-version")
    edit.add_argument("version")
    planning = commands.add_parser("plan")
    source = planning.add_mutually_exclusive_group(required=True)
    source.add_argument("--kind")
    source.add_argument("--request", help="JSON file containing bump and version")
    planning.add_argument("--version")
    release = commands.add_parser("publish")
    release.add_argument("--source", required=True)
    release.add_argument("--version", required=True)
    release.add_argument("--kind", required=True)
    args = parser.parse_args()
    try:
        if args.command == "set-version":
            set_version(args.version)
        elif args.command == "plan":
            if args.request:
                require(args.version is None, "--request cannot be combined with --version")
                plan(*request_settings(args.request))
            else:
                plan(args.kind, args.version or "")
        else:
            publish(args.source, args.version, args.kind)
    except (RuntimeError, OSError, ValueError) as error:
        print(f"::error::{error}")
        raise SystemExit(1) from error


if __name__ == "__main__":
    main()
