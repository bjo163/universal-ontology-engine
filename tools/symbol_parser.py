#!/usr/bin/env python3
"""Small dependency-free symbol span parser for common languages.

This is intentionally conservative: it reports high-confidence declarations and
never claims AST-level accuracy. Unknown constructs are simply omitted.
"""
from __future__ import annotations

import re
from pathlib import Path
from typing import Any

LANG_BY_SUFFIX = {
    ".rs": "rust", ".ts": "typescript", ".tsx": "typescript", ".js": "javascript",
    ".jsx": "javascript", ".go": "go", ".py": "python", ".java": "java", ".kt": "kotlin",
}

PATTERNS: dict[str, list[tuple[str, str]]] = {
    "rust": [
        ("function", r"\b(?:pub(?:\([^)]*\))?\s+)?(?:async\s+)?fn\s+([A-Za-z_][A-Za-z0-9_]*)\b"),
        ("struct", r"\b(?:pub\s+)?struct\s+([A-Za-z_][A-Za-z0-9_]*)\b"),
        ("enum", r"\b(?:pub\s+)?enum\s+([A-Za-z_][A-Za-z0-9_]*)\b"),
        ("trait", r"\b(?:pub\s+)?trait\s+([A-Za-z_][A-Za-z0-9_]*)\b"),
        ("type", r"\b(?:pub\s+)?type\s+([A-Za-z_][A-Za-z0-9_]*)\b"),
        ("impl", r"\bimpl(?:<[^>]+>)?\s+([A-Za-z_][A-Za-z0-9_:]*)\b"),
    ],
    "typescript": [
        ("function", r"\b(?:export\s+)?(?:default\s+)?(?:async\s+)?function\s+([A-Za-z_$][\w$]*)\s*\("),
        ("class", r"\b(?:export\s+)?(?:abstract\s+)?class\s+([A-Za-z_$][\w$]*)\b"),
        ("interface", r"\b(?:export\s+)?interface\s+([A-Za-z_$][\w$]*)\b"),
        ("type", r"\b(?:export\s+)?type\s+([A-Za-z_$][\w$]*)\s*="),
        ("enum", r"\b(?:export\s+)?enum\s+([A-Za-z_$][\w$]*)\b"),
        ("arrow-function", r"\b(?:export\s+)?(?:const|let|var)\s+([A-Za-z_$][\w$]*)\s*=\s*(?:async\s+)?(?:\([^)]*\)|[A-Za-z_$][\w$]*)\s*=>"),
    ],
    "javascript": [
        ("function", r"\b(?:export\s+)?(?:default\s+)?(?:async\s+)?function\s+([A-Za-z_$][\w$]*)\s*\("),
        ("class", r"\b(?:export\s+)?class\s+([A-Za-z_$][\w$]*)\b"),
        ("arrow-function", r"\b(?:export\s+)?(?:const|let|var)\s+([A-Za-z_$][\w$]*)\s*=\s*(?:async\s+)?(?:\([^)]*\)|[A-Za-z_$][\w$]*)\s*=>"),
    ],
    "go": [
        ("function", r"\bfunc\s+([A-Za-z_][A-Za-z0-9_]*)\s*\("),
        ("method", r"\bfunc\s*\([^)]*\)\s+([A-Za-z_][A-Za-z0-9_]*)\s*\("),
        ("type", r"\btype\s+([A-Za-z_][A-Za-z0-9_]*)\s+(?:struct|interface)\b"),
    ],
    "python": [
        ("function", r"^[ \t]*(?:async[ \t]+)?def[ \t]+([A-Za-z_][A-Za-z0-9_]*)[ \t]*\("),
        ("class", r"^[ \t]*class[ \t]+([A-Za-z_][A-Za-z0-9_]*)\b"),
    ],
    "java": [
        ("class", r"\b(?:public\s+|protected\s+|private\s+|abstract\s+|final\s+)*class\s+([A-Za-z_][A-Za-z0-9_]*)\b"),
        ("interface", r"\b(?:public\s+|protected\s+|private\s+)*interface\s+([A-Za-z_][A-Za-z0-9_]*)\b"),
        ("enum", r"\b(?:public\s+|protected\s+|private\s+)*enum\s+([A-Za-z_][A-Za-z0-9_]*)\b"),
        ("method", r"\b(?:public\s+|protected\s+|private\s+|static\s+|final\s+|synchronized\s+|native\s+)*[A-Za-z_$][\w$<>\[\], ?]*\s+([A-Za-z_$][\w$]*)\s*\([^;{}]*\)\s*(?:\{|throws\b)"),
    ],
    "kotlin": [
        ("class", r"\b(?:public\s+|private\s+|internal\s+|protected\s+|data\s+|sealed\s+|open\s+)*class\s+([A-Za-z_][A-Za-z0-9_]*)\b"),
        ("interface", r"\b(?:public\s+|private\s+|internal\s+|protected\s+)*interface\s+([A-Za-z_][A-Za-z0-9_]*)\b"),
        ("function", r"\b(?:public\s+|private\s+|internal\s+|protected\s+|suspend\s+|inline\s+)*fun\s+([A-Za-z_][A-Za-z0-9_]*)\s*\("),
    ],
}


def _brace_end(lines: list[str], start: int) -> int:
    depth = 0
    seen = False
    for index in range(start, len(lines)):
        line = lines[index]
        depth += line.count("{") - line.count("}")
        if "{" in line:
            seen = True
        if seen and depth <= 0:
            return index + 1
    return len(lines)


def _python_end(lines: list[str], start: int) -> int:
    base = len(lines[start]) - len(lines[start].lstrip())
    last_code = start + 1
    for index in range(start + 1, len(lines)):
        stripped = lines[index].strip()
        if not stripped:
            continue
        indent = len(lines[index]) - len(lines[index].lstrip())
        if indent <= base:
            return last_code
        last_code = index + 1
    return last_code


def parse_symbols(path: Path, language: str | None = None) -> list[dict[str, Any]]:
    language = language or LANG_BY_SUFFIX.get(path.suffix.lower())
    if not language or language not in PATTERNS or not path.is_file():
        return []
    try:
        text = path.read_text(encoding="utf-8")
    except (OSError, UnicodeDecodeError):
        return []
    lines = text.splitlines()
    found: list[dict[str, Any]] = []
    occupied: set[tuple[int, str]] = set()
    for kind, pattern in PATTERNS[language]:
        for match in re.finditer(pattern, text, flags=re.MULTILINE):
            name = match.group(1)
            line_start = text.count("\n", 0, match.start()) + 1
            key = (line_start, name)
            if key in occupied:
                continue
            occupied.add(key)
            line_end = _python_end(lines, line_start - 1) if language == "python" else _brace_end(lines, line_start - 1)
            found.append({
                "name": name,
                "kind": kind,
                "line_start": line_start,
                "line_end": max(line_start, line_end),
                "language": language,
                "confidence": "high",
                "evidence": "language-parser",
            })
    found.sort(key=lambda item: (item["line_start"], item["name"], item["kind"]))
    return found
