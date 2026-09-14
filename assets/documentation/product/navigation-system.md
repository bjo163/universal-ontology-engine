# OX-DX Navigation System

Navigation should answer:

**Where am I?**

**What am I looking at?**

**How deep am I?**

**What can I resolve next?**

## Navigation dimensions

### Context
Repository, product surface, source, graph scope, comparison set, or active object context.

### Scope
The current bounded set of evidence/nodes.

### Resolution
Current descriptive depth and valid broader/finer transitions.

## Global navigation

Keep small.

Use only for real product-level destinations.

Do not turn every engine concept into a global nav item.

## Context navigation

May show:
- active repository/source;
- selected object;
- structural parent when meaningful;
- relation neighborhood;
- inspection history.

Breadcrumbs are appropriate only where a real hierarchical path exists.

## Resolution navigation

Use Resolution Spine concepts.

Show current level and nearby valid transitions, not all 49 levels by default.

## History

Back/forward history should preserve inspection state where practical:
- focus;
- graph scope;
- resolution;
- filters;
- expanded evidence.

## Deep links

Future interfaces should make important inspection states addressable when technically practical.

## Mobile

Context may become:
- a top scope strip;
- a modal/sheet;
- an expandable context region.

Do not preserve desktop sidebars solely for visual consistency.

## Avoid

- endless nested menus;
- ambiguous “More” as the primary architecture;
- navigation that hides whether movement changed context, focus, or resolution.
