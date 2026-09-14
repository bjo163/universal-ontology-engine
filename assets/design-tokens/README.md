# OX-DX Design Token Handoff v1.0

These files are implementation-friendly mappings of existing canonical brand tokens.

Source of truth:

- `assets/documentation/technical/design-tokens.md`
- `assets/documentation/technical/color-system.md`
- `assets/documentation/technical/typography.md`
- `assets/documentation/visual/motion-grammar.md`

This package does **not** create a second token system.

## Files

- `tokens.json` — consolidated canonical values and product aliases.
- `colors.json` — palette + semantic product mappings.
- `typography.json` — accessible system stacks and roles.
- `spacing.json` — 3 / 7 / 14 / 21 / 35 / 49 / 98 scale.
- `geometry.json` — stroke, anchor, break, cut, radius values.
- `motion.json` — 98 / 196 / 294 / 490 ms + canonical easing.
- `breakpoints.json` — 420 / 840 / 1260 / 1680 px.
- `tokens.css` — CSS custom property handoff.
- `tailwind-theme.json` — Tailwind-friendly theme-extension values.
- `component-aliases.json` — semantic aliases for future component libraries.

## Rule

Product aliases point to canonical values. They must not drift independently.

If a canonical token changes through Brand Bible governance, regenerate/update these mappings in the same change.

## CSS

Import or copy the variables into the future application token layer.

Do not use these files to imply a framework choice.

## Tailwind

`tailwind-theme.json` is a mapping reference. A future Tailwind configuration may consume equivalent values.

## Component libraries

Use `component-aliases.json` as semantic intent, not as a forced component API.

Accessibility/platform conventions may require additional implementation-only values such as error treatments or touch target sizing. Such values must be clearly labeled implementation/platform tokens and must not be presented as new OX-DX brand colors.
