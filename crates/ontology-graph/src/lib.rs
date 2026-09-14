use ontology_core::{Edge, EdgeKind, Node, NodeId, OntologyError, OntologyType};
use ontology_registry::OntologyRegistry;
use serde::{Deserialize, Serialize};
use std::{
    collections::{BTreeMap, BTreeSet},
    sync::Arc,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum EdgeDomain {
    Structural,
    SemanticProjection,
    RuntimeObservation,
    Representation,
    GeneralSemantic,
}

pub const fn edge_domain(kind: EdgeKind) -> EdgeDomain {
    match kind {
        EdgeKind::Contains => EdgeDomain::Structural,
        EdgeKind::ProjectsTo => EdgeDomain::SemanticProjection,
        EdgeKind::ObservedAt => EdgeDomain::RuntimeObservation,
        EdgeKind::RepresentedAs => EdgeDomain::Representation,
        _ => EdgeDomain::GeneralSemantic,
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct ProjectionMaterializationKey {
    pub source_id: NodeId,
    pub target_id: NodeId,
    pub rule_id: String,
    pub rule_version: String,
}

impl ProjectionMaterializationKey {
    pub fn new(
        source_id: NodeId,
        target_id: NodeId,
        rule_id: impl Into<String>,
        rule_version: impl Into<String>,
    ) -> Self {
        Self {
            source_id,
            target_id,
            rule_id: rule_id.into(),
            rule_version: rule_version.into(),
        }
    }
}

#[derive(
    Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, thiserror::Error,
)]
pub enum GraphInvariantViolation {
    #[error("structural containment does not match target parent: {from:?} -> {to:?}")]
    StructuralContainmentMismatch { from: NodeId, to: NodeId },
    #[error("{domain:?} edge must not make its target a structural child: {from:?} -> {to:?}")]
    NonStructuralOwnership {
        domain: EdgeDomain,
        from: NodeId,
        to: NodeId,
    },
    #[error("projection cycle is not allowed: {from:?} -> {to:?}")]
    ProjectionCycle { from: NodeId, to: NodeId },
    #[error("projection references unknown node: {0:?}")]
    UnknownProjectionNode(NodeId),
    #[error("projection source and target must differ: {0:?}")]
    ProjectionSelfEdge(NodeId),
    #[error("PROJECTS_TO is not declared by the ontology registry")]
    ProjectionEdgeUndeclared,
}

#[derive(Debug)]
pub struct OntologyGraph {
    registry: Arc<OntologyRegistry>,
    nodes: BTreeMap<NodeId, Node>,
    edges: BTreeSet<Edge>,
    projection_keys: BTreeSet<ProjectionMaterializationKey>,
}

impl OntologyGraph {
    pub fn new(registry: Arc<OntologyRegistry>) -> Self {
        Self {
            registry,
            nodes: BTreeMap::new(),
            edges: BTreeSet::new(),
            projection_keys: BTreeSet::new(),
        }
    }

    pub fn registry(&self) -> &OntologyRegistry {
        &self.registry
    }

    pub fn insert_node(&mut self, node: Node) -> Result<(), OntologyError> {
        if node.id.as_str().is_empty() {
            return Err(OntologyError::EmptyNodeId);
        }
        if self.nodes.contains_key(&node.id) {
            return Err(OntologyError::DuplicateId(node.id));
        }
        if let Some(parent) = &node.parent {
            let parent_node = self
                .nodes
                .get(parent)
                .ok_or_else(|| OntologyError::UnknownParent(parent.clone()))?;
            if parent_node.ontology_type.level() >= node.ontology_type.level() {
                return Err(OntologyError::InvalidLevelOrdering {
                    parent: parent_node.ontology_type,
                    child: node.ontology_type,
                });
            }
        }
        if let Some(kind) = &node.kind {
            if let Some(owner) = self.registry.owner_of_kind(kind) {
                if owner != node.ontology_type {
                    return Err(OntologyError::KindConflict {
                        kind: kind.clone(),
                        owner,
                    });
                }
            }
        }
        self.nodes.insert(node.id.clone(), node);
        Ok(())
    }

    pub fn add_edge(
        &mut self,
        from: NodeId,
        to: NodeId,
        kind: EdgeKind,
    ) -> Result<(), OntologyError> {
        if !self.nodes.contains_key(&from) {
            return Err(OntologyError::UnknownNode(from));
        }
        if !self.nodes.contains_key(&to) {
            return Err(OntologyError::UnknownNode(to));
        }
        if from == to {
            return Err(OntologyError::SelfEdge(from));
        }
        if !self.registry.supports_edge(kind) {
            return Err(OntologyError::UndeclaredEdgeKind(kind.slug().into()));
        }
        if kind == EdgeKind::Contains {
            let child = self.nodes.get(&to).expect("checked above");
            if child.parent.as_ref() != Some(&from) {
                return Err(OntologyError::InvalidContainment { from, to });
            }
        }
        let edge = Edge {
            from: from.clone(),
            to: to.clone(),
            kind,
        };
        if !self.edges.insert(edge) {
            return Err(OntologyError::DuplicateEdge { from, to, kind });
        }
        Ok(())
    }

    pub fn add_projection(
        &mut self,
        key: ProjectionMaterializationKey,
    ) -> Result<bool, GraphInvariantViolation> {
        if !self.nodes.contains_key(&key.source_id) {
            return Err(GraphInvariantViolation::UnknownProjectionNode(
                key.source_id.clone(),
            ));
        }
        if !self.nodes.contains_key(&key.target_id) {
            return Err(GraphInvariantViolation::UnknownProjectionNode(
                key.target_id.clone(),
            ));
        }
        if key.source_id == key.target_id {
            return Err(GraphInvariantViolation::ProjectionSelfEdge(
                key.source_id.clone(),
            ));
        }
        if !self.registry.supports_edge(EdgeKind::ProjectsTo) {
            return Err(GraphInvariantViolation::ProjectionEdgeUndeclared);
        }
        self.validate_non_structural_ownership(
            &key.source_id,
            &key.target_id,
            EdgeDomain::SemanticProjection,
        )?;
        if self.projection_path_exists(&key.target_id, &key.source_id) {
            return Err(GraphInvariantViolation::ProjectionCycle {
                from: key.source_id.clone(),
                to: key.target_id.clone(),
            });
        }
        if !self.projection_keys.insert(key.clone()) {
            return Ok(false);
        }
        self.edges.insert(Edge {
            from: key.source_id,
            to: key.target_id,
            kind: EdgeKind::ProjectsTo,
        });
        Ok(true)
    }

    pub fn projection_keys(&self) -> impl Iterator<Item = &ProjectionMaterializationKey> {
        self.projection_keys.iter()
    }

    pub fn validate_phase7_invariants(&self) -> Vec<GraphInvariantViolation> {
        let mut violations = BTreeSet::new();
        for edge in &self.edges {
            match edge_domain(edge.kind) {
                EdgeDomain::Structural => {
                    let child = self.nodes.get(&edge.to);
                    if child.and_then(|node| node.parent.as_ref()) != Some(&edge.from) {
                        violations.insert(GraphInvariantViolation::StructuralContainmentMismatch {
                            from: edge.from.clone(),
                            to: edge.to.clone(),
                        });
                    }
                }
                EdgeDomain::SemanticProjection => {
                    if let Err(violation) = self.validate_non_structural_ownership(
                        &edge.from,
                        &edge.to,
                        EdgeDomain::SemanticProjection,
                    ) {
                        violations.insert(violation);
                    }
                    if self.projection_path_exists_excluding(&edge.to, &edge.from, Some(edge)) {
                        violations.insert(GraphInvariantViolation::ProjectionCycle {
                            from: edge.from.clone(),
                            to: edge.to.clone(),
                        });
                    }
                }
                EdgeDomain::RuntimeObservation | EdgeDomain::Representation => {
                    if let Err(violation) = self.validate_non_structural_ownership(
                        &edge.from,
                        &edge.to,
                        edge_domain(edge.kind),
                    ) {
                        violations.insert(violation);
                    }
                }
                EdgeDomain::GeneralSemantic => {}
            }
        }
        violations.into_iter().collect()
    }

    fn validate_non_structural_ownership(
        &self,
        from: &NodeId,
        to: &NodeId,
        domain: EdgeDomain,
    ) -> Result<(), GraphInvariantViolation> {
        let parent_matches = self.nodes.get(to).and_then(|node| node.parent.as_ref()) == Some(from);
        let contains_same_pair = self.edges.contains(&Edge {
            from: from.clone(),
            to: to.clone(),
            kind: EdgeKind::Contains,
        });
        if parent_matches || contains_same_pair {
            return Err(GraphInvariantViolation::NonStructuralOwnership {
                domain,
                from: from.clone(),
                to: to.clone(),
            });
        }
        Ok(())
    }

    fn projection_path_exists(&self, start: &NodeId, goal: &NodeId) -> bool {
        self.projection_path_exists_excluding(start, goal, None)
    }

    fn projection_path_exists_excluding(
        &self,
        start: &NodeId,
        goal: &NodeId,
        excluded: Option<&Edge>,
    ) -> bool {
        let mut pending = vec![start.clone()];
        let mut seen = BTreeSet::new();
        while let Some(current) = pending.pop() {
            if current == *goal {
                return true;
            }
            if !seen.insert(current.clone()) {
                continue;
            }
            for edge in self
                .edges
                .iter()
                .filter(|edge| edge.kind == EdgeKind::ProjectsTo && edge.from == current)
            {
                if excluded == Some(edge) {
                    continue;
                }
                pending.push(edge.to.clone());
            }
        }
        false
    }

    pub fn node(&self, id: &NodeId) -> Option<&Node> {
        self.nodes.get(id)
    }

    pub fn nodes_by_type(&self, ty: OntologyType) -> impl Iterator<Item = &Node> {
        self.nodes
            .values()
            .filter(move |node| node.ontology_type == ty)
    }

    pub fn children(&self, parent: &NodeId) -> impl Iterator<Item = &Node> {
        let parent = parent.clone();
        self.nodes
            .values()
            .filter(move |node| node.parent.as_ref() == Some(&parent))
    }

    pub fn edges(&self) -> impl Iterator<Item = &Edge> {
        self.edges.iter()
    }

    pub fn outgoing(&self, from: &NodeId) -> impl Iterator<Item = &Edge> {
        let from = from.clone();
        self.edges.iter().filter(move |edge| edge.from == from)
    }

    pub fn outgoing_kind(&self, from: &NodeId, kind: EdgeKind) -> impl Iterator<Item = &Edge> {
        let from = from.clone();
        self.edges
            .iter()
            .filter(move |edge| edge.from == from && edge.kind == kind)
    }

    pub fn path_to_root(&self, id: &NodeId) -> Vec<&Node> {
        let mut out = Vec::new();
        let mut current = Some(id);
        let mut seen = BTreeSet::new();
        while let Some(cid) = current {
            if !seen.insert(cid.clone()) {
                break;
            }
            let Some(node) = self.nodes.get(cid) else {
                break;
            };
            current = node.parent.as_ref();
            out.push(node);
        }
        out.reverse();
        out
    }

    pub fn len(&self) -> usize {
        self.nodes.len()
    }
    pub fn edge_len(&self) -> usize {
        self.edges.len()
    }
    pub fn is_empty(&self) -> bool {
        self.nodes.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;

    fn registry() -> Arc<OntologyRegistry> {
        Arc::new(
            OntologyRegistry::from_json(include_str!(
                "../../../specifications/universal-ontology-v1.0.json"
            ))
            .unwrap(),
        )
    }

    fn node(id: &str, parent: Option<&str>, ty: OntologyType) -> Node {
        Node {
            id: NodeId::new(id),
            parent: parent.map(NodeId::new),
            ontology_type: ty,
            kind: None,
            name: None,
            source_span: None,
            materialized: true,
        }
    }

    #[test]
    fn canonical_levels_are_49() {
        assert_eq!(registry().len(), 49);
    }

    #[test]
    fn rejects_bad_level_order() {
        let mut g = OntologyGraph::new(registry());
        g.insert_node(node("bit", None, OntologyType::Bit)).unwrap();
        let err = g
            .insert_node(node("universe", Some("bit"), OntologyType::Universe))
            .unwrap_err();
        assert!(err.to_string().contains("invalid level ordering"));
    }

    #[test]
    fn containment_is_explicit_and_semantic_edges_are_not_forced_into_the_tree() {
        let mut g = OntologyGraph::new(registry());
        g.insert_node(node("u", None, OntologyType::Universe))
            .unwrap();
        g.insert_node(node("repo", Some("u"), OntologyType::Repository))
            .unwrap();
        g.insert_node(node("src", Some("repo"), OntologyType::Source))
            .unwrap();
        g.insert_node(node("event", None, OntologyType::Event))
            .unwrap();
        g.add_edge(NodeId::new("u"), NodeId::new("repo"), EdgeKind::Contains)
            .unwrap();
        g.add_edge(NodeId::new("src"), NodeId::new("event"), EdgeKind::Causes)
            .unwrap();
        assert_eq!(g.edge_len(), 2);
        assert!(g.validate_phase7_invariants().is_empty());
    }

    #[test]
    fn rejects_invalid_contains_and_duplicate_edges() {
        let mut g = OntologyGraph::new(registry());
        g.insert_node(node("u", None, OntologyType::Universe))
            .unwrap();
        g.insert_node(node("repo", Some("u"), OntologyType::Repository))
            .unwrap();
        assert!(matches!(
            g.add_edge(NodeId::new("repo"), NodeId::new("u"), EdgeKind::Contains),
            Err(OntologyError::InvalidContainment { .. })
        ));
        g.add_edge(NodeId::new("u"), NodeId::new("repo"), EdgeKind::Contains)
            .unwrap();
        assert!(matches!(
            g.add_edge(NodeId::new("u"), NodeId::new("repo"), EdgeKind::Contains),
            Err(OntologyError::DuplicateEdge { .. })
        ));
    }

    #[test]
    fn edge_domains_are_explicitly_separated() {
        assert_eq!(edge_domain(EdgeKind::Contains), EdgeDomain::Structural);
        assert_eq!(
            edge_domain(EdgeKind::ProjectsTo),
            EdgeDomain::SemanticProjection
        );
        assert_eq!(
            edge_domain(EdgeKind::ObservedAt),
            EdgeDomain::RuntimeObservation
        );
        assert_eq!(
            edge_domain(EdgeKind::RepresentedAs),
            EdgeDomain::Representation
        );
        assert_eq!(
            edge_domain(EdgeKind::References),
            EdgeDomain::GeneralSemantic
        );
    }

    #[test]
    fn projection_skips_levels_without_becoming_structural_parentage() {
        let mut g = OntologyGraph::new(registry());
        g.insert_node(node("repo", None, OntologyType::Repository))
            .unwrap();
        g.insert_node(node("source", Some("repo"), OntologyType::Source))
            .unwrap();
        g.insert_node(node("semantic:function", None, OntologyType::Function))
            .unwrap();
        g.add_edge(
            NodeId::new("repo"),
            NodeId::new("source"),
            EdgeKind::Contains,
        )
        .unwrap();

        assert!(g
            .add_projection(ProjectionMaterializationKey::new(
                NodeId::new("source"),
                NodeId::new("semantic:function"),
                "semantic.function.v1",
                "1",
            ))
            .unwrap());
        assert!(g.validate_phase7_invariants().is_empty());
        assert!(g
            .node(&NodeId::new("semantic:function"))
            .unwrap()
            .parent
            .is_none());
    }

    #[test]
    fn identical_projection_materialization_is_idempotent() {
        let mut g = OntologyGraph::new(registry());
        g.insert_node(node("source", None, OntologyType::Source))
            .unwrap();
        g.insert_node(node("semantic:function", None, OntologyType::Function))
            .unwrap();
        let key = ProjectionMaterializationKey::new(
            NodeId::new("source"),
            NodeId::new("semantic:function"),
            "semantic.function.v1",
            "1",
        );

        assert!(g.add_projection(key.clone()).unwrap());
        assert!(!g.add_projection(key).unwrap());
        assert_eq!(g.projection_keys().count(), 1);
        assert_eq!(
            g.outgoing_kind(&NodeId::new("source"), EdgeKind::ProjectsTo)
                .count(),
            1
        );
    }

    #[test]
    fn projection_cannot_claim_structural_ownership() {
        let mut g = OntologyGraph::new(registry());
        g.insert_node(node("source", None, OntologyType::Source))
            .unwrap();
        g.insert_node(node(
            "semantic:function",
            Some("source"),
            OntologyType::Function,
        ))
        .unwrap();

        let error = g
            .add_projection(ProjectionMaterializationKey::new(
                NodeId::new("source"),
                NodeId::new("semantic:function"),
                "semantic.function.v1",
                "1",
            ))
            .unwrap_err();
        assert!(matches!(
            error,
            GraphInvariantViolation::NonStructuralOwnership {
                domain: EdgeDomain::SemanticProjection,
                ..
            }
        ));
    }

    #[test]
    fn projection_cycles_are_rejected_deterministically() {
        let mut g = OntologyGraph::new(registry());
        g.insert_node(node("a", None, OntologyType::Entity))
            .unwrap();
        g.insert_node(node("b", None, OntologyType::Function))
            .unwrap();
        g.add_projection(ProjectionMaterializationKey::new(
            NodeId::new("a"),
            NodeId::new("b"),
            "rule.a",
            "1",
        ))
        .unwrap();

        let error = g
            .add_projection(ProjectionMaterializationKey::new(
                NodeId::new("b"),
                NodeId::new("a"),
                "rule.b",
                "1",
            ))
            .unwrap_err();
        assert!(matches!(
            error,
            GraphInvariantViolation::ProjectionCycle { .. }
        ));
    }

    #[test]
    fn traversal_order_is_deterministic() {
        let mut g = OntologyGraph::new(registry());
        g.insert_node(node("u", None, OntologyType::Universe))
            .unwrap();
        g.insert_node(node("repo/b", Some("u"), OntologyType::Repository))
            .unwrap();
        g.insert_node(node("repo/a", Some("u"), OntologyType::Repository))
            .unwrap();
        let ids: Vec<&str> = g
            .children(&NodeId::new("u"))
            .map(|n| n.id.as_str())
            .collect();
        assert_eq!(ids, vec!["repo/a", "repo/b"]);
    }

    #[test]
    fn tracks_path() {
        let mut g = OntologyGraph::new(registry());
        g.insert_node(node("u", None, OntologyType::Universe))
            .unwrap();
        g.insert_node(node("r", Some("u"), OntologyType::Repository))
            .unwrap();
        g.insert_node(node("s", Some("r"), OntologyType::Source))
            .unwrap();
        let path = g.path_to_root(&NodeId::new("s"));
        assert_eq!(path.len(), 3);
        assert_eq!(path[0].ontology_type, OntologyType::Universe);
        assert_eq!(path[2].ontology_type, OntologyType::Source);
    }
}
