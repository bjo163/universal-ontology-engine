#!/usr/bin/env python3
"""Extract the issue referenced by a feature pull request."""
from __future__ import annotations

import argparse
import re

ISSUE_RE = re.compile(r"(?i)\b(?:close[sd]?|fix(?:e[sd])?|resolve[sd]?|refs?|references?)\s+#(\d+)\b")


def linked_issue(body: str) -> int | None:
    match = ISSUE_RE.search(body or "")
    return int(match.group(1)) if match else None


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--body", default="")
    args = parser.parse_args()
    issue = linked_issue(args.body)
    if issue is None:
        print("feature PR must contain an issue reference such as: Refs #123")
        return 1
    print(issue)
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
