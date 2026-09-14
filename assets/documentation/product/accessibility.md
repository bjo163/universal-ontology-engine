# OX-DX Product Accessibility

Accessibility is part of product correctness.

Grunge must never compromise accessibility.

## Semantic structure

Use semantic HTML or equivalent native semantics:
- headings;
- landmarks;
- lists;
- tables;
- buttons;
- links;
- status/live regions only where appropriate.

Do not reconstruct standard controls from decorative divs without reason.

## Keyboard

All core inspection/navigation actions must be keyboard reachable.

Graph and dense technical surfaces require a non-pointer navigation strategy.

## Focus

Use a visible high-contrast focus witness.

Focus must not depend on color alone and must remain distinguishable from selection.

## Screen readers

Expose:
- object label/type;
- relation label and endpoints;
- status text;
- source/provenance text;
- loading/error state;
- meaningful alternative for diagrams/graphs.

## Contrast

Critical text and interactive controls require sufficient contrast.

Muted Steel is annotation, not permission to make essential text unreadable.

## Reduced motion

Respect `prefers-reduced-motion`.

Essential information must remain available without animation.

## Non-color status

Status requires text + shape/icon/geometry in addition to optional color.

## Text alternatives

Provide alternatives for:
- diagrams;
- graph selections;
- visual relation direction;
- image-based release/social content when used in product documentation.

## Touch

Touch targets should follow platform-accessible sizing; do not force the 7 px construction unit into unsafe target sizes.

## Zoom and reflow

Text zoom/reflow must not destroy relationship labels, focus order, or evidence access.

## Grunge boundary

Texture:
- never crosses critical text;
- never obscures focus;
- never reduces essential line contrast below usable levels;
- is removable in high-contrast modes.
