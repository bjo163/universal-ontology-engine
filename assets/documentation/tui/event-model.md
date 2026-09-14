# OX-DX TUI Event Model

Conceptual event classes for a future event loop.

## Keyboard event

Examples:
- navigation;
- input;
- help;
- quit;
- inspect;
- trace;
- expand/collapse;
- resolution move.

UI reaction:
- update focus/selection/view/input;
- redraw affected composition.

## Mouse event

Optional:
- click focus/select;
- scroll;
- limited resize/drag if implemented.

Mouse never owns unique actions.

## Resize

Reaction:
- recompute layout profile;
- preserve focus/selection/resolution/scroll where practical;
- collapse secondary regions as needed.

## Data update

New discovery/graph/evidence data arrived.

Reaction:
- update data model;
- preserve current inspection anchor;
- show new information without stealing focus unless critical.

## Observation update

New observation/runtime evidence.

Reaction:
- update observation rail;
- do not convert observation into ownership/containment;
- no mandatory sound.

## Resolution update

User/system changes current resolution context.

Reaction:
- update spine/marker;
- reveal supported detail;
- preserve broader context/history.

## Diagnostic update

New info/warning/error/fatal.

Reaction:
- add diagnostic;
- update status count;
- steal focus only for policy-defined critical/fatal cases.

## Timer/tick

Only needed for bounded animation or time-aware data.

No idle animation tick is required.

## Principle

Events change presentation state.

They do not silently rewrite ontology/graph semantics.
