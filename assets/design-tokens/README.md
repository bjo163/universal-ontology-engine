# OX-DX Design Token Handoff v1.0

These files are implementation-friendly mappings of existing canonical brand tokens.

Source of truth:

- `assets/documentation/technical/design-tokens.md`
- `assets/documentation/technical/color-system.md`
- `assets/documentation/technical/typography.md`
- `assets/documentation/visual/motion-grammar.md`
- `assets/documentation/OX-DX-SENSORY-BRAND-SYSTEM-v1.0.md`

This package does **not** create a second token system.

## Files

- `tokens.json` — consolidated canonical values and product aliases.
- `colors.json` — palette + semantic product mappings.
- `typography.json` — accessible system stacks and roles.
- `spacing.json` — 3 / 7 / 14 / 21 / 35 / 49 / 98 scale.
- `geometry.json` — stroke, anchor, break, cut, radius values.
- `motion.json` — 98 / 196 / 294 / 490 ms + canonical easing.
- `sensory.json` — sensory intensity, event priority, silence and accessibility aliases.
- `breakpoints.json` — 420 / 840 / 1260 / 1680 px.
- `tokens.css` — CSS custom property handoff.
- `tailwind-theme.json` — Tailwind-friendly theme-extension values.
- `component-aliases.json` — semantic aliases for future component libraries.

## Rule

Product and sensory aliases point to canonical values. They must not drift independently.

If a canonical token changes through Brand Bible governance, update these mappings in the same change.

## Sensory rule

`sensory.json` contains semantic intensity/priority aliases only.

It does not define:
- audio gain;
- LUFS mastering values;
- engine states;
- ontology states.

Audio production targets live in `assets/documentation/sensory/audio-specification.md`.

## CSS / Tailwind / components

These mappings do not select a framework.

Accessibility/platform conventions may add implementation-only values when clearly labeled and must not be presented as new OX-DX brand tokens.
