use std::ops::Deref;

use codegen_rs::structures::gen_enum::NamedComponentSignature;
use codegen_rs::structures::visibility::Visibility;
use codegen_rs::structures::{RustType, TypeDef, TypeDefDeclaration};
use codegen_rs::{FileBuilder, RustCase};

use crate::generator::types::{ConstType, IdTypeAlias, IdTypeUnion, TypeAlias, TypeHelpers};
use crate::Xcb;

pub(crate) fn implement_xid(t: &IdTypeAlias, fb: FileBuilder) -> FileBuilder {
    let rust_name = RustCase::convert_to_valid_rust(&t.name, RustCase::Pascal).unwrap();
    fb.add_type_def_simple_type(Visibility::Public, &rust_name, RustType::in_scope("u32"))
}

pub(crate) fn implement_xid_union(t: &IdTypeUnion, fb: FileBuilder) -> FileBuilder {
    let rust_name = RustCase::convert_to_valid_rust(&t.name, RustCase::Pascal).unwrap();
    fb.add_type_def_simple_type(Visibility::Public, &rust_name, RustType::in_scope("u32"))
}

pub(crate) fn implement_typedef(td: &TypeAlias, xcb: &Xcb, fb: FileBuilder) -> FileBuilder {
    let name = td.rust_entity_name();
    let old = td.aliased.deref().borrow().import_name(&xcb.header);
    fb.add_type_def(TypeDef::Const(TypeDefDeclaration::new(
        Visibility::Public,
        NamedComponentSignature::new_simple_type(name, RustType::in_scope(old)),
    )))
}

pub(crate) fn implement_const_type(ct: &ConstType, fb: FileBuilder) -> FileBuilder {
    let name = ct.rust_entity_name();
    let con = ct.builtin.rust_entity_name();
    fb.add_type_def(TypeDef::Const(TypeDefDeclaration::new(
        Visibility::Public,
        NamedComponentSignature::new_simple_type(&name, RustType::in_scope(&con)),
    )))
}
