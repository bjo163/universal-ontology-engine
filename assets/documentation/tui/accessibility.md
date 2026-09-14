# OX-DX TUI Accessibility

Important information must survive:
- keyboard-only use;
- monochrome;
- 16-color;
- ASCII fallback;
- reduced motion;
- silent mode.

## Keyboard

Every core action has arrow/Enter/Tab/Esc access.

Vi keys are optional aliases.

## Focus

Visible using marker + weight/border, not color alone.

## Status

Text labels remain explicit.

## High contrast

- prefer terminal/user default colors;
- no essential dim text;
- disable optional disturbance;
- use bold/inverse carefully;
- preserve focus marker.

## ASCII

Core layout and semantics remain intact.

## Reduced motion

No semantic information is lost when animation is disabled.

## Silent

Sound is optional.

Terminal bell is not required.

## Screen readers

A future implementation should provide:
- stable logical focus order;
- concise status announcements;
- avoid rapidly rewriting large terminal regions;
- textual equivalents for spatial graph topology.

## Flash

No rapid blinking/flashing.

## User control

Expose explicit preferences for:
- color capability override;
- ASCII;
- reduced motion;
- sound mode;
- mouse.
