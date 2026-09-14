import json
import tempfile
import unittest
from pathlib import Path
import sys

ROOT = Path(__file__).resolve().parents[1]
sys.path.insert(0, str(ROOT / "tools"))
import universe_cli  # type: ignore  # noqa: E402


class UniverseCliTests(unittest.TestCase):
    def setUp(self) -> None:
        self.manifest = {
            "contract_version": "0.2",
            "universe": {"id": "universe", "name": "Universe"},
            "ecosystems": [
                {"id": "rocksoul", "name": "ROCKSOUL", "path": "ecosystem-rocksoul", "lifecycle": "active"},
                {"id": "moonwitness", "name": "MoonWitness", "path": "ecosystem-moonwitness"},
            ],
        }

    def test_validate_accepts_reference_shape(self) -> None:
        self.assertEqual(universe_cli.validate(self.manifest), [])

    def test_validate_rejects_duplicate_ids_and_paths(self) -> None:
        invalid = json.loads(json.dumps(self.manifest))
        invalid["ecosystems"].append({"id": "rocksoul", "name": "Other", "path": "ecosystem-rocksoul"})
        errors = universe_cli.validate(invalid)
        self.assertTrue(any("duplicate ecosystem id" in error for error in errors))
        self.assertTrue(any("duplicate ecosystem path" in error for error in errors))

    def test_discovery_finds_registered_and_unknown(self) -> None:
        with tempfile.TemporaryDirectory() as temp:
            workspace = Path(temp)
            (workspace / "ecosystem-rocksoul").mkdir()
            (workspace / "ecosystem-unknown").mkdir()
            result_path = workspace / "result.json"
            original_print_json = universe_cli.print_json
            try:
                universe_cli.print_json = lambda value: result_path.write_text(json.dumps(value), encoding="utf-8")
                code = universe_cli.command_discover(self.manifest, workspace, True)
            finally:
                universe_cli.print_json = original_print_json
            self.assertEqual(code, 0)
            result = json.loads(result_path.read_text(encoding="utf-8"))
            self.assertEqual([item["path"] for item in result["discovered"]], ["ecosystem-rocksoul", "ecosystem-unknown"])
            self.assertEqual(result["missing"], ["ecosystem-moonwitness"])
            self.assertEqual(result["unknown"], ["ecosystem-unknown"])

    def test_status_distinguishes_missing_and_present(self) -> None:
        with tempfile.TemporaryDirectory() as temp:
            workspace = Path(temp)
            rocksoul = workspace / "ecosystem-rocksoul"
            rocksoul.mkdir()
            (rocksoul / ".git").mkdir()
            captured = []
            original_print_json = universe_cli.print_json
            try:
                universe_cli.print_json = captured.append
                code = universe_cli.command_status(self.manifest, workspace, True)
            finally:
                universe_cli.print_json = original_print_json
            self.assertEqual(code, 0)
            result = captured[0]
            self.assertTrue(result[0]["git_repository"])
            self.assertFalse(result[1]["exists"])


if __name__ == "__main__":
    unittest.main()
