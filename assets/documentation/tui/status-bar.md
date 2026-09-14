# OX-DX TUI Status Bar

Status bar is quiet orientation.

## Fields

May contain:
- context;
- level;
- mode;
- semantic state;
- diagnostic count;
- command hint.

## Wide example

```text
repo:universal-ontology-engine · L22 REPOSITORY · INSPECT · OBSERVED     diag 0E 1W     ? help
```

ASCII:

```text
repo:universal-ontology-engine : L22 REPOSITORY : INSPECT : OBSERVED    diag 0E 1W    ? help
```

## Compact

`L22 · INSPECT · OBSERVED    ?`

ASCII:

`L22 : INSPECT : OBSERVED    ?`

## Priority when shrinking

Keep:
1. level;
2. mode;
3. critical diagnostic indicator;
4. state;
5. context;
6. help hint.

## Rules

- one line preferred;
- no animated telemetry;
- no CPU/memory widgets unless actual TUI responsibility requires them;
- diagnostic counts are real;
- status text survives without color.
