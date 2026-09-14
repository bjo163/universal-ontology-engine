#!/usr/bin/env python3
"""Read-only semantic hierarchy discovery for a local workspace."""
from __future__ import annotations

import argparse
import json
import re
from pathlib import Path
from typing import Any

from symbol_parser import parse_symbols

LEVELS = [
    "UNIVERSE", "ECOSYSTEM", "ORGANIZATION", "DOMAIN", "PROJECT", "REPOSITORY",
    "SOURCE", "UNIT", "MODULE", "COMPONENT", "ELEMENT", "IMPLEMENTATION",
]
IGNORED = {
    ".git", ".next", ".turbo", "node_modules", "target", "dist", "build",
    "coverage", "__pycache__", ".venv", "venv", ".idea", ".vscode",
}
SOURCE_NAMES = {"src", "app", "apps", "packages", "crates", "cmd", "internal", "lib", "libs", "pkg"}
MANIFESTS = ("Cargo.toml", "package.json", "go.mod", "pyproject.toml", "pom.xml", "build.gradle", "build.gradle.kts")
ID_RE = re.compile(r"^[a-z0-9][a-z0-9-]*$")
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
    if path.suffix.lower() == ".tsx" or path.suffix.lower() == ".ts":
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
            children.append(node("SOURCE", child.name, child.relative_to(repo).as_posix(), "directory", language, unit_nodes(child, repo, language)))
    if not children:
        native_files = [p.name for p in repo.iterdir() if p.is_file() and p.suffix.lower() in SOURCE_EXTENSIONS]
        if native_files:
            children.append(node("SOURCE", ".", ".", "repository-root", language, metadata={"evidence": "native-source-files"}))
    return children


def unit_nodes(source: Path, repo: Path, language: str | None) -> list[dict[str, Any]]:
    result: list[dict[str, Any]] = []
    dirs = [p for p in sorted(source.iterdir(), key=lambda p: p.name.lower()) if p.is_dir() and p.name not in IGNORED]
    files = [p for p in sorted(source.iterdir(), key=lambda p: p.name.lower()) if p.is_file()]
    if language == "rust":
        for p in dirs:
            if (p / "Cargo.toml").is_file():
                result.append(node("UNIT", p.name, p.relative_to(repo).as_posix(), "crate", language, module_nodes(p, repo, language), {"evidence": "Cargo.toml"}))
    elif language == "node":
        for p in dirs:
            if (p / "package.json").is_file():
                result.append(node("UNIT", p.name, p.relative_to(repo).as_posix(), "package/workspace-member", language, module_nodes(p, repo, language), {"evidence": "package.json"}))
    elif language == "go":
        for p in dirs:
            result.append(node("UNIT", p.name, p.relative_to(repo).as_posix(), "package-or-command", language, module_nodes(p, repo, language), {"evidence": "Go source grouping"}))
    elif language == "python":
        for p in dirs:
            if (p / "__init__.py").exists() or (p / "pyproject.toml").is_file():
                result.append(node("UNIT", p.name, p.relative_to(repo).as_posix(), "package", language, module_nodes(p, repo, language), {"evidence": "Python package"}))
    elif language in {"java", "kotlin"}:
        for p in dirs:
            result.append(node("UNIT", p.name, p.relative_to(repo).as_posix(), "module/package", language, module_nodes(p, repo, language), {"evidence": "native source grouping"}))
    if not result and (dirs or files):
        result.append(node("UNIT", source.name, source.relative_to(repo).as_posix(), "source-group", language, module_nodes(source, repo, language), {"confidence": "low", "evidence": "fallback source grouping"}))
    return result


def module_nodes(unit: Path, repo: Path, language: str | None) -> list[dict[str, Any]]:
    children = [p for p in sorted(unit.iterdir(), key=lambda p: p.name.lower()) if p.is_dir() and p.name not in IGNORED]
    if children:
        return [node("MODULE", p.name, p.relative_to(repo).as_posix(), "directory-group", language, component_nodes(p, repo, language)) for p in children]
    return [node("MODULE", unit.name, unit.relative_to(repo).as_posix(), "native-group", language, component_nodes(unit, repo, language), {"confidence": "low"})]


def component_nodes(module: Path, repo: Path, language: str | None) -> list[dict[str, Any]]:
    result: list[dict[str, Any]] = []
    for p in sorted(module.iterdir(), key=lambda p: p.name.lower()):
        if p.name in IGNORED:
            continue
        if p.is_file() and p.suffix.lower() in SOURCE_EXTENSIONS:
            result.append(node("COMPONENT", p.stem, p.relative_to(repo).as_posix(), "source-file", language, element_nodes(p, repo, language), {"evidence": "source-file"}))
        elif p.is_dir():
            result.append(node("COMPONENT", p.name, p.relative_to(repo).as_posix(), "directory-component", language, element_nodes(p, repo, language)))
    return result


def element_nodes(path: Path, repo: Path, language: str | None) -> list[dict[str, Any]]:
    rel = path.relative_to(repo).as_posix()
    if not path.is_file():
        implementation = node("IMPLEMENTATION", path.name, rel, "directory-implementation", language, metadata={"evidence": "directory-boundary", "confidence": "low"})
        return [node("ELEMENT", path.name, rel, "native-construct-group", language, [implementation], {"evidence": "directory-boundary", "confidence": "low"})]

    symbols = parse_symbols(path, parser_language(path, language))
    if symbols:
        return [
            node(
                "ELEMENT",
                symbol["name"],
                rel,
                symbol["kind"],
                language,
                [
                    node(
                        "IMPLEMENTATION",
                        symbol["name"],
                        rel,
                        "symbol-span",
                        language,
                        metadata={
                            "symbol": symbol["name"],
                            "kind": symbol["kind"],
                            "span": {"line_start": symbol["line_start"], "line_end": symbol["line_end"]},
                            "evidence": "language-parser",
                        },
                    )
                ],
                {
                    "symbol": symbol["name"],
                    "kind": symbol["kind"],
                    "confidence": symbol["confidence"],
                    "evidence": symbol["evidence"],
                    "span": {"line_start": symbol["line_start"], "line_end": symbol["line_end"]},
                },
            )
            for symbol in symbols
        ]

    implementation = node("IMPLEMENTATION", path.name, rel, "source-file", language, metadata={"evidence": "file-boundary", "confidence": "low"})
    return [node("ELEMENT", path.stem, rel, "source-construct", language, [implementation], {"confidence": "low", "evidence": "parser-no-symbols"})]


def discover(workspace: Path, universe_id: str = "universe") -> dict[str, Any]:
    ecosystems: list[dict[str, Any]] = []
    if not workspace.is_dir():
        return {"level": "UNIVERSE", "id": universe_id, "name": universe_id, "path": ".", "children": [], "metadata": {"read_only": True, "canonical_levels": LEVELS}}
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
    return {"level": "UNIVERSE", "id": universe_id, "name": universe_id, "path": ".", "children": ecosystems, "metadata": {"read_only": True, "canonical_levels": LEVELS}}


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
