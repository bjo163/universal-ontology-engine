# OX-DX Canonical UX Principles

## 1. EVIDENCE FIRST

Show source/evidence before interpretation whenever practical.

A classification should have a discoverable path back to the material that supports it.

## 2. DON'T FORCE THE SHAPE

Do not force information into a UI pattern merely because the component exists.

If the evidence is graph-shaped, preserve relationships.

If the evidence is unresolved, preserve the gap.

## 3. OBSERVATION IS NOT OWNERSHIP

Observations remain visually and structurally distinct from observed objects.

An event attached to a target must not look like the target structurally contains that event unless the technical model explicitly says so.

## 4. PROVENANCE IS NOT IDENTITY

Source path, line, revision, timestamp, or adapter metadata explains where/how evidence was observed.

It must not visually replace the object's semantic identity.

## 5. THE GRAPH CONNECTS

Relationships are first-class.

Do not hide all non-containment relations inside secondary metadata drawers.

## 6. RESOLUTION, NOT REDUCTION

Deeper inspection should reveal relevant structure, evidence, relations, or representation.

Do not equate “deeper” with merely hiding parent context.

## 7. CONTEXT BEFORE ACTION

Before a user interprets, mutates, exports, or acts on a result, expose enough context to understand:
- current object;
- scope;
- evidence;
- relation;
- status;
- provenance where relevant.

## 8. NO FALSE CERTAINTY

Where the underlying model supports the distinction, communicate:
- observed;
- derived;
- projected;
- unknown;
- unverified.

Never invent confidence percentages.

## 9. CONTEXT IS COLLAPSIBLE, NOT DISPOSABLE

Context may move off-canvas or collapse at smaller resolutions, but the user must be able to recover it.

## 10. THE INTERFACE MUST STOP WHERE THE EVIDENCE STOPS

Unknown is a valid product state.

Missing evidence is not an invitation to auto-fill semantics.

## Product test

Every UX decision should answer:

**Does this help the user inspect what exists, trace why it exists, and resolve only what the evidence supports?**
