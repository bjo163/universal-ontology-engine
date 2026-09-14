# OX-DX TUI System v1.0

**Status: CANONICAL TUI HANDOFF**

**Version: v1.0**

Brand authority:

`OX-DX-BRAND-BIBLE-v1.0.md`

Product authority:

`product/README.md`

Sensory authority:

`OX-DX-SENSORY-BRAND-SYSTEM-v1.0.md`

TUI index:

`tui/README.md`

This document defines the OX-DX Terminal User Interface experience system.

It does not implement a TUI, change the current CLI, modify Rust source, alter the ontology, alter schemas, alter graph semantics, or redefine project phases.

---

## 1. Identity

# OX-DX

**Universal Ontology & Experience Engine**

Technical implementation:

**Universal Ontology Engine**

Primary:

**FROM STRUCTURE TO EXPERIENCE.**

Secondary:

**FROM UNIVERSE TO BIT.**

Core:

**EVIDENCE FIRST.**

The TUI is:

# THE OX-DX INSTRUMENT

It is:

**AN INSTRUMENT FOR INSPECTING COMPLEX SYSTEMS.**

It behaves as:

**A LENS.**

Not:

- a dashboard;
- a hacker movie;
- a terminal clone of a web application;
- a generic IDE;
- a shell prompt decoration pack.

---

## 2. Core Experience

Primary flow:

**SEE → TRACE → RESOLVE**

Secondary flow:

**OBSERVE → CONNECT → VERIFY**

Fundamental rules:

**DENSE WHEN THINKING.**

**QUIET WHEN OBSERVING.**

**EVIDENCE FIRST.**

**DON'T FORCE THE SHAPE.**

**OBSERVATION IS NOT OWNERSHIP.**

**PROVENANCE IS NOT IDENTITY.**

**RESOLUTION, NOT REDUCTION.**

---

## 3. Design Equation

**MONOSPACE + SPATIAL STRUCTURE + EVIDENCE + RESOLUTION + CONTROLLED IMPERFECTION**

Monospace is the medium.

The identity comes from:
- source-first evidence;
- typed relation visibility;
- Resolution Spine;
- witness/trace glyph grammar;
- open/interrupted framing;
- controlled density;
- quiet unknown state;
- provenance/observation separation.

Grunge means:

**clean geometry + slightly disturbed surfaces**

Never random dirt.

---

## 4. Current Technical Boundary

Current repository CLI commands observed in source during this handoff:

- `levels`
- `inspect <level>`
- `validate`
- `discover <workspace>`
- `parse <language> <file>`
- `self [root]`

Current global CLI registry argument:

- `--registry <path>`

Current ontology:

- version **1.0.0**
- 49 canonical levels
- 7 zones

Current canonical graph edge kinds:

- `contains`
- `references`
- `specializes`
- `depends_on`
- `invokes`
- `produces`
- `consumes`
- `causes`
- `projects_to`
- `represented_as`
- `observed_at`

Future TUI modes/actions in this document are design vocabulary.

They are not current CLI subcommands unless separately implemented.

---

## 5. Terminal Brand

Canonical specialist:

`tui/tui-brand.md`

Rules:
- use textual `OX-DX`;
- no giant ASCII logo;
- no Nerd Font requirement;
- no random boot text;
- status is explicit text;
- shutdown is quiet.

Startup rhythm:

`signal → identity → system state → ready`

Canonical specialist:

`tui/startup-experience.md`

---

## 6. Prompt

Canonical specialist:

`tui/prompt-language.md`

Preferred wide composition:

```text
OX·<scope>  L<nn>:<TYPE>  <mode>  <state>
›
```

ASCII:

```text
OX:<scope>  L<nn>:<TYPE>  <mode>  <state>
>
```

Prompt communicates inspection coordinates, not shell telemetry.

---

## 7. Terminal Grammar

Canonical specialist:

`tui/terminal-grammar.md`

A recognizably OX-DX surface should normally include:
- stable focus/selection anchor;
- evidence-bearing primary region;
- context/resolution indicator;
- controlled interruption/open boundary;
- separated provenance/observation when relevant.

Distinctions remain explicit:

`focus ≠ selected`

`observed ≠ owned`

`provenance ≠ identity`

`resolution order ≠ containment`

`unknown ≠ error`

`verified ≠ success`

`graph ≠ tree`

---

## 8. Layout

Canonical specialist:

`tui/layout-system.md`

Profiles:

| Profile | Reference |
|---|---|
| COMPACT | 80×24 or smaller |
| STANDARD | 100×30 |
| INSPECTION | 120×40 |
| WIDE | 160×50+ |
| FULL-DEPTH | 200×60+ |

More space reveals more context.

Less space preserves focus.

Wide screens expose:
- graph;
- provenance;
- trace;
- metadata;
- secondary evidence.

They do not simply increase padding.

Responsive rules:

`tui/responsive-terminal.md`

Test matrix:

`tui/layout-test-matrix.md`

---

## 9. Framing

Canonical specialists:

- `tui/framing-language.md`
- `tui/border-system.md`
- `tui/separator-system.md`

Use:
- broken/partial boundaries;
- measurement marks;
- resolution indicators;
- context rails;
- controlled asymmetry.

Do not draw a complete box around everything.

---

## 10. Glyphs

Canonical specialist:

`tui/glyph-system.md`

Core examples:

| Meaning | UTF-8 | ASCII |
|---|---|---|
| structure | ■ | # |
| relation | ─▷ | -> |
| trace | ┄▷ | ..> |
| observation | ◇ | o |
| resolution | ◆ | * |
| signal | · | . |
| unknown | ⋯ | ... |
| warning | ! | ! |
| verified | ✓ | OK |
| error | × | X |
| focus | › | > |
| collapsed | ▸ | > |
| expanded | ▾ | v |

Glyphs never carry meaning alone.

---

## 11. ANSI / Color

Canonical specialists:

- `tui/ansi-color-system.md`
- `tui/color-semantics.md`

Support:
- truecolor;
- 256-color;
- 16-color;
- 8-color;
- monochrome.

The TUI does not depend on truecolor.

Color roles:
- BASE
- STRUCTURE
- FOCUS
- OBSERVATION
- SIGNAL
- WARNING
- ERROR
- UNKNOWN
- VERIFIED

Status remains text + glyph/structure.

---

## 12. Text Hierarchy

Canonical specialist:

`tui/text-hierarchy.md`

Roles:
- TITLE
- SECTION
- PRIMARY
- SECONDARY
- METADATA
- TECHNICAL
- SOURCE
- PROVENANCE
- STATUS
- COMMAND
- ERROR
- WARNING
- UNKNOWN

Source and technical terminology remain readable in plain ANSI output.

---

## 13. Cursor / Focus / Selection

Canonical specialists:

- `tui/cursor-system.md`
- `tui/focus-system.md`
- `tui/selection-system.md`

Cursor answers:

**Where am I?**

It does not answer:

**What does the data mean?**

States remain distinct:
- focused;
- selected;
- expanded;
- active;
- visited.

---

## 14. Resolution Navigation

Canonical specialists:

- `tui/resolution-navigation.md`
- `tui/resolution-compact-mode.md`
- `tui/resolution-deep-view.md`

The 49 canonical TYPEs are a:

**resolution spine over a graph**

not a mandatory 49-deep tree.

The user must be able to know:
- current level;
- broader level;
- finer level;
- neighboring levels;
- current zone;
- available/materialized evidence when known.

Compact mode uses the Seven-Zone Rail.

Deep mode shows:
- level;
- TYPE;
- zone;
- canonical definition;
- evidence;
- actual relationships;
- provenance where available.

---

## 15. Canonical 49-Level Order

```text
01 UNIVERSE
02 CREATION
03 ORDER
04 REALITY
05 REALM
06 WORLD
07 DOMAIN
08 ECOSYSTEM
09 ORGANIZATION
10 COMMUNITY
11 REGION
12 ENVIRONMENT
13 NETWORK
14 CONTEXT
15 PURPOSE
16 MISSION
17 OBJECTIVE
18 PROGRAM
19 PROJECT
20 PRODUCT
21 SYSTEM
22 REPOSITORY
23 SOURCE
24 UNIT
25 MODULE
26 SUBSYSTEM
27 COMPONENT
28 ELEMENT
29 SYMBOL
30 ENTITY
31 PROPERTY
32 RELATION
33 OPERATION
34 FUNCTION
35 BEHAVIOR
36 STATE
37 EVENT
38 PROCESS
39 FLOW
40 TRANSITION
41 ACTION
42 EXECUTION
43 INSTRUCTION
44 EXPRESSION
45 VALUE
46 DATA
47 TOKEN
48 CHARACTER
49 BIT
```

This ordering must not be rewritten by the TUI.

---

## 16. Evidence

Canonical specialist:

`tui/evidence-view.md`

Priority:

`native evidence → classification → relation → provenance → interpretation`

Source must never disappear behind interpretation.

---

## 17. Source

Canonical specialist:

`tui/source-view.md`

Supported fields only when available:
- file;
- path;
- line;
- column;
- span;
- native syntax kind;
- source excerpt.

Source location is provenance, not identity.

---

## 18. Provenance

Canonical specialist:

`tui/provenance-view.md`

Use an offset Provenance Rail.

Do not replace semantic identity with:
- path;
- line;
- timestamp;
- adapter metadata.

---

## 19. Graph

Canonical specialists:

- `tui/graph-view.md`
- `tui/graph-focus.md`

Terminal graph uses spatial text topology.

Do not reproduce a GUI graph literally.

Use exact edge kinds.

Use parent/child wording only for actual structural containment/Node parentage.

Other relationships remain:
- neighbor;
- source/destination;
- projection;
- representation;
- observation;
- dependency;
- reference;
- cause.

---

## 20. Trace

Canonical specialist:

`tui/trace-view.md`

Trace displays:
- source;
- exact relationship;
- observation when relevant;
- path;
- resolution;
- provenance.

Trace must not look like ownership.

---

## 21. Observation

Canonical specialist:

`tui/observation-view.md`

**OBSERVATION IS NOT OWNERSHIP.**

Observation is offset from the structural object.

Runtime/syntax evidence never becomes a structural child merely because it appears beside the target.

---

## 22. Unknown

Canonical specialist:

`tui/unknown-view.md`

Unknown uses:
- absence;
- open boundary;
- partial trace;
- quiet marker;
- explicit UNKNOWN text.

Unknown does not flash.

Unknown does not default to red.

Unknown is not error.

---

## 23. Verification

Canonical specialist:

`tui/verification-view.md`

Verification is:

**stable · settled · precise**

Use VERIFIED only for an actual named check at a known scope.

No celebratory animation.

---

## 24. Diagnostics / Errors

Canonical specialists:

- `tui/diagnostics.md`
- `tui/error-system.md`

Diagnostic levels:
- INFO
- NOTICE
- WARNING
- ERROR
- FATAL

Errors answer:
- what happened;
- where;
- why if known;
- what evidence remains;
- what can be done next.

Preferred language:

`Malformed input observed.`

Not:

`Something went wrong.`

---

## 25. Loading / Process

Canonical specialist:

`tui/loading-system.md`

Process vocabulary:
- DISCOVERING
- INSPECTING
- PARSING
- MAPPING
- TRACING
- RESOLVING
- VERIFYING

No fake percentages.

No noisy infinite spinner.

Static fallback always exists.

---

## 26. Command / Help / Status

Canonical specialists:

- `tui/command-bar.md`
- `tui/help-system.md`
- `tui/status-bar.md`
- `tui/status-system.md`

Current CLI commands remain visibly separate from future TUI vocabulary.

Help is contextual.

Status bar stays quiet.

---

## 27. Modes

Canonical specialist:

`tui/modes.md`

Future interaction modes:
- DISCOVER
- INSPECT
- GRAPH
- TRACE
- OBSERVE
- RESOLVE
- VERIFY
- DIAGNOSTIC

These are TUI design modes.

They are not automatically CLI commands or engine states.

---

## 28. Density

Canonical specialist:

`tui/density-modes.md`

Modes:
- LOW
- MEDIUM
- HIGH
- INSPECTION

**DENSE WHEN THINKING.**

Density changes information volume, not semantic meaning.

---

## 29. Keyboard

Canonical specialist:

`tui/keymap.md`

Arrow keys are primary.

Optional:
- j/k;
- h/l.

Vi behavior is not mandatory.

Core global actions:
- `?` help;
- `q` quit;
- `Esc` return/close when safe;
- Enter inspect/activate;
- Tab region navigation.

Key behavior is contextual and visibly documented.

---

## 30. Mouse

Canonical specialist:

`tui/mouse-support.md`

Mouse is optional.

Keyboard remains complete.

Mouse may supplement:
- focus;
- selection;
- scrolling.

Do not require mouse.

Do not drag graph nodes merely to mimic GUI graph software.

---

## 31. Sound / Silence

Canonical specialist:

`tui/sound-integration.md`

Default:

**SILENT**

Optional:

**MINIMAL**

Alerts:

**NOTIFY**

Routine navigation never beeps.

Unknown/loading remain silent by default.

Sensory traceability:

`tui/sensory-traceability.md`

---

## 32. Motion

Canonical specialist:

`tui/motion-system.md`

Allowed:
- focus/cursor movement;
- line reveal;
- small state changes;
- bounded process cadence;
- short resolution/trace settle.

Avoid:
- full-screen effect clears;
- rapid flashing;
- fake hacker typing;
- Matrix rain;
- ambient spinners.

Reduced motion renders final state directly.

Animation specifications:
- `tui/animation-frames/resolve.md`
- `tui/animation-frames/trace.md`
- `tui/animation-frames/scan.md`
- `tui/animation-frames/verify.md`

---

## 33. ASCII / Compatibility

Canonical specialists:

- `tui/ascii-fallback.md`
- `tui/compatibility.md`

Compatibility levels:

- LEVEL A — modern truecolor UTF-8
- LEVEL B — 256-color UTF-8
- LEVEL C — 16-color UTF-8
- LEVEL D — basic ASCII

Graceful degradation is mandatory.

---

## 34. Screens

Canonical screen spec:

`tui/screens.md`

Covers:
1. startup;
2. workspace discovery;
3. ontology levels;
4. inspect;
5. evidence;
6. graph;
7. trace;
8. observation;
9. provenance;
10. diagnostics;
11. verification;
12. command/help.

Implementation-ready textual mockups:

`tui/sample-screens.md`

---

## 35. Machine-Readable Tokens

Canonical file:

`tui/tui-tokens.json`

Contains:
- colors;
- ANSI mappings;
- terminal-cell spacing;
- borders;
- glyphs;
- focus rules;
- status vocabularies;
- density;
- motion;
- timing;
- sound modes;
- resolution;
- keymap;
- accessibility.

Terminal-cell spacing is medium-specific implementation mapping.

It does not replace canonical brand pixel tokens.

---

## 36. Component Architecture

Canonical component map:

`tui/component-map.md`

Components:
- Shell
- Context Bar
- Resolution Spine
- Evidence View
- Source View
- Graph View
- Trace View
- Observation View
- Provenance View
- Diagnostics
- Status Bar
- Command Bar
- Help Surface
- Focus Marker
- Resolution Marker

Contracts:

`tui/component-contracts.md`

Each contract defines:
- purpose;
- inputs;
- state;
- events;
- render responsibility;
- keyboard behavior;
- fallback behavior.

---

## 37. Conceptual State Model

Canonical specialist:

`tui/state-model.md`

Conceptual state:
- context;
- selection;
- focus;
- resolution;
- view;
- filter;
- diagnostic;
- mode.

This is not a prescribed Rust struct hierarchy.

---

## 38. Event Model

Canonical specialist:

`tui/event-model.md`

Event classes:
- keyboard;
- mouse;
- resize;
- data update;
- observation update;
- resolution update;
- diagnostic update;
- bounded timer/tick.

No idle animation timer is required.

---

## 39. Render Model

Canonical specialist:

`tui/render-model.md`

Persistent:
- context;
- primary view;
- selection;
- resolution;
- status/command.

Ephemeral:
- bounded animation;
- temporary notices;
- process cadence.

Render is event-driven.

No constant redraw requirement.

---

## 40. Ratatui / Crossterm Target

Canonical specialist:

`tui/ratatui-mapping.md`

Likely responsibilities:
- Shell → Ratatui Layout/Rects;
- bars → Paragraph/Line/Span;
- spine → List/custom composition;
- evidence/source → Paragraph/List/Table as appropriate;
- graph → custom Widget + linear fallback;
- diagnostics → List + detail;
- command → Paragraph + terminal cursor;
- events → Crossterm keyboard/mouse/resize.

Use standard widgets where conventional.

Use custom rendering only where OX-DX grammar genuinely needs it.

No implementation code is created by this phase.

---

## 41. Performance

Canonical specialist:

`tui/performance.md`

Target:

**lightweight · responsive · stable**

- event-driven redraw;
- no idle animation loop;
- bounded 2–5 frame animations;
- background updates must not starve input;
- no unbounded logs/history;
- no sensory dependency for correctness.

---

## 42. Accessibility

Canonical specialist:

`tui/accessibility.md`

Support:
- keyboard;
- non-color statuses;
- high contrast;
- ASCII fallback;
- reduced motion;
- silent mode.

Important information survives all modes.

---

## 43. Brand Traceability

Canonical specialist:

`tui/brand-traceability.md`

At least 18 mappings connect:

`BRAND PRINCIPLE → TUI RULE → VISIBLE BEHAVIOR → USER EXPERIENCE`

The TUI is not a detached product theme.

It is a direct expression of the Brand Bible.

---

## 44. Originality

Canonical audit:

`tui/originality-audit.md`

Intentional separation documented from:
- btop;
- htop;
- lazygit;
- k9s;
- Neovim;
- Helix;
- ranger;
- Warp;
- Starship.

No claim of absolute global uniqueness.

Signature without logo:
- Resolution Spine;
- evidence-first hierarchy;
- observation separation;
- trace language;
- asymmetric framing;
- controlled density;
- quiet unknown state;
- distinctive terminal-safe glyph grammar.

---

## 45. Do / Don't

Canonical specialist:

`tui/do-and-dont.md`

### DO
- show evidence;
- show context;
- show resolution;
- support keyboard;
- use quiet states;
- preserve provenance;
- separate observation;
- use exact relations;
- degrade gracefully.

### DON'T
- simulate hacker movies;
- glow everything;
- animate everything;
- beep everything;
- decorate with ASCII noise;
- hide evidence;
- turn unknown into error;
- turn observation into ownership;
- force 49-level nesting;
- rebuild web UI in terminal.

---

## 46. Validation

Canonical audit:

`tui/validation-audit.md`

Required gates:
- brand;
- visual grammar;
- terminal grammar;
- layout;
- color;
- glyphs;
- resolution;
- evidence;
- graph;
- trace;
- observation;
- provenance;
- status;
- keymap;
- motion;
- sound;
- silence;
- accessibility;
- responsive behavior;
- Ratatui mapping;
- originality.

---

## 47. Implementation Boundary

This TUI system does **not** modify:
- Rust source;
- CLI implementation;
- ontology;
- schema;
- graph semantics;
- phase definitions.

It does not create:
- desktop GUI;
- web GUI;
- website;
- Next.js;
- Vercel;
- mobile UI.

It is a design/system specification only.

---

# Final TUI Identity

The terminal should let the user feel:

**“I can see the system.”**

**“I can follow the evidence.”**

**“I can go deeper.”**

**“Nothing is pretending to be more certain than it is.”**

The TUI is:

# A LENS.

Not:

# A DASHBOARD.

**DENSE WHEN THINKING.**

**QUIET WHEN OBSERVING.**

**EVIDENCE FIRST.**

**DON'T FORCE THE SHAPE.**

**OBSERVATION IS NOT OWNERSHIP.**

**PROVENANCE IS NOT IDENTITY.**

**RESOLUTION, NOT REDUCTION.**

**FROM STRUCTURE TO EXPERIENCE.**

**FROM UNIVERSE TO BIT.**
