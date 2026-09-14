# OX-DX Component States

Every major component should specify only states that make sense for that component.

Canonical state vocabulary for component behavior:

## DEFAULT
Stable, readable base state.

## HOVER
Preview additional context without changing identity or committing selection.

## FOCUS
Keyboard-accessible witness state. Must remain visible in high contrast and without color.

## SELECTED
Adds explicit witness/selection geometry while preserving the object's native identity.

## EXPANDED
Reveals more evidence, relations, detail, or representation.

## DISABLED
Action unavailable. Explain reason when it matters.

## LOADING
Operation in progress. State what operation is occurring when known.

## ERROR
Operation failed. Preserve evidence/context and identify failure class.

## UNKNOWN
Current evidence cannot establish the requested state/relationship/value.

## State rules

- Hover is never required to access essential information.
- Focus and selected are not the same state.
- Expanded means more context/resolution, not a bigger decorative box.
- Disabled is not merely lower opacity if the state must remain legible.
- Loading never shows fake percentages.
- Error never erases the evidence that helps diagnose it.
- Unknown is a valid terminal state.
- Status vocabulary such as OBSERVED or PROJECTED belongs to domain/product meaning, not generic component-interaction state.
