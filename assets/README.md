# OX-DX Assets

This directory contains the canonical OX-DX brand foundation, visual language, and derived asset system.

## Canonical identity

- Master logo: `brand/logo/ox-dx-logo-primary.svg`
- Master icon: `brand/icon/ox-dx-icon.svg`
- Wordmark: `brand/wordmark/ox-dx-wordmark.svg`
- Lockups: `brand/lockups/`
- Approved visual symbols: `brand/symbols/`
- Compact glyph notation: `brand/glyphs/`

## Visual language

- Patterns: `vector/patterns/`
- Controlled grunge textures: `vector/textures/`
- Web backgrounds: `web/backgrounds/`
- Header / hero / footer: `web/header/`, `web/hero/`, `web/footer/`
- GitHub system: `web/github/`
- README system: `web/readme/`
- Release system: `web/releases/`
- Social / OpenGraph system: `web/social/`

## Documentation

Start with `documentation/brand-guidelines.md`, then use the manifesto, voice, tagline, logo, governance, color, typography, token, and motion specifications as the shared language for future website, documentation, UI, release, and product work.

## Source / editable

- Existing deterministic base renderer: `source/design/render_brand_assets.py`
- Brand-system renderer: `source/design/render_brand_system.py`
- Approved master geometry: `source/design/ox-dx-master-geometry.svg`

Future contributors must derive new assets from the canonical geometry and documented tokens. Do not redraw the symbol, modify the wordmark, invent palette values, or embed new technical ontology semantics in decorative graphics.

## Communication source hierarchy

- Voice and tone: `documentation/brand-voice.md`
- Message hierarchy: `documentation/communication/messaging-framework.md`
- Canonical terminology: `documentation/communication/terminology.md`
- Philosophical/engineering boundary: `documentation/communication/quranic-inspiration.md`
- Status claims: `documentation/communication/status-language.md`

Technical truth still comes from the repository's normative specifications, engineering standards, implementation, tests, and generated release/status evidence.
