#!/usr/bin/env python3
"""Universe registry/discovery CLI. Stdlib-only and dependency-free."""

from __future__ import annotations

import argparse
import json
import sys
from pathlib import Path
from typing import Any

ROOT = Path(__file__).resolve().parents[1]
DEFAULT_MANIFEST = ROOT / "universe.json"
CONTRACT_VERSION = "1.0"
FOUNDATION_HIERARCHY = [
    "UNIVERSE", "CREATION", "ORDER", "REALITY", "REALM", "WORLD", "DOMAIN",
    "ECOSYSTEM", "ORGANIZATION", "COMMUNITY", "REGION", "ENVIRONMENT", "NETWORK", "CONTEXT",
    "PURPOSE", "MISSION", "OBJECTIVE", "PROGRAM", "PROJECT", "PRODUCT", "SYSTEM",
    "REPOSITORY", "SOURCE", "UNIT", "MODULE", "SUBSYSTEM", "COMPONENT", "ELEMENT",
    "SYMBOL", "ENTITY", "PROPERTY", "RELATION", "OPERATION", "FUNCTION", "BEHAVIOR",
    "STATE", "EVENT", "PROCESS", "FLOW", "TRANSITION", "ACTION", "EXECUTION",
    "INSTRUCTION", "EXPRESSION", "VALUE", "DATA", "TOKEN", "CHARACTER", "BIT",
]


def fail(message: str) -> int:
    print(f"error: {message}", file=sys.stderr)
    return 1


def load_manifest(path: Path) -> dict[str, Any]:
    try:
        data = json.loads(path.read_text(encoding="utf-8"))
    except FileNotFoundError as exc:
        raise ValueError(f"manifest not found: {path}") from exc
    except json.JSONDecodeError as exc:
        raise ValueError(f"invalid JSON in {path}: {exc}") from exc
    if not isinstance(data, dict):
        raise ValueError("manifest root must be an object")
    return data


def ecosystems(data: dict[str, Any]) -> list[dict[str, Any]]:
    items = data.get("ecosystems")
    if not isinstance(items, list):
        raise ValueError("manifest ecosystems must be a list")
    return [item for item in items if isinstance(item, dict)]


def validate(data: dict[str, Any]) -> list[str]:
    errors: list[str] = []
    if data.get("contract_version") != CONTRACT_VERSION:
        errors.append(f"contract_version must be {CONTRACT_VERSION}")
    if data.get("ontology") != "Universal Ontology v1.0.0":
        errors.append("ontology must be Universal Ontology v1.0.0")

    universe = data.get("universe")
    if not isinstance(universe, dict) or not universe.get("id") or not universe.get("name"):
        errors.append("universe requires id and name")

    ids: set[str] = set()
    paths: set[str] = set()
    for index, ecosystem in enumerate(ecosystems(data)):
        for field in ("id", "name", "path"):
            if not ecosystem.get(field):
                errors.append(f"ecosystems[{index}] requires {field}")
        ecosystem_id = str(ecosystem.get("id", ""))
        path = str(ecosystem.get("path", ""))
        if ecosystem_id and ecosystem_id in ids:
            errors.append(f"duplicate ecosystem id: {ecosystem_id}")
        if path and path in paths:
            errors.append(f"duplicate ecosystem path: {path}")
        if path:
            normalized = path.replace("\\", "/")
            if normalized.startswith("/") or normalized == ".." or "/../" in f"/{normalized}/":
                errors.append(f"unsafe ecosystem path: {path}")
        ids.add(ecosystem_id)
        paths.add(path)
    return errors


def foundation_summary() -> dict[str, Any]:
    return {
        "version": CONTRACT_VERSION,
        "ontology": "Universal Ontology v1.0.0",
        "hierarchy": FOUNDATION_HIERARCHY,
        "canonical_levels": 49,
        "zones": 7,
        "levels_per_zone": 7,
        "single_foundation": True,
        "optional_layers": ["ORGANIZATION", "DOMAIN"],
        "semantic_layers_not_directories": ["UNIT", "MODULE", "SUBSYSTEM", "COMPONENT", "ELEMENT"],
    }


def print_json(data: Any) -> None:
    print(json.dumps(data, indent=2, sort_keys=True))


def command_list(data: dict[str, Any], as_json: bool) -> int:
    items = ecosystems(data)
    if as_json:
        print_json(items)
        return 0
    for item in items:
        lifecycle = item.get("lifecycle", "unknown")
        print(f"{item['id']}\t{item['name']}\t{lifecycle}\t{item['path']}")
    return 0


def command_inspect(data: dict[str, Any], ecosystem_id: str, as_json: bool) -> int:
    match = next((item for item in ecosystems(data) if item.get("id") == ecosystem_id), None)
    if match is None:
        return fail(f"ecosystem not registered: {ecosystem_id}")
    if as_json:
        print_json(match)
    else:
        for key, value in match.items():
            print(f"{key}: {value}")
    return 0


def command_validate(data: dict[str, Any], as_json: bool) -> int:
    errors = validate(data)
    result = {"valid": not errors, "errors": errors, "ecosystem_count": len(ecosystems(data)), "foundation": foundation_summary()}
    if as_json:
        print_json(result)
    elif errors:
        for error in errors:
            print(f"FAIL: {error}")
    else:
        print(f"VALID: {len(ecosystems(data))} ecosystem(s); Universal Ontology v1.0.0; 49 canonical levels")
    return 0 if not errors else 1


def git_repository(path: Path) -> bool:
    return (path / ".git").is_dir()


def command_status(data: dict[str, Any], workspace: Path, as_json: bool) -> int:
    rows: list[dict[str, Any]] = []
    for item in ecosystems(data):
        path = workspace / str(item["path"])
        rows.append({"id": item["id"], "name": item["name"], "registered_path": item["path"], "exists": path.is_dir(), "git_repository": git_repository(path), "absolute_path": str(path.resolve())})
    if as_json:
        print_json(rows)
    else:
        for row in rows:
            state = "present" if row["exists"] else "missing"
            git = "git" if row["git_repository"] else "not-git"
            print(f"{row['id']}\t{state}\t{git}\t{row['registered_path']}")
    return 0


def discover(data: dict[str, Any], workspace: Path) -> dict[str, Any]:
    registered = {str(item["path"]): str(item["id"]) for item in ecosystems(data)}
    discovered: list[dict[str, Any]] = []
    if workspace.is_dir():
        for child in sorted(workspace.iterdir(), key=lambda p: p.name.lower()):
            if child.is_dir() and child.name.startswith("ecosystem-"):
                discovered.append({"directory": child.name, "path": child.name, "registered": child.name in registered, "id": registered.get(child.name), "git_repository": git_repository(child)})
    missing = sorted(path for path in registered if not (workspace / path).is_dir())
    unknown = sorted(item["path"] for item in discovered if not item["registered"])
    return {"workspace": str(workspace.resolve()), "discovered": discovered, "missing": missing, "unknown": unknown}


def command_discover(data: dict[str, Any], workspace: Path, as_json: bool) -> int:
    result = discover(data, workspace)
    if as_json:
        print_json(result)
    else:
        for item in result["discovered"]:
            mark = "registered" if item["registered"] else "UNKNOWN"
            print(f"{item['path']}\t{mark}\t{item.get('id') or '-'}")
        for path in result["missing"]:
            print(f"MISSING\t{path}")
        for path in result["unknown"]:
            print(f"UNKNOWN\t{path}")
    return 0


def read_ecosystem_manifest(path: Path) -> dict[str, Any] | None:
    manifest = path / "ecosystem.json"
    if not manifest.is_file():
        return None
    try:
        data = json.loads(manifest.read_text(encoding="utf-8"))
    except (OSError, json.JSONDecodeError):
        return None
    return data if isinstance(data, dict) else None


def command_doctor(data: dict[str, Any], workspace: Path, as_json: bool) -> int:
    registry_errors = validate(data)
    discovery = discover(data, workspace)
    rows: list[dict[str, Any]] = []
    errors = list(registry_errors)
    for item in ecosystems(data):
        path = workspace / str(item["path"])
        local = read_ecosystem_manifest(path) if path.is_dir() else None
        entry_errors: list[str] = []
        if not path.is_dir():
            entry_errors.append("workspace directory missing")
        elif local is None:
            entry_errors.append("ecosystem.json missing or invalid")
        else:
            local_id = local.get("id")
            if local_id != item["id"]:
                entry_errors.append(f"identity mismatch: registry={item['id']} local={local_id!r}")
        rows.append({"id": item["id"], "path": item["path"], "healthy": not entry_errors, "errors": entry_errors})
        errors.extend(f"{item['id']}: {error}" for error in entry_errors)
    if discovery["unknown"]:
        errors.extend(f"unknown ecosystem directory: {path}" for path in discovery["unknown"])
    result = {"healthy": not errors, "registry_errors": registry_errors, "foundation": foundation_summary(), "ecosystems": rows, "discovery": discovery}
    if as_json:
        print_json(result)
    else:
        print(f"HEALTHY: {result['healthy']}")
        for error in errors:
            print(f"FAIL: {error}")
    return 0 if result["healthy"] else 1


def build_parser() -> argparse.ArgumentParser:
    parser = argparse.ArgumentParser(prog="universe", description="Inspect and validate a Universe registry/workspace")
    parser.add_argument("--manifest", type=Path, default=DEFAULT_MANIFEST)
    parser.add_argument("--json", action="store_true", dest="as_json", help="emit machine-readable JSON")
    sub = parser.add_subparsers(dest="command", required=True)
    sub.add_parser("list", help="list registered ecosystems")
    inspect = sub.add_parser("inspect", help="inspect one registered ecosystem")
    inspect.add_argument("id")
    sub.add_parser("validate", help="validate the registry")
    discover_parser = sub.add_parser("discover", help="scan a workspace for ecosystem-* directories")
    discover_parser.add_argument("workspace", type=Path)
    status = sub.add_parser("status", help="show registered ecosystem filesystem status")
    status.add_argument("workspace", type=Path)
    doctor = sub.add_parser("doctor", help="check registry, workspace, and ecosystem manifests")
    doctor.add_argument("workspace", type=Path)
    return parser


def main(argv: list[str] | None = None) -> int:
    parser = build_parser()
    args = parser.parse_args(argv)
    try:
        data = load_manifest(args.manifest)
    except ValueError as exc:
        return fail(str(exc))
    if args.command == "list": return command_list(data, args.as_json)
    if args.command == "inspect": return command_inspect(data, args.id, args.as_json)
    if args.command == "validate": return command_validate(data, args.as_json)
    if args.command == "discover": return command_discover(data, args.workspace, args.as_json)
    if args.command == "status": return command_status(data, args.workspace, args.as_json)
    if args.command == "doctor": return command_doctor(data, args.workspace, args.as_json)
    return fail("unknown command")


if __name__ == "__main__":
    raise SystemExit(main())
