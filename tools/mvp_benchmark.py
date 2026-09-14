#!/usr/bin/env python3
import argparse
import json
import platform
import resource
import statistics
import subprocess
import sys
import time
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
REGISTRY = ROOT / "specifications/universal-ontology-v1.0.json"


def run_case(binary: Path, name: str, workspace: Path, flags: list[str], runs: int):
    durations = []
    last = None
    output_bytes = 0
    for _ in range(runs):
        command = [
            str(binary),
            "--registry",
            str(REGISTRY),
            "discover",
            str(workspace),
            *flags,
        ]
        start = time.perf_counter()
        completed = subprocess.run(command, capture_output=True, text=True, check=False)
        elapsed_ms = (time.perf_counter() - start) * 1000.0
        if completed.returncode != 0:
            raise SystemExit(
                f"benchmark case {name!r} failed with {completed.returncode}: {completed.stderr}"
            )
        last = json.loads(completed.stdout)
        output_bytes = len(completed.stdout.encode("utf-8"))
        durations.append(elapsed_ms)

    assert last is not None
    return {
        "name": name,
        "workspace": str(workspace.relative_to(ROOT)) if workspace != ROOT else ".",
        "flags": flags,
        "runs": runs,
        "median_ms": round(statistics.median(durations), 3),
        "min_ms": round(min(durations), 3),
        "max_ms": round(max(durations), 3),
        "output_bytes": output_bytes,
        "nodes": last["nodes"],
        "edges": last["edges"],
        "observations": len(last["observations"]),
        "mode": last["mode"],
        "schema": last["schema"],
    }


def main():
    parser = argparse.ArgumentParser(description="Measure reproducible OX-DX MVP discovery baselines")
    parser.add_argument("--binary", required=True, type=Path)
    parser.add_argument("--output", required=True, type=Path)
    parser.add_argument("--runs", type=int, default=3)
    args = parser.parse_args()

    binary = args.binary.resolve()
    if args.runs < 1:
        raise SystemExit("--runs must be >= 1")

    cases = [
        ("fixture-rust", ROOT / "fixtures/mvp/rust", ["--rust-ast"]),
        ("fixture-typescript", ROOT / "fixtures/mvp/typescript", ["--syntax"]),
        ("fixture-python", ROOT / "fixtures/mvp/python", ["--syntax"]),
        ("self-structural-depth4", ROOT, ["--max-depth", "4"]),
    ]

    results = [run_case(binary, name, workspace, flags, args.runs) for name, workspace, flags in cases]
    rustc = subprocess.run(["rustc", "--version"], capture_output=True, text=True, check=False)
    payload = {
        "schema": "universal-ontology-engine/performance-baseline-v1",
        "ontology": "1.0.0",
        "environment": {
            "platform": platform.platform(),
            "machine": platform.machine(),
            "python": sys.version.split()[0],
            "rustc": rustc.stdout.strip() if rustc.returncode == 0 else "unknown",
        },
        "max_rss_kib_observed": resource.getrusage(resource.RUSAGE_CHILDREN).ru_maxrss,
        "cases": results,
        "policy": {
            "numbers_are_baselines_not_promises": True,
            "severe_regression_threshold": "3x median time or 3x output size on committed local fixtures",
            "network_repositories_are_measured_separately": True,
        },
    }
    args.output.write_text(json.dumps(payload, indent=2, sort_keys=True) + "\n", encoding="utf-8")
    print(json.dumps(payload, indent=2, sort_keys=True))


if __name__ == "__main__":
    main()
