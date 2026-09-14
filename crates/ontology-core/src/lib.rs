use serde::{Deserialize, Serialize};
use std::fmt;

pub const ONTOLOGY_VERSION: &str = "1.0.0";
pub const LEVEL_COUNT: usize = 49;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[repr(u8)]
pub enum OntologyType {
    Universe = 1,
    Creation,
    Order,
    Reality,
    Realm,
    World,
    Domain,
    Ecosystem,
    Organization,
    Community,
    Region,
    Environment,
    Network,
    Context,
    Purpose,
    Mission,
    Objective,
    Program,
    Project,
    Product,
    System,
    Repository,
    Source,
    Unit,
    Module,
    Subsystem,
    Component,
    Element,
    Symbol,
    Entity,
    Property,
    Relation,
    Operation,
    Function,
    Behavior,
    State,
    Event,
    Process,
    Flow,
    Transition,
    Action,
    Execution,
    Instruction,
    Expression,
    Value,
    Data,
    Token,
    Character,
    Bit,
}

impl OntologyType {
    pub const ALL: [OntologyType; LEVEL_COUNT] = [
        Self::Universe, Self::Creation, Self::Order, Self::Reality, Self::Realm, Self::World,
        Self::Domain, Self::Ecosystem, Self::Organization, Self::Community, Self::Region,
        Self::Environment, Self::Network, Self::Context, Self::Purpose, Self::Mission,
        Self::Objective, Self::Program, Self::Project, Self::Product, Self::System, Self::Repository,
        Self::Source, Self::Unit, Self::Module, Self::Subsystem, Self::Component, Self::Element,
        Self::Symbol, Self::Entity, Self::Property, Self::Relation, Self::Operation, Self::Function,
        Self::Behavior, Self::State, Self::Event, Self::Process, Self::Flow, Self::Transition,
        Self::Action, Self::Execution, Self::Instruction, Self::Expression, Self::Value, Self::Data,
        Self::Token, Self::Character, Self::Bit,
    ];

    pub const fn level(self) -> u8 { self as u8 }

    pub const fn zone(self) -> u8 { ((self.level() - 1) / 7) + 1 }

    pub const fn slug(self) -> &'static str {
        match self {
            Self::Universe => "UNIVERSE", Self::Creation => "CREATION", Self::Order => "ORDER",
            Self::Reality => "REALITY", Self::Realm => "REALM", Self::World => "WORLD", Self::Domain => "DOMAIN",
            Self::Ecosystem => "ECOSYSTEM", Self::Organization => "ORGANIZATION", Self::Community => "COMMUNITY",
            Self::Region => "REGION", Self::Environment => "ENVIRONMENT", Self::Network => "NETWORK", Self::Context => "CONTEXT",
            Self::Purpose => "PURPOSE", Self::Mission => "MISSION", Self::Objective => "OBJECTIVE", Self::Program => "PROGRAM",
            Self::Project => "PROJECT", Self::Product => "PRODUCT", Self::System => "SYSTEM", Self::Repository => "REPOSITORY",
            Self::Source => "SOURCE", Self::Unit => "UNIT", Self::Module => "MODULE", Self::Subsystem => "SUBSYSTEM",
            Self::Component => "COMPONENT", Self::Element => "ELEMENT", Self::Symbol => "SYMBOL", Self::Entity => "ENTITY",
            Self::Property => "PROPERTY", Self::Relation => "RELATION", Self::Operation => "OPERATION", Self::Function => "FUNCTION",
            Self::Behavior => "BEHAVIOR", Self::State => "STATE", Self::Event => "EVENT", Self::Process => "PROCESS",
            Self::Flow => "FLOW", Self::Transition => "TRANSITION", Self::Action => "ACTION", Self::Execution => "EXECUTION",
            Self::Instruction => "INSTRUCTION", Self::Expression => "EXPRESSION", Self::Value => "VALUE", Self::Data => "DATA",
            Self::Token => "TOKEN", Self::Character => "CHARACTER", Self::Bit => "BIT",
        }
    }
}

impl fmt::Display for OntologyType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { f.write_str(self.slug()) }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct NodeId(pub String);

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SourceSpan {
    pub line_start: u32,
    pub column_start: u32,
    pub line_end: u32,
    pub column_end: u32,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum EdgeKind {
    Contains,
    References,
    Specializes,
    DependsOn,
    Invokes,
    Produces,
    Consumes,
    Causes,
    ProjectsTo,
    RepresentedAs,
    ObservedAt,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Node {
    pub id: NodeId,
    pub parent: Option<NodeId>,
    pub ontology_type: OntologyType,
    pub kind: Option<String>,
    pub name: Option<String>,
    pub source_span: Option<SourceSpan>,
    pub materialized: bool,
}

#[derive(Debug, thiserror::Error)]
pub enum OntologyError {
    #[error("duplicate node id: {0:?}")]
    DuplicateId(NodeId),
    #[error("unknown parent: {0:?}")]
    UnknownParent(NodeId),
    #[error("invalid level ordering: {parent} -> {child}")]
    InvalidLevelOrdering { parent: OntologyType, child: OntologyType },
    #[error("kind `{kind}` is already owned by type {owner}")]
    KindConflict { kind: String, owner: OntologyType },
}

pub fn canonical_path() -> &'static [OntologyType; LEVEL_COUNT] { &OntologyType::ALL }
