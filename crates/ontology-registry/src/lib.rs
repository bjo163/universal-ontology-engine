use ontology_core::{OntologyError, OntologyType, LEVEL_COUNT, ONTOLOGY_VERSION};
use serde::Deserialize;
use std::{collections::{HashMap, HashSet}, fs, path::Path};

#[derive(Debug, Clone, Deserialize)]
pub struct OntologyDocument {
    pub title: String,
    pub contract_version: String,
    pub shape: Shape,
    pub rules: Rules,
    pub zones: Vec<Zone>,
    #[serde(default)]
    pub edge_classes: Vec<String>,
    #[serde(default)]
    pub kind_examples: Vec<KindExample>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Shape {
    pub zones: usize,
    pub levels_per_zone: usize,
    pub canonical_levels: usize,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Rules {
    pub type_is_canonical_level: bool,
    pub kind_is_specialization: bool,
    pub kind_must_not_create_level: bool,
    pub filesystem_is_not_canonical_hierarchy: bool,
    pub projection_is_distinct_from_containment: bool,
    pub representation_is_distinct_from_semantics: bool,
    pub stable_identity_required: bool,
    pub intermediate_levels_may_be_unmaterialized: bool,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Zone {
    pub id: String,
    pub name: String,
    pub levels: Vec<LevelDefinition>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct LevelDefinition {
    pub index: u8,
    #[serde(rename = "type")]
    pub ontology_type: String,
    pub definition: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct KindExample {
    #[serde(rename = "type")]
    pub ontology_type: String,
    pub kind: String,
}

#[derive(Debug, thiserror::Error)]
pub enum RegistryError {
    #[error("I/O while reading ontology registry: {0}")]
    Io(#[from] std::io::Error),
    #[error("invalid ontology JSON: {0}")]
    Json(#[from] serde_json::Error),
    #[error("ontology version `{actual}` does not match engine `{expected}`")]
    Version { actual: String, expected: String },
    #[error("invalid ontology shape: zones={zones}, levels_per_zone={levels}, canonical_levels={canonical}")]
    Shape { zones: usize, levels: usize, canonical: usize },
    #[error("expected {expected} levels but registry contains {actual}")]
    LevelCount { expected: usize, actual: usize },
    #[error("duplicate canonical level index: {0}")]
    DuplicateIndex(u8),
    #[error("non-contiguous canonical level index: expected {expected}, found {found}")]
    NonContiguousIndex { expected: u8, found: u8 },
    #[error("unknown ontology type `{0}`")]
    UnknownType(String),
    #[error("registry type mismatch at level {level}: contract=`{contract}`, engine=`{engine}`")]
    TypeMismatch { level: u8, contract: String, engine: String },
    #[error("duplicate ontology type `{0}`")]
    DuplicateType(String),
    #[error("kind `{kind}` is assigned to multiple types: `{first}` and `{second}`")]
    KindConflict { kind: String, first: String, second: String },
    #[error("registry rule `{0}` must be true")]
    RuleViolation(&'static str),
}

#[derive(Debug, Clone)]
pub struct OntologyRegistry {
    document: OntologyDocument,
    by_type: HashMap<OntologyType, LevelDefinition>,
    by_index: Vec<OntologyType>,
    kind_owners: HashMap<String, OntologyType>,
}

impl OntologyRegistry {
    pub fn load(path: impl AsRef<Path>) -> Result<Self, RegistryError> {
        let text = fs::read_to_string(path)?;
        Self::from_json(&text)
    }

    pub fn from_json(text: &str) -> Result<Self, RegistryError> {
        let document: OntologyDocument = serde_json::from_str(text)?;
        Self::from_document(document)
    }

    pub fn from_document(document: OntologyDocument) -> Result<Self, RegistryError> {
        if document.contract_version != ONTOLOGY_VERSION {
            return Err(RegistryError::Version {
                actual: document.contract_version.clone(),
                expected: ONTOLOGY_VERSION,
            });
        }
        if document.shape.zones != 7 || document.shape.levels_per_zone != 7 || document.shape.canonical_levels != LEVEL_COUNT {
            return Err(RegistryError::Shape {
                zones: document.shape.zones,
                levels: document.shape.levels_per_zone,
                canonical: document.shape.canonical_levels,
            });
        }
        if !document.rules.type_is_canonical_level { return Err(RegistryError::RuleViolation("type_is_canonical_level")); }
        if !document.rules.kind_is_specialization { return Err(RegistryError::RuleViolation("kind_is_specialization")); }
        if !document.rules.kind_must_not_create_level { return Err(RegistryError::RuleViolation("kind_must_not_create_level")); }
        if !document.rules.filesystem_is_not_canonical_hierarchy { return Err(RegistryError::RuleViolation("filesystem_is_not_canonical_hierarchy")); }
        if !document.rules.projection_is_distinct_from_containment { return Err(RegistryError::RuleViolation("projection_is_distinct_from_containment")); }
        if !document.rules.representation_is_distinct_from_semantics { return Err(RegistryError::RuleViolation("representation_is_distinct_from_semantics")); }
        if !document.rules.stable_identity_required { return Err(RegistryError::RuleViolation("stable_identity_required")); }

        let levels: Vec<&LevelDefinition> = document.zones.iter().flat_map(|zone| zone.levels.iter()).collect();
        if levels.len() != LEVEL_COUNT {
            return Err(RegistryError::LevelCount { expected: LEVEL_COUNT, actual: levels.len() });
        }

        let mut seen_indices = HashSet::new();
        let mut seen_types = HashSet::new();
        let mut by_type = HashMap::with_capacity(LEVEL_COUNT);
        let mut by_index = vec![OntologyType::Universe; LEVEL_COUNT];

        for (position, definition) in levels.iter().enumerate() {
            if !seen_indices.insert(definition.index) {
                return Err(RegistryError::DuplicateIndex(definition.index));
            }
            let expected_index = (position + 1) as u8;
            if definition.index != expected_index {
                return Err(RegistryError::NonContiguousIndex { expected: expected_index, found: definition.index });
            }
            if !seen_types.insert(definition.ontology_type.clone()) {
                return Err(RegistryError::DuplicateType(definition.ontology_type.clone()));
            }
            let engine_type = OntologyType::ALL[position];
            if definition.ontology_type != engine_type.slug() {
                return Err(RegistryError::TypeMismatch {
                    level: definition.index,
                    contract: definition.ontology_type.clone(),
                    engine: engine_type.slug().to_owned(),
                });
            }
            by_type.insert(engine_type, (*definition).clone());
            by_index[position] = engine_type;
        }

        let mut kind_owners = HashMap::new();
        for example in &document.kind_examples {
            let ty = parse_type(&example.ontology_type)?;
            if let Some(existing) = kind_owners.insert(example.kind.clone(), ty) {
                if existing != ty {
                    return Err(RegistryError::KindConflict {
                        kind: example.kind.clone(),
                        first: existing.slug().to_owned(),
                        second: ty.slug().to_owned(),
                    });
                }
            }
        }

        Ok(Self { document, by_type, by_index, kind_owners })
    }

    pub fn document(&self) -> &OntologyDocument { &self.document }
    pub fn len(&self) -> usize { self.by_index.len() }
    pub fn is_empty(&self) -> bool { self.by_index.is_empty() }
    pub fn level(&self, one_based: u8) -> Option<OntologyType> { self.by_index.get(one_based.saturating_sub(1) as usize).copied() }
    pub fn definition(&self, ty: OntologyType) -> Option<&LevelDefinition> { self.by_type.get(&ty) }
    pub fn owner_of_kind(&self, kind: &str) -> Option<OntologyType> { self.kind_owners.get(kind).copied() }
    pub fn all(&self) -> &[OntologyType] { &self.by_index }
}

fn parse_type(value: &str) -> Result<OntologyType, RegistryError> {
    OntologyType::ALL
        .into_iter()
        .find(|ty| ty.slug() == value)
        .ok_or_else(|| RegistryError::UnknownType(value.to_owned()))
}

impl From<OntologyError> for RegistryError {
    fn from(value: OntologyError) -> Self {
        RegistryError::UnknownType(value.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn registry_json() -> String {
        include_str!("../../../specifications/universal-ontology-v1.0.json").to_owned()
    }

    #[test]
    fn loads_canonical_registry() {
        let registry = OntologyRegistry::from_json(&registry_json()).unwrap();
        assert_eq!(registry.len(), 49);
        assert_eq!(registry.level(1), Some(OntologyType::Universe));
        assert_eq!(registry.level(49), Some(OntologyType::Bit));
    }

    #[test]
    fn registry_is_source_of_truth_for_definitions() {
        let registry = OntologyRegistry::from_json(&registry_json()).unwrap();
        assert_eq!(registry.definition(OntologyType::Bit).unwrap().definition, "Single binary information unit.");
    }

    #[test]
    fn kind_namespace_is_scoped_to_one_type() {
        let registry = OntologyRegistry::from_json(&registry_json()).unwrap();
        assert_eq!(registry.owner_of_kind("rust-crate"), Some(OntologyType::Unit));
        assert_eq!(registry.owner_of_kind("function"), Some(OntologyType::Element));
    }
}
