#!/usr/bin/env python3
"""Compatibility entry point for universal hierarchy discovery."""
from __future__ import annotations

import sys
from pathlib import Path

TOOLS = Path(__file__).resolve().parent
if str(TOOLS) not in sys.path:
    sys.path.insert(0, str(TOOLS))

from hierarchy_discover import main

if __name__ == "__main__":
    raise SystemExit(main())
