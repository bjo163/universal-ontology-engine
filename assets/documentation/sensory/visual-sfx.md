# OX-DX Visual SFX Library

Visual effect family names are **visual behavior labels only**.

They are not ontology TYPEs, engine phases, or graph semantics.

## TRACE

Purpose: show that something was followed, observed, or connected.

Behavior:
- partial persistence;
- variable density;
- secondary rail;
- context-aware disappearance;
- stronger persistence near a confirmed anchor.

Persistence:
- routine trace: 196–490 ms;
- inspector/provenance trace may remain while the inspected relation is active.

Removal:
- context exit;
- focus change;
- explicit dismissal;
- relation no longer in scope.

Scale:
- compact: one rail + anchor;
- wide: provenance tail + local relation context.

Never use a generic glowing trail.

## SCAN

Purpose: communicate active examination of a bounded region.

Behavior:
- local witness zones activate in nonuniform sequence;
- density changes across multiple field segments;
- the target remains fixed;
- no full-width laser line.

Use only when an actual inspection/discovery operation is occurring.

## REVEAL

Purpose: introduce evidence/context that became available.

Behavior:
- reveal begins from a valid anchor or edge;
- surrounding frame/label appears after the evidence carrier;
- no opacity-only theatrical fade as the signature.

## RESOLVE

Signature visual effect.

Behavior:
- fragmented carriers become more coherent;
- relation/persistence clarifies;
- at least one invariant remains stable;
- unresolved gaps that still lack evidence remain open.

Meaning:
**unresolved representation → clearer supported relation/detail**

Never:
**unknown → magically true**

## FRAGMENT

Purpose: show representation loss, interruption, or unavailable continuity.

Behavior:
- local segments separate;
- primary identity remains stable;
- fragmentation stays bounded.

## ALIGN

Purpose: show two related fields entering a shared coordinate/context.

Behavior:
- anchors approach common rail/alignment;
- objects do not merge into one identity.

## CONNECT

Purpose: reveal a supported relation.

Behavior:
- persistence grows from both endpoints toward a relation field or from known source toward destination;
- relation label appears when semantics matter.

## DISCONNECT

Purpose: indicate relation removal/unavailability.

Behavior:
- relation rail loses persistence locally;
- endpoints remain intact.

## SIGNAL

Purpose: mark a meaningful active event.

Behavior:
- cadence/pulse rather than glow;
- short and bounded.

## DISTORT

Purpose: indicate interference, representation loss, or boundary crossing.

Behavior:
- local offset/dropout;
- must recover or settle.

## COMPRESS / EXPAND

Purpose: change visible density/context.

Behavior:
- secondary context collapses/returns while primary anchor remains stable.

## FOCUS

Purpose: establish active inspection target.

Behavior:
- witness frame/anchor;
- no dramatic scale-up.

## OBSERVE

Purpose: show that a target is being observed.

Behavior:
- target remains visually independent;
- offset frame, measurement, trace, context signal appear around it.

## VERIFY

Purpose: show a named verification completed successfully.

Behavior:
- local noise/persistence irregularity settles;
- a small confirmation geometry stabilizes;
- no explosion/confetti.

## UNKNOWN

Purpose: communicate unresolved state.

Behavior:
- incomplete geometry;
- missing continuity;
- partial trace;
- reduced density;
- open boundary.

Unknown should feel quiet, not broken.

## ERROR

Purpose: communicate local operational friction/failure.

Behavior:
- local break;
- offset;
- signal loss;
- structural interruption;
- short recovery/settling into readable error state.

No screen-wide red animation.

## TRANSITION

Purpose: move between contexts.

Behavior derives from resolution:
- layers separate;
- relation becomes visible;
- fragment aligns;
- witness frame enters;
- signal converges.

Avoid generic swipe, spin, flash, or glitch as default transitions.

## Intensity

| Level | Name | Use |
|---|---|---|
| 0 | NONE | silence/static |
| 1 | MICRO | routine focus/hover/selection |
| 2 | SUBTLE | standard inspect/trace/observe |
| 3 | NOTICEABLE | resolve/verify/context shift |
| 4 | MAJOR | major product reveal/change |
| 5 | CINEMATIC | brand film, launch, installation |

Most product UI uses **1–2**.

Use the smallest effective level.
