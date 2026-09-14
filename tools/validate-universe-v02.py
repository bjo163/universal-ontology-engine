#!/usr/bin/env python3
"""Stdlib-only validation for the Universe Foundation reference manifest v0.2."""

from __future__ import annotations

import json
import re
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
MANIFEST = ROOT / "specifications" / "universe-contract.instance.json"
ID_PATTERN = re.compile(r"^[a-z0-9][a-z0-9-]*$")


def fail(message: str) -> None:
    raise SystemExit(f"validation failed: {message}")


def main() -> int:
    try:
        data = json.loads(MANIFEST.read_text(encoding="utf-8"))
    except FileNotFoundError:
        fail(f"missing {MANIFEST}")
    except json.JSONDecodeError as exc:
        fail(f"invalid JSON: {exc}")

    if data.get("contract_version") != "0.2":
        fail("unexpected contract_version")

    universe = data.get("universe")
    if not isinstance(universe, dict) or not universe.get("id") or not universe.get("name"):
        fail("universe requires id and name")
    if not ID_PATTERN.fullmatch(str(universe["id"])):
        fail("universe id must use lowercase kebab-case")

    ecosystems = data.get("ecosystems")
    if not isinstance(ecosystems, list) or not ecosystems:
        fail("ecosystems must be a non-empty list")

    ids: set[str] = set()
    paths: set[str] = set()
    for ecosystem in ecosystems:
        if not isinstance(ecosystem, dict):
            fail("each ecosystem must be an object")
        ecosystem_id = ecosystem.get("id")
        path = ecosystem.get("path")
        if not ecosystem_id or not ecosystem.get("name") or not path:
            fail("each ecosystem requires id, name, and path")
        if not ID_PATTERN.fullmatch(str(ecosystem_id)):
            fail(f"invalid ecosystem id: {ecosystem_id}")
        if ecosystem_id in ids:
            fail(f"duplicate ecosystem id: {ecosystem_id}")
        if path in paths:
            fail(f"duplicate ecosystem path: {path}")
        normalized = str(path).replace("\\", "/")
        if normalized.startswith("/") or normalized == ".." or "/../" in f"/{normalized}/":
            fail(f"unsafe ecosystem path: {path}")
        ids.add(str(ecosystem_id))
        paths.add(str(path))

    print(f"Universe Foundation valid: {len(ecosystems)} ecosystem(s)")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
