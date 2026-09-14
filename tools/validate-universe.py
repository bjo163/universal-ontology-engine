#!/usr/bin/env python3
"""Validate the canonical Universe manifest and single foundation contract. Stdlib-only."""

from __future__ import annotations

import json
import re
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
MANIFEST = ROOT / "universe.json"
CONTRACT = ROOT / "specifications" / "foundation-contract.json"
ID_PATTERN = re.compile(r"^[a-z0-9][a-z0-9-]*$")
CONTRACT_VERSION = "0.3"
CANONICAL_HIERARCHY = [
    "UNIVERSE", "ECOSYSTEM", "ORGANIZATION", "DOMAIN", "PROJECT", "REPOSITORY",
    "SOURCE", "UNIT", "MODULE", "COMPONENT", "ELEMENT", "IMPLEMENTATION",
]


def fail(message: str) -> None:
    raise SystemExit(f"validation failed: {message}")


def load_json(path: Path) -> dict:
    try:
        data = json.loads(path.read_text(encoding="utf-8"))
    except FileNotFoundError:
        fail(f"missing {path}")
    except json.JSONDecodeError as exc:
        fail(f"invalid JSON in {path}: {exc}")
    if not isinstance(data, dict):
        fail(f"{path.name} root must be an object")
    return data


def main() -> int:
    data = load_json(MANIFEST)
    contract = load_json(CONTRACT)

    if data.get("contract_version") != CONTRACT_VERSION:
        fail("unexpected manifest contract_version")
    if contract.get("version") != CONTRACT_VERSION:
        fail("unexpected foundation contract version")
    if contract.get("hierarchy") != CANONICAL_HIERARCHY:
        fail("foundation hierarchy does not match canonical hierarchy")
    if contract.get("optionalLayers") != ["ORGANIZATION", "DOMAIN"]:
        fail("optional foundation layers are invalid")
    rules = contract.get("rules", {})
    if rules.get("singleFoundation") is not True:
        fail("singleFoundation rule must be true")
    if rules.get("semanticDirectories") is not False:
        fail("semanticDirectories rule must be false")
    if rules.get("externalSystems") != "relationship":
        fail("externalSystems rule must be relationship")

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
    for index, ecosystem in enumerate(ecosystems):
        if not isinstance(ecosystem, dict):
            fail(f"ecosystems[{index}] must be an object")
        ecosystem_id = ecosystem.get("id")
        path = ecosystem.get("path")
        if not ecosystem_id or not ecosystem.get("name") or not path:
            fail(f"ecosystems[{index}] requires id, name, and path")
        ecosystem_id = str(ecosystem_id)
        path = str(path)
        if not ID_PATTERN.fullmatch(ecosystem_id):
            fail(f"invalid ecosystem id: {ecosystem_id}")
        if ecosystem_id in ids:
            fail(f"duplicate ecosystem id: {ecosystem_id}")
        if path in paths:
            fail(f"duplicate ecosystem path: {path}")
        normalized = path.replace("\\", "/")
        if normalized.startswith("/") or normalized == ".." or "/../" in f"/{normalized}/":
            fail(f"unsafe ecosystem path: {path}")
        ids.add(ecosystem_id)
        paths.add(path)

    print(f"Universe Foundation v{CONTRACT_VERSION} valid: {len(ecosystems)} ecosystem(s); complete hierarchy enforced")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
