# OX-DX Naming Anti-Patterns

Naming should reduce ambiguity, not manufacture an ecosystem.

## 1. Repository accidentally becomes the brand

**BAD**

> “Universal Ontology Engine” is now the only public brand because that is the repository name.

**BETTER**

> OX-DX is the public brand. `universal-ontology-engine` remains the current technical repository.

## 2. Same name for every layer

**BAD**

> OX-DX = brand = engine = crate = API = schema.

**BETTER**

> OX-DX → Universal Ontology & Experience Engine → Universal Ontology Engine → `ontology-graph` / normative specification files.

## 3. Duplicate product ownership

**BAD**

> OX-DX Explorer and OX-DX Console both own the same graph-inspection experience with no boundary.

**BETTER**

> One interface owns graph inspection, or each interface has a documented distinct responsibility.

## 4. Brand prefix everywhere

**BAD**

- `ox-dx-core`
- `ox-dx-graph`
- `ox-dx-registry`
- `ox-dx-rust`

solely for cosmetic consistency.

**BETTER**

Keep precise internal crates such as `ontology-core` and `ontology-graph`.

## 5. Religious authority in technical naming

**BAD**

> Invent a sacred/Arabic name for an algorithm to imply deeper authority.

**BETTER**

> Use a neutral technical name that states the component's responsibility. Keep Quranic inspiration in philosophy, worldview, reflection, and visual language.

## 6. Excessive AI naming

**BAD**

> `ox-dx-ai-core` when the component is actually a graph library.

**BETTER**

> `ontology-graph`.

## 7. Hype suffix stack

**BAD**

- Ultra
- Pro
- Quantum
- NextGen
- 360
- X

without a specific product distinction.

**BETTER**

Use a clear noun tied to responsibility.

## 8. Cute foundational infrastructure names

**BAD**

> A whimsical codename becomes the permanent ontology schema name.

**BETTER**

> Foundational contracts use precise descriptive names.

Codenames, if ever used, are temporary project-management labels, not normative identifiers.

## 9. Renaming stable implementation names unnecessarily

**BAD**

> Rename `ontology-graph` to `ox-dx-graph` only because the public brand changed.

**BETTER**

> Preserve technical history unless migration creates concrete user value.

## 10. Future name presented as existing

**BAD**

> “Install OX-DX SDK” before an SDK exists.

**BETTER**

> “A future SDK may use the OX-DX namespace if a supported SDK contract is established.”

## 11. Product name creates technical semantics

**BAD**

> Because the UI calls something a “realm,” the ontology must classify it as TYPE=REALM.

**BETTER**

> Interface language cannot silently redefine the normative ontology.

## 12. Version names become brand names

**BAD**

> OX-DX v1 because the ontology is v1.0.0.

**BETTER**

> Universal Ontology v1.0.0 and Universal Ontology Engine release versions remain separate identities.

## Decision rule

If a new name adds more explanation than clarity, do not add it.
