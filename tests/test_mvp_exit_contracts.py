import json
import unittest
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]


class MvpExitContractsTest(unittest.TestCase):
    def load_json(self, path: str):
        return json.loads((ROOT / path).read_text(encoding="utf-8"))

    def test_discovery_schema_is_versioned_and_additive(self):
        schema = self.load_json("schemas/discovery-output-v1.schema.json")
        self.assertEqual(
            schema["properties"]["schema"]["const"],
            "universal-ontology-engine/discovery-v1",
        )
        required = set(schema["required"])
        self.assertTrue(
            {
                "schema",
                "ontology",
                "workspace",
                "read_only",
                "mode",
                "nodes",
                "edges",
                "nodes_by_level",
                "observations",
            }.issubset(required)
        )
        self.assertTrue(schema["additionalProperties"])
        self.assertEqual(
            set(schema["properties"]["mode"]["enum"]),
            {"structural", "rust-ast", "syntax-projection"},
        )

    def test_language_capability_matrix_covers_phase_six(self):
        matrix = self.load_json("specifications/language-capabilities-v1.json")
        self.assertEqual(matrix["phase"], 6)
        self.assertEqual(
            set(matrix["languages"]),
            {"rust", "typescript", "javascript", "python", "go", "java", "kotlin"},
        )
        self.assertEqual(matrix["languages"]["rust"]["constructs"]["function"], "FUNCTION")
        self.assertEqual(matrix["languages"]["typescript"]["constructs"]["interface"], "ENTITY")
        self.assertIn("semantic equivalence across languages", matrix["non_goals"])

    def test_external_reference_matrix_is_pinned(self):
        matrix = self.load_json("specifications/external-reference-matrix-v1.json")
        refs = matrix["references"]
        self.assertEqual(len(refs), 4)
        self.assertEqual(
            {item["category"] for item in refs},
            {"frontend-ui", "rust-heavy", "mixed-language", "small-sparse-product"},
        )
        for item in refs:
            self.assertRegex(item["ref"], r"^[0-9a-f]{40}$")
            self.assertIn(item["mode"], {"syntax", "rust-ast"})
            self.assertTrue(item["assertions"])

    def test_cli_contract_has_non_overlapping_exit_classes(self):
        contract = (ROOT / "docs/contracts/cli-v1.md").read_text(encoding="utf-8")
        for row in [
            "`0` | success",
            "`1` | health gate",
            "`2` | input/configuration",
            "`3` | engine/invariant",
        ]:
            self.assertIn(row, contract)
        self.assertIn("--rust-ast", contract)
        self.assertIn("--syntax", contract)
        self.assertIn("JSON only", contract)

    def test_safety_contract_matches_preflight_constants(self):
        source = (ROOT / "crates/ontology-cli/src/preflight.rs").read_text(encoding="utf-8")
        self.assertIn("2 * 1024 * 1024", source)
        self.assertIn("100_000", source)
        self.assertIn("HARD_MAX_DEPTH: usize = 256", source)
        self.assertIn("file_type.is_symlink()", source)

    def test_release_provenance_requires_lockfile_and_separate_versions(self):
        self.assertTrue((ROOT / "Cargo.lock").is_file())
        contract = (ROOT / "docs/contracts/release-provenance-v1.md").read_text(encoding="utf-8")
        self.assertIn("ontology contract: `1.0.0`", contract)
        self.assertIn("`Cargo.lock` is committed", contract)
        self.assertIn("stable tag", contract)


if __name__ == "__main__":
    unittest.main()
