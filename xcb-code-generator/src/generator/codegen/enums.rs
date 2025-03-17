use std::cell::RefCell;
use std::collections::HashSet;
use std::ops::Deref;
use std::rc::Rc;

use codegen_rs::structures::gen_enum::NamedComponentSignature;
use codegen_rs::structures::method::Argument;
use codegen_rs::structures::visibility::Visibility;
use codegen_rs::structures::{ComponentSignature, Ownership, RustType, Signature};
use codegen_rs::{
    ConstantBuilder, ContainerStructBuilder, FileBuilder, ImplBuilder, MethodBuilder, RustCase,
};

use crate::generator::codegen::functions::bitmask_enum_case_name;
use crate::generator::codegen::xcb::implement_from_uints;
use crate::generator::types::{BitmaskEnum, TypeHelpers, ValueEnum, WrappedType, XcbType};
use crate::{Xcb, FIX_LEN_FROM_BYTES, FIX_LEN_SERIALIZE};

pub(crate) fn implement_bitmask_enum(be: &BitmaskEnum, mut fb: FileBuilder) -> FileBuilder {
    let rust_name = RustCase::convert_to_valid_rust(&be.name, RustCase::Pascal).unwrap();
    let mut bm = ContainerStructBuilder::new(&rust_name)
        .set_visibility(Visibility::Public)
        .add_derive_in_scope("Debug")
        .add_derive_in_scope("Default")
        .add_derive_in_scope("Copy")
        .add_derive_in_scope("Clone")
        .add_derive_in_scope("Eq")
        .add_derive_in_scope("PartialEq");
    if be.concrete_number_types.deref().borrow().is_empty() {
        //eprintln!("Skipping unreferenced enum {}", be.name);
        return fb;
    }
    let (largest, _) = find_largest_concrete_builtins(be.concrete_number_types.clone());
    if let XcbType::Builtin(xc) = &*largest.deref().borrow() {
        bm = bm.add_contained(Visibility::Public, RustType::in_scope(xc.rust_name()));
    } else {
        panic!("Largest type not xcb builtin {largest:?}");
    }

    let byte_size = largest.byte_size().unwrap();
    let bytes = format!("[u8; {byte_size}]");
    let fix_len_ser_impl = ImplBuilder::new(Signature::simple(RustType::in_scope(&rust_name)))
        .implement_for(Signature::simple(RustType::in_scope(format!(
            "{FIX_LEN_SERIALIZE}<{byte_size}>",
        ))))
        .add_method(
            MethodBuilder::new("serialize_fixed")
                .add_simple_annotation("inline")
                .set_self_ownership(Ownership::Owned)
                .set_visibility(Visibility::Public)
                .set_body("self.0.serialize_fixed()")
                .set_return_type(ComponentSignature::Signature(Signature::simple(
                    RustType::in_scope(bytes),
                ))),
        );
    let fix_len_from_bytes_impl =
        ImplBuilder::new(Signature::simple(RustType::in_scope(&rust_name)))
            .implement_for(Signature::simple(RustType::in_scope(format!(
                "{FIX_LEN_FROM_BYTES}<{byte_size}>",
            ))))
            .add_method(
                MethodBuilder::new("from_bytes")
                    .add_simple_annotation("inline")
                    .add_argument(Argument::new(
                        Ownership::Ref,
                        NamedComponentSignature::new_simple_type(
                            "bytes",
                            RustType::in_scope("[u8]"),
                        ),
                    ))
                    .set_visibility(Visibility::Public)
                    .set_body(format!(
                        "Ok(Self({}::from_bytes(bytes)?))",
                        largest.rust_entity_name()
                    ))
                    .set_return_type(ComponentSignature::Signature(Signature::simple(
                        RustType::in_scope("crate::error::Result<Self>"),
                    ))),
            );
    let mut case_impl = ImplBuilder::new(Signature::simple(RustType::in_scope(&rust_name)));
    if let Some(def) = be.default.as_ref() {
        case_impl = case_impl.add_const(
            ConstantBuilder::const_builder(
                RustCase::convert_to_valid_rust(&def.key, RustCase::Scream).unwrap(),
                RustType::in_scope("Self"),
                format!("Self({})", def.value),
            )
            .set_visibility(Visibility::Public),
        );
    }
    for case in &be.kvs {
        let k = bitmask_enum_case_name(&case.key);
        case_impl = case_impl.add_const(
            ConstantBuilder::const_builder(
                &k,
                RustType::in_scope("Self"),
                format!("Self(1 << {})", case.value),
            )
            .set_visibility(Visibility::Public),
        );
    }
    fb = fb
        .add_container_struct(bm)
        .add_impl(case_impl)
        .add_impl(fix_len_ser_impl)
        .add_impl(fix_len_from_bytes_impl);
    let wrap_with = if largest.is_builtin_alias() {
        None
    } else {
        Some(largest.clone())
    };
    if !largest.is_builtin() && !largest.is_builtin_alias() {
        fb = fb.add_impl(
            ImplBuilder::new(Signature::simple(RustType::in_scope(&rust_name)))
                .implement_for(Signature::simple(RustType::in_scope(format!(
                    "From<{}>",
                    largest.rust_entity_name()
                ))))
                .add_method(
                    MethodBuilder::new("from")
                        .add_argument(Argument::new(
                            Ownership::Owned,
                            NamedComponentSignature::new(
                                "val",
                                ComponentSignature::Signature(Signature::simple(
                                    RustType::in_scope(largest.rust_entity_name()),
                                )),
                            ),
                        ))
                        .set_return_type(ComponentSignature::Signature(Signature::simple(
                            RustType::in_scope("Self"),
                        )))
                        .set_body("Self(val)"),
                ),
        );
    }
    fb = implement_from_uints(
        &rust_name,
        largest.byte_size().unwrap() * 8,
        32,
        wrap_with,
        fb,
    );
    fb = fb.add_any(format!("crate::implement_bit_ops!({rust_name});\n"));
    fb
}

pub(crate) fn find_largest_concrete_builtins(
    input: Rc<RefCell<Vec<WrappedType>>>,
) -> (WrappedType, Option<WrappedType>) {
    let all = input
        .deref()
        .borrow()
        .iter()
        .cloned()
        .collect::<Vec<WrappedType>>();
    if all.len() > 1 {
        let res = all
            .iter()
            .filter(|w| w.is_builtin())
            .max_by(|a, b| a.byte_size().unwrap().cmp(&b.byte_size().unwrap()));
        if let Some(res) = res {
            let mut unique_names = HashSet::new();
            return (
                res.clone(),
                all.into_iter().filter(|w| !w.is_builtin()).find(|w| {
                    if unique_names.contains(&w.rust_entity_name()) {
                        false
                    } else {
                        unique_names.insert(w.rust_entity_name());
                        true
                    }
                }),
            );
        }
    }
    let mut unique_names = HashSet::new();
    let max = all
        .iter()
        .max_by(|a, b| a.byte_size().unwrap().cmp(&b.byte_size().unwrap()))
        .unwrap();
    (
        max.clone(),
        all.into_iter().find(|w| {
            if unique_names.contains(&w.rust_entity_name()) {
                false
            } else {
                unique_names.insert(w.rust_entity_name());
                true
            }
        }),
    )
}

pub(crate) fn implement_value_enum(ve: &ValueEnum, xcb: &Xcb, mut fb: FileBuilder) -> FileBuilder {
    let rust_name = ve.rust_name();
    let mut bm = ContainerStructBuilder::new(&rust_name)
        .set_visibility(Visibility::Public)
        .add_derive_in_scope("Debug")
        .add_derive_in_scope("Default")
        .add_derive_in_scope("Copy")
        .add_derive_in_scope("Clone")
        .add_derive_in_scope("PartialEq");
    if ve.concrete_number_types.deref().borrow().is_empty() {
        //eprintln!("Skipping unreferenced enum {}", ve.name);
        return fb;
    }
    let (largest, rest) = find_largest_concrete_builtins(ve.concrete_number_types.clone());
    let largest = if let Some(other) = rest {
        if other.byte_size().unwrap() >= largest.byte_size().unwrap() {
            other
        } else {
            largest
        }
    } else {
        largest
    };
    let largest_import_name = largest.borrow().deref().import_name(&xcb.header);
    bm = bm.add_contained(Visibility::Public, RustType::in_scope(&largest_import_name));
    let byte_size = largest.byte_size().unwrap();
    let bytes = format!("[u8; {byte_size}]");
    let fix_len_ser_impl = ImplBuilder::new(Signature::simple(RustType::in_scope(&rust_name)))
        .implement_for(Signature::simple(RustType::in_scope(format!(
            "{FIX_LEN_SERIALIZE}<{byte_size}>"
        ))))
        .add_method(
            MethodBuilder::new("serialize_fixed")
                .add_simple_annotation("inline")
                .set_self_ownership(Ownership::Owned)
                .set_visibility(Visibility::Public)
                .set_body("self.0.serialize_fixed()")
                .set_return_type(ComponentSignature::Signature(Signature::simple(
                    RustType::in_scope(bytes),
                ))),
        );
    let fix_len_from_bytes_impl =
        ImplBuilder::new(Signature::simple(RustType::in_scope(&rust_name)))
            .implement_for(Signature::simple(RustType::in_scope(format!(
                "{FIX_LEN_FROM_BYTES}<{byte_size}>"
            ))))
            .add_method(
                MethodBuilder::new("from_bytes")
                    .add_simple_annotation("inline")
                    .add_argument(Argument::new(
                        Ownership::Ref,
                        NamedComponentSignature::new_simple_type(
                            "bytes",
                            RustType::in_scope("[u8]"),
                        ),
                    ))
                    .set_visibility(Visibility::Public)
                    .set_body(format!(
                        "Ok(Self({}::from_bytes(bytes)?))",
                        largest.rust_entity_name()
                    ))
                    .set_return_type(ComponentSignature::Signature(Signature::simple(
                        RustType::in_scope("crate::error::Result<Self>"),
                    ))),
            );
    let mut case_builder = ImplBuilder::new(Signature::simple(RustType::in_scope(&rust_name)));
    for kv in &ve.kvs {
        let k = bitmask_enum_case_name(&kv.key);
        let val = format!("Self({})", kv.value);
        case_builder = case_builder.add_const(
            ConstantBuilder::const_builder(&k, RustType::in_scope("Self"), &val)
                .set_visibility(Visibility::Public),
        );
    }

    fb = fb
        .add_container_struct(bm)
        .add_impl(case_builder)
        .add_impl(fix_len_ser_impl)
        .add_impl(fix_len_from_bytes_impl);
    let wrap_with = if largest.is_builtin_alias() {
        None
    } else {
        Some(largest.clone())
    };

    if !largest.is_builtin() && !largest.is_builtin_alias() && largest.byte_size().unwrap() != 4 {
        fb = fb.add_impl(
            ImplBuilder::new(Signature::simple(RustType::in_scope(&rust_name)))
                .implement_for(Signature::simple(RustType::in_scope(format!(
                    "From<{}>",
                    largest.rust_entity_name()
                ))))
                .add_method(
                    MethodBuilder::new("from")
                        .add_argument(Argument::new(
                            Ownership::Owned,
                            NamedComponentSignature::new(
                                "val",
                                ComponentSignature::Signature(Signature::simple(
                                    RustType::in_scope(largest.rust_entity_name()),
                                )),
                            ),
                        ))
                        .set_return_type(ComponentSignature::Signature(Signature::simple(
                            RustType::in_scope("Self"),
                        )))
                        .set_body("Self(val)"),
                ),
        );
    }
    fb = implement_from_uints(&rust_name, byte_size * 8, 32, wrap_with, fb);
    fb
}
