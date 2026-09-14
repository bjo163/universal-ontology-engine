#!/usr/bin/env python3
"""Validate the Universal Ontology v1.0 contract and top-level registry. Stdlib-only."""

from __future__ import annotations

import json
import re
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
MANIFEST = ROOT / "universe.json"
CONTRACT = ROOT / "specifications" / "foundation-contract.instance.json"
ID_PATTERN = re.compile(r"^[a-z0-9][a-z0-9-]*$")
CONTRACT_VERSION = "1.0.0"
CANONICAL_HIERARCHY = [
    "UNIVERSE", "CREATION", "ORDER", "REALITY", "REALM", "WORLD", "DOMAIN",
    "ECOSYSTEM", "ORGANIZATION", "COMMUNITY", "REGION", "ENVIRONMENT", "NETWORK", "CONTEXT",
    "PURPOSE", "MISSION", "OBJECTIVE", "PROGRAM", "PROJECT", "PRODUCT", "SYSTEM",
    "REPOSITORY", "SOURCE", "UNIT", "MODULE", "SUBSYSTEM", "COMPONENT", "ELEMENT",
    "SYMBOL", "ENTITY", "PROPERTY", "RELATION", "OPERATION", "FUNCTION", "BEHAVIOR",
    "STATE", "EVENT", "PROCESS", "FLOW", "TRANSITION", "ACTION", "EXECUTION",
    "INSTRUCTION", "EXPRESSION", "VALUE", "DATA", "TOKEN", "CHARACTER", "BIT",
]
FORBIDDEN_FOUNDATION_REFS = {"ecosystem-foundation", "project-foundation", "repository-foundation"}


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

    if data.get("contract_version") not in {"1.0", "1.0.0"}:
        fail("unexpected manifest contract_version")
    if contract.get("version") != CONTRACT_VERSION:
        fail("unexpected foundation contract version")
    if contract.get("ontology") != "Universal Ontology v1.0.0":
        fail("unexpected ontology identifier")
    if contract.get("zones") != 7 or contract.get("levelsPerZone") != 7 or contract.get("canonicalLevels") != 49:
        fail("ontology shape must be 7 x 7 = 49")
    if contract.get("hierarchy") != CANONICAL_HIERARCHY:
        fail("foundation hierarchy does not match canonical 49-level hierarchy")
    if contract.get("optionalLevels") != ["ORGANIZATION", "DOMAIN"]:
        fail("optional foundation levels are invalid")

    rules = contract.get("rules", {})
    expected_rules = {
        "singleFoundation": True,
        "semanticDirectories": False,
        "externalSystems": "relationship",
        "stableIdentity": True,
        "typeIsCanonicalLevel": True,
        "kindIsSpecialization": True,
        "projectionDistinctFromContainment": True,
        "representationDistinctFromSemantics": True,
        "intermediateLevelsMayBeUnmaterialized": True,
        "creatorOutsideModel": True,
    }
    for name, expected in expected_rules.items():
        if rules.get(name) != expected:
            fail(f"{name} rule is invalid")

    universe = data.get("universe")
    if not isinstance(universe, dict) or not universe.get("id") or not universe.get("name"):
        fail("universe requires id and name")
    if not ID_PATTERN.fullmatch(str(universe["id"])):
        fail("universe id must use lowercase kebab-case")

    serialized = json.dumps(data, sort_keys=True)
    for forbidden in FORBIDDEN_FOUNDATION_REFS:
        if forbidden in serialized:
            fail(f"competing foundation reference found: {forbidden}")

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

    print(f"Universal Ontology v{CONTRACT_VERSION} valid: {len(ecosystems)} ecosystem(s); 49 canonical levels; single foundation")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
