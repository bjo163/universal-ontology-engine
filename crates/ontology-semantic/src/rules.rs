use crate::resolution::{ProjectionCandidate, RuleEvaluation, UnresolvedCode, UnresolvedResult};
use crate::SemanticTarget;
use ontology_core::{EdgeKind, NodeId, OntologyType};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum EvidenceKind {
    NativeNode,
    NativeSyntax,
    GraphRelation,
    Provenance,
    Normalized,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProjectionEvidence {
    pub source_id: NodeId,
    pub source_type: OntologyType,
    pub evidence_kinds: BTreeSet<EvidenceKind>,
    pub fields: BTreeMap<String, String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProjectionRule {
    pub rule_id: String,
    pub rule_version: String,
    pub required_evidence: BTreeSet<EvidenceKind>,
    pub optional_evidence: BTreeSet<EvidenceKind>,
    pub required_fields: BTreeSet<String>,
    pub optional_fields: BTreeSet<String>,
    pub target: SemanticTarget,
    pub edge_kind: EdgeKind,
    pub missing_required_outcome: UnresolvedCode,
}

[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum RuleContractError {
    #[error("rule id must not be empty")]
    EmptyRuleId,
    #[error("rule version must not be empty")]
    EmptyRuleVersion,
    #[error("projection rules must emit PROJECTS_TO")]
    InvalidEdgeKind,
    #[error("required and optional evidence overlap")]
    EvidenceOverlap,
    #[error("required and optional fields overlap")]
    FieldOverlap,
    #[error("field names must not be empty")]
    EmptyFieldName,
    #[error("missing-required outcome must be insufficient or invalid_evidence")]
    InvalidMissingOutcome,
}

impl ProjectionRule {
    pub fn validate(&self) -> Result<(), RuleContractError> {
        if self.rule_id.trim().is_empty() { return Err(RuleContractError::EmptyRuleId); }
        if self.rule_version.trim().is_empty() { return Err(RuleContractError::EmptyRuleVersion); }
        if self.edge_kind != EdgeKind::ProjectsTo { return Err(RuleContractError::InvalidEdgeKind); }
        if !self.required_evidence.is_disjoint(&self.optional_evidence) { return Err(RuleContractError::EvidenceOverlap); }
        if !self.required_fields.is_disjoint(&self.optional_fields) { return Err(RuleContractError::FieldOverlap); }
        if self.required_fields.iter().chain(self.optional_fields.iter()).any(|field| field.trim().is_empty()) { return Err(RuleContractError::EmptyFieldName); }
        if !matches!(self.missing_required_outcome, UnresolvedCode::Insufficient | UnresolvedCode::InvalidEvidence) { return Err(RuleContractError::InvalidMissingOutcome); }
        Ok(())
    }

    pub fn stable_key(&self) -> (&str, &str, OntologyType, Option<&str>) {
        (&self.rule_id, &self.rule_version, self.target.ontology_type, self.target.kind.as_deref())
    }

    pub fn evaluate(&self, evidence: &ProjectionEvidence) -> Result<RuleEvaluation, RuleContractError> {
        self.validate()?
        let accepted: BTreeSet<_> = self.required_evidence.union(&self.optional_evidence).copied().collect();
        if evidence.evidence_kinds.is_disjoint(&accepted) {
            return Ok(RuleEvaluation::Unresolved(UnresolvedResult::single(evidence.source_id.clone(), UnresolvedCode::Unsupported, self.rule_id.clone())));
        }
        if !self.required_evidence.is_subset(&evidence.evidence_kinds)
            || self.required_fields.iter().any(|field| !evidence.fields.contains_key(field))
        {
            return Ok(RuleEvaluation::Unresolved(UnresolvedResult::single(evidence.source_id.clone(), self.missing_required_outcome, self.rule_id.clone())));
        }
        Ok(RuleEvaluation::Candidate(ProjectionCandidate {
            source_id: evidence.source_id.clone(),
            rule_id: self.rule_id.clone(),
            rule_version: self.rule_version.clone(),
            target: self.target.clone(),
            edge_kind: self.edge_kind,
        }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn rule() -> ProjectionRule {
        ProjectionRule {
            rule_id: "semantic.function.v1".into(),
            rule_version: "1".into(),
            required_evidence: BTreeSet::from([EvidenceKind::NativeSyntax]),
            optional_evidence: BTreeSet::from([EvidenceKind::Provenance]),
            required_fields: BTreeSet::from(["native_kind".into()]),
            optional_fields: BTreeSet::new(),
            target: SemanticTarget::new(OntologyType::Function, None::<String>),
            edge_kind: EdgeKind::ProjectsTo,
            missing_required_outcome: UnresolvedCode::Insufficient,
        }
    }

    #[test]
    fn same_rule_and_evidence_are_deterministic() {
        let evidence = ProjectionEvidence {
            source_id: NodeId::new("source"),
            source_type: OntologyType::Element,
            evidence_kinds: BTreeSet::from([EvidenceKind::NativeSyntax]),
            fields: BTreeMap::from([("native_kind".into(), "function".into())]),
        };
        assert_eq!(rule().evaluate(&evidence).unwrap(), rule().evaluate(&evidence).unwrap());
    }

    #[test]
    fn missing_required_evidence_never_invents_projection() {
        let evidence = ProjectionEvidence {
            source_id: NodeId::new("source"),
            source_type: OntologyType::Element,
            evidence_kinds: BTreeSet::from([EvidenceKind::NativeSyntax]),
            fields: BTreeMap::new(),
        };
        let RuleEvaluation::Unresolved(result) = rule().evaluate(&evidence).unwrap() else { panic!("expected unresolved") };
        assert!(result.codes.contains(&UnresolvedCode::Insufficient));
    }
}
