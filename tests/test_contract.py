#!/usr/bin/env python3
"""Stdlib-only tests for the complete Universe Foundation contract."""

from __future__ import annotations

import json
import re
import unittest
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
UNIVERSE = ROOT / "universe.json"
CONTRACT = ROOT / "specifications" / "universe-contract.instance.json"
FOUNDATION = ROOT / "specifications" / "foundation-contract.instance.json"
ID_PATTERN = re.compile(r"^[a-z0-9][a-z0-9-]*$")
EXPECTED_HIERARCHY = [
    "UNIVERSE", "ECOSYSTEM", "ORGANIZATION", "DOMAIN", "PROJECT", "REPOSITORY",
    "SOURCE", "UNIT", "MODULE", "COMPONENT", "ELEMENT", "IMPLEMENTATION",
]


class UniverseContractTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls) -> None:
        cls.universe = json.loads(UNIVERSE.read_text(encoding="utf-8"))
        cls.contract = json.loads(CONTRACT.read_text(encoding="utf-8"))
        cls.foundation = json.loads(FOUNDATION.read_text(encoding="utf-8"))

    def test_contract_version(self) -> None:
        self.assertEqual(self.universe["contract_version"], "0.3")
        self.assertEqual(self.contract["contract_version"], "0.3")
        self.assertEqual(self.foundation["version"], "0.3")

    def test_complete_hierarchy(self) -> None:
        self.assertEqual(self.contract["hierarchy"], EXPECTED_HIERARCHY)
        self.assertEqual(self.foundation["hierarchy"], EXPECTED_HIERARCHY)
        self.assertEqual(self.foundation["optionalLayers"], ["ORGANIZATION", "DOMAIN"])

    def test_single_foundation_rules(self) -> None:
        rules = self.foundation["rules"]
        self.assertTrue(rules["singleFoundation"])
        self.assertFalse(rules["semanticDirectories"])
        self.assertEqual(rules["externalSystems"], "relationship")
        self.assertTrue(rules["stableIdentity"])
        self.assertTrue(rules["parentScopedIdentity"])

    def test_universe_identity(self) -> None:
        universe = self.universe["universe"]
        self.assertTrue(ID_PATTERN.fullmatch(universe["id"]))
        self.assertTrue(universe["name"])

    def test_ecosystems_have_unique_ids_and_paths(self) -> None:
        ecosystems = self.universe["ecosystems"]
        ids = [item["id"] for item in ecosystems]
        paths = [item["path"] for item in ecosystems]
        self.assertEqual(len(ids), len(set(ids)))
        self.assertEqual(len(paths), len(set(paths)))
        for item in ecosystems:
            self.assertTrue(ID_PATTERN.fullmatch(item["id"]))
            self.assertTrue(item["path"])
            self.assertNotIn("contract", item)


if __name__ == "__main__":
    unittest.main()
