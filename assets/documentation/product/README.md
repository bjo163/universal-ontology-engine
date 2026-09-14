# OX-DX Product Design System — Brand → Product Handoff v1.0

**Status: CANONICAL PRODUCT-DESIGN HANDOFF**

Master brand authority: `../OX-DX-BRAND-BIBLE-v1.0.md`

This directory translates OX-DX brand rules into practical product-design guidance. It does **not** implement a website, dashboard, application, CLI change, ontology change, or engine behavior.

## Product metaphor

**THE PRODUCT IS AN INSTRUMENT.**

The user is inspecting a system.

The interface is the instrument.

The evidence is the material.

The ontology is the vocabulary.

The graph is the connective tissue.

Resolution is the movement.

The user remains the investigator.

Canonical product statement:

**THE UI SHOULD FEEL LIKE INSPECTING A SYSTEM, NOT OPERATING A DASHBOARD.**

Primary interaction model:

`OBSERVE → TRACE → RESOLVE`

Secondary explanatory model:

`SEE → INSPECT → CONNECT → RESOLVE → UNDERSTAND`

These are interaction models, not engine pipelines.

## System map

- Philosophy: `ui-philosophy.md`
- UX principles: `ux-principles.md`
- Information hierarchy: `information-hierarchy.md`
- UI layers: `ui-layers.md`
- Component families: `component-map.md`
- Component specifications: `component-specifications.md`
- Component states: `component-states.md`
- Graph: `graph-ui.md`
- Resolution: `resolution-ui.md`
- Evidence: `evidence-viewer.md`
- Provenance: `provenance-ui.md`
- Observation: `observation-ui.md`
- Representation: `representation-ui.md`
- Status: `status-system.md`
- Certainty: `certainty-language.md`
- Typography: `product-typography.md`
- Color: `product-color.md`
- Geometry: `product-geometry.md`
- Density: `density-system.md`
- Layout: `layout-system.md`
- Navigation: `navigation-system.md`
- Interaction: `interaction-principles.md`
- Command surface: `command-surface.md`
- Empty/error/loading: `empty-states.md`, `error-states.md`, `loading-states.md`
- Motion: `product-motion.md`
- Accessibility: `accessibility.md`
- Responsive: `product-responsive.md`
- Data density: `data-density.md`
- AI boundary: `intelligence-boundary.md`
- Do / Don't: `do-and-dont.md`
- Brand traceability: `brand-traceability.md`
- Design decisions: `design-decisions.md`
- Originality review: `originality-review.md`
- Handoff audit: `product-handoff-audit.md`

## Product design language

OX-DX should not become another dark developer dashboard.

Avoid defaulting to:
- generic sidebar + KPI cards;
- CRUD-first forms;
- neon cyberpunk decoration;
- glass panels;
- dashboard widgets chosen because a component library already has them.

Prefer:
- inspection surfaces;
- evidence-first hierarchy;
- explicit graph relations;
- contextual resolution;
- provenance kept visible;
- deliberate voids;
- controlled asymmetry;
- restrained status expression;
- progressive disclosure.

## Implementation handoff

Implementation-friendly tokens live in:

`assets/design-tokens/`

These files map existing Brand Bible tokens to CSS/Tailwind/component-library consumption. They do not create a second token system.

## Semantic boundary

Product UI vocabulary never redefines:
- ontology TYPEs;
- KINDs;
- graph edge semantics;
- phase gates;
- implementation status;
- CLI behavior.

If a product concept is not backed by engine data, the interface must label it as conceptual, unavailable, unknown, or future—not invent evidence.
