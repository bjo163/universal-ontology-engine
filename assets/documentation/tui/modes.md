# OX-DX Future TUI Modes

These are future interaction modes.

They are not current CLI subcommands unless explicitly listed as current elsewhere.

| Mode | Purpose | Primary surface | Key actions | Status indicator |
|---|---|---|---|---|
| DISCOVER | inspect discovered workspace evidence | discovery/evidence | inspect, filter, open source | DISCOVER |
| INSPECT | focus one object/evidence set | evidence/inspector | inspect, trace, resolve | INSPECT |
| GRAPH | traverse typed relationships | graph topology | neighbor, trace, expand | GRAPH |
| TRACE | follow relation/provenance path | trace path | next/prev hop, inspect | TRACE |
| OBSERVE | inspect observations separately from structure | observation rail | inspect, trace target/source | OBSERVE |
| RESOLVE | navigate canonical resolution spine | resolution view | broader/finer, inspect | RESOLVE |
| VERIFY | inspect named validation | verification | run/inspect result if implemented | VERIFY |
| DIAGNOSTIC | inspect operational messages | diagnostics | filter severity, inspect evidence | DIAG |

## Mode rule

Mode describes current interaction task.

It does not change ontology meaning.

## Switching

Mode change should preserve:
- selected object where compatible;
- context;
- resolution;
- history.

Do not reset the entire workspace unless necessary.
