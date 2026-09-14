# OX-DX Future Product Extensions

These are possible responsibility boundaries.

They are not existing products unless separately implemented.

## OX-DX Web

Potential role:
- public/product web interface;
- graph/evidence exploration;
- interactive documentation or inspection.

Must not redefine ontology or engine semantics.

## OX-DX Explorer

Potential role:
- dedicated graph/resolution/evidence exploration interface.

Create only if its responsibility is distinct from OX-DX Web.

## OX-DX Docs

Potential role:
- public documentation property.

Existing in-repository technical documentation remains valid until there is a real operational reason to separate it.

## OX-DX CLI

Potential future public wrapper/name.

Current implementation remains `ontology-engine`.

No CLI rename is authorized by this handoff.

## OX-DX SDK

Potential developer-facing package only if a genuine supported SDK contract exists.

Do not create an SDK name for ordinary internal crates.

## Rule

A future extension must have:
1. a distinct audience;
2. a distinct responsibility;
3. a technical owner;
4. an independent lifecycle or operational reason;
5. no duplicate ownership.

Brand architecture remains authoritative.
