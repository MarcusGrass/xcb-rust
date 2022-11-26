use codegen_rs::structures::gen_enum::NamedComponentSignature;
use codegen_rs::structures::method::Argument;
use codegen_rs::structures::{ComponentSignature, Ownership, RustType, Signature};
use codegen_rs::{FileBuilder, ImplBuilder, MethodBuilder};

use crate::generator::types::{TypeHelpers, WrappedType, XcbType};

pub(crate) fn implement_from_uints(
    name: &str,
    size: usize,
    from: usize,
    wrap_with: Option<WrappedType>,
    mut fb: FileBuilder,
) -> FileBuilder {
    let impl_for = if from == 32 {
        vec!["u32", "u16", "u8"]
    } else if from == 16 {
        vec!["u16", "u8"]
    } else if from == 8 {
        vec!["u8"]
    } else {
        panic!("Bad size {size}");
    };
    for u in impl_for {
        let body = if let Some(wrap) = wrap_with.clone() {
            if let XcbType::Builtin(_) = wrap.borrow().clone() {
                format!("Self(val as u{})", size)
            } else {
                format!("Self({}::from(val as u{}))", wrap.rust_entity_name(), size)
            }
        } else {
            format!("Self(val as u{})", size)
        };
        let impl_body = ImplBuilder::new(Signature::simple(RustType::in_scope(name)))
            .implement_for(Signature::simple(RustType::in_scope(format!("From<{u}>"))))
            .add_method(
                MethodBuilder::new("from")
                    .add_argument(Argument::new(
                        Ownership::Owned,
                        NamedComponentSignature::new_simple_type("val", RustType::in_scope(u)),
                    ))
                    .add_simple_annotation("inline")
                    .set_body(body)
                    .set_return_type(ComponentSignature::Signature(Signature::simple(
                        RustType::in_scope("Self"),
                    ))),
            );
        fb = fb.add_impl(impl_body);
    }
    fb
}
