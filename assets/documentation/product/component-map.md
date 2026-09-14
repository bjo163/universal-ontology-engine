# OX-DX Component Family Map

This is design architecture, not implementation.

## NAVIGATION

- Global Navigation
- Context Navigation
- Resolution Navigation

Purpose: establish scope, orientation, and depth without endless nested menus.

## OBSERVATION

- Evidence Viewer
- Source Viewer
- Event Viewer
- Observation Marker

Purpose: expose what was seen and keep observation distinct from structural ownership.

## STRUCTURE

- Structure View
- Graph Explorer
- Relationship View

Purpose: expose structural and non-structural relationships without forcing one tree.

## RESOLUTION

- Resolution Spine
- Level Inspector
- Detail Surface

Purpose: move from broad context toward justified finer detail.

## PROVENANCE

- Source Location
- Trace Viewer
- Origin Panel
- Provenance Rail

Purpose: show where/how evidence came from without redefining identity.

## STATUS

Future product vocabulary:
- Observed
- Verified
- Derived
- Projected
- Unverified
- Unknown
- Experimental
- Gated
- Planned

These labels are product-design vocabulary and must only be used where the underlying data/status supports them.

## SYSTEM

- Inspector
- Command Surface
- Diagnostics
- Technical Table
- Raw Representation Viewer

## Component creation rule

Before a component exists, answer:

1. **WHAT IS THIS?**
2. **WHY DOES IT EXIST?**
3. **WHAT EVIDENCE DOES IT DISPLAY?**
4. **WHAT CONTEXT DOES IT REQUIRE?**
5. **WHAT RELATIONSHIPS DOES IT EXPOSE?**
6. **WHAT ACTIONS ARE SAFE?**

If a component cannot answer these, it should not exist merely because a UI library provides it.
