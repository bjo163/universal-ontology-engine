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
            result = universe_cli.discover(self.manifest, workspace)
            self.assertEqual([item["path"] for item in result["discovered"]], ["ecosystem-rocksoul", "ecosystem-unknown"])
            self.assertEqual(result["missing"], ["ecosystem-moonwitness"])
            self.assertEqual(result["unknown"], ["ecosystem-unknown"])

    def test_status_distinguishes_missing_and_present(self) -> None:
        with tempfile.TemporaryDirectory() as temp:
            workspace = Path(temp)
            rocksoul = workspace / "ecosystem-rocksoul"
            rocksoul.mkdir()
            (rocksoul / ".git").mkdir()
            self.assertTrue((universe_cli.command_status(self.manifest, workspace, True) == 0))

    def test_doctor_accepts_matching_local_manifest(self) -> None:
        with tempfile.TemporaryDirectory() as temp:
            workspace = Path(temp)
            for item in self.manifest["ecosystems"]:
                path = workspace / item["path"]
                path.mkdir()
                (path / "ecosystem.json").write_text(
                    json.dumps({"id": item["id"], "name": item["name"]}),
                    encoding="utf-8",
                )
            captured: list[object] = []
            original_print_json = universe_cli.print_json
            try:
                universe_cli.print_json = captured.append
                code = universe_cli.command_doctor(self.manifest, workspace, True)
            finally:
                universe_cli.print_json = original_print_json
            self.assertEqual(code, 0)
            self.assertTrue(captured[0]["healthy"])

    def test_doctor_rejects_identity_mismatch(self) -> None:
        with tempfile.TemporaryDirectory() as temp:
            workspace = Path(temp)
            rocksoul = workspace / "ecosystem-rocksoul"
            rocksoul.mkdir()
            (rocksoul / "ecosystem.json").write_text(
                json.dumps({"id": "wrong-id", "name": "ROCKSOUL"}),
                encoding="utf-8",
            )
            moon = workspace / "ecosystem-moonwitness"
            moon.mkdir()
            (moon / "ecosystem.json").write_text(
                json.dumps({"id": "moonwitness", "name": "MoonWitness"}),
                encoding="utf-8",
            )
            captured: list[object] = []
            original_print_json = universe_cli.print_json
            try:
                universe_cli.print_json = captured.append
                code = universe_cli.command_doctor(self.manifest, workspace, True)
            finally:
                universe_cli.print_json = original_print_json
            self.assertEqual(code, 1)
            self.assertFalse(captured[0]["healthy"])


if __name__ == "__main__":
    unittest.main()
