use crate::ast::identifier::Identifier;
use crate::ast::ty::Type;
use crate::source_tracking::fragment::Fragment;

#[derive(Debug)]
pub struct ConstDecl {
    matching_source: Fragment,
    name: Identifier,
    ty: Type,
    value: () // todo
}
