# OX-DX TUI Diagnostics

Diagnostics are precise operational messages.

| Level | Symbol UTF-8 | ASCII | Priority | Screen behavior |
|---|---|---|---|---|
| INFO | `·` | `.` | low | log/rail only |
| NOTICE | `◇` | `o` | low–medium | local status/rail |
| WARNING | `!` | `!` | medium | visible local message |
| ERROR | `×` | `X` | high | focused failure region |
| FATAL | `×!` | `X!` | highest | stop affected workflow; preserve evidence |

## INFO

Facts that may aid inspection.

No sound.

## NOTICE

Meaningful state change not requiring corrective action.

Usually silent.

## WARNING

Action may be required.

Show:
- cause/context;
- affected scope;
- next action if known.

Optional minimal notification sound only if user enables NOTIFY.

## ERROR

Operation failed.

Preserve source/evidence.

## FATAL

Workflow cannot safely continue.

Do not erase terminal context before showing it.

## Diagnostic count

Status bar may show:

`diag 2W 1E`

ASCII-safe.

Counts must be real.

## Color

Color reinforces severity.

Text + symbol remains primary.
