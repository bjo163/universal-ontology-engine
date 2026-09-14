# OX-DX Domain Architecture

Domain architecture follows product architecture. It must not create product boundaries by itself.

## Primary target

Public brand:

**OX-DX**

Target domain:

`ox-dx.com`

**Status: PLANNED DOMAIN TARGET**

This document does not claim that `ox-dx.com` is registered, owned, delegated, active, or deployed.

## Default rule

Prefer one public domain until operational evidence requires another surface.

Do not create subdomains just because they look architecturally complete.

## Potential future subdomains

Only when a real service exists:

| Candidate | Intended responsibility | Status |
|---|---|---|
| `docs.ox-dx.com` | Public OX-DX documentation | PLANNED / CONDITIONAL |
| `api.ox-dx.com` | Public API endpoint | PLANNED / CONDITIONAL |
| `app.ox-dx.com` | Interactive application/interface | PLANNED / CONDITIONAL |

These names reserve direction only.

## When a subdomain is justified

Use a subdomain when there is a real boundary in at least one of:
- deployment;
- security;
- operational ownership;
- product lifecycle;
- caching/routing;
- audience/interaction model.

Do not split by documentation category or internal crate.

## Website naming

The public website should present:

**OX-DX**

Descriptor:

**Universal Ontology & Experience Engine**

A future implementation repository may use:

`ox-dx-web`

Status: **PLANNED / CONDITIONAL**

Do not create or deploy it as part of brand architecture work.

## Compatibility

If domains ever move:

1. preserve redirects where possible;
2. update canonical links;
3. maintain old URLs long enough for external consumers;
4. do not change technical package names merely because a domain changed.
