# OX-DX Repository Naming Policy

Repository names are implementation identifiers. They support the brand; they do not automatically become the brand.

## Current repository

**Verified current repository:**

`bjo163/universal-ontology-engine`

Status: **CURRENT / VALID / DO NOT RENAME IN THIS TASK**

The repository name remains technically descriptive and historically useful.

## Default future public family

When a new repository represents a distinct public OX-DX surface, prefer:

`ox-dx-<responsibility>`

Examples of valid patterns **only if those repositories are actually needed**:

- `ox-dx-web`
- `ox-dx-docs`
- `ox-dx-cli`
- `ox-dx-sdk`

These examples are **PLANNED naming patterns**, not proof that such repositories exist.

## When to use `ox-dx-*`

Use the public family when all are true:

1. the repository represents a user-facing or ecosystem-facing OX-DX surface;
2. the responsibility is distinct from the current engine repository;
3. users benefit from recognizing the OX-DX namespace;
4. the repository is independently versioned/deployed/consumed or has a real operational boundary;
5. creating a repository is simpler than keeping the responsibility in an existing owner.

## When to keep a descriptive technical name

Use a descriptive technical repository name when:

- it is foundational infrastructure;
- the technical concept matters more than the public brand;
- an established name already has users/history;
- branding the name would reduce precision;
- the repository primarily implements a standard/specification rather than a public product surface.

The current `universal-ontology-engine` fits this rule.

## Repository creation test

Before creating a repository:

```text
NEW RESPONSIBILITY?
    │
    ├── no  → keep existing owner
    │
    └── yes
         │
         ├── independent lifecycle/deployment/consumption?
         │       ├── no  → prefer existing repository
         │       └── yes
         │
         └── public OX-DX surface?
                 ├── yes → consider ox-dx-*
                 └── no  → consider descriptive technical name
```

## Avoid repository-name explosion

Do not create one repository for every:
- crate;
- command;
- ontology TYPE;
- adapter;
- visual asset class;
- documentation section.

A new repository creates ownership, release, automation, security, and maintenance cost.

## Legacy/current names

Existing repository names remain valid unless a deliberate migration is separately approved.

A naming convention is not a retroactive migration order.

## Migration rule

If a repository is ever renamed:

1. document old and new names;
2. preserve redirects/aliases where the platform allows;
3. update cross-references;
4. preserve release history;
5. communicate compatibility impact;
6. mark the old name as deprecated where necessary;
7. never rename only for cosmetic consistency.
