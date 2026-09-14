# OX-DX TUI Resolution Navigation

The 49 canonical TYPEs are a **resolution spine over a graph**.

They are not mandatory filesystem nesting.

## Resolution Spine

Always expose:
- current level;
- exact TYPE;
- immediate higher/broader level;
- immediate lower/finer level;
- current zone;
- whether evidence exists at the current level;
- whether neighboring levels are materialized when known.

## Compact spine

```text
   20 PRODUCT
   21 SYSTEM
◆  22 REPOSITORY   evidence: 1
   23 SOURCE        evidence: 12
   24 UNIT          evidence: 7
```

ASCII:

```text
   20 PRODUCT
   21 SYSTEM
*  22 REPOSITORY   evidence: 1
   23 SOURCE        evidence: 12
   24 UNIT          evidence: 7
```

Evidence counts appear only when real counts exist.

## Navigation actions

Future TUI interaction vocabulary:
- move broader;
- move finer;
- inspect;
- expand context;
- collapse context;
- resolution shift;
- return/history.

Do not call a resolution shift “zoom” in copy unless the UI explicitly distinguishes visual zoom from ontology resolution.

## Keyboard intent

- Up/Down: move focus through visible spine entries.
- Enter: inspect focused level/object.
- Left/Right: move broader/finer when the current context supports it.
- Esc/Backspace: return to previous inspection context.
- `[` / `]`: optional resolution shortcuts.
- `j/k/h/l`: optional aliases, never mandatory.

## Materialization

Canonical level existence and resource materialization are different.

Display:
- exact canonical TYPE always;
- materialized marker only when actual evidence exists;
- `UNMATERIALIZED` when that state is known;
- `UNKNOWN` when evidence does not establish materialization.

Do not manufacture placeholder nodes.

## Seven-Zone Map

Canonical zone order:

1. EXISTENCE — 01–07
2. CONTEXT — 08–14
3. INTENT — 15–21
4. STRUCTURE — 22–28
5. SEMANTIC — 29–35
6. DYNAMIC — 36–42
7. REPRESENTATION — 43–49

A compact zone map is available on demand.

## Full canonical order

```text
01 UNIVERSE      02 CREATION      03 ORDER         04 REALITY
05 REALM         06 WORLD         07 DOMAIN

08 ECOSYSTEM     09 ORGANIZATION  10 COMMUNITY     11 REGION
12 ENVIRONMENT   13 NETWORK       14 CONTEXT

15 PURPOSE       16 MISSION       17 OBJECTIVE      18 PROGRAM
19 PROJECT       20 PRODUCT       21 SYSTEM

22 REPOSITORY    23 SOURCE        24 UNIT           25 MODULE
26 SUBSYSTEM     27 COMPONENT     28 ELEMENT

29 SYMBOL        30 ENTITY        31 PROPERTY       32 RELATION
33 OPERATION     34 FUNCTION      35 BEHAVIOR

36 STATE         37 EVENT         38 PROCESS        39 FLOW
40 TRANSITION    41 ACTION        42 EXECUTION

43 INSTRUCTION   44 EXPRESSION    45 VALUE          46 DATA
47 TOKEN         48 CHARACTER     49 BIT
```

Wrapping is allowed; ordering is not.

## Narrow terminal strategy

At 80×24:
- show current level ±2;
- show zone label;
- show exact current TYPE;
- show evidence count/status;
- use `levels` or help action to open the full map.

## Deep mode

At larger sizes:
- show full current zone;
- adjacent zones summarized;
- evidence and relations alongside the spine.

See `resolution-deep-view.md`.
