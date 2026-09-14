# OX-DX Ratatui / Crossterm Mapping

This is a future implementation mapping, not code.

## Shell

Likely:
- Ratatui `Layout` / `Rect`;
- application-owned layout profile logic.

Do not force every region into a `Block`.

## Context Bar / Status Bar

Likely:
- `Paragraph`;
- `Line` / `Span`.

## Resolution Spine

Likely:
- `List` for compact navigation;
- custom rendering/Paragraph for seven-zone map;
- `ListState` or app-owned focus state.

## Evidence View

Likely:
- composed `Paragraph`, `List`, optional `Table`;
- source view owns scroll.

## Source View

Likely:
- `Paragraph`;
- styled `Line`/spans;
- `ScrollBar` optional.

## Graph View

Likely:
- custom `Widget`/buffer rendering for terminal topology;
- grouped `List` fallback for dense/compact mode.

Do not force graph topology into `Tree` semantics.

## Trace View

Likely:
- `List` or custom Paragraph lines.

## Observation / Provenance

Likely:
- `Paragraph`/small composed widgets;
- partial border rendering may require custom `Widget`.

## Diagnostics

Likely:
- `List` + detail `Paragraph`;
- `Table` when structured diagnostic fields compare well.

## Command Bar

Likely:
- `Paragraph` for input rendering;
- Crossterm key events;
- terminal-native cursor placement when editing.

A text-input crate is optional; do not assume one.

## Help Surface

Likely:
- `Clear` + `Paragraph`/List for overlay where appropriate;
- compact one-line `Paragraph` for help strip.

## Borders

Standard `Block` borders may be used where useful, but OX-DX partial/interrupted frames will often need custom buffer drawing or selective borders.

## Events

Crossterm:
- key;
- mouse optional;
- resize.

A future event loop may use polling or async integration; this spec does not prescribe architecture.

## Styling

Ratatui `Style`/modifiers:
- capability-aware colors;
- bold/reversed selectively;
- no essential dim-only distinction.

## Principle

Use standard widgets for standard behavior.

Use custom rendering only where OX-DX grammar or graph topology genuinely requires it.
