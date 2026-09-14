# OX-DX Command Surface

A future OX-DX product may expose a command-oriented interface.

This document is naming and UX guidance only.

It does not modify the current `ontology-engine` CLI.

## Conceptual product commands

Potential verbs:
- inspect
- discover
- trace
- resolve
- observe
- verify
- query

A command appears only when the underlying product capability exists.

## Command model

Each command should expose:
- verb;
- target;
- scope;
- optional qualifiers;
- expected evidence/result.

Example conceptual form:

`inspect <target> --scope <context>`

Do not copy this into the current CLI unless separately designed and implemented.

## Search vs command

Search finds known addressable things.

Command invokes an action.

Do not mix them into one “magic bar” that silently performs mutations.

## Suggestions

Suggestions must clearly distinguish:
- navigation;
- read-only inspection;
- validation;
- mutation, if a future product supports it.

## Keyboard

A command surface should be keyboard-first and screen-reader accessible.

## Copy

Prefer:
- Inspect source
- Trace relation
- Resolve representation
- Verify registry

Avoid:
- Do magic
- Ask AI
- Fix everything

## AI boundary

If future AI suggestions appear in a command surface, they must be visually labeled and separated from observed/verified evidence.
