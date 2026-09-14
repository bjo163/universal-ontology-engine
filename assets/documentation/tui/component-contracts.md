# OX-DX TUI Component Contracts

These contracts target future Ratatui/Crossterm implementation without prescribing internal architecture.

## Shell

**Purpose:** compose terminal regions.  
**Inputs:** terminal size, active mode, context, component visibility.  
**State:** focused region, layout profile.  
**Events:** resize, focus-region change.  
**Render:** context + primary + optional secondary + status + command.  
**Keyboard:** Tab/Shift+Tab region traversal.  
**Fallback:** single-column primary surface.

## Context Bar

**Purpose:** orient user.  
**Inputs:** scope, current object summary, resolution, mode.  
**State:** none beyond display.  
**Events:** context/mode/resolution update.  
**Render:** compact one-line context.  
**Keyboard:** optionally focusable if context switching exists.  
**Fallback:** plain text line.

## Resolution Spine

**Purpose:** navigate canonical TYPE order.  
**Inputs:** 49 canonical levels, current level, materialization/evidence summaries.  
**State:** focused level, current level.  
**Events:** broader/finer/focus/inspect.  
**Render:** compact or deep spine.  
**Keyboard:** arrows, Enter, optional brackets/vi aliases.  
**Fallback:** indexed text list.

## Evidence View

**Purpose:** source-first evidence inspection.  
**Inputs:** native evidence, classification, relations, provenance summary.  
**State:** focus subsection, scroll.  
**Events:** inspect/trace/source selection.  
**Render:** evidence before interpretation.  
**Keyboard:** arrows/PageUp/PageDown/Enter.  
**Fallback:** linear text sections.

## Source View

**Purpose:** show native source/span.  
**Inputs:** source text/path/span/native kind.  
**State:** scroll/focused span.  
**Events:** scroll, span focus.  
**Render:** line numbers + source + witness.  
**Keyboard:** arrows/PageUp/PageDown.  
**Fallback:** plain text with `|` marker.

## Graph View

**Purpose:** traverse typed edges.  
**Inputs:** focused node, neighbors, exact edge kinds, scope/resolution.  
**State:** focus, expansion, traversal history.  
**Events:** neighbor, expand, collapse, trace, inspect.  
**Render:** spatial topology or grouped relation list.  
**Keyboard:** arrows, Enter, t/e/c.  
**Fallback:** linear edge list.

## Trace View

**Purpose:** follow an ordered inspection path.  
**Inputs:** hops, edge kinds, source/target, provenance/resolution.  
**State:** focused hop.  
**Events:** next/previous/inspect/return.  
**Render:** numbered hops with exact relation labels.  
**Keyboard:** Up/Down/Enter/Esc.  
**Fallback:** plain numbered list.

## Observation View

**Purpose:** keep observations separate from structure.  
**Inputs:** observation, target, relation, provenance.  
**State:** focused observation.  
**Events:** inspect/trace target/source.  
**Render:** offset witness rail.  
**Keyboard:** arrows/Enter/t.  
**Fallback:** labeled OBSERVATION section.

## Provenance View

**Purpose:** show where/how evidence was observed.  
**Inputs:** source groups, revision/span/observer/time where present.  
**State:** selected source group.  
**Events:** switch source, inspect source.  
**Render:** provenance rail.  
**Keyboard:** Up/Down/Enter.  
**Fallback:** key/value text.

## Diagnostics

**Purpose:** expose operational messages and retained evidence.  
**Inputs:** diagnostics with severity/scope/evidence links.  
**State:** filter/focused diagnostic.  
**Events:** filter/focus/inspect evidence.  
**Render:** severity list + detail.  
**Keyboard:** arrows, Enter, filter keys if implemented.  
**Fallback:** sequential text log.

## Status Bar

**Purpose:** quiet orientation.  
**Inputs:** context/level/mode/state/diagnostic counts.  
**State:** none.  
**Events:** updates.  
**Render:** one line.  
**Keyboard:** none by default.  
**Fallback:** plain text.

## Command Bar

**Purpose:** input and context actions.  
**Inputs:** input buffer, suggestions, command/action registry.  
**State:** cursor/history/suggestion focus.  
**Events:** key input, submit, cancel.  
**Render:** prompt + suggestions.  
**Keyboard:** text entry/arrows/Enter/Esc.  
**Fallback:** simple `>` input.

## Help Surface

**Purpose:** context-relevant key/action help.  
**Inputs:** active mode/component key bindings.  
**State:** scroll/filter if full help.  
**Events:** open/close/scroll.  
**Render:** compact strip or overlay.  
**Keyboard:** ?, F1, Esc.  
**Fallback:** plain list.

## Focus Marker

**Purpose:** show interaction location.  
**Inputs:** focused target.  
**State:** focus only.  
**Events:** focus changes.  
**Render:** `›`/`>` + optional border emphasis.  
**Keyboard:** none.  
**Fallback:** `>`.

## Resolution Marker

**Purpose:** show current canonical level.  
**Inputs:** level number/TYPE.  
**State:** current resolution.  
**Events:** resolution update.  
**Render:** `◆ Lnn TYPE` / `* Lnn TYPE`.  
**Keyboard:** delegated to Resolution Spine.  
**Fallback:** exact text.
