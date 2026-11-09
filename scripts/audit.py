#!/usr/bin/env python3
"""Check the published crate's zero-dependency and package boundaries."""

import json
from pathlib import Path
import subprocess
import tarfile
import tomllib


def check_manifest(manifest):
    """Reject dependencies in every Cargo dependency table, including targets."""
    for key, value in manifest.items():
        if key in {"dependencies", "dev-dependencies", "build-dependencies"}:
            assert not value, f"third-party dependency table is nonempty: {key}"
        elif isinstance(value, dict):
            check_manifest(value)


def main():
    manifest = tomllib.loads(Path("Cargo.toml").read_text())
    check_manifest(manifest)
    assert not Path("build.rs").exists(), "build.rs is forbidden"
    assert manifest["package"].get("build", False) is False, "build script is forbidden"
    source = Path("src/lib.rs").read_text()
    assert "#![no_std]" in source, "the crate must always declare #![no_std]"
    assert "extern crate alloc" not in source, "alloc must not enter the core"
    metadata = json.loads(subprocess.check_output([
        "cargo", "metadata", "--locked", "--offline", "--all-features", "--format-version", "1"
    ], text=True))
    assert len(metadata["packages"]) == 1, "dependency tree contains another package"
    assert not metadata["packages"][0]["dependencies"], "Cargo reports dependencies"
    assert len(metadata["workspace_members"]) == 1, "comparison must be an isolated workspace"
    version = manifest["package"]["version"]
    package = Path(metadata["target_directory"]) / "package" / f"zclap-{version}.crate"
    assert package.exists(), "run cargo package before audit.py"
    with tarfile.open(package) as archive:
        names = {member.name.split("/", 1)[1] for member in archive.getmembers()}
        assert {"Cargo.toml", "README.md", "LICENSE", "src/lib.rs"} <= names
        assert not any(name.startswith(("comparison/", ".github/", "scripts/")) for name in names)
        assert "build.rs" not in names
        packaged = archive.extractfile(f"zclap-{version}/Cargo.toml").read()
        check_manifest(tomllib.loads(packaged.decode()))
    print(f"zclap {version}: zero dependencies; no build script; package boundary verified")


if __name__ == "__main__":
    main()
