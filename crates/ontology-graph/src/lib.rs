use ontology_core::{Edge, EdgeKind, Node, NodeId, OntologyError, OntologyType};
use ontology_registry::OntologyRegistry;
use std::{collections::{BTreeMap, BTreeSet}, sync::Arc};

#[derive(Debug)]
pub struct OntologyGraph {
    registry: Arc<OntologyRegistry>,
    nodes: BTreeMap<NodeId, Node>,
    edges: BTreeSet<Edge>,
}

impl OntologyGraph {
    pub fn new(registry: Arc<OntologyRegistry>) -> Self {
        Self { registry, nodes: BTreeMap::new(), edges: BTreeSet::new() }
    }

    pub fn registry(&self) -> &OntologyRegistry { &self.registry }

    pub fn insert_node(&mut self, node: Node) -> Result<(), OntologyError> {
        if node.id.as_str().is_empty() { return Err(OntologyError::EmptyNodeId); }
        if self.nodes.contains_key(&node.id) { return Err(OntologyError::DuplicateId(node.id)); }
        if let Some(parent) = &node.parent {
            let parent_node = self.nodes.get(parent).ok_or_else(|| OntologyError::UnknownParent(parent.clone()))?;
            if parent_node.ontology_type.level() >= node.ontology_type.level() {
                return Err(OntologyError::InvalidLevelOrdering { parent: parent_node.ontology_type, child: node.ontology_type });
            }
        }
        if let Some(kind) = &node.kind {
            if let Some(owner) = self.registry.owner_of_kind(kind) {
                if owner != node.ontology_type {
                    return Err(OntologyError::KindConflict { kind: kind.clone(), owner });
                }
            }
        }
        self.nodes.insert(node.id.clone(), node);
        Ok(())
    }

    pub fn add_edge(&mut self, from: NodeId, to: NodeId, kind: EdgeKind) -> Result<(), OntologyError> {
        if !self.nodes.contains_key(&from) { return Err(OntologyError::UnknownNode(from)); }
        if !self.nodes.contains_key(&to) { return Err(OntologyError::UnknownNode(to)); }
        if from == to { return Err(OntologyError::SelfEdge(from)); }
        if !self.registry.supports_edge(kind) { return Err(OntologyError::UndeclaredEdgeKind(kind.slug().into())); }
        if kind == EdgeKind::Contains {
            let child = self.nodes.get(&to).expect("checked above");
            if child.parent.as_ref() != Some(&from) {
                return Err(OntologyError::InvalidContainment { from, to });
            }
        }
        let edge = Edge { from: from.clone(), to: to.clone(), kind };
        if !self.edges.insert(edge) { return Err(OntologyError::DuplicateEdge { from, to, kind }); }
        Ok(())
    }

    pub fn node(&self, id: &NodeId) -> Option<&Node> { self.nodes.get(id) }

    pub fn nodes_by_type(&self, ty: OntologyType) -> impl Iterator<Item = &Node> {
        self.nodes.values().filter(move |node| node.ontology_type == ty)
    }

    pub fn children(&self, parent: &NodeId) -> impl Iterator<Item = &Node> {
        let parent = parent.clone();
        self.nodes.values().filter(move |node| node.parent.as_ref() == Some(&parent))
    }

    pub fn edges(&self) -> impl Iterator<Item = &Edge> { self.edges.iter() }

    pub fn outgoing(&self, from: &NodeId) -> impl Iterator<Item = &Edge> {
        let from = from.clone();
        self.edges.iter().filter(move |edge| edge.from == from)
    }

    pub fn outgoing_kind(&self, from: &NodeId, kind: EdgeKind) -> impl Iterator<Item = &Edge> {
        let from = from.clone();
        self.edges.iter().filter(move |edge| edge.from == from && edge.kind == kind)
    }

    pub fn path_to_root(&self, id: &NodeId) -> Vec<&Node> {
        let mut out = Vec::new();
        let mut current = Some(id);
        let mut seen = BTreeSet::new();
        while let Some(cid) = current {
            if !seen.insert(cid.clone()) { break; }
            let Some(node) = self.nodes.get(cid) else { break; };
            current = node.parent.as_ref();
            out.push(node);
        }
        out.reverse();
        out
    }

    pub fn len(&self) -> usize { self.nodes.len() }
    pub fn edge_len(&self) -> usize { self.edges.len() }
    pub fn is_empty(&self) -> bool { self.nodes.is_empty() }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;

    fn registry() -> Arc<OntologyRegistry> {
        Arc::new(OntologyRegistry::from_json(include_str!("../../../specifications/universal-ontology-v1.0.json")).unwrap())
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
    fn canonical_levels_are_49() { assert_eq!(registry().len(), 49); }

    #[test]
    fn rejects_bad_level_order() {
        let mut g = OntologyGraph::new(registry());
        g.insert_node(node("u", None, OntologyType::Universe)).unwrap();
        let err = g.insert_node(node("b", Some("u"), OntologyType::Bit)).unwrap_err();
        assert!(err.to_string().contains("invalid level ordering"));
    }

    #[test]
    fn containment_is_explicit_and_semantic_edges_are_not_forced_into_the_tree() {
        let mut g = OntologyGraph::new(registry());
        g.insert_node(node("u", None, OntologyType::Universe)).unwrap();
        g.insert_node(node("repo", Some("u"), OntologyType::Repository)).unwrap();
        g.insert_node(node("src", Some("repo"), OntologyType::Source)).unwrap();
        g.insert_node(node("event", None, OntologyType::Event)).unwrap();
        g.add_edge(NodeId::new("u"), NodeId::new("repo"), EdgeKind::Contains).unwrap();
        g.add_edge(NodeId::new("src"), NodeId::new("event"), EdgeKind::Causes).unwrap();
        assert_eq!(g.edge_len(), 2);
    }

    #[test]
    fn rejects_invalid_contains_and_duplicate_edges() {
        let mut g = OntologyGraph::new(registry());
        g.insert_node(node("u", None, OntologyType::Universe)).unwrap();
        g.insert_node(node("repo", Some("u"), OntologyType::Repository)).unwrap();
        assert!(matches!(g.add_edge(NodeId::new("repo"), NodeId::new("u"), EdgeKind::Contains), Err(OntologyError::InvalidContainment { .. })));
        g.add_edge(NodeId::new("u"), NodeId::new("repo"), EdgeKind::Contains).unwrap();
        assert!(matches!(g.add_edge(NodeId::new("u"), NodeId::new("repo"), EdgeKind::Contains), Err(OntologyError::DuplicateEdge { .. })));
    }

    #[test]
    fn traversal_order_is_deterministic() {
        let mut g = OntologyGraph::new(registry());
        g.insert_node(node("u", None, OntologyType::Universe)).unwrap();
        g.insert_node(node("repo/b", Some("u"), OntologyType::Repository)).unwrap();
        g.insert_node(node("repo/a", Some("u"), OntologyType::Repository)).unwrap();
        let ids: Vec<&str> = g.children(&NodeId::new("u")).map(|n| n.id.as_str()).collect();
        assert_eq!(ids, vec!["repo/a", "repo/b"]);
    }

    #[test]
    fn tracks_path() {
        let mut g = OntologyGraph::new(registry());
        g.insert_node(node("u", None, OntologyType::Universe)).unwrap();
        g.insert_node(node("r", Some("u"), OntologyType::Repository)).unwrap();
        g.insert_node(node("s", Some("r"), OntologyType::Source)).unwrap();
        let path = g.path_to_root(&NodeId::new("s"));
        assert_eq!(path.len(), 3);
        assert_eq!(path[0].ontology_type, OntologyType::Universe);
        assert_eq!(path[2].ontology_type, OntologyType::Source);
    }
}
