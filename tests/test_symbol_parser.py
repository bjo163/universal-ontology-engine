from __future__ import annotations

import tempfile
import unittest
from pathlib import Path
import sys

ROOT = Path(__file__).resolve().parents[1]
sys.path.insert(0, str(ROOT / "tools"))
from symbol_parser import parse_symbols  # type: ignore  # noqa: E402


class SymbolParserTests(unittest.TestCase):
    def test_typescript_function_and_arrow(self) -> None:
        with tempfile.TemporaryDirectory() as temp:
            path = Path(temp) / "app.ts"
            path.write_text(
                "export function start() {\n  return true\n}\nconst stop = () => false\n",
                encoding="utf-8",
            )
            symbols = parse_symbols(path)
            self.assertEqual([(s["kind"], s["name"]) for s in symbols], [("function", "start"), ("arrow-function", "stop")])
            self.assertTrue(all(s["evidence"] == "language-parser" for s in symbols))

    def test_go_function_and_type(self) -> None:
        with tempfile.TemporaryDirectory() as temp:
            path = Path(temp) / "main.go"
            path.write_text(
                "package main\n\ntype Engine struct {}\n\nfunc Run() {\n}\n",
                encoding="utf-8",
            )
            symbols = parse_symbols(path)
            self.assertEqual([(s["kind"], s["name"]) for s in symbols], [("type", "Engine"), ("function", "Run")])

    def test_java_class_interface_and_method(self) -> None:
        with tempfile.TemporaryDirectory() as temp:
            path = Path(temp) / "Engine.java"
            path.write_text(
                "public class Engine {\n  public void run() { }\n}\n",
                encoding="utf-8",
            )
            symbols = parse_symbols(path)
            self.assertTrue(any(s["name"] == "Engine" and s["kind"] == "class" for s in symbols))
            self.assertTrue(any(s["name"] == "run" and s["kind"] == "method" for s in symbols))


if __name__ == "__main__":
    unittest.main()
