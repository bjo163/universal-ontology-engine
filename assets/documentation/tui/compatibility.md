# OX-DX Terminal Compatibility Levels

## LEVEL A — MODERN TRUECOLOR UTF-8

Capabilities:
- UTF-8 box/glyph set;
- truecolor;
- mouse optional;
- full restrained styling.

## LEVEL B — 256-COLOR UTF-8

Capabilities:
- UTF-8 glyphs;
- 256-color approximation;
- same structure as A.

## LEVEL C — 16-COLOR UTF-8

Capabilities:
- UTF-8 glyphs;
- ANSI 16;
- reduced palette;
- structure/status remains label-driven.

## LEVEL D — BASIC ASCII

Capabilities:
- ASCII characters;
- monochrome or basic ANSI;
- no Unicode dependency;
- no mouse requirement;
- no sensory animation requirement.

## Graceful degradation

A → B:
- color approximation only.

B → C:
- semantic accents collapse to standard terminal colors;
- labels/glyphs preserve distinctions.

C → D:
- Unicode glyphs become ASCII;
- box drawing becomes `| - +`;
- status remains textual;
- relation names remain exact.

## Environment constraints

Support conceptually:
- local terminals;
- SSH;
- tmux/screen;
- CI/log-like fallback where interactivity is unavailable.

A future implementation should detect capabilities conservatively and allow user override.

## Non-goals

Do not require:
- GPU terminal;
- Nerd Fonts;
- ligatures;
- sixel/kitty images;
- truecolor;
- mouse.
