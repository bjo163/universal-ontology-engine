use ontology_core::{EdgeKind, Node, NodeId, OntologyError, OntologyType};
use std::collections::{HashMap, HashSet};

#[derive(Debug, Default)]
pub struct OntologyGraph {
    nodes: HashMap<NodeId, Node>,
    edges: Vec<(NodeId, NodeId, EdgeKind)>,
    kind_owners: HashMap<String, OntologyType>,
}

impl OntologyGraph {
    pub fn new() -> Self { Self::default() }

    pub fn insert_node(&mut self, node: Node) -> Result<(), OntologyError> {
        if self.nodes.contains_key(&node.id) { return Err(OntologyError::DuplicateId(node.id)); }
        if let Some(parent) = &node.parent {
            let parent_node = self.nodes.get(parent).ok_or_else(|| OntologyError::UnknownParent(parent.clone()))?;
            if parent_node.ontology_type.level() >= node.ontology_type.level() {
                return Err(OntologyError::InvalidLevelOrdering {
                    parent: parent_node.ontology_type,
                    child: node.ontology_type,
                });
            }
        }
        if let Some(kind) = &node.kind {
            if let Some(owner) = self.kind_owners.get(kind) {
                if *owner != node.ontology_type {
                    return Err(OntologyError::KindConflict { kind: kind.clone(), owner: *owner });
                }
            } else {
                self.kind_owners.insert(kind.clone(), node.ontology_type);
            }
        }
        self.nodes.insert(node.id.clone(), node);
        Ok(())
    }

    pub fn add_edge(&mut self, from: NodeId, to: NodeId, kind: EdgeKind) -> Result<(), OntologyError> {
        if !self.nodes.contains_key(&from) { return Err(OntologyError::UnknownParent(from)); }
        if !self.nodes.contains_key(&to) { return Err(OntologyError::UnknownParent(to)); }
        self.edges.push((from, to, kind));
        Ok(())
    }

    pub fn node(&self, id: &NodeId) -> Option<&Node> { self.nodes.get(id) }

    pub fn nodes_by_type(&self, ty: OntologyType) -> impl Iterator<Item = &Node> {
        self.nodes.values().filter(move |n| n.ontology_type == ty)
    }

    pub fn children(&self, parent: &NodeId) -> impl Iterator<Item = &Node> {
        self.nodes.values().filter(move |n| n.parent.as_ref() == Some(parent))
    }

    pub fn path_to_root(&self, id: &NodeId) -> Vec<&Node> {
        let mut out = Vec::new();
        let mut current = Some(id.clone());
        let mut seen = HashSet::new();
        while let Some(cid) = current {
            if !seen.insert(cid.clone()) { break; }
            let Some(node) = self.nodes.get(&cid) else { break; };
            current = node.parent.clone();
            out.push(node);
        }
        out.reverse();
        out
    }

    pub fn len(&self) -> usize { self.nodes.len() }
    pub fn is_empty(&self) -> bool { self.nodes.is_empty() }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn node(id: &str, parent: Option<&str>, ty: OntologyType) -> Node {
        Node {
            id: NodeId(id.into()),
            parent: parent.map(|v| NodeId(v.into())),
            ontology_type: ty,
            kind: None,
            name: None,
            source_span: None,
            materialized: true,
        }
    }

    #[test]
    fn canonical_levels_are_49() { assert_eq!(OntologyType::ALL.len(), 49); }

    #[test]
    fn rejects_bad_level_order() {
        let mut g = OntologyGraph::new();
        g.insert_node(node("u", None, OntologyType::Universe)).unwrap();
        let err = g.insert_node(node("b", Some("u"), OntologyType::Bit)).unwrap_err();
        assert!(err.to_string().contains("invalid level ordering") || err.to_string().contains("unknown"));
    }

    #[test]
    fn tracks_path() {
        let mut g = OntologyGraph::new();
        g.insert_node(node("u", None, OntologyType::Universe)).unwrap();
        g.insert_node(node("r", Some("u"), OntologyType::Repository)).unwrap();
        g.insert_node(node("s", Some("r"), OntologyType::Source)).unwrap();
        let path = g.path_to_root(&NodeId("s".into()));
        assert_eq!(path.len(), 3);
        assert_eq!(path[0].ontology_type, OntologyType::Universe);
        assert_eq!(path[2].ontology_type, OntologyType::Source);
    }
}
