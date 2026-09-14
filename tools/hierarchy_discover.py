#!/usr/bin/env python3
"""Read-only semantic discovery for a local workspace; canonical levels are optional observations."""
from __future__ import annotations

import argparse
import json
import re
from pathlib import Path
from typing import Any

from symbol_parser import parse_symbols

LEVELS = [
    "UNIVERSE", "CREATION", "ORDER", "REALITY", "REALM", "WORLD", "DOMAIN",
    "ECOSYSTEM", "ORGANIZATION", "COMMUNITY", "REGION", "ENVIRONMENT", "NETWORK", "CONTEXT",
    "PURPOSE", "MISSION", "OBJECTIVE", "PROGRAM", "PROJECT", "PRODUCT", "SYSTEM",
    "REPOSITORY", "SOURCE", "UNIT", "MODULE", "SUBSYSTEM", "COMPONENT", "ELEMENT",
    "SYMBOL", "ENTITY", "PROPERTY", "RELATION", "OPERATION", "FUNCTION", "BEHAVIOR",
    "STATE", "EVENT", "PROCESS", "FLOW", "TRANSITION", "ACTION", "EXECUTION",
    "INSTRUCTION", "EXPRESSION", "VALUE", "DATA", "TOKEN", "CHARACTER", "BIT",
]
IGNORED = {
    ".git", ".next", ".turbo", "node_modules", "target", "dist", "build",
    "coverage", "__pycache__", ".venv", "venv", ".idea", ".vscode", "vendor",
}
SOURCE_NAMES = {"src", "app", "apps", "packages", "crates", "cmd", "internal", "lib", "libs", "pkg"}
MANIFESTS = ("Cargo.toml", "package.json", "go.mod", "pyproject.toml", "setup.py", "pom.xml", "build.gradle", "build.gradle.kts")
SOURCE_EXTENSIONS = {".rs", ".ts", ".tsx", ".js", ".jsx", ".go", ".py", ".java", ".kt"}


def stable_id(level: str, name: str, path: str) -> str:
    raw = f"{level}:{path or name}".lower().replace("\\", "/")
    return re.sub(r"[^a-z0-9:/._-]+", "-", raw)


def node(
    level: str,
    name: str,
    path: str,
    native_type: str | None = None,
    language: str | None = None,
    children: list[dict[str, Any]] | None = None,
    metadata: dict[str, Any] | None = None,
) -> dict[str, Any]:
    result: dict[str, Any] = {"level": level, "id": stable_id(level, name, path), "name": name, "path": path or "."}
    if native_type:
        result["native_type"] = native_type
    if language:
        result["language"] = language
    if children:
        result["children"] = children
    if metadata:
        result["metadata"] = metadata
    return result


def detect_language(repo: Path) -> str | None:
    names = {p.name for p in repo.iterdir() if p.is_file()}
    if "Cargo.toml" in names:
        return "rust"
    if "package.json" in names or "pnpm-workspace.yaml" in names:
        return "node"
    if "go.mod" in names:
        return "go"
    if "pyproject.toml" in names or "setup.py" in names:
        return "python"
    if "pom.xml" in names or "build.gradle" in names or "build.gradle.kts" in names:
        return "java"
    if any(p.suffix.lower() == ".kt" for p in repo.iterdir() if p.is_file()):
        return "kotlin"
    return None


def parser_language(path: Path, language: str | None) -> str | None:
    if path.suffix.lower() in {".tsx", ".ts"}:
        return "typescript" if language == "node" else language
    if path.suffix.lower() in {".jsx", ".js"}:
        return "javascript" if language == "node" else language
    return language


def is_repo(path: Path) -> bool:
    return (path / ".git").exists() or any((path / manifest).is_file() for manifest in MANIFESTS)


def source_nodes(repo: Path) -> list[dict[str, Any]]:
    children: list[dict[str, Any]] = []
    language = detect_language(repo)
    for child in sorted(repo.iterdir(), key=lambda p: p.name.lower()):
        if not child.is_dir() or child.name in IGNORED:
            continue
        if child.name in SOURCE_NAMES:
            children.append(source_node(child, repo, language))
    if not children:
        native_files = [p for p in sorted(repo.iterdir(), key=lambda p: p.name.lower()) if p.is_file() and p.suffix.lower() in SOURCE_EXTENSIONS]
        if native_files:
            children.append(source_node(repo, repo, language, repository_root=True))
    return children


def source_node(source: Path, repo: Path, language: str | None, repository_root: bool = False) -> dict[str, Any]:
    rel = "." if repository_root else source.relative_to(repo).as_posix()
    children = structural_children(source, repo, language)
    metadata = {"evidence": "repository-root source files" if repository_root else "native source boundary"}
    if not has_native_unit(source, language):
        metadata["unmaterialized_levels"] = ["UNIT", "MODULE"]
    return node("SOURCE", source.name if not repository_root else ".", rel, "repository-root" if repository_root else "directory", language, children, metadata)


def has_native_unit(source: Path, language: str | None) -> bool:
    if language == "rust":
        return any((p / "Cargo.toml").is_file() for p in source.iterdir() if p.is_dir() and p.name not in IGNORED)
    if language == "node":
        return any((p / "package.json").is_file() for p in source.iterdir() if p.is_dir() and p.name not in IGNORED)
    if language == "python":
        return any((p / "__init__.py").exists() or (p / "pyproject.toml").is_file() for p in source.iterdir() if p.is_dir() and p.name not in IGNORED)
    return False


def structural_children(source: Path, repo: Path, language: str | None) -> list[dict[str, Any]]:
    result: list[dict[str, Any]] = []
    for child in sorted(source.iterdir(), key=lambda p: p.name.lower()):
        if child.name in IGNORED:
            continue
        if child.is_dir() and child.name.startswith('.'):
            continue
        if child.is_file() and child.suffix.lower() in SOURCE_EXTENSIONS:
            result.append(component_node(child, repo, language))
        elif child.is_dir():
            result.extend(component_nodes_recursive(child, repo, language))
    return result


def component_nodes_recursive(path: Path, repo: Path, language: str | None) -> list[dict[str, Any]]:
    files = [p for p in sorted(path.rglob("*"), key=lambda p: p.as_posix().lower()) if p.is_file() and p.suffix.lower() in SOURCE_EXTENSIONS and not any(part in IGNORED for part in p.relative_to(repo).parts)]
    if files:
        return [component_node(p, repo, language) for p in files]
    return []


def component_node(path: Path, repo: Path, language: str | None) -> dict[str, Any]:
    rel = path.relative_to(repo).as_posix()
    symbols = parse_symbols(path, parser_language(path, language))
    elements = [
        node("ELEMENT", symbol["name"], rel, symbol["kind"], language, metadata={
            "symbol": symbol["name"],
            "kind": symbol["kind"],
            "confidence": symbol["confidence"],
            "evidence": symbol["evidence"],
            "span": {"line_start": symbol["line_start"], "line_end": symbol["line_end"]},
            "execution_model": "runtime phase; not a containment child",
        })
        for symbol in symbols
    ]
    if not elements:
        elements = [node("ELEMENT", path.stem, rel, "source-construct", language, metadata={"confidence": "low", "evidence": "parser-no-symbols"})]
    return node("COMPONENT", path.stem, rel, "source-file", language, elements, {"evidence": "source-file"})


def discover(workspace: Path, universe_id: str = "universe") -> dict[str, Any]:
    """Discover observed structure while allowing unmaterialized canonical levels."""
    ecosystems: list[dict[str, Any]] = []
    if workspace.is_dir():
        for path in sorted(workspace.iterdir(), key=lambda p: p.name.lower()):
            if not path.is_dir() or path.name.startswith(".") or not path.name.startswith("ecosystem-"):
                continue
            eco_id = path.name.removeprefix("ecosystem-")
            projects: list[dict[str, Any]] = []
            for child in sorted(path.iterdir(), key=lambda p: p.name.lower()):
                if not child.is_dir() or child.name in IGNORED:
                    continue
                repositories: list[dict[str, Any]] = []
                if is_repo(child):
                    repositories.append(node("REPOSITORY", child.name, child.relative_to(workspace).as_posix(), "git-or-native-project", detect_language(child), source_nodes(child)))
                else:
                    for repo in sorted([p for p in child.iterdir() if p.is_dir() and p.name not in IGNORED], key=lambda p: p.name.lower()):
                        if is_repo(repo):
                            repositories.append(node("REPOSITORY", repo.name, repo.relative_to(workspace).as_posix(), "git-or-native-project", detect_language(repo), source_nodes(repo)))
                if repositories:
                    projects.append(node("PROJECT", child.name, child.relative_to(workspace).as_posix(), "workspace-project", children=repositories))
            ecosystems.append(node("ECOSYSTEM", eco_id, path.relative_to(workspace).as_posix(), "workspace-directory", children=projects, metadata={"discovery": "filesystem"}))

    return {
        "level": "UNIVERSE",
        "id": universe_id,
        "name": universe_id,
        "path": ".",
        "children": ecosystems,
        "metadata": {
            "read_only": True,
            "canonical_levels": LEVELS,
            "canonical_level_count": len(LEVELS),
            "creator_outside_model": True,
            "intermediate_levels_may_be_unmaterialized": True,
            "omitted_semantic_levels_are_not_invented": True,
            "runtime_execution_is_not_containment": True,
        },
    }


def main(argv: list[str] | None = None) -> int:
    parser = argparse.ArgumentParser(description="Discover universal hierarchy without modifying a workspace")
    parser.add_argument("workspace", type=Path)
    parser.add_argument("--universe-id", default="universe")
    parser.add_argument("--json", action="store_true", dest="as_json")
    args = parser.parse_args(argv)
    result = discover(args.workspace, args.universe_id)
    print(json.dumps(result, indent=2))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
