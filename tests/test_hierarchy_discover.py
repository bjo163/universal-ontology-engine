from __future__ import annotations

import json
import tempfile
import unittest
from pathlib import Path
import sys

ROOT = Path(__file__).resolve().parents[1]
sys.path.insert(0, str(ROOT / "tools"))
import hierarchy_discover  # type: ignore  # noqa: E402
from symbol_parser import parse_symbols  # type: ignore  # noqa: E402


class HierarchyDiscoveryTests(unittest.TestCase):
    def test_discovers_complete_semantic_chain_without_canonical_directories(self) -> None:
        with tempfile.TemporaryDirectory() as temp:
            workspace = Path(temp)
            repo = workspace / "ecosystem-rocksoul" / "rocksoul-ui"
            source = repo / "src"
            module = source / "ui"
            module.mkdir(parents=True)
            (repo / ".git").mkdir()
            (repo / "package.json").write_text('{"name":"rocksoul-ui"}', encoding="utf-8")
            (module / "app.tsx").write_text(
                "export function App() {\n  return null\n}\n\nexport function Boot() {\n  return App()\n}\n",
                encoding="utf-8",
            )

            tree = hierarchy_discover.discover(workspace)
            ecosystem = tree["children"][0]
            project = ecosystem["children"][0]
            repo_node = project["children"][0]
            source_node = repo_node["children"][0]
            unit_node = source_node["children"][0]
            module_node = unit_node["children"][0]
            component_node = module_node["children"][0]
            elements = component_node["children"]

            self.assertEqual(tree["level"], "UNIVERSE")
            self.assertEqual(repo_node["level"], "REPOSITORY")
            self.assertEqual(source_node["level"], "SOURCE")
            self.assertEqual(unit_node["level"], "UNIT")
            self.assertEqual(module_node["level"], "MODULE")
            self.assertEqual(component_node["level"], "COMPONENT")
            self.assertEqual([item["level"] for item in elements], ["ELEMENT", "ELEMENT"])
            self.assertEqual([item["name"] for item in elements], ["App", "Boot"])
            self.assertEqual(elements[0]["metadata"]["evidence"], "language-parser")
            self.assertEqual(elements[0]["metadata"]["span"], {"line_start": 1, "line_end": 3})
            self.assertEqual(elements[1]["metadata"]["span"], {"line_start": 5, "line_end": 7})
            self.assertEqual(elements[0]["children"][0]["level"], "EXECUTION")
            self.assertEqual(elements[0]["children"][0]["metadata"]["span"], {"line_start": 1, "line_end": 3})
            self.assertEqual(unit_node["language"], "node")
            self.assertFalse((repo / "unit").exists())
            self.assertEqual(tree["metadata"]["canonical_level_count"], 49)

    def test_python_symbol_spans_are_detected(self) -> None:
        with tempfile.TemporaryDirectory() as temp:
            path = Path(temp) / "main.py"
            path.write_text(
                "class Service:\n    def run(self):\n        return True\n\ndef helper():\n    return False\n",
                encoding="utf-8",
            )
            symbols = parse_symbols(path, "python")
            self.assertEqual([(s["kind"], s["name"]) for s in symbols], [("class", "Service"), ("function", "run"), ("function", "helper")])
            self.assertEqual(symbols[0]["line_start"], 1)
            self.assertEqual(symbols[0]["line_end"], 3)
            self.assertEqual(symbols[-1]["line_start"], 5)
            self.assertEqual(symbols[-1]["line_end"], 6)

    def test_rust_symbol_spans_are_detected(self) -> None:
        with tempfile.TemporaryDirectory() as temp:
            path = Path(temp) / "lib.rs"
            path.write_text(
                "pub struct Engine;\n\npub fn run() {\n}\n",
                encoding="utf-8",
            )
            symbols = parse_symbols(path, "rust")
            self.assertEqual([(s["kind"], s["name"]) for s in symbols], [("struct", "Engine"), ("function", "run")])
            self.assertEqual(symbols[1]["line_start"], 3)
            self.assertEqual(symbols[1]["line_end"], 4)

    def test_json_is_serializable(self) -> None:
        with tempfile.TemporaryDirectory() as temp:
            workspace = Path(temp) 
            (workspace / "ecosystem-test").mkdir()
            output = hierarchy_discover.discover(workspace)
            json.dumps(output)
            self.assertEqual(output["metadata"]["read_only"], True)
            self.assertEqual(output["metadata"]["canonical_level_count"], 49)


if __name__ == "__main__":
    unittest.main()
