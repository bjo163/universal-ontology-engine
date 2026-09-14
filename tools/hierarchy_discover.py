#!/usr/bin/env python3
"""Read-only semantic hierarchy discovery for a local workspace."""
from __future__ import annotations

import argparse
import json
import re
from pathlib import Path
from typing import Any

LEVELS = [
    "UNIVERSE", "ECOSYSTEM", "ORGANIZATION", "DOMAIN", "PROJECT", "REPOSITORY",
    "SOURCE", "UNIT", "MODULE", "COMPONENT", "ELEMENT", "IMPLEMENTATION",
]
OPTIONAL = {"ORGANIZATION", "DOMAIN"}
IGNORED = {
    ".git", ".next", ".turbo", "node_modules", "target", "dist", "build",
    "coverage", "__pycache__", ".venv", "venv", ".idea", ".vscode",
}
SOURCE_NAMES = {"src", "app", "apps", "packages", "crates", "cmd", "internal", "lib", "libs", "pkg"}
MANIFESTS = ("Cargo.toml", "package.json", "go.mod", "pyproject.toml", "pom.xml", "build.gradle", "build.gradle.kts")
ID_RE = re.compile(r"^[a-z0-9][a-z0-9-]*$")


def stable_id(level: str, name: str, path: str) -> str:
    raw = f"{level}:{path or name}".lower().replace('\\', '/')
    raw = re.sub(r"[^a-z0-9:/._-]+", "-", raw)
    return raw


def node(level: str, name: str, path: str, native_type: str | None = None, language: str | None = None, children: list[dict[str, Any]] | None = None, metadata: dict[str, Any] | None = None) -> dict[str, Any]:
    result: dict[str, Any] = {
        "level": level,
        "id": stable_id(level, name, path),
        "name": name,
        "path": path or ".",
    }
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
    return None


def is_repo(path: Path) -> bool:
    return (path / ".git").exists() or any((path / m).is_file() for m in MANIFESTS)


def source_nodes(repo: Path) -> list[dict[str, Any]]:
    children: list[dict[str, Any]] = []
    language = detect_language(repo)
    for child in sorted(repo.iterdir(), key=lambda p: p.name.lower()):
        if not child.is_dir() or child.name in IGNORED:
            continue
        if child.name in SOURCE_NAMES:
            unit_children = unit_nodes(child, repo, language)
            children.append(node("SOURCE", child.name, child.relative_to(repo).as_posix(), "directory", language, unit_children))
    if not children:
        native_files = [p.name for p in repo.iterdir() if p.is_file() and p.suffix in {".rs", ".ts", ".tsx", ".js", ".jsx", ".go", ".py", ".java", ".kt"}]
        if native_files:
            children.append(node("SOURCE", ".", ".", "repository-root", language, metadata={"evidence": "native-source-files"}))
    return children


def unit_nodes(source: Path, repo: Path, language: str | None) -> list[dict[str, Any]]:
    result: list[dict[str, Any]] = []
    dirs = [p for p in sorted(source.iterdir(), key=lambda p: p.name.lower()) if p.is_dir() and p.name not in IGNORED]
    files = [p for p in sorted(source.iterdir(), key=lambda p: p.name.lower()) if p.is_file()]
    if language == "rust":
        for p in dirs:
            cargo = p / "Cargo.toml"
            if cargo.is_file():
                result.append(node("UNIT", p.name, p.relative_to(repo).as_posix(), "crate", language, module_nodes(p, repo, language), {"evidence": "Cargo.toml"}))
    elif language == "node":
        for p in dirs:
            if (p / "package.json").is_file() or (source / "package.json").is_file():
                result.append(node("UNIT", p.name, p.relative_to(repo).as_posix(), "package/workspace-member", language, module_nodes(p, repo, language), {"evidence": "package.json"}))
    elif language == "go":
        for p in dirs:
            result.append(node("UNIT", p.name, p.relative_to(repo).as_posix(), "package-or-command", language, module_nodes(p, repo, language), {"evidence": "Go source grouping"}))
    elif language == "python":
        for p in dirs:
            if any((p / marker).exists() for marker in ("__init__.py", "pyproject.toml")):
                result.append(node("UNIT", p.name, p.relative_to(repo).as_posix(), "package", language, module_nodes(p, repo, language), {"evidence": "Python package"}))
    elif language == "java":
        for p in dirs:
            result.append(node("UNIT", p.name, p.relative_to(repo).as_posix(), "module/package", language, module_nodes(p, repo, language), {"evidence": "Java source grouping"}))
    if not result and (dirs or files):
        result.append(node("UNIT", source.name, source.relative_to(repo).as_posix(), "source-group", language, module_nodes(source, repo, language), {"confidence": "low", "evidence": "fallback source grouping"}))
    return result


def module_nodes(unit: Path, repo: Path, language: str | None) -> list[dict[str, Any]]:
    result: list[dict[str, Any]] = []
    children = [p for p in sorted(unit.iterdir(), key=lambda p: p.name.lower()) if p.is_dir() and p.name not in IGNORED]
    for p in children:
        result.append(node("MODULE", p.name, p.relative_to(repo).as_posix(), "directory-group", language, component_nodes(p, repo, language)))
    if not result:
        result.append(node("MODULE", unit.name, unit.relative_to(repo).as_posix(), "native-group", language, component_nodes(unit, repo, language), {"confidence": "low"}))
    return result


def component_nodes(module: Path, repo: Path, language: str | None) -> list[dict[str, Any]]:
    result: list[dict[str, Any]] = []
    for p in sorted(module.iterdir(), key=lambda p: p.name.lower()):
        if p.name in IGNORED:
            continue
        if p.is_file() and p.suffix in {".rs", ".ts", ".tsx", ".js", ".jsx", ".go", ".py", ".java", ".kt"}:
            result.append(node("COMPONENT", p.stem, p.relative_to(repo).as_posix(), "source-file", language, element_nodes(p, repo, language)))
        elif p.is_dir():
            result.append(node("COMPONENT", p.name, p.relative_to(repo).as_posix(), "directory-component", language, element_nodes(p, repo, language)))
    return result


def element_nodes(path: Path, repo: Path, language: str | None) -> list[dict[str, Any]]:
    if not path.is_file():
        return [node("ELEMENT", path.name, path.relative_to(repo).as_posix(), "native-construct-group", language, [node("IMPLEMENTATION", path.name, path.relative_to(repo).as_posix(), "directory-implementation", language)])]
    suffix = path.suffix
    native = {".rs": "function/type/impl", ".ts": "function/class/type", ".tsx": "component/function", ".js": "function/class", ".jsx": "component/function", ".go": "function/type", ".py": "function/class", ".java": "class/method", ".kt": "class/function"}.get(suffix, "source-construct")
    implementation = node("IMPLEMENTATION", path.name, path.relative_to(repo).as_posix(), "source-file", language, metadata={"evidence": "file-boundary", "note": "symbol-level refinement requires language parser"})
    return [node("ELEMENT", path.stem, path.relative_to(repo).as_posix(), native, language, [implementation], {"confidence": "medium"})]


def discover(workspace: Path, universe_id: str = "universe") -> dict[str, Any]:
    ecosystems: list[dict[str, Any]] = []
    for path in sorted(workspace.iterdir(), key=lambda p: p.name.lower()) if workspace.is_dir() else []:
        if not path.is_dir() or path.name.startswith("."):
            continue
        if path.name.startswith("ecosystem-"):
            eco_id = path.name.removeprefix("ecosystem-")
            projects: list[dict[str, Any]] = []
            for child in sorted(path.iterdir(), key=lambda p: p.name.lower()):
                if child.is_dir() and child.name not in IGNORED:
                    repositories: list[dict[str, Any]] = []
                    if is_repo(child):
                        repositories.append(node("REPOSITORY", child.name, child.relative_to(workspace).as_posix(), "git-or-native-project", detect_language(child), source_nodes(child)))
                    else:
                        for repo in child.iterdir() if child.is_dir() else []:
                            if repo.is_dir() and repo.name not in IGNORED and is_repo(repo):
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
