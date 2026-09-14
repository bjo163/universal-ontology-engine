# OX-DX TUI Sensory Traceability

TUI default sound mode is SILENT.

| TUI behavior | Visual SFX | Sound | Motion | Silence behavior |
|---|---|---|---|---|
| inspect | OBSERVE/REVEAL | optional Witness in MINIMAL | local witness reveal | fully functional |
| trace | TRACE | optional Trace cue | 2–4 frame persistence | fully functional |
| observe | OBSERVE | normally none | offset witness appears | default |
| resolve | RESOLVE | optional Settle | supported detail gains persistence | final state direct |
| verify | VERIFY | optional Verify cue | local settle | label/check remains |
| unknown | UNKNOWN | none | none/minimal settle | canonical default |
| error | ERROR | optional alert cue | local break then readable state | text/glyph remain |

## Rules

- sound never adds unique meaning;
- motion never adds unique meaning;
- terminal bell is not routine brand feedback;
- no fake scan line;
- no rapid flash;
- reduced motion renders final state;
- ASCII fallback keeps event semantics.

## Intensity

Routine TUI:
- 0 NONE;
- 1 MICRO;
- 2 SUBTLE.

Noticeable Level 3 only for meaningful context/verification/error transitions.

Level 4–5 are not routine TUI interaction levels.
