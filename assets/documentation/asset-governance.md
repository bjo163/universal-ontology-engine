# OX-DX Asset Governance

## Asset classes

### MASTER ASSET

The authoritative editable geometry or specification from which identity-critical variants derive. Master SVG geometry is canonical.

### DERIVED ASSET

A PNG, WEBP, alternate background treatment, layout crop, or other export produced from a master or approved composition. Derived assets must not silently change geometry.

### EXPERIMENTAL ASSET

A candidate visual that may be reviewed but must not replace canonical identity assets or appear as the default brand system without approval.

### DEPRECATED ASSET

An asset intentionally retained for migration or history but no longer recommended for new usage.

## Status vocabulary

- `CANONICAL` — source of truth.
- `APPROVED` — valid derived or supporting asset.
- `EXPERIMENTAL` — review-only.
- `DEPRECATED` — retained but should not be used for new work.

## Rules

1. Master SVG is canonical; PNG / WEBP are exports.
2. Color or background variants must preserve the approved geometry.
3. New symbols, glyphs, patterns, and compositions must inherit documented design tokens.
4. Grunge is a surface treatment and must never corrupt master logo geometry.
5. Decorative symbols are not ontology semantics and may not create technical levels, relations, or claims.
6. Experimental assets cannot become canonical without explicit project-owner approval.
7. Do not commit proprietary fonts, generation caches, temporary previews, or model metadata.
8. Deterministic filenames use `ox-dx-{category}-{name}-{variant}.{ext}` where a variant is needed.
9. Every canonical / approved asset must be listed in `assets/ASSET_INDEX.md`.
10. Validation must pass before brand-system changes are committed.
