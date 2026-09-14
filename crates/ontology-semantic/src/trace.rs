use crate::{
    normalization::NormalizedEvidence,
    resolution::{ProjectionCandidate, ResolutionOutcome, UnresolvedCode},
    SemanticTarget, SEMANTIC_PROJECTION_RFC_ID,
};
use ontology_core::{EdgeKind, NodeId, SourceSpan};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};

pub const PROJECTION_TRACE_SCHEMA: &str = "oxdx.semantic-projection.trace/v1";

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct RuleRef {
    pub rule_id: String,
    pub rule_version: String,
}

impl RuleRef {
    pub fn new(rule_id: impl Into<String>, rule_version: impl Into<String>) -> Self {
        Self {
            rule_id: rule_id.into(),
            rule_version: rule_version.into(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProjectionOrigin {
    pub repository: Option<String>,
    pub source: Option<String>,
    pub span: Option<SourceSpan>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NativeEvidenceTrace {
    pub language: Option<String>,
    pub native_kind: Option<String>,
    pub metadata: BTreeMap<String, String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TraceOutcome {
    Projected {
        target: SemanticTarget,
        edge_kind: EdgeKind,
    },
    Unresolved {
        codes: BTreeSet<UnresolvedCode>,
        candidate_targets: BTreeSet<SemanticTarget>,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProjectionTrace {
    pub schema: String,
    pub projection_rfc: String,
    pub source_id: NodeId,
    pub origin: Option<ProjectionOrigin>,
    pub native: NativeEvidenceTrace,
    pub evaluated_rules: BTreeSet<RuleRef>,
    pub normalized_evidence: Vec<NormalizedEvidence>,
    pub outcome: TraceOutcome,
}

#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum TraceContractError {
    #[error("resolution source does not match trace source")]
    SourceMismatch,
    #[error("accepted projection candidate is inconsistent with resolution outcome")]
    InvalidAcceptedCandidate,
    #[error("missing version for unresolved rule `{0}`")]
    MissingRuleVersion(String),
    #[error("multiple versions supplied for rule `{0}`")]
    RuleVersionConflict(String),
}

impl ProjectionTrace {
    pub fn from_resolution(
        source_id: NodeId,
        origin: Option<ProjectionOrigin>,
        native: NativeEvidenceTrace,
        mut normalized_evidence: Vec<NormalizedEvidence>,
        evaluated_rules: impl IntoIterator<Item = RuleRef>,
        resolution: ResolutionOutcome,
    ) -> Result<Self, TraceContractError> {
        normalized_evidence.sort();
        normalized_evidence.dedup();

        let mut rules: BTreeSet<RuleRef> = evaluated_rules.into_iter().collect();
        ensure_single_version_per_rule(&rules)?;

        let outcome = match resolution {
            ResolutionOutcome::Accepted {
                source_id: resolved_source,
                target,
                supporting_candidates,
            } => {
                if resolved_source != source_id {
                    return Err(TraceContractError::SourceMismatch);
                }
                for candidate in &supporting_candidates {
                    validate_accepted_candidate(&source_id, &target, candidate)?;
                    rules.insert(RuleRef::new(
                        candidate.rule_id.clone(),
                        candidate.rule_version.clone(),
                    ));
                }
                ensure_single_version_per_rule(&rules)?;
                TraceOutcome::Projected {
                    target,
                    edge_kind: EdgeKind::ProjectsTo,
                }
            }
            ResolutionOutcome::Unresolved(result) => {
                if result.source_id != source_id {
                    return Err(TraceContractError::SourceMismatch);
                }
                let versions = index_rule_versions(&rules)?;
                for rule_id in &result.rule_ids {
                    if !versions.contains_key(rule_id) {
                        return Err(TraceContractError::MissingRuleVersion(rule_id.clone()));
                    }
                }
                TraceOutcome::Unresolved {
                    codes: result.codes,
                    candidate_targets: result.candidate_targets,
                }
            }
        };

        Ok(Self {
            schema: PROJECTION_TRACE_SCHEMA.to_owned(),
            projection_rfc: SEMANTIC_PROJECTION_RFC_ID.to_owned(),
            source_id,
            origin,
            native,
            evaluated_rules: rules,
            normalized_evidence,
            outcome,
        })
    }
}

fn validate_accepted_candidate(
    source_id: &NodeId,
    target: &SemanticTarget,
    candidate: &ProjectionCandidate,
) -> Result<(), TraceContractError> {
    if &candidate.source_id != source_id
        || &candidate.target != target
        || candidate.edge_kind != EdgeKind::ProjectsTo
    {
        return Err(TraceContractError::InvalidAcceptedCandidate);
    }
    Ok(())
}

fn ensure_single_version_per_rule(rules: &BTreeSet<RuleRef>) -> Result<(), TraceContractError> {
    index_rule_versions(rules).map(|_| ())
}

fn index_rule_versions(
    rules: &BTreeSet<RuleRef>,
) -> Result<BTreeMap<String, String>, TraceContractError> {
    let mut versions = BTreeMap::new();
    for rule in rules {
        match versions.get(&rule.rule_id) {
            Some(existing) if existing != &rule.rule_version => {
                return Err(TraceContractError::RuleVersionConflict(
                    rule.rule_id.clone(),
                ));
            }
            Some(_) => {}
            None => {
                versions.insert(rule.rule_id.clone(), rule.rule_version.clone());
            }
        }
    }
    Ok(versions)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::resolution::{ProjectionCandidate, ResolutionOutcome, UnresolvedResult};
    use ontology_core::OntologyType;

    fn native() -> NativeEvidenceTrace {
        NativeEvidenceTrace {
            language: Some("rust".into()),
            native_kind: Some("function".into()),
            metadata: BTreeMap::from([("visibility".into(), "public".into())]),
        }
    }

    fn accepted() -> ResolutionOutcome {
        ResolutionOutcome::Accepted {
            source_id: NodeId::new("source:fn"),
            target: SemanticTarget::new(OntologyType::Function, None::<String>),
            supporting_candidates: vec![ProjectionCandidate {
                source_id: NodeId::new("source:fn"),
                rule_id: "semantic.function.v1".into(),
                rule_version: "1".into(),
                target: SemanticTarget::new(OntologyType::Function, None::<String>),
                edge_kind: EdgeKind::ProjectsTo,
            }],
        }
    }

    #[test]
    fn identical_inputs_serialize_byte_identically() {
        let first = ProjectionTrace::from_resolution(
            NodeId::new("source:fn"),
            None,
            native(),
            Vec::new(),
            Vec::<RuleRef>::new(),
            accepted(),
        )
        .unwrap();
        let second = ProjectionTrace::from_resolution(
            NodeId::new("source:fn"),
            None,
            native(),
            Vec::new(),
            Vec::<RuleRef>::new(),
            accepted(),
        )
        .unwrap();

        assert_eq!(
            serde_json::to_vec(&first).unwrap(),
            serde_json::to_vec(&second).unwrap()
        );
    }

    #[test]
    fn unresolved_reason_and_rule_version_are_preserved() {
        let unresolved = ResolutionOutcome::Unresolved(UnresolvedResult {
            source_id: NodeId::new("source:unknown"),
            codes: BTreeSet::from([UnresolvedCode::Ambiguous]),
            candidate_targets: BTreeSet::from([
                SemanticTarget::new(OntologyType::Function, None::<String>),
                SemanticTarget::new(OntologyType::Entity, None::<String>),
            ]),
            rule_ids: BTreeSet::from(["semantic.ambiguous.v1".into()]),
        });
        let trace = ProjectionTrace::from_resolution(
            NodeId::new("source:unknown"),
            None,
            native(),
            Vec::new(),
            [RuleRef::new("semantic.ambiguous.v1", "1")],
            unresolved,
        )
        .unwrap();

        assert!(trace
            .evaluated_rules
            .contains(&RuleRef::new("semantic.ambiguous.v1", "1")));
        let TraceOutcome::Unresolved { codes, .. } = trace.outcome else {
            panic!("expected unresolved trace")
        };
        assert!(codes.contains(&UnresolvedCode::Ambiguous));
    }

    #[test]
    fn unresolved_rule_without_version_is_rejected() {
        let unresolved = ResolutionOutcome::Unresolved(UnresolvedResult {
            source_id: NodeId::new("source:unknown"),
            codes: BTreeSet::from([UnresolvedCode::Insufficient]),
            candidate_targets: BTreeSet::new(),
            rule_ids: BTreeSet::from(["semantic.missing.v1".into()]),
        });
        let error = ProjectionTrace::from_resolution(
            NodeId::new("source:unknown"),
            None,
            native(),
            Vec::new(),
            Vec::<RuleRef>::new(),
            unresolved,
        )
        .unwrap_err();

        assert_eq!(
            error,
            TraceContractError::MissingRuleVersion("semantic.missing.v1".into())
        );
    }
}
