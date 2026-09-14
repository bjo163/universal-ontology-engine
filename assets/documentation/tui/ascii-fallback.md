# OX-DX ASCII Fallback

Every important visual element has an ASCII-safe form.

## Core mapping

| Unicode | ASCII |
|---|---|
| `◆` | `*` |
| `◇` | `o` |
| `›` | `>` |
| `✓` | `OK` |
| `×` | `X` |
| `▸` | `>` |
| `▾` | `v` |
| `─` | `-` |
| `│` | `|` |
| `┄` | `.` |
| `→/▷` | `->` / `>` |
| `·` | `.` |
| `⋯` | `...` |

## ASCII-safe graph

```text
* SOURCE --contains--> UNIT
         ..observed_at--> EVENT
```

## ASCII-safe resolution

```text
  21 SYSTEM
* 22 REPOSITORY
  23 SOURCE
```

## 16-color

Use standard ANSI colors only as reinforcement.

## Monochrome

Use labels + markers + spacing.

## No Unicode assumption

Implementation should provide an explicit ASCII mode in addition to capability detection.

## Width

Do not use characters with uncertain width in core layout.

## Validation checklist

ASCII mode must retain:
- focus;
- selected state;
- observation separation;
- relation type;
- current level;
- error/warning/unknown distinction;
- verification status.
