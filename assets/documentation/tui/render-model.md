# OX-DX TUI Render Model

Optimize for terminal efficiency and stable inspection.

## Persistent

Usually persistent:
- Shell layout;
- context bar;
- selected object;
- current resolution;
- status bar;
- command bar;
- current primary view.

## Ephemeral

Short-lived:
- process cadence;
- temporary focus preview;
- transient notice;
- bounded animation frame.

## Re-render triggers

Render when:
- input changes focus/selection/view;
- data changes;
- observation/diagnostic changes;
- terminal resizes;
- bounded animation advances;
- process status changes.

No continuous idle redraw requirement.

## Static regions

Do not rewrite unchanged high-volume source/graph content solely to animate a small status cue if implementation can avoid it.

Ratatui may render a full frame; the conceptual rule is to avoid unnecessary state churn/timers.

## Animation

2–5 frames.

Keep current focus/source/relation anchor stable.

## Scroll

Scroll state belongs to the surface that owns the content.

Switching focus regions should not reset unrelated scroll offsets.

## Data separation

Rendering consumes data/status.

It does not infer unsupported semantics to fill visual gaps.

## Terminal clearing

Use normal alternate-screen lifecycle where appropriate in implementation.

Do not use repeated clear-screen commands as an effect.
