# OX-DX Design Tokens

## Color

| Token | Value | Use |
|---|---|---|
| void | `#07090A` | Primary dark canvas |
| charcoal | `#111418` | Raised dark surface |
| graphite | `#20252B` | Secondary dark surface |
| white | `#F5F7F8` | Primary foreground |
| muted | `#99A2AA` | Annotation / secondary text |
| acid | `#8CFF00` | Optional accent |
| cyan | `#00D8FF` | Primary signal accent |
| ultraviolet | `#9C4DFF` | Optional secondary accent |

The canonical logo must always remain valid in pure black or pure white. Neon accents are optional.

## Geometry

- Base spacing unit: **8 px**.
- Core grid: multiples of **8 px**; larger editorial layouts may use 32 / 64 px modules.
- Corner radius: **0 px** for brand geometry; product UI may define its own tokens.
- Canonical symbol stroke at 160 px artboard: **10 px**.
- Fine annotation line: **1–2 px**.
- Clear space: at least **1/4 of the symbol width** around the canonical mark.
- Minimum symbol size: **16 px**. At 16–24 px, use the simplified favicon/icon variant and omit annotation ticks.

## Contrast and background

- Dark-first usage: `#F5F7F8` on `#07090A`.
- Light usage: pure black / `#000000` on `#F5F7F8`.
- Do not place the canonical logo over noisy imagery without a quiet field.
- Grunge belongs to the surface layer, never inside the master geometry.
