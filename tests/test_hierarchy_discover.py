from __future__ import annotations

import json
import tempfile
import unittest
from pathlib import Path

import sys

ROOT = Path(__file__).resolve().parents[1]
sys.path.insert(0, str(ROOT / "tools"))
import hierarchy_discover  # type: ignore  # noqa: E402


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
            (module / "app.tsx").write_text("export function App() { return null }\n", encoding="utf-8")

            tree = hierarchy_discover.discover(workspace)
            self.assertEqual(tree["level"], "UNIVERSE")
            ecosystem = tree["children"][0]
            self.assertEqual(ecosystem["level"], "ECOSYSTEM")
            project = ecosystem["children"][0]
            repo_node = project["children"][0]
            source_node = repo_node["children"][0]
            unit_node = source_node["children"][0]
            module_node = unit_node["children"][0]
            component_node = module_node["children"][0]
            element_node = component_node["children"][0]
            implementation_node = element_node["children"][0]

            self.assertEqual(repo_node["level"], "REPOSITORY")
            self.assertEqual(source_node["level"], "SOURCE")
            self.assertEqual(unit_node["level"], "UNIT")
            self.assertEqual(module_node["level"], "MODULE")
            self.assertEqual(component_node["level"], "COMPONENT")
            self.assertEqual(element_node["level"], "ELEMENT")
            self.assertEqual(implementation_node["level"], "IMPLEMENTATION")
            self.assertEqual(unit_node["language"], "node")
            self.assertFalse((repo / "unit").exists())

    def test_json_is_serializable(self) -> None:
        with tempfile.TemporaryDirectory() as temp:
            workspace = Path(temp)
            (workspace / "ecosystem-test").mkdir()
            output = hierarchy_discover.discover(workspace)
            json.dumps(output)
            self.assertEqual(output["metadata"]["read_only"], True)


if __name__ == "__main__":
    unittest.main()
