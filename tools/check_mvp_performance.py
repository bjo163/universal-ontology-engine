#!/usr/bin/env python3
import argparse
import json
from pathlib import Path


def load(path: Path):
    return json.loads(path.read_text(encoding="utf-8"))


def index_cases(payload):
    return {case["name"]: case for case in payload["cases"]}


def main():
    parser = argparse.ArgumentParser(description="Check OX-DX MVP benchmark against the frozen broad regression budget")
    parser.add_argument("--baseline", required=True, type=Path)
    parser.add_argument("--current", required=True, type=Path)
    parser.add_argument("--factor", type=float, default=3.0)
    args = parser.parse_args()

    baseline = load(args.baseline)
    current = load(args.current)
    if baseline.get("schema") != "universal-ontology-engine/performance-baseline-v1":
        raise SystemExit("unsupported baseline schema")
    if current.get("schema") != baseline.get("schema"):
        raise SystemExit("current benchmark schema does not match baseline")

    baseline_cases = index_cases(baseline)
    current_cases = index_cases(current)
    failures = []
    for name, expected in baseline_cases.items():
        actual = current_cases.get(name)
        if actual is None:
            failures.append(f"missing benchmark case: {name}")
            continue
        median_limit = expected["median_ms"] * args.factor
        output_limit = expected["output_bytes"] * args.factor
        if actual["median_ms"] > median_limit:
            failures.append(
                f"{name}: median {actual['median_ms']}ms exceeds {median_limit:.3f}ms ({args.factor}x baseline)"
            )
        if actual["output_bytes"] > output_limit:
            failures.append(
                f"{name}: output {actual['output_bytes']} bytes exceeds {output_limit:.0f} bytes ({args.factor}x baseline)"
            )
        if actual["mode"] != expected["mode"]:
            failures.append(f"{name}: mode changed from {expected['mode']} to {actual['mode']}")
        if actual["schema"] != expected["schema"]:
            failures.append(f"{name}: discovery schema changed")

    if failures:
        for failure in failures:
            print(f"PERFORMANCE REGRESSION: {failure}")
        raise SystemExit(1)

    print(f"PASS: {len(baseline_cases)} benchmark case(s) within {args.factor}x MVP regression budget")


if __name__ == "__main__":
    main()
