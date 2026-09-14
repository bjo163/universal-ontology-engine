# OX-DX TUI Text Hierarchy

Hierarchy must survive plain ANSI output.

## TITLE

Use:
- uppercase `OX-DX` or exact object title;
- bold if supported;
- one line.

## SECTION

Uppercase compact label:

`EVIDENCE`

`RELATIONS`

`PROVENANCE`

No oversized ASCII headings.

## PRIMARY

Current object or most important evidence.

Use normal/bold foreground.

## SECONDARY

Related context.

Lower visual weight but still readable.

## METADATA

Compact aligned labels.

Example:

`kind        rust-crate`

## TECHNICAL

Exact:
- TYPE;
- KIND;
- edge names;
- identifiers;
- commands;
- registry values.

Use monospace naturally; do not add fake code syntax.

## SOURCE

Native source text receives priority.

Line numbers are secondary.

## PROVENANCE

Source/path/span/revision context stays visually distinct from identity.

## STATUS

Always textual:
- `OBSERVED`
- `VERIFIED`
- `UNKNOWN`
- etc.

## COMMAND

Input line and command hints.

## ERROR

Precise failure class + message.

## WARNING

Actionable caution.

## UNKNOWN

Explicit `UNKNOWN` or `UNRESOLVED` copy with quiet geometry.

## Style attributes

Use sparingly:
- bold for focus/primary;
- underline for active actionable labels if needed;
- inverse for focus cursor/selected row in low-color terminals.

Avoid widespread dim text if it harms readability.
