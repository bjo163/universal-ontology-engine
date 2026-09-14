# OX-DX TUI Sample Screens

Implementation-ready text mockups. Values marked `<...>` are runtime placeholders.

## Startup

```text
OX-DX
Universal Ontology & Experience Engine

ontology   1.0.0 · 49 levels
registry   specifications/universal-ontology-v1.0.json
mode       evidence-first inspection

READY
```

## Workspace discovery

```text
OX·repo  DISCOVER
◆ workspace  <path>
│
│  REPOSITORY  <name>
│  SOURCE      <count>
│  UNIT        <count>
│
┄ evidence     read-only discovery
────────────────────────   ───────────────
L22 REPOSITORY · DISCOVER                ? help
›
```

## Ontology levels

```text
RESOLUTION · Z4 STRUCTURE

   20 PRODUCT
   21 SYSTEM
◆  22 REPOSITORY   <evidence-state>
   23 SOURCE
   24 UNIT

definition
Version-controlled artifact boundary.

[ ] broader   [ ] finer   Enter inspect
```

## Inspect

```text
OX·repo  L22:REPOSITORY  INSPECT

◆ <object name>
│ TYPE   REPOSITORY
│ KIND   <if available>
│ state  <supported state>
│
├ EVIDENCE
│ <primary evidence summary>
│
┄ RELATIONS
  contains → <count if real>

status <state>                              ? help
```

## Evidence

```text
EVIDENCE  ◆ <object>

SOURCE
  <line> │ <native source>
         │ ^ <observed span>

CLASSIFICATION
  TYPE        <TYPE>
  KIND        <kind or unavailable>

RELATIONS
  <edge-kind> → <target>

PROVENANCE
  source      <source>
  span        <span>
```

## Graph

```text
GRAPH · L<nn> <TYPE>

             <edge-kind>
        ┌ ┄┄┄┄┄┄┄▷ <neighbor>
        │
◆ <focus> ──<edge-kind>──▷ <neighbor>
        │
        └ ┄┄<edge-kind>──▷ <neighbor>

↑↓←→ neighbor   Enter inspect   t trace
```

## Trace

```text
TRACE · <scope>

01 ◆ <source>
02 ┄ <edge-kind> ▷ <node>
03 ┄ <edge-kind> ▷ <node>

hop 02/03
Enter inspect   ↑↓ hop   Esc return
```

## Observation

```text
STRUCTURE
◆ <target>

        ◇ OBSERVATION
        ┄ kind       <kind>
        ┄ relation   observed_at
        ┄ source     <source>
```

## Provenance

```text
PROVENANCE

◇ source 01
┄ path      <path>
┄ revision  <if available>
┄ span      <if available>
┄ observer  <if available>

source 1/<n>
```

## Diagnostics

```text
DIAGNOSTICS

· INFO     <message>
! WARNING  <message>
› × ERROR  <message>

ERROR
what     <failure>
where    <scope>
evidence preserved
next     <safe action>
```

## Verification

```text
VERIFY · <scope>

✓ <check>   PASS
✓ <check>   PASS
× <check>   FAIL

state  <VERIFIED or ERROR>
```

## Command/help

```text
COMMAND

› <input>

CURRENT CLI
  levels
  inspect <level>
  validate
  discover <workspace>
  parse <language> <file>
  self [root]

TUI ACTIONS
  trace · resolve · observe · verify · query
  future interaction vocabulary
```

These mockups intentionally avoid web-style cards and full rectangular panels.
