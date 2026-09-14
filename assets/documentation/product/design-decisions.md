# OX-DX Product Design Decisions

This is the design-governance record for the Brand → Product Handoff v1.0.

## Decision 001 — Do not use a generic dashboard as the product metaphor

**WHY**  
OX-DX is about inspection, evidence, graph relationships, and resolution. KPI-card dashboards imply business monitoring/management as the primary task.

**CONSEQUENCE**  
Layouts prioritize current object, evidence, relations, provenance, and detail.

**REJECTED ALTERNATIVE**  
Permanent sidebar + header + 3-column card grid as the default shell.

---

## Decision 002 — Graph relationships are first-class

**WHY**  
The technical model distinguishes containment from projection, representation, observation, reference, and other relation classes.

**CONSEQUENCE**  
Relationship View / Graph Surface is foundational, and non-containment edges cannot disappear into generic metadata.

**REJECTED ALTERNATIVE**  
Tree navigation as the universal information architecture.

---

## Decision 003 — Observation receives separate visual language

**WHY**  
Observation is not ownership.

**CONSEQUENCE**  
Observation Marker uses offset witness geometry and relation rather than nesting under the target.

**REJECTED ALTERNATIVE**  
Place events/executions visually inside target nodes.

---

## Decision 004 — Provenance remains visible but secondary

**WHY**  
Provenance supports traceability but does not define identity.

**CONSEQUENCE**  
Use Provenance Rails/Tails, inspectable source metadata, and explicit origin context.

**REJECTED ALTERNATIVE**  
Use paths/line numbers as the primary visual identity.

---

## Decision 005 — Resolution is an interaction model

**WHY**  
The ontology spans broad-to-fine descriptive scales while allowing unmaterialized gaps.

**CONSEQUENCE**  
Resolution Spine supports movement across justified detail without pretending a 49-deep tree exists.

**REJECTED ALTERNATIVE**  
Folder-tree drilldown through every canonical TYPE.

---

## Decision 006 — Status is text + geometry first, color second

**WHY**  
Accessibility and technical honesty require meaning to survive monochrome.

**CONSEQUENCE**  
Every status combines label with icon/geometry/persistence.

**REJECTED ALTERNATIVE**  
Green = verified, yellow = uncertain, red = unknown without textual definition.

---

## Decision 007 — Density is task-dependent

**WHY**  
Technical inspection can be dense; conceptual reasoning benefits from space.

**CONSEQUENCE**  
LOW / MEDIUM / HIGH / INSPECTION density modes guide composition.

**REJECTED ALTERNATIVE**  
Always-dense “developer UI” as an identity trait.

---

## Decision 008 — Conventional controls may remain conventional

**WHY**  
Originality must not reduce usability.

**CONSEQUENCE**  
Search, close, checkbox, accessibility, native form controls may use established patterns.

**REJECTED ALTERNATIVE**  
Redesign every basic control into Interrupted Evidence Field geometry.

---

## Decision 009 — AI is subordinate to evidence

**WHY**  
AI-generated interpretation is not observed or verified evidence.

**CONSEQUENCE**  
Future AI suggestions must be labeled and traceable; they cannot silently rewrite evidence.

**REJECTED ALTERNATIVE**  
AI assistant/chat object as the primary product identity.

---

## Decision 010 — Product tokens extend by mapping, not by reinvention

**WHY**  
Brand Bible already defines palette, spacing, geometry, motion, and typography.

**CONSEQUENCE**  
`assets/design-tokens/` maps those values into implementation formats.

**REJECTED ALTERNATIVE**  
A new UI-specific 4/8/12 spacing scale, new neon colors, or separate animation timings.
