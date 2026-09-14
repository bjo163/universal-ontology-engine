from __future__ import annotations

import sys
import unittest
from pathlib import Path

TOOLS = Path(__file__).resolve().parents[1] / "tools"
sys.path.insert(0, str(TOOLS))

from sync_readme import END, START, render_self_status, update_readme_text


class ReadmeSyncTests(unittest.TestCase):
    def snapshot(self, *, crates: int = 7) -> dict[str, object]:
        return {
            "repository": "universal-ontology-engine",
            "status": "healthy",
            "ontology": "1.0.0",
            "canonical_levels": 49,
            "workspace_crates": crates,
            "rust_sources": 8,
            "read_only": True,
        }

    def test_inserts_generated_block_before_canonical_levels(self) -> None:
        readme = "# Title\n\nIntro.\n\n## Canonical levels\n\nBody.\n"
        result = update_readme_text(readme, render_self_status(self.snapshot()))
        self.assertEqual(result.count(START), 1)
        self.assertEqual(result.count(END), 1)
        self.assertLess(result.index(START), result.index("## Canonical levels"))
        self.assertIn("| Workspace crates | **7** |", result)

    def test_replaces_generated_block_without_duplication(self) -> None:
        readme = "# Title\n\n" + render_self_status(self.snapshot()) + "\n\n## Canonical levels\n"
        result = update_readme_text(readme, render_self_status(self.snapshot(crates=8)))
        self.assertEqual(result.count(START), 1)
        self.assertEqual(result.count(END), 1)
        self.assertIn("| Workspace crates | **8** |", result)
        self.assertNotIn("| Workspace crates | **7** |", result)

    def test_rejects_incomplete_marker_pair(self) -> None:
        with self.assertRaises(ValueError):
            update_readme_text("# Title\n\n" + START + "\n", render_self_status(self.snapshot()))


if __name__ == "__main__":
    unittest.main()
