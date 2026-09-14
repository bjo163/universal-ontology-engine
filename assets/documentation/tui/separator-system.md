# OX-DX TUI Separator System

Separators indicate real structural distinctions.

## Structural separator

Unicode:

`─────   ─────`

ASCII:

`-----   -----`

The gap is intentional.

## Context separator

Unicode:

`context · relation · scope`

ASCII:

`context : relation : scope`

## Resolution separator

Unicode:

`L21 SYSTEM  ›  L22 REPOSITORY`

ASCII:

`L21 SYSTEM  >  L22 REPOSITORY`

This is navigation notation, not an edge claim.

## Trace separator

Unicode:

`source ┄┄▷ target`

ASCII:

`source ..> target`

Use only as a visual trace path; exact graph edge kind must be separately labeled where semantics matter.

## Warning separator

Unicode:

`! ── message`

ASCII:

`! -- message`

## Rules

- avoid decorative repeated characters;
- no `=====` banners everywhere;
- no ornamental ASCII flourishes;
- relation arrows do not substitute for exact relation labels;
- separators must collapse safely in narrow terminals.
