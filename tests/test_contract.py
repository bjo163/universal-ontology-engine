#!/usr/bin/env python3
"""Small stdlib-only tests for the reference Universe manifest."""

from __future__ import annotations

import json
import re
import unittest
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
MANIFEST = ROOT / "specifications" / "universe-contract.instance.json"
ID_PATTERN = re.compile(r"^[a-z0-9][a-z0-9-]*$")


class UniverseContractTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls) -> None:
        cls.data = json.loads(MANIFEST.read_text(encoding="utf-8"))

    def test_contract_version(self) -> None:
        self.assertEqual(self.data["contract_version"], "0.2")

    def test_universe_identity(self) -> None:
        universe = self.data["universe"]
        self.assertTrue(ID_PATTERN.fullmatch(universe["id"]))
        self.assertTrue(universe["name"])

    def test_ecosystems_have_unique_ids_and_paths(self) -> None:
        ecosystems = self.data["ecosystems"]
        ids = [item["id"] for item in ecosystems]
        paths = [item["path"] for item in ecosystems]
        self.assertEqual(len(ids), len(set(ids)))
        self.assertEqual(len(paths), len(set(paths)))
        for item in ecosystems:
            self.assertTrue(ID_PATTERN.fullmatch(item["id"]))
            self.assertTrue(item["path"])


if __name__ == "__main__":
    unittest.main()
