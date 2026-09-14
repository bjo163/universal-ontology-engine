use crate::SemanticTarget;
use ontology_core::{EdgeKind, NodeId};
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum UnresolvedCode {
    Unsupported,
    Insufficient,
    Ambiguous,
    Conflicting,
    InvalidEvidence,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct ProjectionCandidate {
    pub source_id: NodeId,
    pub rule_id: String,
    pub rule_version: String,
    pub target: SemanticTarget,
    pub edge_kind: EdgeKind,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UnresolvedResult {
    pub source_id: NodeId,
    pub codes: BTreeSet<UnresolvedCode>,
    pub candidate_targets: BTreeSet<SemanticTarget>,
    pub rule_ids: BTreeSet<String>,
}

impl UnresolvedResult {
    pub fn single(source_id: NodeId, code: UnresolvedCode, rule_id: impl Into<String>) -> Self {
        Self {
            source_id,
            codes: BTreeSet::from([code]),
            candidate_targets: BTreeSet::new(),
            rule_ids: BTreeSet::from([rule_id.into()]),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum RuleEvaluation {
    Candidate(ProjectionCandidate),
    Unresolved(UnresolvedResult),
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct ConflictPair {
    pub left_rule_id: String,
    pub right_rule_id: String,
}

impl ConflictPair {
    pub fn new(a: impl Into<String>, b: impl Into<String>) -> Self {
        let (a, b) = (a.into(), b.into());
        if a <= b {
            Self {
                left_rule_id: a,
                right_rule_id: b,
            }
        } else {
            Self {
                left_rule_id: b,
                right_rule_id: a,
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ResolutionOutcome {
    Accepted {
        source_id: NodeId,
        target: SemanticTarget,
        supporting_candidates: Vec<ProjectionCandidate>,
    },
    Unresolved(UnresolvedResult),
}

pub fn resolve_evaluations(
    source_id: NodeId,
    evaluations: impl IntoIterator<Item = RuleEvaluation>,
    conflicts: &[ConflictPair],
) -> ResolutionOutcome {
    let mut candidates = Vec::new();
    let mut unresolved = UnresolvedResult {
        source_id: source_id.clone(),
        codes: BTreeSet::new(),
        candidate_targets: BTreeSet::new(),
        rule_ids: BTreeSet::new(),
    };

    for evaluation in evaluations {
        match evaluation {
            RuleEvaluation::Candidate(candidate) => candidates.push(candidate),
            RuleEvaluation::Unresolved(result) => {
                unresolved.codes.extend(result.codes);
                unresolved.rule_ids.extend(result.rule_ids);
                unresolved
                    .candidate_targets
                    .extend(result.candidate_targets);
            }
        }
    }

    candidates.sort();
    candidates.dedup();

    if candidates
        .iter()
        .any(|candidate| candidate.edge_kind != EdgeKind::ProjectsTo)
    {
        unresolved.codes.insert(UnresolvedCode::InvalidEvidence);
        add_context(&mut unresolved, &candidates);
        return ResolutionOutcome::Unresolved(unresolved);
    }

    let active_rules: BTreeSet<_> = candidates
        .iter()
        .map(|candidate| candidate.rule_id.clone())
        .collect();
    if conflicts.iter().any(|pair| {
        active_rules.contains(&pair.left_rule_id) && active_rules.contains(&pair.right_rule_id)
    }) {
        unresolved.codes.insert(UnresolvedCode::Conflicting);
        add_context(&mut unresolved, &candidates);
        return ResolutionOutcome::Unresolved(unresolved);
    }

    let targets: BTreeSet<_> = candidates
        .iter()
        .map(|candidate| candidate.target.clone())
        .collect();
    if targets.len() > 1 {
        unresolved.codes.insert(UnresolvedCode::Ambiguous);
        add_context(&mut unresolved, &candidates);
        return ResolutionOutcome::Unresolved(unresolved);
    }

    if let Some(target) = targets.into_iter().next() {
        return ResolutionOutcome::Accepted {
            source_id,
            target,
            supporting_candidates: candidates,
        };
    }

    if unresolved.codes.is_empty() {
        unresolved.codes.insert(UnresolvedCode::Unsupported);
    }
    ResolutionOutcome::Unresolved(unresolved)
}

fn add_context(result: &mut UnresolvedResult, candidates: &[ProjectionCandidate]) {
    for candidate in candidates {
        result.candidate_targets.insert(candidate.target.clone());
        result.rule_ids.insert(candidate.rule_id.clone());
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ontology_core::OntologyType;

    fn candidate(rule: &str, target: OntologyType) -> RuleEvaluation {
        RuleEvaluation::Candidate(ProjectionCandidate {
            source_id: NodeId::new("source"),
            rule_id: rule.to_owned(),
            rule_version: "1".to_owned(),
            target: SemanticTarget::new(target, None::<String>),
            edge_kind: EdgeKind::ProjectsTo,
        })
    }

    #[test]
    fn rule_order_never_decides_truth() {
        let a = candidate("b", OntologyType::Function);
        let b = candidate("a", OntologyType::Function);
        assert_eq!(
            resolve_evaluations(NodeId::new("source"), [a.clone(), b.clone()], &[]),
            resolve_evaluations(NodeId::new("source"), [b, a], &[])
        );
    }

    #[test]
    fn competing_targets_are_ambiguous() {
        let ResolutionOutcome::Unresolved(result) = resolve_evaluations(
            NodeId::new("source"),
            [
                candidate("a", OntologyType::Function),
                candidate("b", OntologyType::Entity),
            ],
            &[],
        ) else {
            panic!("expected unresolved")
        };
        assert!(result.codes.contains(&UnresolvedCode::Ambiguous));
    }

    #[test]
    fn explicit_conflict_is_preserved() {
        let ResolutionOutcome::Unresolved(result) = resolve_evaluations(
            NodeId::new("source"),
            [
                candidate("a", OntologyType::Function),
                candidate("b", OntologyType::Function),
            ],
            &[ConflictPair::new("a", "b")],
        ) else {
            panic!("expected unresolved")
        };
        assert!(result.codes.contains(&UnresolvedCode::Conflicting));
    }
}
