# OX-DX TUI Conceptual State Model

This is UX state, not a prescribed Rust struct layout.

## Context

What bounded workspace/source/graph scope is active.

Fields conceptually:
- scope identifier;
- context label;
- optional history parent.

## Selection

Current inspection subject(s).

Distinct from focus.

## Focus

Current keyboard/pointer interaction target:
- region;
- row/node/hop;
- input cursor.

## Resolution

Current ontology level/context:
- level 1–49;
- exact TYPE;
- neighboring level availability/materialization summaries where known.

## View

Active surface:
- startup;
- discover;
- inspect;
- evidence;
- graph;
- trace;
- observe;
- provenance;
- diagnostics;
- verify;
- help/command.

## Filter

Visible subset constraints:
- relation kind;
- diagnostic severity;
- text query;
- other supported filter.

Filters must be explicit.

## Diagnostic

Current diagnostic collection + focused item.

## Mode

Future interaction mode:
- DISCOVER
- INSPECT
- GRAPH
- TRACE
- OBSERVE
- RESOLVE
- VERIFY
- DIAGNOSTIC

## Cross-state rules

- focus can change without selection changing;
- resize does not change semantic selection/resolution;
- view can change while context remains;
- filters do not change graph semantics;
- mode does not create technical state;
- unknown remains data/status state, not absence of UI state.
