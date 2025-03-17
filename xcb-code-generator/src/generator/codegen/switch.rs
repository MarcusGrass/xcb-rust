use std::cell::RefCell;
use std::fmt::Write;
use std::ops::Deref;
use std::rc::Rc;

use codegen_rs::structures::gen_enum::NamedComponentSignature;
use codegen_rs::structures::method::Argument;
use codegen_rs::structures::visibility::Visibility;
use codegen_rs::structures::{ComponentSignature, Ownership, RustType, Signature};
use codegen_rs::{EnumBuilder, FileBuilder, ImplBuilder, MethodBuilder, RustCase, StructBuilder};

use crate::generator::codegen::functions::{
    bitmask_enum_case_name, fields_as_args, from_bytes_length_expr, get_sorted_required_by_members,
    get_sorted_required_fields, member_size,
};
use crate::generator::codegen::plain_struct::implement_plain;
use crate::generator::codegen::{
    AsRustFormatted, BYTES_VEC_SER_FUNCTION, BYTE_VEC_FROM_BYTES, FIX_LEN_FROM_BYTES_MACRO,
    FIX_LEN_SER_FUNCTION, FROM_BYTES_ERROR, RESULT, SERIALIZE_ERROR, VAR_LEN_FROM_BYTES_MACRO,
    VAR_LEN_SER_FUNCTION,
};
use crate::generator::types::{
    Constructed, EntityField, EntityMember, FieldNumberRef, SourceType, SwitchEnum, SwitchStruct,
    TypeHelpers, WrappedType, XcbExpression, XcbType,
};
use crate::{dump, Xcb};

pub(crate) fn write_mask_expr(expr: &XcbExpression) -> String {
    let mut out = String::new();
    match expr {
        XcbExpression::Binop(b) => {
            let _ = out.write_fmt(format_args!(
                "{}({}, {})",
                b.op.into_op_expr(false),
                write_mask_expr(b.left.as_ref()),
                write_mask_expr(b.right.as_ref()),
            ));
        }
        XcbExpression::Unop(u) => {
            let _ = out.write_fmt(format_args!(
                "{}({})",
                u.op.into_op_expr(false),
                write_mask_expr(u.tgt.as_ref())
            ));
        }
        XcbExpression::Fieldref(f) => {
            out.push_str(&f.name.to_rust_snake());
        }
        XcbExpression::Paramref(pr) => {
            out.push_str(&pr.name.to_rust_snake());
        }
        XcbExpression::Popcount(pc) => {
            let _ = out.write_fmt(format_args!(
                "{}.count_ones()",
                write_mask_expr(pc.as_ref())
            ));
        }
        XcbExpression::Sumof(so) => {
            assert!(so.expr.is_some(), "Sum of in switch");
            dump!(out, "{} as usize", so.entity.name.to_rust_snake());
        }
        XcbExpression::ListLengthExpr => {
            dump!(out, ".len()");
        }
        XcbExpression::ListForeignFieldRef(_) => {
            dump!(out, "{} as usize");
        }
        XcbExpression::Value(v) => {
            let _ = out.write_fmt(format_args!("{v}"));
        }
        XcbExpression::Enumref(_) | XcbExpression::ListelementRef | XcbExpression::Bit(_) => {}
    }
    out
}

#[derive(Debug, Clone)]
pub(crate) struct SwitchMember {
    pub(crate) em: EntityMember,
    pub(crate) enum_ref: String,
}

impl SwitchMember {
    fn new(em: EntityMember, enum_ref: String) -> Self {
        Self { em, enum_ref }
    }
}

pub(crate) fn implement_switch_struct(
    ss: &SwitchStruct,
    xcb: &Xcb,
    mut fb: FileBuilder,
) -> FileBuilder {
    let mut sb = StructBuilder::new(ss.rust_entity_name())
        .set_visibility(Visibility::Public)
        .add_derive_in_scope("Default")
        .add_derive_in_scope("Debug")
        .add_derive_in_scope("Clone")
        .add_derive_in_scope("PartialEq");
    let mut case_no = 0;

    let mut copy_type = true;
    let mut members = vec![];
    let effective = ss.source_type.resolve_effective_source(ss.used_by.clone());
    for case in &ss.bit_cases {
        let enum_ref = case
            .expressions
            .iter()
            .find_map(|c| {
                if let XcbExpression::Enumref(eref) = c {
                    Some(format!(
                        "{}::{}",
                        eref.kind.concrete.import_name(&xcb.header),
                        RustCase::convert_to_valid_rust(&eref.name, RustCase::Scream).unwrap()
                    ))
                } else {
                    None
                }
            })
            .unwrap();
        if case.switches.is_empty() {
            let num_members = case.fields.iter().filter(|f| f.is_constructable()).count();
            // Just unpack into the parent switch instead of creating a sub-struct
            if num_members == 1 {
                members.extend(
                    case.fields
                        .clone()
                        .into_iter()
                        .map(|ef| SwitchMember::new(ef, enum_ref.clone())),
                );
                continue;
            }
        }
        let name = if let Some(cn) = &case.name {
            RustCase::convert_to_valid_rust(cn, RustCase::Pascal).unwrap()
        } else {
            let out = format!("{}BitCase{case_no}", ss.rust_entity_name());
            case_no += 1;
            out
        };

        let required_args = get_sorted_required_by_members(&case.fields);
        //eprintln!("{} effective :{:?}", name, effective);
        fb = implement_plain(
            RustCase::convert_to_valid_rust(&name, RustCase::Pascal).unwrap(),
            &case.fields,
            &case.switches,
            effective,
            false,
            xcb,
            fb,
        );
        let fixed = case
            .switches
            .is_empty()
            .then(|| member_size(&case.fields))
            .flatten();
        if fixed.is_none() {
            copy_type = false;
        }
        members.push(SwitchMember::new(
            EntityMember::Field(EntityField {
                name: name.clone(),
                kind: FieldNumberRef {
                    concrete: Rc::new(RefCell::new(XcbType::Constructed(Constructed {
                        name,
                        byte_size: fixed,
                        required_args,
                        source_type: effective,
                    }))),
                    reference_type: None,
                },
            }),
            enum_ref,
        ));
    }

    let mut builder_methods = Vec::new();
    for field in members.iter().filter(|m| m.em.is_constructable()) {
        sb = sb.add_field(
            Visibility::Public,
            NamedComponentSignature::new(
                field.em.rust_field_name(),
                ComponentSignature::Signature(Signature::simple(RustType::in_scope(format!(
                    "Option<{}>",
                    field.em.import_name(&xcb.header)
                )))),
            ),
        );
        builder_methods.push(
            MethodBuilder::new(field.em.rust_field_name())
                .set_visibility(Visibility::Public)
                .add_simple_annotation("inline")
                .add_argument_in_scope_simple_type(
                    Ownership::Owned,
                    field.em.rust_field_name(),
                    field.em.import_name(&xcb.header),
                )
                .set_self_ownership(Ownership::OwnedMut)
                .set_return_type(ComponentSignature::Signature(Signature::simple(
                    RustType::in_scope("Self"),
                )))
                .set_body(format!(
                    "self.{} = Some({});\nself",
                    field.em.rust_field_name(),
                    field.em.rust_field_name()
                )),
        );
    }
    if copy_type {
        sb = sb.add_derive_in_scope("Copy");
    }
    let mut imp = impl_switch(&sb.name, &ss.switch_expr, &members, effective, xcb);
    for bm in builder_methods {
        imp = imp.add_method(bm);
    }
    fb.add_struct(sb).add_impl(imp)
}

pub(crate) fn find_switch_enum_ref_type(switch_struct: &SwitchStruct) -> WrappedType {
    let mut e = None;
    for case in &switch_struct.bit_cases {
        let enum_ref = case
            .expressions
            .iter()
            .find_map(|c| {
                if let XcbExpression::Enumref(eref) = c {
                    Some(eref.kind.concrete.clone())
                } else {
                    None
                }
            })
            .unwrap();
        e = Some(enum_ref);
    }
    e.unwrap()
}

fn impl_switch(
    name: &str,
    expr: &XcbExpression,
    members: &[SwitchMember],
    effective_type: SourceType,
    xcb: &Xcb,
) -> ImplBuilder {
    let mut imp = ImplBuilder::new(Signature::simple(RustType::in_scope(name)));
    let mut req_fields = expr.required_fields();
    let mut from_bytes_builder = MethodBuilder::new("from_bytes")
        .set_visibility(Visibility::Public)
        .add_simple_annotation("inline")
        .add_argument(Argument::new(
            Ownership::Ref,
            NamedComponentSignature::new(
                "buf",
                ComponentSignature::Signature(Signature::simple(RustType::in_scope("[u8]"))),
            ),
        ))
        .set_return_type(ComponentSignature::Signature(Signature::simple(
            RustType::in_scope(format!("{RESULT}<(Self, usize)>")),
        )));
    let serialize_into_builder = MethodBuilder::new("serialize_into")
        .set_visibility(Visibility::Public)
        .add_simple_annotation("inline")
        .set_self_ownership(Ownership::Owned)
        .add_argument(Argument::new(
            Ownership::MutRef,
            NamedComponentSignature::new(
                "buf",
                ComponentSignature::Signature(Signature::simple(RustType::in_scope("[u8]"))),
            ),
        ))
        .set_return_type(ComponentSignature::Signature(Signature::simple(
            RustType::in_scope(format!("{RESULT}<usize>")),
        )));
    let mut serialize_into_body = String::new();
    let mut switch_expr_builder = MethodBuilder::new("switch_expr")
        .set_visibility(Visibility::Public)
        .add_simple_annotation("inline")
        .add_simple_annotation("must_use")
        .set_self_ownership(Ownership::Ref);
    let mut switch_expr_body = String::new();
    dump!(
        serialize_into_body,
        "let mut offset = 0;\nlet buf_ptr = buf;\n"
    );
    let expr = write_mask_expr(expr);
    let mut from_bytes_body = String::new();
    let _ = from_bytes_body.write_fmt(format_args!(
        "let mask = {expr};\nlet mut offset = 0;\nlet mut slf = Self::default();\nlet buf_ptr = buf;\n"
    ));
    let mut switch_expr_type = None;
    for member in members {
        match &member.em {
            EntityMember::Field(f) => {
                if let XcbType::Constructed(c) = &*f.kind.concrete.deref().borrow() {
                    req_fields.extend(c.required_args.clone());
                }
                let append = if f.kind.reference_type.is_some() {
                    ".into()"
                } else {
                    ""
                };
                let use_type = f.kind.use_field().import_name(&xcb.header);
                let fname = f.name.to_rust_snake();
                dump!(
                    switch_expr_body,
                    "if self.{}.is_some() {{ mask |= {};}}\n",
                    fname,
                    member.enum_ref
                );
                switch_expr_type = Some(member.enum_ref.clone());
                dump!(
                    serialize_into_body,
                    "if let Some({}) = self.{} {{\n",
                    fname,
                    fname
                );
                if let Some(sz) = f.kind.concrete.byte_size() {
                    dump!(
                        serialize_into_body,
                        "buf_ptr.get_mut(offset..offset + {}).ok_or({SERIALIZE_ERROR})?.copy_from_slice(&{}.serialize_fixed());\n",
                        sz,
                        fname
                    );
                    dump!(serialize_into_body, "offset += {};\n", sz);
                    let _ = from_bytes_body
                        .write_fmt(format_args!("if mask & {}.0 != 0 {{\n", member.enum_ref));
                    let _ = from_bytes_body.write_fmt(format_args!(
                        "slf.{} = Some({}::from_bytes(buf_ptr.get(offset..).ok_or({FROM_BYTES_ERROR})?)?{});\n",
                        f.name.to_rust_snake(),
                        f.kind.concrete.import_name(&xcb.header),
                        append
                    ));
                    let _ = from_bytes_body.write_fmt(format_args!("offset += {sz};\n}}\n"));
                } else {
                    dump!(
                        serialize_into_body,
                        "offset += {}.serialize_into(buf_ptr.get_mut(offset..).ok_or({SERIALIZE_ERROR})?)?;\n",
                        fname
                    );
                    let _ = from_bytes_body
                        .write_fmt(format_args!("if mask & {}.0 != 0 {{\n", member.enum_ref));
                    let req = member.em.required_args();
                    let req = get_sorted_required_fields(&req);
                    dump!(
                        from_bytes_body,
                        "let ({}, new_offset) = {}::from_bytes(buf_ptr.get(offset..).ok_or({FROM_BYTES_ERROR})?, \n",
                        f.name.to_rust_snake(),
                        use_type
                    );
                    for req in req {
                        dump!(from_bytes_body, "{},\n", req.name.to_rust_snake());
                    }
                    dump!(from_bytes_body, ")?;\n");
                    from_bytes_body.push_str("offset += new_offset;\n");
                    let _ = from_bytes_body.write_fmt(format_args!(
                        "slf.{} = Some({}{});\n}}\n",
                        f.name.to_rust_snake(),
                        f.name.to_rust_snake(),
                        append
                    ));
                }
                dump!(serialize_into_body, "}\n");
            }
            EntityMember::List(l) => {
                let fname = l.field.name.to_rust_snake();
                let use_type = l.field.kind.use_field().import_name(&xcb.header);
                dump!(
                    serialize_into_body,
                    "if let Some({}) = self.{} {{\n",
                    fname,
                    fname
                );
                if let Some(sz) = l.field.kind.concrete.byte_size() {
                    dump!(
                        serialize_into_body,
                        "let out_len = {}.len() * {};\n",
                        fname,
                        sz
                    );
                    if sz == 1 {
                        dump!(
                            serialize_into_body,
                            "{}(buf_ptr.get_mut(offset..).ok_or({})?, &{})?;\n",
                            BYTES_VEC_SER_FUNCTION,
                            SERIALIZE_ERROR,
                            fname
                        );
                    } else {
                        dump!(
                            serialize_into_body,
                            "{}(buf_ptr.get_mut(offset..).ok_or({})?, {})?;\n",
                            FIX_LEN_SER_FUNCTION,
                            SERIALIZE_ERROR,
                            fname
                        );
                    }

                    dump!(serialize_into_body, "offset += out_len;\n");
                    let _ = from_bytes_body
                        .write_fmt(format_args!("if mask & {}.0 != 0 {{\n", member.enum_ref));
                    let expr = l.length_expr.as_ref().unwrap();
                    req_fields.extend(expr.required_fields());
                    let expr = from_bytes_length_expr(expr, false, false);
                    let _ =
                        from_bytes_body.write_fmt(format_args!("let length = {expr} as usize;\n"));
                    if sz == 1
                        && (l.field.kind.use_field().is_builtin()
                            || l.field.kind.use_field().is_builtin_alias())
                    {
                        let _ = from_bytes_body.write_fmt(format_args!(
                            "slf.{fname} = Some({BYTE_VEC_FROM_BYTES}(buf_ptr.get(offset..).ok_or({FROM_BYTES_ERROR})?, length)?);\n"
                        ));
                    } else {
                        let _ = from_bytes_body.write_fmt(format_args!(
                            "slf.{fname} = Some({FIX_LEN_FROM_BYTES_MACRO}(buf_ptr.get(offset..).ok_or({FROM_BYTES_ERROR})?, {use_type}, length, {sz}));\n"
                        ));
                    }

                    from_bytes_body.push_str("offset += length;\n}\n");
                } else {
                    dump!(
                        serialize_into_body,
                        "offset += {}(buf_ptr.get_mut(offset..).ok_or({})?, {})?;\n",
                        VAR_LEN_SER_FUNCTION,
                        SERIALIZE_ERROR,
                        fname
                    );
                    let expr = l.length_expr.as_ref().unwrap();
                    req_fields.extend(expr.required_fields());
                    let expr = from_bytes_length_expr(expr, false, false);
                    let _ = from_bytes_body
                        .write_fmt(format_args!("if mask & {}.0 != 0 {{\n", member.enum_ref));
                    let _ =
                        from_bytes_body.write_fmt(format_args!("let length = {expr} as usize;\n"));
                    let _ = from_bytes_body.write_fmt(format_args!(
                        "let {} = {}(buf_ptr.get(offset..).ok_or({})?, {}, offset, length);\n",
                        l.field.name.to_rust_snake(),
                        VAR_LEN_FROM_BYTES_MACRO,
                        FROM_BYTES_ERROR,
                        use_type,
                    ));
                    let _ = from_bytes_body.write_fmt(format_args!(
                        "slf.{} = Some({});\n}}\n",
                        l.field.name.to_rust_snake(),
                        l.field.name.to_rust_snake()
                    ));
                }
                dump!(serialize_into_body, "}\n");
            }
            _ => {}
        }
    }
    let switch_expr_raw = switch_expr_type.unwrap();
    let switch_type = switch_expr_raw.rsplit_once("::").unwrap().0;
    let switch_expr_body =
        format!("let mut mask = {switch_type}::default();\n{switch_expr_body}mask");
    switch_expr_builder = switch_expr_builder
        .set_body(switch_expr_body)
        .set_return_type(ComponentSignature::Signature(Signature::simple(
            RustType::in_scope(switch_type),
        )));
    for arg in fields_as_args(&req_fields, &xcb.header) {
        from_bytes_builder = from_bytes_builder.add_argument(arg);
    }
    from_bytes_body.push_str("Ok((slf, offset))");
    dump!(serialize_into_body, "Ok(offset)");
    if effective_type.should_serialize() {
        imp = imp.add_method(serialize_into_builder.set_body(serialize_into_body));
    }
    if effective_type.should_deserialize() {
        imp = imp.add_method(from_bytes_builder.set_body(from_bytes_body));
    }
    imp.add_method(switch_expr_builder)
}

pub(crate) struct SwitchEnumMember {
    name: String,
    enum_ref: String,
    fixed_bytes: Option<usize>,
    required_arg: Option<EntityField>,
}

pub(crate) fn implement_switch_enum(
    ss: &SwitchEnum,
    xcb: &Xcb,
    mut fb: FileBuilder,
) -> FileBuilder {
    let effective = ss.source_type.resolve_effective_source(ss.used_by.clone());
    let mut eb = EnumBuilder::new(ss.rust_entity_name())
        .set_visibility(Visibility::Public)
        .add_derive_in_scope("Debug")
        .add_derive_in_scope("Clone")
        .add_derive_in_scope("PartialEq");
    let mut i = 0;
    let mut members = vec![];
    let mut required: Option<EntityField> = None;
    for case in &ss.enum_cases {
        // One or no args, same for all cases
        let req = get_sorted_required_by_members(&case.fields);
        assert!(req.is_empty() || req.len() == 1);
        if req.len() == 1 {
            assert!(required.is_none() || required.unwrap().name == req[0].name);
            required = Some(req[0].clone());
        }
    }
    for case in &ss.enum_cases {
        let eref = case
            .expressions
            .iter()
            .find_map(|expr| {
                if let XcbExpression::Enumref(ef) = expr {
                    Some(format!(
                        "{}::{}.0",
                        ef.kind.concrete.import_name(&xcb.header),
                        bitmask_enum_case_name(&ef.name)
                    ))
                } else {
                    None
                }
            })
            .unwrap();
        let name = case.name.clone().map_or_else(
            || {
                let out = format!("{}AnonCase{i}", ss.rust_entity_name());
                i += 1;
                out
            },
            |n| {
                format!(
                    "{}{}",
                    ss.rust_entity_name(),
                    RustCase::convert_to_valid_rust(&n, RustCase::Pascal).unwrap()
                )
            },
        );
        let fixed = member_size(&case.fields);
        members.push(SwitchEnumMember {
            name: name.clone(),
            enum_ref: eref,
            fixed_bytes: fixed,
            required_arg: required.clone(),
        });
        fb = implement_plain(
            name.clone(),
            &case.fields,
            &case.switches,
            effective,
            false,
            xcb,
            fb,
        );
        eb = eb.add_type_member(&name, Signature::simple(RustType::in_scope(&name)));
        //sb = add_fields(sb, &case.fields, Some(option.clone()), xcb);
    }

    eb = eb.add_tag_member("BadValue");
    let mut from_bytes_builder = MethodBuilder::new("from_bytes")
        .set_visibility(Visibility::Public)
        .set_return_type(ComponentSignature::Signature(Signature::simple(
            RustType::in_scope(format!("{RESULT}<(Self, usize)>")),
        )))
        .add_argument_in_scope_simple_type(Ownership::Ref, "bytes", "[u8]");

    let mut serialize_body = String::new();
    let mut from_bytes_body = String::new();

    let args = fields_as_args(&ss.switch_expr.required_fields(), &xcb.header);

    dump!(
        from_bytes_body,
        "let mask = {};\n",
        write_mask_expr(&ss.switch_expr)
    );
    dump!(serialize_body, "let buf_ptr = buf;\n");
    dump!(serialize_body, "match self {\n");
    for member in &members {
        let from_bytes_call = if let Some(req) = &member.required_arg {
            format!("from_bytes(bytes, {})?", req.name.to_rust_snake())
        } else {
            "from_bytes(bytes)?".to_string()
        };
        if let Some(bytes) = member.fixed_bytes {
            dump!(
                serialize_body,
                "Self::{}(case) => {{\n buf_ptr.get_mut(..{bytes}).ok_or({SERIALIZE_ERROR})?.copy_from_slice(&case.serialize_fixed());\n Ok({bytes})\n}}\n",
                member.name
            );
            dump!(
                from_bytes_body,
                "if mask & {} == 0 {{\n return Ok((Self::{}({}::{from_bytes_call}), {bytes})); }}\n",
                member.enum_ref,
                member.name,
                member.name
            );
        } else {
            dump!(
                serialize_body,
                "Self::{}(case) => {{\n case.serialize_into(buf_ptr)\n}}\n",
                member.name
            );
            dump!(
                from_bytes_body,
                "if mask & {} == 0 {{\n let (parsed, offset) = {}::{from_bytes_call};\n return Ok((Self::{}(parsed), offset)); }}\n",
                member.enum_ref,
                member.name,
                member.name
            );
        }
    }
    dump!(serialize_body, "Self::BadValue => Ok(0)\n");
    dump!(serialize_body, "}\n");
    let serialize_builder = MethodBuilder::new("serialize_into").set_visibility(Visibility::Public);
    for arg in args {
        from_bytes_builder = from_bytes_builder.add_argument(arg);
    }
    dump!(from_bytes_body, "Ok((Self::BadValue, 0))");
    if let Some(req) = required {
        let arg = fields_as_args(&[req], &xcb.header);
        from_bytes_builder = from_bytes_builder.add_argument(arg[0].clone());
    }
    let from_bytes = ImplBuilder::new(Signature::simple(RustType::in_scope(&eb.name)))
        .add_method(from_bytes_builder.set_body(from_bytes_body));
    let serialize = ImplBuilder::new(Signature::simple(RustType::in_scope(&eb.name))).add_method(
        serialize_builder
            .set_body(serialize_body)
            .add_argument_in_scope_simple_type(Ownership::MutRef, "buf", "[u8]")
            .set_return_type(ComponentSignature::Signature(Signature::simple(
                RustType::in_scope(format!("{RESULT}<usize>")),
            )))
            .set_self_ownership(Ownership::Owned),
    );

    fb = fb.add_enum(eb);
    if effective.should_serialize() {
        fb = fb.add_impl(serialize);
    }
    if effective.should_deserialize() {
        fb = fb.add_impl(from_bytes);
    }
    fb
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        let affect_which = 8;
        let clear = 8;
        let select_all = 8;
        let res = core::ops::BitAnd::bitand(
            affect_which,
            core::ops::BitAnd::bitand(core::ops::Not::not(clear), core::ops::Not::not(select_all)),
        );
        eprintln!("{res}");
    }
}
