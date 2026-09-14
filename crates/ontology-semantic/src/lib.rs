use ontology_core::OntologyType;
use serde::{Deserialize, Serialize};

pub mod normalization;
pub mod resolution;
pub mod rules;

pub const SEMANTIC_PROJECTION_RFC_ID: &str = "oxdx.semantic-projection.rfc/v1";

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct SemanticTarget {
    pub ontology_type: OntologyType,
    pub kind: Option<String>,
}

impl SemanticTarget {
    pub fn new(ontology_type: OntologyType, kind: Option<impl Into<String>>) -> Self {
        Self {
            ontology_type,
            kind: kind.map(Into::into),
        }
    }
}
