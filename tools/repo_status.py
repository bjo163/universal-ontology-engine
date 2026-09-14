#!/usr/bin/env python3
"""Render repository automation status from explicit CI inputs."""
from __future__ import annotations

import argparse
import json
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
SELF_PATH = ROOT / ".ontology" / "self.json"
DEFAULT_OUTPUT = ROOT / ".ontology" / "status.json"


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--main-version", required=True)
    parser.add_argument("--dev-version", required=True)
    parser.add_argument("--stable-release", default="")
    parser.add_argument("--dev-release", default="")
    parser.add_argument("--branch-relation", required=True)
    parser.add_argument("--ci-state", default="unknown")
    parser.add_argument("--output", type=Path, default=DEFAULT_OUTPUT)
    args = parser.parse_args()

    self_snapshot = json.loads(SELF_PATH.read_text(encoding="utf-8"))
    healthy = self_snapshot.get("status") == "healthy" and args.branch_relation != "diverged"
    data = {
        "schema": "universal-ontology-engine/repository-status-v1",
        "repository": "universal-ontology-engine",
        "health": "healthy" if healthy else "degraded",
        "self": self_snapshot.get("status", "unknown"),
        "ci": args.ci_state,
        "versions": {"main": args.main_version, "dev": args.dev_version},
        "releases": {"stable": args.stable_release or None, "dev": args.dev_release or None},
        "branches": {"model": "feature/* -> dev -> main", "relation": args.branch_relation},
        "automation": {
            "stable_back_sync": "enabled",
            "changelog_sync": "enabled",
            "issue_gate": "enabled",
            "self_steward": "enabled",
            "external_scan": "manual_gated"
        }
    }
    args.output.parent.mkdir(parents=True, exist_ok=True)
    args.output.write_text(json.dumps(data, indent=2, sort_keys=True) + "\n", encoding="utf-8")
    print(f"repository status written to {args.output}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
