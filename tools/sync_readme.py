#!/usr/bin/env python3
"""Synchronize the generated self-status block in README.md.

The block is derived only from .ontology/self.json. Human-authored README
sections remain untouched.
"""
from __future__ import annotations

import argparse
import json
from pathlib import Path
from typing import Any

ROOT = Path(__file__).resolve().parents[1]
README_PATH = ROOT / "README.md"
SELF_PATH = ROOT / ".ontology" / "self.json"
START = "<!-- AUTO:SELF_STATUS:START -->"
END = "<!-- AUTO:SELF_STATUS:END -->"
INSERT_BEFORE = "## Canonical levels"


def load_snapshot(path: Path = SELF_PATH) -> dict[str, Any]:
    with path.open("r", encoding="utf-8") as handle:
        data = json.load(handle)
    if not isinstance(data, dict):
        raise ValueError("self snapshot must be a JSON object")
    return data


def render_self_status(snapshot: dict[str, Any]) -> str:
    status = str(snapshot.get("status", "unknown")).lower()
    status_label = "✅ HEALTHY" if status == "healthy" else f"⚠️ {status.upper()}"
    return "\n".join(
        [
            START,
            "## Self status",
            "",
            "> Auto-generated from `.ontology/self.json`. Do not edit this block manually.",
            "",
            "| Signal | Value |",
            "|---|---:|",
            f"| Repository | `{snapshot.get('repository', 'unknown')}` |",
            f"| Self health | **{status_label}** |",
            f"| Ontology | `{snapshot.get('ontology', 'unknown')}` |",
            f"| Canonical levels | **{snapshot.get('canonical_levels', 'unknown')}** |",
            f"| Workspace crates | **{snapshot.get('workspace_crates', 'unknown')}** |",
            f"| Rust sources | **{snapshot.get('rust_sources', 'unknown')}** |",
            f"| Read-only inspection | **{'YES' if snapshot.get('read_only') is True else 'NO'}** |",
            "",
            "Self inspection command:",
            "",
            "```bash",
            "cargo run -p ontology-engine -- self --json",
            "```",
            END,
        ]
    )


def update_readme_text(readme: str, block: str) -> str:
    has_start = START in readme
    has_end = END in readme
    if has_start != has_end:
        raise ValueError("README contains an incomplete generated self-status marker pair")

    if has_start:
        prefix, remainder = readme.split(START, 1)
        _, suffix = remainder.split(END, 1)
        return prefix.rstrip() + "\n\n" + block + suffix

    marker_index = readme.find(INSERT_BEFORE)
    if marker_index < 0:
        return readme.rstrip() + "\n\n" + block + "\n"

    prefix = readme[:marker_index].rstrip()
    suffix = readme[marker_index:].lstrip()
    return prefix + "\n\n" + block + "\n\n" + suffix


def synchronized_readme(readme_path: Path = README_PATH, self_path: Path = SELF_PATH) -> str:
    readme = readme_path.read_text(encoding="utf-8")
    return update_readme_text(readme, render_self_status(load_snapshot(self_path)))


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument(
        "--check",
        action="store_true",
        help="fail when README.md differs from the deterministic generated form",
    )
    args = parser.parse_args()

    current = README_PATH.read_text(encoding="utf-8")
    expected = synchronized_readme()

    if args.check:
        if current != expected:
            print("README self-status block is out of sync; run: python tools/sync_readme.py")
            return 1
        print("README self-status block is synchronized")
        return 0

    if current == expected:
        print("README self-status block is already synchronized")
        return 0

    README_PATH.write_text(expected, encoding="utf-8")
    print("README self-status block synchronized")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
