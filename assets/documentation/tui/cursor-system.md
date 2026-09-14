# OX-DX TUI Cursor System

Cursor communicates **where the user is**, not what the data means.

## Default cursor

When no text input is active:
- terminal hardware cursor may be hidden;
- focus marker identifies current interactive target.

## Focus cursor

Unicode: `›`

ASCII: `>`

Placed adjacent to focused action/item.

## Selection cursor

Selection is not the cursor.

Use selection geometry/marker plus focus marker if the selected object is also focused.

## Resolution cursor

Resolution navigation uses a dedicated rail anchor:

Unicode: `◆ L22 REPOSITORY`

ASCII: `* L22 REPOSITORY`

This indicates current resolution location, not generic focus.

## Input cursor

Use the terminal-native text cursor in command/input fields.

Do not fake a blinking custom cursor if native cursor is available.

## Blink

Respect terminal/user preferences.

Do not use rapid blinking as OX-DX identity.

## Mouse

Mouse hover never becomes the only cursor/focus indicator.

Keyboard focus remains canonical.
