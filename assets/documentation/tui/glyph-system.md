# OX-DX TUI Glyph System

No Nerd Font dependency.

Every important glyph has ASCII fallback.

| Meaning | Unicode | ASCII | Use |
|---|---|---|---|
| structure | `■` | `#` | structural/materialized object |
| relation | `─▷` | `->` | directional relation when direction matters |
| trace | `┄▷` | `..>` | trace/provenance path |
| observation | `◇` | `o` | offset witness/observation |
| resolution | `◆` | `*` | current resolution anchor |
| signal | `·` | `.` | low-weight active signal/cadence |
| unknown | `⋯` | `...` | unresolved/open continuity |
| warning | `!` | `!` | warning |
| verified | `✓` | `OK` | named verification passed |
| error | `×` | `X` | operational failure |
| focus | `›` | `>` | current focus/input witness |
| selected | `◆` | `*` | committed selection; context distinguishes from resolution |
| collapsed | `▸` | `>` | collapsible region closed |
| expanded | `▾` | `v` | collapsible region open |

## Support assumptions

### Level A/B/C UTF-8
May use:
- box drawing;
- geometric shapes;
- check mark.

### Level D ASCII
Use only:
- `| - + > < * # . ! X v [ ]`

## Ambiguity rule

Because `◆` may represent selected or resolution anchor, never rely on glyph alone.

Pair with:
- position;
- label;
- context.

## Avoid

- emoji;
- private-use glyphs;
- Nerd Font icons;
- ambiguous pictograms;
- decorative Unicode that may render double-width.
