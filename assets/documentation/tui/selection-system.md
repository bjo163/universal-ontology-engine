# OX-DX TUI Selection System

These states are distinct.

## FOCUSED

Current keyboard/pointer interaction target.

Marker:
- `›` / `>`.

## SELECTED

Object chosen as current inspection subject.

Marker:
- `◆` / `*`;
- explicit selected context in status bar.

Selection persists as focus moves when the interaction needs it.

## EXPANDED

Region/object details are open.

Marker:
- `▾` / `v`.

## ACTIVE

Operation/mode currently running or engaged.

Use exact text:
- `TRACING`;
- `DISCOVERING`;
- etc.

Do not use selection styling as activity styling.

## VISITED

Previously inspected/history item.

Use low-weight history notation.

Do not use link-purple convention as a semantic requirement.

## Composition

A selected object may also be focused:

```text
› ◆ SOURCE  crates/ontology-core/src/lib.rs
```

Expanded:

```text
  ◆ SOURCE  crates/ontology-core/src/lib.rs
    ▾ EVIDENCE
```

ASCII fallback preserves labels/markers.

## Rule

Never collapse all five states into one bright background highlight.
