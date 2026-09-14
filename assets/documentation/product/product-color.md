# OX-DX Product Color System

Source: `../technical/color-system.md`.

No new palette is created here.

| Product role | Canonical token | Value | Use |
|---|---|---|---|
| BASE | Void | #07090A | primary dark canvas |
| SURFACE | Charcoal | #111418 | raised inspection surface |
| SURFACE / BORDER | Graphite | #20252B | boundaries and depth |
| TEXT | White | #F5F7F8 | primary readable foreground |
| MUTED | Muted Steel | #99A2AA | annotation and secondary text |
| STRUCTURE | White / Black | #F5F7F8 / #000000 | structural geometry by surface |
| SIGNAL / OBSERVATION | Electric Cyan | #00D8FF | witness/observation reinforcement |
| ATTENTION / VERIFIED | Acid | #8CFF00 | rare attention or verified reinforcement |
| PROJECTION | Ultraviolet | #9C4DFF | projected/displaced views |
| UNKNOWN | no dedicated bright color | — | express using break/void + label |

## Restraint

Do not assign a bright color to every semantic role.

Structure and text carry most meaning.

Accents reinforce selected distinctions.

## Status

Status markers must use label + geometry. Color is secondary.

## Unknown

Unknown uses:
- explicit text;
- deliberate break/void;
- restrained neutral treatment.

Do not use red merely because the system lacks evidence.

## Error

Product error color is not defined by the brand palette in this handoff.

Implementations should choose an accessible error treatment consistent with platform conventions without pretending the color is a canonical ontology/status token.

## Light / high contrast

Follow canonical surface rules.

Meaning must survive:
- monochrome;
- high contrast;
- reduced accent visibility.

## Forbidden

- rainbow TYPE palette;
- neon role explosion;
- gradient-as-meaning;
- status encoded only by hue;
- invented brand red/orange/yellow merely to mimic common dashboards.
