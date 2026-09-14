# OX-DX Communication Safety

OX-DX should never manufacture credibility.

## Prohibited patterns

### False technical claims

Do not say a capability exists when the repository does not implement or verify it.

### Fake benchmarks

Do not publish invented throughput, accuracy, scale, latency, memory, or comparison numbers.

### Fake authority

Do not imply endorsement, certification, scientific consensus, or institutional authority that does not exist.

### Fabricated references

Do not invent papers, standards, repository evidence, test results, quotations, or citations.

### Fabricated Quranic material

Do not invent verses, Arabic text, translations, or claims of divine validation.

### False implementation status

Do not convert `PLANNED` or `GATED` work into “implemented,” “ready,” or “complete.”

### Marketing disguised as engineering evidence

A slogan is not a benchmark. A diagram is not proof. A roadmap is not an implementation. A successful demo is not universal validation.

## Language of uncertainty

When evidence is incomplete, say so.

Preferred:
- “not established by the current evidence”;
- “not implemented at this scope”;
- “planned”;
- “gated”;
- “observed in this input”;
- “supported by this test”;
- “inferred from the following evidence”.

## Claim ladder

Use the weakest truthful claim that communicates the result:

```text
OBSERVED
  ↓
SUPPORTED
  ↓
VERIFIED AT A NAMED SCOPE
  ↓
IMPLEMENTED
  ↓
HARDENED AT A NAMED SCOPE
```

Do not jump upward without evidence.

## Forbidden hype in affirmative project claims

Avoid unsupported phrases such as:
- revolutionary;
- AI-powered;
- truth engine;
- understands everything;
- world's first;
- fully universal;
- seamless;
- next-generation.

These terms may appear in documentation only as explicit examples of wording to reject.

## Final safety test

Before publishing:

**Would a skeptical engineer be able to identify the evidence behind this claim?**

If not, narrow the claim.
