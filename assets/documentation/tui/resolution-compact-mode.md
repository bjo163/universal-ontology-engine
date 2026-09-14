# OX-DX 49-Level Compact Mode

The compact representation must preserve the full canonical order without forcing 49 visible rows.

## Primary compact design — Seven-Zone Rail

```text
Z1  01–07 EXISTENCE
Z2  08–14 CONTEXT
Z3  15–21 INTENT
Z4 ◆22–28 STRUCTURE
Z5  29–35 SEMANTIC
Z6  36–42 DYNAMIC
Z7  43–49 REPRESENTATION

◆ L22 REPOSITORY
↑ L21 SYSTEM
↓ L23 SOURCE
```

ASCII:

```text
Z1  01-07 EXISTENCE
Z2  08-14 CONTEXT
Z3  15-21 INTENT
Z4 *22-28 STRUCTURE
Z5  29-35 SEMANTIC
Z6  36-42 DYNAMIC
Z7  43-49 REPRESENTATION

* L22 REPOSITORY
^ L21 SYSTEM
v L23 SOURCE
```

The `*` before 22–28 marks the active zone, not a new ontology token.

## On-demand exact map

Opening the level map shows exact canonical names in canonical order.

No abbreviations become ontology names.

## Narrow mode

At 80×24:
- 7 zone rows;
- current level;
- immediate broader/finer;
- current definition/evidence summary below if space remains.

## Navigation

Up/Down:
- move zone/level focus depending focused subregion.

Left/Right or `[`/`]`:
- broader/finer.

Enter:
- inspect.

## Rule

Compact mode compresses **display**, never ontology ordering or semantics.
