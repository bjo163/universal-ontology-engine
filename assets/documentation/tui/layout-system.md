# OX-DX TUI Layout System

Layouts reveal context according to terminal capacity.

**More columns → more context.**

**Fewer columns → more focus.**

## Core regions

- CONTEXT — scope, mode, resolution
- PRIMARY — current object/evidence/graph
- SECONDARY — related evidence, relations, metadata
- STATUS — state/diagnostics
- COMMAND — prompt/actions/help hints

## COMPACT — 80×24 or smaller

Primary:
- single main surface.

Context:
- one-line context bar.

Secondary:
- hidden/on-demand.

Status:
- one-line bottom status.

Command:
- one line.

Priority:
1. current object;
2. level;
3. primary evidence;
4. status;
5. navigation.

## STANDARD — 100×30

Primary:
- ~70–75% height/width emphasis.

Secondary:
- collapsible right/bottom region.

Context:
- top line + resolution marker.

Status/command:
- bottom 2 lines.

## INSPECTION — 120×40

Primary:
- source/evidence or focused graph region.

Secondary:
- provenance/relations side rail.

Context:
- top context line;
- compact Resolution Spine.

Status/command:
- stable bottom region.

## WIDE — 160×50+

Primary:
- 60–70% main field.

Secondary:
- dedicated relation/provenance panel.

Optional:
- graph neighborhood beside evidence.

Do not enlarge typography; reveal more context.

## FULL-DEPTH — 200×60+

Use for deep technical work.

Possible regions:
- context/resolution rail;
- primary evidence/source;
- graph/relations;
- provenance/diagnostics;
- command/status.

Keep one dominant surface.

Do not create symmetric dashboard quadrants.

## Resize policy

When width shrinks:
1. remove secondary annotations;
2. collapse provenance rail;
3. collapse graph neighborhood;
4. reduce resolution rail to compact mode;
5. preserve current target/evidence/status.

When height shrinks:
1. reduce help hints;
2. collapse secondary metadata;
3. keep command/status;
4. make primary surface scrollable.

## Asymmetry

The TUI uses controlled asymmetry:
- dominant main field;
- narrower contextual rail;
- deliberate empty gaps;
- interrupted boundaries.

Not every region receives a box.
