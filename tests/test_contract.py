#!/usr/bin/env python3
"""Stdlib-only tests for Universal Ontology v1.0."""

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
    "UNIVERSE", "CREATION", "ORDER", "REALITY", "REALM", "WORLD", "DOMAIN",
    "ECOSYSTEM", "ORGANIZATION", "COMMUNITY", "REGION", "ENVIRONMENT", "NETWORK", "CONTEXT",
    "PURPOSE", "MISSION", "OBJECTIVE", "PROGRAM", "PROJECT", "PRODUCT", "SYSTEM",
    "REPOSITORY", "SOURCE", "UNIT", "MODULE", "SUBSYSTEM", "COMPONENT", "ELEMENT",
    "SYMBOL", "ENTITY", "PROPERTY", "RELATION", "OPERATION", "FUNCTION", "BEHAVIOR",
    "STATE", "EVENT", "PROCESS", "FLOW", "TRANSITION", "ACTION", "EXECUTION",
    "INSTRUCTION", "EXPRESSION", "VALUE", "DATA", "TOKEN", "CHARACTER", "BIT",
]


class UniverseContractTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls) -> None:
        cls.universe = json.loads(UNIVERSE.read_text(encoding="utf-8"))
        cls.contract = json.loads(CONTRACT.read_text(encoding="utf-8"))
        cls.foundation = json.loads(FOUNDATION.read_text(encoding="utf-8"))

    def test_contract_version_and_shape(self) -> None:
        self.assertEqual(self.universe["contract_version"], "1.0")
        self.assertEqual(self.contract["contract_version"], "1.0.0")
        self.assertEqual(self.foundation["version"], "1.0.0")
        self.assertEqual(self.contract["ontology"], "Universal Ontology v1.0.0")
        self.assertEqual(self.foundation["ontology"], "Universal Ontology v1.0.0")
        self.assertEqual(self.contract["hierarchy"], EXPECTED_HIERARCHY)
        self.assertEqual(self.foundation["hierarchy"], EXPECTED_HIERARCHY)
        self.assertEqual(len(EXPECTED_HIERARCHY), 49)
        self.assertEqual(self.contract["zones"], 7)
        self.assertEqual(self.contract["levelsPerZone"], 7)
        self.assertEqual(self.contract["canonicalLevels"], 49)

    def test_optional_levels(self) -> None:
        self.assertEqual(self.foundation["optionalLevels"], ["ORGANIZATION", "DOMAIN"])

    def test_single_foundation_rules(self) -> None:
        rules = self.foundation["rules"]
        self.assertTrue(rules["singleFoundation"])
        self.assertFalse(rules["semanticDirectories"])
        self.assertEqual(rules["externalSystems"], "relationship")
        self.assertTrue(rules["stableIdentity"])
        self.assertTrue(rules["typeIsCanonicalLevel"])
        self.assertTrue(rules["kindIsSpecialization"])
        self.assertTrue(rules["projectionDistinctFromContainment"])
        self.assertTrue(rules["representationDistinctFromSemantics"])
        self.assertTrue(rules["intermediateLevelsMayBeUnmaterialized"])

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
