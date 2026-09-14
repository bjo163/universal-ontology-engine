use ontology_core::{EdgeKind, Node, NodeId, OntologyType};
use ontology_graph::{OntologyGraph, ProjectionMaterializationKey};
use ontology_registry::OntologyRegistry;
use ontology_semantic::{
    normalization::{
        foundation_rules, normalize_candidates, NativeEvidenceDescriptor, NormalizationClass,
        NormalizationRule,
    },
    resolution::{
        resolve_evaluations, ConflictPair, ResolutionOutcome, RuleEvaluation, UnresolvedCode,
    },
    rules::{EvidenceKind, ProjectionEvidence, ProjectionRule},
    trace::{NativeEvidenceTrace, ProjectionTrace, RuleRef, TraceOutcome},
    SemanticTarget,
};
use serde::Deserialize;
use std::{
    collections::{BTreeMap, BTreeSet},
    sync::Arc,
};

const FIXTURE: &str = include_str!("../../../fixtures/semantic-projection-v1/cases.json");
const REGISTRY: &str = include_str!("../../../specifications/universal-ontology-v1.0.json");

#[derive(Debug, Deserialize)]
struct Manifest {
    schema: String,
    rfc: String,
    positive: Vec<PositiveCase>,
    unresolved: Vec<UnresolvedCase>,
    graph: Vec<GraphCase>,
}

#[derive(Debug, Deserialize)]
struct PositiveCase {
    id: String,
    source_id: String,
    language: String,
    native_kind: String,
    source_type: String,
    expected_normalization_rule: String,
    expected_normalization_class: String,
    semantic_rule_id: String,
    semantic_rule_version: String,
    target_type: String,
    metadata: BTreeMap<String, String>,
}

#[derive(Debug, Deserialize)]
struct UnresolvedCase {
    id: String,
    mode: String,
    source_id: String,
    language: String,
    native_kind: String,
    source_type: String,
    expected_code: String,
    reason: String,
}

#[derive(Debug, Deserialize)]
struct GraphCase {
    id: String,
    source_id: String,
    source_type: String,
    target_id: String,
    target_type: String,
    rule_id: String,
    rule_version: String,
    expected_edge: String,
    reason: String,
}

fn manifest() -> Manifest {
    serde_json::from_str(FIXTURE).expect("fixture manifest must be valid JSON")
}

fn ontology_type(slug: &str) -> OntologyType {
    OntologyType::from_slug(slug).unwrap_or_else(|| panic!("unknown ontology type {slug}"))
}

fn normalization_class(value: &str) -> NormalizationClass {
    match value {
        "DeclaredType" => NormalizationClass::DeclaredType,
        "CallableDeclaration" => NormalizationClass::CallableDeclaration,
        "CallableBinding" => NormalizationClass::CallableBinding,
        "MethodDeclaration" => NormalizationClass::MethodDeclaration,
        "NamedValueBinding" => NormalizationClass::NamedValueBinding,
        "PropertyBinding" => NormalizationClass::PropertyBinding,
        "ModuleBoundaryStatement" => NormalizationClass::ModuleBoundaryStatement,
        other => panic!("unknown normalization class {other}"),
    }
}

fn unresolved_code(value: &str) -> UnresolvedCode {
    match value {
        "unsupported" => UnresolvedCode::Unsupported,
        "insufficient" => UnresolvedCode::Insufficient,
        "ambiguous" => UnresolvedCode::Ambiguous,
        "conflicting" => UnresolvedCode::Conflicting,
        "invalid_evidence" => UnresolvedCode::InvalidEvidence,
        other => panic!("unknown unresolved code {other}"),
    }
}

fn descriptor(
    language: &str,
    native_kind: &str,
    source_type: &str,
) -> NativeEvidenceDescriptor {
    NativeEvidenceDescriptor {
        language: language.to_owned(),
        native_kind: native_kind.to_owned(),
        source_type: ontology_type(source_type),
    }
}

fn conformance_rule(
    rule_id: &str,
    rule_version: &str,
    target: OntologyType,
) -> ProjectionRule {
    ProjectionRule {
        rule_id: rule_id.to_owned(),
        rule_version: rule_version.to_owned(),
        required_evidence: BTreeSet::from([EvidenceKind::NativeSyntax, EvidenceKind::Normalized]),
        optional_evidence: BTreeSet::from([EvidenceKind::Provenance]),
        required_fields: BTreeSet::from([
            "language".to_owned(),
            "native_kind".to_owned(),
            "normalization_rule_id".to_owned(),
        ]),
        optional_fields: BTreeSet::from(["name".to_owned(), "scope".to_owned()]),
        target: SemanticTarget::new(target, None::<String>),
        edge_kind: EdgeKind::ProjectsTo,
        missing_required_outcome: UnresolvedCode::Insufficient,
    }
}

fn projection_evidence(
    source_id: &str,
    source_type: OntologyType,
    language: &str,
    native_kind: &str,
    normalization_rule_id: Option<&str>,
    metadata: &BTreeMap<String, String>,
) -> ProjectionEvidence {
    let mut fields = metadata.clone();
    fields.insert("language".into(), language.into());
    fields.insert("native_kind".into(), native_kind.into());
    if let Some(rule_id) = normalization_rule_id {
        fields.insert("normalization_rule_id".into(), rule_id.into());
    }
    ProjectionEvidence {
        source_id: NodeId::new(source_id),
        source_type,
        evidence_kinds: BTreeSet::from([EvidenceKind::NativeSyntax, EvidenceKind::Normalized]),
        fields,
    }
}

fn native_trace(
    language: &str,
    native_kind: &str,
    metadata: BTreeMap<String, String>,
) -> NativeEvidenceTrace {
    NativeEvidenceTrace {
        language: Some(language.to_owned()),
        native_kind: Some(native_kind.to_owned()),
        metadata,
    }
}

fn run_positive(case: &PositiveCase) -> ProjectionTrace {
    let native = descriptor(&case.language, &case.native_kind, &case.source_type);
    let normalized = normalize_candidates(&native, foundation_rules());
    assert_eq!(normalized.len(), 1, "{} normalization cardinality", case.id);
    assert_eq!(
        normalized[0].rule_id, case.expected_normalization_rule,
        "{} normalization rule",
        case.id
    );
    assert_eq!(
        normalized[0].class,
        normalization_class(&case.expected_normalization_class),
        "{} normalization class",
        case.id
    );
    assert_eq!(normalized[0].original, native, "{} native evidence", case.id);

    let target = ontology_type(&case.target_type);
    let rule = conformance_rule(
        &case.semantic_rule_id,
        &case.semantic_rule_version,
        target,
    );
    let evidence = projection_evidence(
        &case.source_id,
        native.source_type,
        &case.language,
        &case.native_kind,
        Some(&normalized[0].rule_id),
        &case.metadata,
    );
    let evaluation = rule.evaluate(&evidence).expect("valid conformance rule");
    let resolution = resolve_evaluations(evidence.source_id.clone(), [evaluation], &[]);
    let trace = ProjectionTrace::from_resolution(
        evidence.source_id.clone(),
        None,
        native_trace(&case.language, &case.native_kind, case.metadata.clone()),
        normalized,
        [RuleRef::new(
            case.semantic_rule_id.clone(),
            case.semantic_rule_version.clone(),
        )],
        resolution,
    )
    .expect("positive trace must satisfy contract");

    let TraceOutcome::Projected {
        target: actual,
        edge_kind,
    } = &trace.outcome
    else {
        panic!("{} expected accepted projection", case.id)
    };
    assert_eq!(actual.ontology_type, target, "{} target", case.id);
    assert_eq!(*edge_kind, EdgeKind::ProjectsTo, "{} edge", case.id);
    trace
}

fn run_unresolved(case: &UnresolvedCase) -> ProjectionTrace {
    assert!(!case.reason.trim().is_empty(), "{} must explain negative case", case.id);
    let native = descriptor(&case.language, &case.native_kind, &case.source_type);
    let normalized = normalize_candidates(&native, foundation_rules());
    let expected = unresolved_code(&case.expected_code);

    let (resolution, rule_refs) = match case.mode.as_str() {
        "unsupported" => {
            assert!(normalized.is_empty(), "{} must not normalize", case.id);
            (
                resolve_evaluations(NodeId::new(&case.source_id), [], &[]),
                Vec::new(),
            )
        }
        "insufficient" => {
            assert_eq!(normalized.len(), 1, "{} must have native normalization", case.id);
            let rule = conformance_rule("conformance.insufficient.v1", "1", OntologyType::Function);
            let evidence = projection_evidence(
                &case.source_id,
                native.source_type,
                &case.language,
                &case.native_kind,
                None,
                &BTreeMap::new(),
            );
            let evaluation = rule.evaluate(&evidence).expect("valid rule");
            (
                resolve_evaluations(evidence.source_id, [evaluation], &[]),
                vec![RuleRef::new("conformance.insufficient.v1", "1")],
            )
        }
        "ambiguous" => {
            assert_eq!(normalized.len(), 1, "{} must normalize", case.id);
            let evidence = projection_evidence(
                &case.source_id,
                native.source_type,
                &case.language,
                &case.native_kind,
                Some(&normalized[0].rule_id),
                &BTreeMap::new(),
            );
            let function = conformance_rule("conformance.ambiguous.function", "1", OntologyType::Function);
            let entity = conformance_rule("conformance.ambiguous.entity", "1", OntologyType::Entity);
            let evaluations = [
                function.evaluate(&evidence).expect("valid rule"),
                entity.evaluate(&evidence).expect("valid rule"),
            ];
            (
                resolve_evaluations(evidence.source_id, evaluations, &[]),
                vec![
                    RuleRef::new("conformance.ambiguous.function", "1"),
                    RuleRef::new("conformance.ambiguous.entity", "1"),
                ],
            )
        }
        "conflicting" => {
            assert_eq!(normalized.len(), 1, "{} must normalize", case.id);
            let evidence = projection_evidence(
                &case.source_id,
                native.source_type,
                &case.language,
                &case.native_kind,
                Some(&normalized[0].rule_id),
                &BTreeMap::new(),
            );
            let left = conformance_rule("conformance.conflict.left", "1", OntologyType::Function);
            let right = conformance_rule("conformance.conflict.right", "1", OntologyType::Function);
            let evaluations = [
                left.evaluate(&evidence).expect("valid rule"),
                right.evaluate(&evidence).expect("valid rule"),
            ];
            (
                resolve_evaluations(
                    evidence.source_id,
                    evaluations,
                    &[ConflictPair::new(
                        "conformance.conflict.left",
                        "conformance.conflict.right",
                    )],
                ),
                vec![
                    RuleRef::new("conformance.conflict.left", "1"),
                    RuleRef::new("conformance.conflict.right", "1"),
                ],
            )
        }
        other => panic!("unknown unresolved mode {other}"),
    };

    let trace = ProjectionTrace::from_resolution(
        NodeId::new(&case.source_id),
        None,
        native_trace(&case.language, &case.native_kind, BTreeMap::new()),
        normalized,
        rule_refs,
        resolution,
    )
    .expect("unresolved trace must satisfy contract");

    let TraceOutcome::Unresolved { codes, .. } = &trace.outcome else {
        panic!("{} must not invent PROJECTS_TO", case.id)
    };
    assert!(codes.contains(&expected), "{} unresolved code", case.id);
    trace
}

fn graph_summary(case: &GraphCase) -> String {
    assert_eq!(case.expected_edge, "PROJECTS_TO");
    assert!(!case.reason.trim().is_empty());
    let registry = Arc::new(OntologyRegistry::from_json(REGISTRY).expect("valid registry"));
    let mut graph = OntologyGraph::new(registry);
    graph
        .insert_node(Node {
            id: NodeId::new(&case.source_id),
            parent: None,
            ontology_type: ontology_type(&case.source_type),
            kind: None,
            name: None,
            source_span: None,
            materialized: true,
        })
        .unwrap();
    graph
        .insert_node(Node {
            id: NodeId::new(&case.target_id),
            parent: None,
            ontology_type: ontology_type(&case.target_type),
            kind: None,
            name: None,
            source_span: None,
            materialized: true,
        })
        .unwrap();
    assert!(graph
        .add_projection(ProjectionMaterializationKey::new(
            NodeId::new(&case.source_id),
            NodeId::new(&case.target_id),
            &case.rule_id,
            &case.rule_version,
        ))
        .unwrap());
    assert!(graph.validate_phase7_invariants().is_empty());
    assert!(graph.node(&NodeId::new(&case.target_id)).unwrap().parent.is_none());
    assert_eq!(
        graph
            .outgoing_kind(&NodeId::new(&case.source_id), EdgeKind::ProjectsTo)
            .count(),
        1
    );
    format!(
        "{}:{}:{}:{}",
        case.source_id, case.target_id, case.rule_id, case.rule_version
    )
}

fn run_suite() -> BTreeMap<String, String> {
    let manifest = manifest();
    assert_eq!(
        manifest.schema,
        "universal-ontology-engine/semantic-conformance-v1"
    );
    assert_eq!(manifest.rfc, "oxdx.semantic-projection.rfc/v1");

    let mut out = BTreeMap::new();
    for case in &manifest.positive {
        let trace = run_positive(case);
        out.insert(case.id.clone(), serde_json::to_string(&trace).unwrap());
    }
    for case in &manifest.unresolved {
        let trace = run_unresolved(case);
        out.insert(case.id.clone(), serde_json::to_string(&trace).unwrap());
    }
    for case in &manifest.graph {
        out.insert(case.id.clone(), graph_summary(case));
    }
    out
}

#[test]
fn committed_corpus_executes_without_network_or_language_bypass() {
    let output = run_suite();
    for required in ["rust", "typescript", "javascript", "python", "go", "java", "kotlin"] {
        assert!(
            output.keys().any(|key| key.starts_with(required)),
            "missing positive conformance case for {required}"
        );
    }
}

#[test]
fn repeated_suite_runs_are_byte_identical() {
    let first = serde_json::to_vec(&run_suite()).unwrap();
    let second = serde_json::to_vec(&run_suite()).unwrap();
    assert_eq!(first, second);
}

#[test]
fn duplicate_names_in_different_scopes_keep_distinct_source_identity() {
    let manifest = manifest();
    let a = manifest
        .positive
        .iter()
        .find(|case| case.id == "python-function-scope-a")
        .unwrap();
    let b = manifest
        .positive
        .iter()
        .find(|case| case.id == "python-function-scope-b")
        .unwrap();
    assert_eq!(a.metadata.get("name"), b.metadata.get("name"));
    assert_ne!(a.metadata.get("scope"), b.metadata.get("scope"));
    assert_ne!(run_positive(a).source_id, run_positive(b).source_id);
}

#[test]
fn language_specific_rule_cannot_change_unrelated_normalization() {
    let evidence = descriptor("typescript", "function", "FUNCTION");
    let baseline = normalize_candidates(&evidence, foundation_rules());
    let mut extended = foundation_rules();
    extended.push(NormalizationRule {
        rule_id: "norm.kotlin.fixture-only".into(),
        rule_version: "1".into(),
        language: "kotlin".into(),
        native_kind: "fixture_only".into(),
        source_type: OntologyType::Function,
        class: NormalizationClass::CallableDeclaration,
    });
    assert_eq!(baseline, normalize_candidates(&evidence, extended));
}

#[test]
fn unresolved_cases_never_materialize_an_accepted_projection() {
    for case in &manifest().unresolved {
        let trace = run_unresolved(case);
        assert!(matches!(trace.outcome, TraceOutcome::Unresolved { .. }));
    }
}

#[test]
fn fixture_rule_order_does_not_choose_ambiguous_truth() {
    let native = descriptor("python", "function", "FUNCTION");
    let normalized = normalize_candidates(&native, foundation_rules());
    let evidence = projection_evidence(
        "fixture:order",
        native.source_type,
        "python",
        "function",
        Some(&normalized[0].rule_id),
        &BTreeMap::new(),
    );
    let a = conformance_rule("rule.a", "1", OntologyType::Function)
        .evaluate(&evidence)
        .unwrap();
    let b = conformance_rule("rule.b", "1", OntologyType::Entity)
        .evaluate(&evidence)
        .unwrap();
    let first = resolve_evaluations(evidence.source_id.clone(), [a.clone(), b.clone()], &[]);
    let second = resolve_evaluations(evidence.source_id, [b, a], &[]);
    assert_eq!(first, second);
    let ResolutionOutcome::Unresolved(result) = first else {
        panic!("competing fixture targets must remain unresolved")
    };
    assert!(result.codes.contains(&UnresolvedCode::Ambiguous));
}
