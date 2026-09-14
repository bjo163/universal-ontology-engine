# OX-DX TUI Verification View

Verification should feel:

**stable · settled · precise**

## Requirement

Use VERIFIED only when a named validation/check has actually passed.

Always show verification scope when ambiguity matters.

## Example

```text
VERIFY

✓ registry          PASS
✓ canonical levels  49
✓ ontology          1.0.0

scope  specifications/universal-ontology-v1.0.json
state  VERIFIED
```

ASCII:

```text
VERIFY

OK registry          PASS
OK canonical levels  49
OK ontology          1.0.0

scope  specifications/universal-ontology-v1.0.json
state  VERIFIED
```

Values in production come from actual validation.

## Failure

Failed verification becomes diagnostic/error state.

Do not show partial failure as “almost verified”.

## Motion

Optional:
- local irregularity settles;
- confirmation marker appears.

No confetti or celebratory animation.

## Sound

Default silent or optional minimal VERIFY cue.

Never required.
