#!/usr/bin/env python3
"""Render repository status in README and generate CHANGELOG from release JSON."""
from __future__ import annotations

import argparse
import json
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
README = ROOT / "README.md"
STATUS = ROOT / ".ontology" / "status.json"
CHANGELOG = ROOT / "CHANGELOG.md"
START = "<!-- AUTO:REPO_STATUS:START -->"
END = "<!-- AUTO:REPO_STATUS:END -->"
INSERT_BEFORE = "## Canonical levels"


def render_status(data: dict) -> str:
    return "\n".join([
        START,
        "## Repository automation status",
        "",
        "> Auto-generated. Human-authored sections remain outside this block.",
        "",
        "| Signal | Value |",
        "|---|---:|",
        f"| Health | **{data.get('health', 'unknown').upper()}** |",
        f"| CI | `{data.get('ci', 'unknown')}` |",
        f"| Stable release | `{data.get('releases', {}).get('stable') or 'none'}` |",
        f"| Dev release | `{data.get('releases', {}).get('dev') or 'none'}` |",
        f"| Main version | `{data.get('versions', {}).get('main', 'unknown')}` |",
        f"| Dev version | `{data.get('versions', {}).get('dev', 'unknown')}` |",
        f"| Branch relation | `{data.get('branches', {}).get('relation', 'unknown')}` |",
        f"| Branch model | `{data.get('branches', {}).get('model', 'unknown')}` |",
        "",
        "See [`CHANGELOG.md`](CHANGELOG.md) for generated release history.",
        END,
    ])


def replace_or_insert(text: str, block: str) -> str:
    if (START in text) != (END in text):
        raise ValueError("README contains incomplete repository-status markers")
    if START in text:
        prefix, tail = text.split(START, 1)
        _, suffix = tail.split(END, 1)
        return prefix.rstrip() + "\n\n" + block + suffix
    index = text.find(INSERT_BEFORE)
    if index < 0:
        return text.rstrip() + "\n\n" + block + "\n"
    return text[:index].rstrip() + "\n\n" + block + "\n\n" + text[index:].lstrip()


def render_changelog(releases: list[dict]) -> str:
    lines = [
        "# Changelog",
        "",
        "> Auto-generated from GitHub Releases. Do not edit manually.",
        "",
    ]
    for release in releases:
        tag = release.get("tag_name", "unknown")
        channel = "DEV" if release.get("prerelease") else "STABLE"
        published = (release.get("published_at") or "unknown").split("T")[0]
        lines += [f"## {tag} — {channel} — {published}", ""]
        body = (release.get("body") or "").strip()
        lines += [body if body else "No generated notes.", ""]
    return "\n".join(lines).rstrip() + "\n"


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--releases-json", type=Path, required=True)
    parser.add_argument("--check", action="store_true")
    args = parser.parse_args()

    status = json.loads(STATUS.read_text(encoding="utf-8"))
    releases = json.loads(args.releases_json.read_text(encoding="utf-8"))
    current_readme = README.read_text(encoding="utf-8")
    expected_readme = replace_or_insert(current_readme, render_status(status))
    expected_changelog = render_changelog(releases)

    if args.check:
        ok = current_readme == expected_readme and CHANGELOG.exists() and CHANGELOG.read_text(encoding="utf-8") == expected_changelog
        print("repository docs synchronized" if ok else "repository docs out of sync")
        return 0 if ok else 1

    README.write_text(expected_readme, encoding="utf-8")
    CHANGELOG.write_text(expected_changelog, encoding="utf-8")
    print("README repository status and CHANGELOG synchronized")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
