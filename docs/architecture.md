# Engine Architecture

Universal Ontology Engine is the executable reference implementation for Universal Ontology v1.0.

## Invariants

1. `type` is exactly one of the 49 canonical ontology levels.
2. `kind` is a specialization owned by exactly one `type`.
3. Node identity is stable and independent from filesystem paths.
4. `contains` is distinct from `projects_to` and `represented_as`.
5. Missing physical layers are reported as unmaterialized rather than treated as invalid ontology.
6. Representation readers are bounded and streaming-capable.

## Resolution model

```text
STRUCTURAL CONTAINMENT
Universe → ... → Repository → Source → Unit → Module → Component → Element

SEMANTIC / DYNAMIC PROJECTION
Element → Function / Behavior / State / Event / Execution

REPRESENTATION
Execution → Instruction → Expression → Value → Data → Token → Character → Bit
```

## Bit boundary

`BIT` is a representation endpoint. The engine must not imply that a bit is a semantic child of a source-level construct. A projection/representation edge carries that relationship.

Future binary readers must support bounded ranges, malformed-input detection, encoding/endianness metadata, and cancellation without requiring whole-file materialization.
