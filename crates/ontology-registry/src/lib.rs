use ontology_core::{EdgeKind, OntologyType, LEVEL_COUNT, ONTOLOGY_VERSION};
use serde::Deserialize;
use std::{
    collections::{HashMap, HashSet},
    fs,
    path::Path,
};

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
    #[error("ontology title `{0}` is not canonical")]
    TitleMismatch(String),
    #[error("invalid ontology shape: zones={zones}, levels_per_zone={levels}, canonical_levels={canonical}")]
    Shape {
        zones: usize,
        levels: usize,
        canonical: usize,
    },
    #[error("expected {expected} levels but registry contains {actual}")]
    LevelCount { expected: usize, actual: usize },
    #[error("zone {zone} must contain exactly 7 levels, found {actual}")]
    ZoneLevelCount { zone: String, actual: usize },
    #[error("zone id `{zone}` does not match canonical zone `{expected}`")]
    ZoneMismatch { zone: String, expected: String },
    #[error("duplicate canonical level index: {0}")]
    DuplicateIndex(u8),
    #[error("non-contiguous canonical level index: expected {expected}, found {found}")]
    NonContiguousIndex { expected: u8, found: u8 },
    #[error("level {index} is assigned to wrong zone `{zone}`")]
    LevelInWrongZone { index: u8, zone: String },
    #[error("unknown ontology type `{0}`")]
    UnknownType(String),
    #[error("registry type mismatch at level {level}: contract=`{contract}`, engine=`{engine}`")]
    TypeMismatch {
        level: u8,
        contract: String,
        engine: String,
    },
    #[error("duplicate ontology type `{0}`")]
    DuplicateType(String),
    #[error("kind `{kind}` is assigned to multiple types: `{first}` and `{second}`")]
    KindConflict {
        kind: String,
        first: String,
        second: String,
    },
    #[error("edge class `{0}` is unknown to the engine")]
    UnknownEdgeKind(String),
    #[error("edge class registry is missing `{0}`")]
    MissingEdgeKind(String),
    #[error("registry rule `{0}` must be true")]
    RuleViolation(&'static str),
}

#[derive(Debug, Clone)]
pub struct OntologyRegistry {
    document: OntologyDocument,
    by_type: HashMap<OntologyType, LevelDefinition>,
    by_index: Vec<OntologyType>,
    kind_owners: HashMap<String, OntologyType>,
    edge_kinds: HashSet<EdgeKind>,
}

impl OntologyRegistry {
    pub fn load(path: impl AsRef<Path>) -> Result<Self, RegistryError> {
        Self::from_json(&fs::read_to_string(path)?)
    }
    pub fn from_json(text: &str) -> Result<Self, RegistryError> {
        Self::from_document(serde_json::from_str(text)?)
    }

    pub fn from_document(document: OntologyDocument) -> Result<Self, RegistryError> {
        if document.contract_version != ONTOLOGY_VERSION {
            return Err(RegistryError::Version {
                actual: document.contract_version.clone(),
                expected: ONTOLOGY_VERSION.to_string(),
            });
        }
        if document.title != "Universal Ontology v1.0" {
            return Err(RegistryError::TitleMismatch(document.title.clone()));
        }
        if document.shape.zones != 7
            || document.shape.levels_per_zone != 7
            || document.shape.canonical_levels != LEVEL_COUNT
        {
            return Err(RegistryError::Shape {
                zones: document.shape.zones,
                levels: document.shape.levels_per_zone,
                canonical: document.shape.canonical_levels,
            });
        }

        let rules = &document.rules;
        for (enabled, name) in [
            (rules.type_is_canonical_level, "type_is_canonical_level"),
            (rules.kind_is_specialization, "kind_is_specialization"),
            (
                rules.kind_must_not_create_level,
                "kind_must_not_create_level",
            ),
            (
                rules.filesystem_is_not_canonical_hierarchy,
                "filesystem_is_not_canonical_hierarchy",
            ),
            (
                rules.projection_is_distinct_from_containment,
                "projection_is_distinct_from_containment",
            ),
            (
                rules.representation_is_distinct_from_semantics,
                "representation_is_distinct_from_semantics",
            ),
            (rules.stable_identity_required, "stable_identity_required"),
            (
                rules.intermediate_levels_may_be_unmaterialized,
                "intermediate_levels_may_be_unmaterialized",
            ),
        ] {
            if !enabled {
                return Err(RegistryError::RuleViolation(name));
            }
        }

        if document.edge_classes.len() != EdgeKind::ALL.len() {
            return Err(RegistryError::MissingEdgeKind(
                "complete canonical edge class set".into(),
            ));
        }
        let mut edge_kinds = HashSet::new();
        for name in &document.edge_classes {
            edge_kinds.insert(
                EdgeKind::from_slug(name)
                    .ok_or_else(|| RegistryError::UnknownEdgeKind(name.clone()))?,
            );
        }
        for kind in EdgeKind::ALL {
            if !edge_kinds.contains(&kind) {
                return Err(RegistryError::MissingEdgeKind(kind.slug().into()));
            }
        }

        const ZONE_IDS: [&str; 7] = [
            "existence",
            "context",
            "intent",
            "structure",
            "semantic",
            "dynamic",
            "representation",
        ];
        if document.zones.len() != 7 {
            return Err(RegistryError::Shape {
                zones: document.zones.len(),
                levels: document.shape.levels_per_zone,
                canonical: document.shape.canonical_levels,
            });
        }
        for (position, zone) in document.zones.iter().enumerate() {
            if zone.id != ZONE_IDS[position] {
                return Err(RegistryError::ZoneMismatch {
                    zone: zone.id.clone(),
                    expected: ZONE_IDS[position].into(),
                });
            }
            if zone.levels.len() != 7 {
                return Err(RegistryError::ZoneLevelCount {
                    zone: zone.id.clone(),
                    actual: zone.levels.len(),
                });
            }
        }

        let mut seen_indices = HashSet::new();
        let mut seen_types = HashSet::new();
        let mut by_type = HashMap::with_capacity(LEVEL_COUNT);
        let mut by_index = vec![OntologyType::Universe; LEVEL_COUNT];
        for (position, definition) in document
            .zones
            .iter()
            .flat_map(|zone| zone.levels.iter())
            .enumerate()
        {
            if !seen_indices.insert(definition.index) {
                return Err(RegistryError::DuplicateIndex(definition.index));
            }
            let expected_index = (position + 1) as u8;
            if definition.index != expected_index {
                return Err(RegistryError::NonContiguousIndex {
                    expected: expected_index,
                    found: definition.index,
                });
            }
            let expected_zone = (position / 7) + 1;
            let actual_zone = document
                .zones
                .iter()
                .position(|zone| {
                    zone.levels
                        .iter()
                        .any(|level| level.index == definition.index)
                })
                .map(|v| v + 1)
                .unwrap_or_default();
            if actual_zone != expected_zone {
                return Err(RegistryError::LevelInWrongZone {
                    index: definition.index,
                    zone: actual_zone.to_string(),
                });
            }
            if !seen_types.insert(definition.ontology_type.clone()) {
                return Err(RegistryError::DuplicateType(
                    definition.ontology_type.clone(),
                ));
            }
            let engine_type = OntologyType::ALL[position];
            if definition.ontology_type != engine_type.slug() {
                return Err(RegistryError::TypeMismatch {
                    level: definition.index,
                    contract: definition.ontology_type.clone(),
                    engine: engine_type.slug().to_owned(),
                });
            }
            by_type.insert(engine_type, definition.clone());
            by_index[position] = engine_type;
        }

        let mut kind_owners = HashMap::new();
        for example in &document.kind_examples {
            let ty = OntologyType::from_slug(&example.ontology_type)
                .ok_or_else(|| RegistryError::UnknownType(example.ontology_type.clone()))?;
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
        Ok(Self {
            document,
            by_type,
            by_index,
            kind_owners,
            edge_kinds,
        })
    }

    pub fn document(&self) -> &OntologyDocument {
        &self.document
    }
    pub fn len(&self) -> usize {
        self.by_index.len()
    }
    pub fn is_empty(&self) -> bool {
        self.by_index.is_empty()
    }
    pub fn level(&self, one_based: u8) -> Option<OntologyType> {
        self.by_index
            .get(one_based.saturating_sub(1) as usize)
            .copied()
    }
    pub fn definition(&self, ty: OntologyType) -> Option<&LevelDefinition> {
        self.by_type.get(&ty)
    }
    pub fn owner_of_kind(&self, kind: &str) -> Option<OntologyType> {
        self.kind_owners.get(kind).copied()
    }
    pub fn edge_kind(&self, slug: &str) -> Option<EdgeKind> {
        EdgeKind::from_slug(slug).filter(|kind| self.edge_kinds.contains(kind))
    }
    pub fn supports_edge(&self, kind: EdgeKind) -> bool {
        self.edge_kinds.contains(&kind)
    }
    pub fn all(&self) -> &[OntologyType] {
        &self.by_index
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
        assert_eq!(
            registry.definition(OntologyType::Bit).unwrap().definition,
            "Single binary information unit."
        );
    }
    #[test]
    fn edge_registry_is_complete() {
        let registry = OntologyRegistry::from_json(&registry_json()).unwrap();
        assert_eq!(
            registry.edge_kind("projects_to"),
            Some(EdgeKind::ProjectsTo)
        );
        assert!(registry.supports_edge(EdgeKind::Contains));
    }
    #[test]
    fn kind_namespace_is_scoped_to_one_type() {
        let registry = OntologyRegistry::from_json(&registry_json()).unwrap();
        assert_eq!(
            registry.owner_of_kind("rust-crate"),
            Some(OntologyType::Unit)
        );
        assert_eq!(
            registry.owner_of_kind("function"),
            Some(OntologyType::Element)
        );
    }
}
