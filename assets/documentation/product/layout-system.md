# OX-DX Product Layout System

OX-DX layout uses controlled asymmetry and recoverable context.

It does not default to a standard dashboard shell.

## Regions

### Context Region
Answers where the user is and what scope is active.

### Primary Surface
Current object, evidence, graph neighborhood, or inspection target.

### Secondary Surface
Related evidence, relations, comparison, or contextual interpretation.

### Inspection Rail
Provenance, status, metadata, relation details, or commands.

### Detail Surface
Expanded source, representation, event, or raw technical view.

## Seven-lane inspiration

Wide layouts may use the Seven-Lane Interrupted Field:
- three lanes primary;
- one movable void;
- three lanes context/inspection.

This is a design logic, not a mandatory CSS grid.

## Practical implementation

Real applications may use CSS Grid/Flexbox and conventional responsive primitives.

The visual identity comes from:
- asymmetric emphasis;
- intentional void;
- interrupted boundaries;
- witness/trace relationships;
- context prioritization.

Not from forcing exactly seven DOM columns.

## Common patterns

### Instrument
Context rail + large primary inspection field + recoverable detail.

### Compare
Two unequal primary fields separated by an intentional void / relation region.

### Trace
Primary object + horizontal/vertical provenance/relationship rail.

### Resolve
Context stays stable while finer detail replaces or expands a portion of the primary field.

## Avoid

- permanent left sidebar merely because dashboards use one;
- 3-column KPI card grids;
- giant empty cards;
- every region boxed identically;
- decorative asymmetry that harms scan order.
