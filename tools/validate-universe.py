#!/usr/bin/env python3
"""Stdlib-only validation for the Universe Foundation reference manifest."""

from __future__ import annotations

import json
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
MANIFEST = ROOT / "specifications" / "universe-contract.instance.json"


def fail(message: str) -> None:
    raise SystemExit(f"validation failed: {message}")


def main() -> int:
    try:
        data = json.loads(MANIFEST.read_text(encoding="utf-8"))
    except FileNotFoundError:
        fail(f"missing {MANIFEST}")
    except json.JSONDecodeError as exc:
        fail(f"invalid JSON: {exc}")

    if data.get("contract_version") != "0.1":
        fail("unexpected contract_version")

    universe = data.get("universe")
    if not isinstance(universe, dict) or not universe.get("id") or not universe.get("name"):
        fail("universe requires id and name")

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
        if ecosystem_id in ids:
            fail(f"duplicate ecosystem id: {ecosystem_id}")
        if path in paths:
            fail(f"duplicate ecosystem path: {path}")
        ids.add(ecosystem_id)
        paths.add(path)

    print(f"Universe Foundation valid: {len(ecosystems)} ecosystem(s)")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
