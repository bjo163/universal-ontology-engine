# OX-DX Representation UI

Future product interfaces must distinguish:

**meaning ≠ representation ≠ encoding ≠ raw data**

Do not flatten these layers.

## Conceptual inspection path

```text
SEMANTIC OBJECT
   ↓ REPRESENTED_AS
REPRESENTATION
   ↓ interpreted with
ENCODING / FORMAT CONTEXT
   ↓ exposes
DATA / TOKEN / CHARACTER / BIT
```

This is conceptual product architecture, not a guaranteed physical chain for every object.

## Representation surface

Show:
- source semantic object or value;
- representation relation;
- format or encoding context when known;
- concrete data;
- finer representation only when technically available.

## Raw data

Raw data is concrete, not automatically more authoritative.

Do not imply that lower-level representation is more true than higher-level semantic evidence.

## Encoding

Character or textual interpretation requires encoding context. If encoding is unknown, say so.

## Micro-resolution

A future Raw Representation Viewer may expose offsets, ranges, bytes, tokens, characters, or bits only when provided by actual implementation.

Never simulate low-level depth for atmosphere.

## Interaction

Resolving deeper reveals more concrete representation while retaining semantic context, encoding context, provenance, and representation relation.
