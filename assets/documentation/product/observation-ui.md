# OX-DX Observation UI

**OBSERVATION IS NOT OWNERSHIP.**

Observation UI must look attached to a target without appearing structurally contained by it.

## Observation Marker

Canonical behavior:
- keep target geometry unchanged;
- add an offset witness bracket;
- add a witness anchor;
- label the observation type or status;
- connect through an observation relation where supported.

## Runtime observation

When runtime evidence exists, display it as a distinct evidence layer.

An `EXECUTION` observed at an `ELEMENT` should visually express an `OBSERVED_AT` relationship rather than nesting execution inside the element.

## Observation detail

Possible fields when supported:
- observation kind;
- observed target;
- timestamp;
- source or collector;
- state/event data;
- provenance;
- verification status.

Do not display unsupported fields.

## Multiple observations

Use separate witness anchors, an aggregated observation rail, or filtering/grouping controls. Do not convert repeated observations into structural children.

## Interaction

Hover previews context.

Focus establishes a keyboard witness.

Select inspects the observation.

Trace follows source, target, or provenance.

## Monochrome

Cyan may reinforce observation, but witness geometry and text must carry meaning without color.
