#!/usr/bin/env python3
"""Compute and persist release versions for dev/stable channels."""
from __future__ import annotations

import argparse
import os
import re
from pathlib import Path

SEMVER_RE = re.compile(r"^(?P<major>0|[1-9]\d*)\.(?P<minor>0|[1-9]\d*)\.(?P<patch>0|[1-9]\d*)(?:-(?P<pre>[0-9A-Za-z.-]+))?$")
WORKSPACE_VERSION_RE = re.compile(r'(?m)^(version\s*=\s*)"([^"]+)"\s*$')


def parse_version(value: str) -> tuple[int, int, int, str | None]:
    match = SEMVER_RE.fullmatch(value.strip())
    if not match:
        raise ValueError(f"invalid semver: {value}")
    return (
        int(match.group("major")),
        int(match.group("minor")),
        int(match.group("patch")),
        match.group("pre"),
    )


def strip_tag(value: str | None) -> str | None:
    if not value:
        return None
    return value[1:] if value.startswith("v") else value


def next_dev(current: str, serial: int, stable_tag: str | None = None) -> str:
    major, minor, patch, pre = parse_version(current)
    stable = strip_tag(stable_tag)
    if stable:
        s_major, s_minor, s_patch, s_pre = parse_version(stable)
        if s_pre is not None:
            raise ValueError(f"stable tag must not be a prerelease: {stable_tag}")
        major, minor, patch = s_major, s_minor, s_patch + 1
    elif pre and pre.startswith("dev."):
        # Before the first stable tag exists, keep the same candidate core.
        pass
    else:
        patch += 1
    return f"{major}.{minor}.{patch}-dev.{serial}"


def stable_version(current: str) -> str:
    major, minor, patch, _pre = parse_version(current)
    return f"{major}.{minor}.{patch}"


def read_workspace_version(cargo_toml: Path) -> str:
    text = cargo_toml.read_text(encoding="utf-8")
    match = WORKSPACE_VERSION_RE.search(text)
    if not match:
        raise ValueError(f"workspace package version not found in {cargo_toml}")
    return match.group(2)


def write_workspace_version(cargo_toml: Path, version: str) -> None:
    text = cargo_toml.read_text(encoding="utf-8")
    replaced, count = WORKSPACE_VERSION_RE.subn(lambda m: f'{m.group(1)}"{version}"', text, count=1)
    if count != 1:
        raise ValueError(f"expected exactly one workspace package version in {cargo_toml}")
    cargo_toml.write_text(replaced, encoding="utf-8")


def emit_output(version: str, changed: bool) -> None:
    output = os.environ.get("GITHUB_OUTPUT")
    if output:
        with open(output, "a", encoding="utf-8") as handle:
            handle.write(f"version={version}\n")
            handle.write(f"tag=v{version}\n")
            handle.write(f"changed={'true' if changed else 'false'}\n")
    print(version)


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("channel", choices=("dev", "stable"))
    parser.add_argument("--cargo", default="Cargo.toml")
    parser.add_argument("--serial", type=int)
    parser.add_argument("--stable-tag")
    args = parser.parse_args()

    cargo_toml = Path(args.cargo)
    current = read_workspace_version(cargo_toml)
    if args.channel == "dev":
        if args.serial is None or args.serial < 1:
            parser.error("dev channel requires --serial >= 1")
        target = next_dev(current, args.serial, args.stable_tag)
    else:
        target = stable_version(current)

    changed = target != current
    if changed:
        write_workspace_version(cargo_toml, target)
    emit_output(target, changed)
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
