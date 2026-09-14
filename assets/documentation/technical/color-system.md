# OX-DX Color System

OX-DX is dark-first and monochrome-capable. Neon colors are signals, not decoration.

| Role | Name | HEX | RGB | Usage | Contrast / behavior |
|---|---|---|---|---|---|
| PRIMARY / SURFACE | Void | `#07090A` | `7, 9, 10` | Primary canvas, hero, GitHub/release backgrounds | Pair with White; safest high-contrast environment. |
| SECONDARY / SURFACE | Charcoal | `#111418` | `17, 20, 24` | Raised or alternate dark surface | Pair with White or Muted; avoid low-opacity cyan text. |
| SURFACE | Graphite | `#20252B` | `32, 37, 43` | Secondary technical panels / backgrounds | Pair with White; use Muted only at readable sizes. |
| TEXT | White | `#F5F7F8` | `245, 247, 248` | Primary text and geometry | Canonical foreground on dark surfaces. |
| MUTED | Muted Steel | `#99A2AA` | `153, 162, 170` | Annotation, secondary labels, grid lines | Never use for tiny critical text on Graphite. |
| ACCENT | Acid | `#8CFF00` | `140, 255, 0` | Milestones, success-like editorial signal, rare emphasis | Signal only; do not flood large surfaces. |
| SIGNAL | Electric Cyan | `#00D8FF` | `0, 216, 255` | Observation node, resolution axis, primary technical signal | Canonical accent. Use against dark fields. |
| ACCENT | Ultraviolet | `#9C4DFF` | `156, 77, 255` | Secondary editorial / discussion signal | Avoid as body text at small sizes. |
| INVERSE | Black | `#000000` | `0, 0, 0` | Monochrome mark on light backgrounds | Pair with White/light surfaces. |

## Rules

1. The logo must work in pure black or pure white.
2. Cyan is the default signal accent. Acid and Ultraviolet are contextual, not competing primaries.
3. Do not add new palette colors without explicit brand approval.
4. Grunge uses opacity and texture, not extra colors.
5. Critical text must prioritize contrast over atmosphere.
