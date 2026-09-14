# OX-DX TUI Screen Specifications

These are conceptual screens, not implementation.

## 1. Startup

Purpose: establish identity and actual initialized state.

Layout:
- identity;
- ontology/registry state;
- readiness.

Primary interaction: continue into workspace.

Secondary: inspect startup error if present.

Status: initialization/ready/error.

Responsive: same content, fewer metadata lines on compact terminals.

## 2. Workspace Discovery

Purpose: inspect discovered repository/workspace evidence.

Layout:
- context bar;
- discovery summary;
- object list/evidence;
- optional relation summary.

Primary: inspect discovered object.

Secondary: filter/source.

Status: DISCOVER mode.

Responsive: compact list only at 80×24.

## 3. Ontology Levels

Purpose: navigate 49-level resolution spine.

Layout:
- current zone/level;
- spine;
- definition/evidence.

Primary: broader/finer movement.

Secondary: inspect materialized evidence.

Status: RESOLVE mode.

Responsive: ±2 levels on compact, full zone/full map on wide.

## 4. Inspect

Purpose: inspect one selected object.

Layout:
- identity;
- evidence;
- relations;
- provenance summary.

Primary: inspect evidence/relations.

Secondary: trace/resolve.

Status: INSPECT.

Responsive: secondary rails collapse first.

## 5. Evidence

Purpose: source-first evidence inspection.

Layout:
- source;
- classification;
- relations;
- provenance.

Primary: scroll/select source evidence.

Secondary: inspect classification/relation.

Status: evidence status.

Responsive: source remains dominant.

## 6. Graph

Purpose: traverse typed graph relationships.

Layout:
- focused node;
- spatial topology;
- relation list;
- context.

Primary: neighbor traversal.

Secondary: trace/inspect.

Status: GRAPH.

Responsive: graph becomes linear relation list in compact mode.

## 7. Trace

Purpose: follow evidence/relation/provenance path.

Layout:
- ordered hops;
- exact edge kinds;
- source/target detail.

Primary: next/previous hop.

Secondary: inspect hop.

Status: TRACE.

Responsive: one hop at a time on compact.

## 8. Observation

Purpose: inspect observations separately from structure.

Layout:
- structural target;
- offset observation rail;
- provenance.

Primary: inspect observation.

Secondary: trace target/source.

Status: OBSERVE.

Responsive: observation becomes stacked but remains labeled separate.

## 9. Provenance

Purpose: show where/how evidence was observed.

Layout:
- source groups;
- span/revision/observer;
- warnings.

Primary: inspect source.

Secondary: switch source group.

Status: provenance context.

Responsive: one source group at a time.

## 10. Diagnostics

Purpose: inspect operational info/warning/error/fatal.

Layout:
- severity list;
- focused diagnostic;
- preserved evidence.

Primary: inspect diagnostic.

Secondary: filter severity.

Status: DIAG.

Responsive: list then detail.

## 11. Verification

Purpose: inspect named validation result.

Layout:
- check list;
- scope;
- failures/evidence.

Primary: inspect check.

Secondary: trace failed evidence.

Status: VERIFY/VERIFIED/ERROR.

Responsive: vertical list.

## 12. Command / Help

Purpose: expose context-relevant actions.

Layout:
- command input or help list;
- current context hint.

Primary: execute/navigate.

Secondary: search/filter help.

Status: current mode remains visible.

Responsive: modal/full-screen overlay on compact.
