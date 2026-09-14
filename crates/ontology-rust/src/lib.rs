use ontology_core::OntologyType;
use proc_macro2::Span;
use serde::Serialize;
use syn::{spanned::Spanned, visit::Visit, Expr, File, ImplItem, Item, Type};

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SourceLocation {
    pub line_start: usize,
    pub column_start: usize,
    pub line_end: usize,
    pub column_end: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct RustAstObservation {
    pub ontology_type: OntologyType,
    pub native_kind: String,
    pub name: Option<String>,
    pub location: SourceLocation,
}

#[derive(Debug, thiserror::Error)]
pub enum RustAstError {
    #[error("failed to parse Rust source: {0}")]
    Parse(#[from] syn::Error),
}

pub fn parse(source: &str) -> Result<Vec<RustAstObservation>, RustAstError> {
    let file = syn::parse_file(source)?;
    Ok(collect(&file))
}

pub fn parse_file(source: &str) -> Result<File, RustAstError> {
    Ok(syn::parse_file(source)?)
}

pub fn collect(file: &File) -> Vec<RustAstObservation> {
    let mut visitor = RustVisitor::default();
    visitor.visit_file(file);
    visitor.observations.sort_by(|a, b| {
        (
            a.location.line_start,
            a.location.column_start,
            a.location.line_end,
            a.ontology_type.level(),
        )
            .cmp(&(
                b.location.line_start,
                b.location.column_start,
                b.location.line_end,
                b.ontology_type.level(),
            ))
    });
    visitor.observations
}

#[derive(Default)]
struct RustVisitor {
    observations: Vec<RustAstObservation>,
}

impl RustVisitor {
    fn push(
        &mut self,
        ty: OntologyType,
        native_kind: impl Into<String>,
        name: Option<String>,
        span: Span,
    ) {
        self.observations.push(RustAstObservation {
            ontology_type: ty,
            native_kind: native_kind.into(),
            name,
            location: location(span),
        });
    }

    fn type_name(ty: &Type) -> Option<String> {
        match ty {
            Type::Path(path) => path.path.segments.last().map(|s| s.ident.to_string()),
            _ => None,
        }
    }

    fn expression_name(expr: &Expr) -> Option<String> {
        match expr {
            Expr::Path(path) => path.path.segments.last().map(|s| s.ident.to_string()),
            _ => None,
        }
    }
}

impl<'ast> Visit<'ast> for RustVisitor {
    fn visit_item(&mut self, item: &'ast Item) {
        match item {
            Item::Const(item) => self.push(
                OntologyType::Value,
                "const",
                Some(item.ident.to_string()),
                item.span(),
            ),
            Item::Static(item) => self.push(
                OntologyType::Value,
                "static",
                Some(item.ident.to_string()),
                item.span(),
            ),
            Item::Enum(item) => self.push(
                OntologyType::Entity,
                "enum",
                Some(item.ident.to_string()),
                item.span(),
            ),
            Item::Fn(item) => self.push(
                OntologyType::Function,
                "function",
                Some(item.sig.ident.to_string()),
                item.span(),
            ),
            Item::Impl(item) => self.push(OntologyType::Relation, "impl", None, item.span()),
            Item::Macro(item) => self.push(
                OntologyType::Operation,
                "macro",
                item.ident.as_ref().map(ToString::to_string),
                item.span(),
            ),
            Item::Mod(item) => self.push(
                OntologyType::Module,
                "mod",
                Some(item.ident.to_string()),
                item.span(),
            ),
            Item::Struct(item) => self.push(
                OntologyType::Entity,
                "struct",
                Some(item.ident.to_string()),
                item.span(),
            ),
            Item::Trait(item) => self.push(
                OntologyType::Entity,
                "trait",
                Some(item.ident.to_string()),
                item.span(),
            ),
            Item::TraitAlias(item) => self.push(
                OntologyType::Entity,
                "trait_alias",
                Some(item.ident.to_string()),
                item.span(),
            ),
            Item::Type(item) => self.push(
                OntologyType::Entity,
                "type_alias",
                Some(item.ident.to_string()),
                item.span(),
            ),
            Item::Union(item) => self.push(
                OntologyType::Entity,
                "union",
                Some(item.ident.to_string()),
                item.span(),
            ),
            Item::Use(item) => self.push(OntologyType::Instruction, "use", None, item.span()),
            Item::ExternCrate(item) => self.push(
                OntologyType::Instruction,
                "extern_crate",
                Some(item.ident.to_string()),
                item.span(),
            ),
            Item::ForeignMod(item) => {
                self.push(OntologyType::Entity, "foreign_mod", None, item.span())
            }
            Item::Verbatim(_) => {}
            _ => {}
        }
        syn::visit::visit_item(self, item);
    }

    fn visit_field(&mut self, field: &'ast syn::Field) {
        let name = field
            .ident
            .as_ref()
            .map(ToString::to_string)
            .or_else(|| Self::type_name(&field.ty));
        self.push(OntologyType::Property, "field", name, field.span());
        syn::visit::visit_field(self, field);
    }

    fn visit_impl_item(&mut self, item: &'ast ImplItem) {
        match item {
            ImplItem::Const(item) => self.push(
                OntologyType::Value,
                "impl_const",
                Some(item.ident.to_string()),
                item.span(),
            ),
            ImplItem::Fn(item) => self.push(
                OntologyType::Operation,
                "method",
                Some(item.sig.ident.to_string()),
                item.span(),
            ),
            ImplItem::Type(item) => self.push(
                OntologyType::Entity,
                "associated_type",
                Some(item.ident.to_string()),
                item.span(),
            ),
            ImplItem::Macro(item) => {
                self.push(OntologyType::Operation, "impl_macro", None, item.span())
            }
            ImplItem::Verbatim(_) => {}
            _ => {}
        }
        syn::visit::visit_impl_item(self, item);
    }

    fn visit_expr(&mut self, expr: &'ast Expr) {
        if let Expr::Call(call) = expr {
            self.push(
                OntologyType::Operation,
                "call",
                Self::expression_name(&call.func),
                expr.span(),
            );
        }
        syn::visit::visit_expr(self, expr);
    }
}

fn location(span: Span) -> SourceLocation {
    let start = span.start();
    let end = span.end();
    SourceLocation {
        line_start: start.line,
        column_start: start.column,
        line_end: end.line,
        column_end: end.column,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extracts_core_rust_semantics() {
        let source = r#"
            struct User { id: u64 }
            const LIMIT: usize = 10;
            fn run(value: u64) -> u64 { value }
            impl User { fn load(&self) {} }
        "#;
        let observations = parse(source).unwrap();
        assert!(observations
            .iter()
            .any(|o| o.ontology_type == OntologyType::Entity && o.name.as_deref() == Some("User")));
        assert!(observations
            .iter()
            .any(|o| o.ontology_type == OntologyType::Property && o.name.as_deref() == Some("id")));
        assert!(observations
            .iter()
            .any(|o| o.ontology_type == OntologyType::Value && o.name.as_deref() == Some("LIMIT")));
        assert!(
            observations
                .iter()
                .any(|o| o.ontology_type == OntologyType::Function
                    && o.name.as_deref() == Some("run"))
        );
        assert!(observations.iter().any(
            |o| o.ontology_type == OntologyType::Operation && o.name.as_deref() == Some("load")
        ));
    }

    #[test]
    fn locations_are_recorded() {
        let observations = parse("fn hello() {}\n").unwrap();
        let function = observations
            .iter()
            .find(|o| o.name.as_deref() == Some("hello"))
            .unwrap();
        assert_eq!(function.location.line_start, 1);
    }

    #[test]
    fn malformed_rust_is_an_explicit_error() {
        assert!(parse("fn broken( {").is_err());
    }
}
