use std::collections::{HashMap, HashSet};
use std::fmt::Write;
use std::ops::Deref;

use codegen_rs::structures::gen_enum::NamedComponentSignature;
use codegen_rs::structures::method::Argument;
use codegen_rs::structures::visibility::Visibility;
use codegen_rs::structures::{ComponentSignature, Ownership, RustType, Signature};
use codegen_rs::{FileBuilder, ImplBuilder, MethodBuilder, StructBuilder};

use crate::generator::codegen::functions::{
    add_fields, fields_as_args, find_derive_fields, from_bytes_length_expr,
    get_sorted_required_by_members, get_sorted_required_fields, member_size,
    reduce_required_from_container, serialize_length_expr,
};
use crate::generator::codegen::{
    AsRustFormatted, BYTES_VEC_SER_FUNCTION, BYTE_VEC_FROM_BYTES, FIX_LEN_FROM_BYTES_MACRO,
    FIX_LEN_SER_FUNCTION, FROM_BYTES_ERROR, RESULT, SERIALIZE_ERROR, VAR_LEN_FROM_BYTES,
    VAR_LEN_FROM_BYTES_MACRO, VAR_LEN_SERIALIZE, VAR_LEN_SER_FUNCTION,
};
use crate::generator::types::{
    EntityField, EntityMember, SourceType, TypeHelpers, WrappedType, XcbBuiltin, XcbExpression,
    XcbStruct, XcbType,
};
use crate::{dump, Xcb, FIX_LEN_FROM_BYTES, FIX_LEN_SERIALIZE};

pub(crate) fn implement_plain_struct(s: &XcbStruct, xcb: &Xcb, fb: FileBuilder) -> FileBuilder {
    implement_plain(
        s.rust_entity_name(),
        &s.members,
        &s.switch.clone().into_iter().collect::<Vec<WrappedType>>(),
        s.source_type.resolve_effective_source(s.used_by.clone()),
        false,
        xcb,
        fb,
    )
}

pub(crate) fn implement_plain(
    rust_name: String,
    members: &[EntityMember],
    switches: &[WrappedType],
    source_type: SourceType,
    exclude: bool,
    xcb: &Xcb,
    mut fb: FileBuilder,
) -> FileBuilder {
    let mut sb = StructBuilder::new(rust_name.clone())
        .set_visibility(Visibility::Public)
        .add_derive_in_scope("Debug")
        .add_derive_in_scope("Clone")
        .add_derive_in_scope("PartialEq");
    let excl_names = if !exclude {
        let excl = find_derive_fields(members);
        excl.iter()
            .map(|e| e.name.clone())
            .collect::<HashSet<String>>()
    } else {
        HashSet::new()
    };

    sb = add_fields(sb, members, None, &excl_names, xcb);
    let fixed_size = switches.is_empty().then(|| member_size(members)).flatten();
    let mut impls = vec![];
    if let Some(bytes) = fixed_size {
        if bytes == 130 {
            for member in members {
                eprintln!(
                    "{}: {}",
                    if member.is_constructable() {
                        format!(
                            "{}: {}, sz",
                            member.rust_field_name(),
                            member.rust_entity_name()
                        )
                    } else {
                        "No construct.".to_string()
                    },
                    member.byte_size().unwrap()
                );
            }
            panic!("")
        }
        sb = sb.add_derive_in_scope("Copy");
        if source_type.should_serialize() {
            let fix_len_ser = ImplBuilder::new(Signature::simple(RustType::in_scope(&rust_name)))
                .implement_for(Signature::simple(RustType::in_scope(format!(
                    "{}<{}>",
                    FIX_LEN_SERIALIZE, bytes
                ))))
                .add_method(
                    MethodBuilder::new("serialize_fixed")
                        .set_body(fixed_members_serialize_impl(members))
                        .add_simple_annotation("inline")
                        .set_self_ownership(Ownership::Owned)
                        .set_return_type(ComponentSignature::Signature(Signature::simple(
                            RustType::in_scope(format!("[u8; {}]", bytes)),
                        ))),
                );
            impls.push(fix_len_ser);
        }
        if source_type.should_deserialize() {
            let fix_len_from_bytes =
                ImplBuilder::new(Signature::simple(RustType::in_scope(&rust_name)))
                    .implement_for(Signature::simple(RustType::in_scope(format!(
                        "{}<{}>",
                        FIX_LEN_FROM_BYTES, bytes
                    ))))
                    .add_method(
                        MethodBuilder::new("from_bytes")
                            .add_argument(Argument::new(
                                Ownership::Ref,
                                NamedComponentSignature::new_simple_type(
                                    "bytes",
                                    RustType::in_scope("[u8]"),
                                ),
                            ))
                            .add_simple_annotation("inline")
                            .set_body(fixed_members_parse_impl(&rust_name, members, xcb))
                            .set_return_type(ComponentSignature::Signature(Signature::simple(
                                RustType::in_scope(format!("{}<Self>", RESULT)),
                            ))),
                    );
            impls.push(fix_len_from_bytes);
        }
        fb = fb.add_struct(sb);
        for imp in impls {
            fb = fb.add_impl(imp);
        }
        fb
    } else {
        if source_type.should_serialize() {
            let serialize_into =
                ImplBuilder::new(Signature::simple(RustType::in_scope(&rust_name)))
                    .implement_for(Signature::simple(RustType::in_scope(VAR_LEN_SERIALIZE)))
                    .add_method(var_member_serialize(members, switches));
            impls.push(serialize_into);
        }

        if source_type.should_deserialize() {
            let required_args = get_sorted_required_by_members(members);
            let mut from_bytes = ImplBuilder::new(Signature::simple(RustType::in_scope(
                &rust_name,
            )))
            .add_method(var_member_from_bytes(
                &rust_name,
                members,
                switches,
                &required_args,
                &excl_names,
                xcb,
            ));
            if required_args.is_empty() {
                from_bytes = from_bytes
                    .implement_for(Signature::simple(RustType::in_scope(VAR_LEN_FROM_BYTES)));
            }
            impls.push(from_bytes);
        }
        for sw in switches {
            sb = sb.add_field_in_scope_simple_type(
                Visibility::Public,
                sw.rust_field_name(),
                sw.rust_entity_name(),
            );
        }
        fb = fb.add_struct(sb);
        for imp in impls {
            fb = fb.add_impl(imp);
        }
        fb
    }
}

pub(crate) struct FixedField {
    pub(crate) var_ref: VarRef,
    pub(crate) byte_size: usize,
    pub(crate) needs_serialize: bool,
}

pub(crate) enum VarRef {
    FixedValue(usize),
    RefValue(String, bool),
    FixedList(String, usize),
}

impl FixedField {
    pub(crate) fn put_in_var(&self) -> Option<String> {
        match &self.var_ref {
            VarRef::FixedValue(_) => None,
            VarRef::RefValue(rv, _) => {
                (self.byte_size > 1 || self.needs_serialize).then(|| rv.clone())
            }
            VarRef::FixedList(_, _) => None,
        }
    }
    pub(crate) fn push_insert_expr(&self, out: &mut String, self_ref: bool) {
        let ins_self = if self_ref { "self." } else { "" };
        if self.byte_size == 1 {
            match &self.var_ref {
                VarRef::FixedValue(v) => {
                    let _ = out.write_fmt(format_args!("{},\n", v));
                }
                VarRef::RefValue(rv, wrap) => {
                    if *wrap {
                        let _ = out.write_fmt(format_args!("{ins_self}{}.0", rv));
                        out.push_str(" as u8,\n");
                    } else if self.needs_serialize {
                        let _ = out.write_fmt(format_args!("{}_bytes[0],\n", rv));
                    } else {
                        let _ = out.write_fmt(format_args!("{ins_self}{},\n", rv));
                    }
                }
                VarRef::FixedList(member, _) => {
                    let _ = out.write_fmt(format_args!("{member}[0],\n"));
                }
            }
        } else {
            for i in 0..self.byte_size {
                match &self.var_ref {
                    VarRef::FixedValue(f) => {
                        let _ = out.write_fmt(format_args!("{},\n", f));
                    }
                    VarRef::RefValue(rv, _) => {
                        let _ = out.write_fmt(format_args!("{}_bytes[{i}],\n", rv));
                    }
                    VarRef::FixedList(member, sz) => {
                        if *sz == 1 {
                            let _ = out.write_fmt(format_args!("{ins_self}{member}[{i}],\n"));
                        } else {
                            unimplemented!("Not in spec");
                        }
                    }
                }
            }
        }
    }
}

pub(crate) fn fixed_members_serialize_impl(members: &[EntityMember]) -> String {
    let mut out = String::new();
    let mut ff = vec![];
    for member in members {
        match member {
            EntityMember::Field(f) => {
                let use_field = f.kind.concrete.clone();
                let wrap = f.kind.reference_type.is_some();
                let fname = f.name.to_rust_snake();
                let f = FixedField {
                    needs_serialize: should_serialize(f) && !wrap,
                    var_ref: VarRef::RefValue(fname, wrap),
                    byte_size: use_field.byte_size().unwrap(),
                };
                ff.push(f);
            }
            EntityMember::Pad(p) => {
                ff.push(FixedField {
                    needs_serialize: false,
                    var_ref: VarRef::FixedValue(0),
                    byte_size: *p,
                });
            }
            EntityMember::List(el) => {
                let fname = el.field.name.to_rust_snake();
                let count = el.fixed_count.unwrap();
                let member_size = el.field.kind.use_field().byte_size().unwrap();
                let sz = count * member_size;

                if member_size == 1 {
                    ff.push(FixedField {
                        var_ref: VarRef::FixedList(fname, member_size),
                        byte_size: sz,
                        needs_serialize: false,
                    });
                } else {
                    let local_name = format!("{fname}_{count}");
                    for count in 0..count {
                        dump!(out, "let {local_name}: [u8; {member_size}] = self.{}[{count}].serialize_fixed();\n", fname);
                    }
                    ff.push(FixedField {
                        var_ref: VarRef::FixedList(local_name, member_size),
                        byte_size: sz,
                        needs_serialize: false,
                    });
                }
            }
            EntityMember::StartAlign(_) => {}
            _ => panic!("Bad member in fixed struct {member:?}"),
        }
    }
    for member in &ff {
        if let Some(field) = member.put_in_var() {
            let _ = out.write_fmt(format_args!(
                "let {}_bytes = self.{}.serialize_fixed();\n",
                &field, &field
            ));
        }
    }
    out.push_str("[\n");
    for member in &ff {
        member.push_insert_expr(&mut out, true);
    }
    out.push_str("]\n");
    out
}

fn fixed_members_parse_impl(_name: &str, members: &[EntityMember], xcb: &Xcb) -> String {
    let mut out = "Ok(Self {\n".to_string();
    let mut offset = 0;
    for member in members {
        match member {
            EntityMember::Field(f) => {
                let sz = f.kind.concrete.byte_size().unwrap();
                if sz == 0 {
                    let _ = out.write_fmt(format_args!("{}: (),\n", f.name.to_rust_snake()));
                    continue;
                }
                if let Some(_ref_type) = &f.kind.reference_type {
                    if f.kind.concrete.is_builtin() || f.kind.concrete.is_builtin_alias() {
                        let _ = out.write_fmt(format_args!(
                            "{}: {}::from_bytes(bytes.get({offset}..{}).ok_or({})?)?.into(),\n",
                            f.name.to_rust_snake(),
                            f.kind.concrete.deref().borrow().import_name(&xcb.header),
                            offset + sz,
                            FROM_BYTES_ERROR,
                        ));
                    } else {
                        let _ = out.write_fmt(format_args!(
                            "{}: {}::from_bytes(bytes.get({offset}..{}).ok_or({})?)?.into(),\n",
                            f.name.to_rust_snake(),
                            f.kind.concrete.deref().borrow().import_name(&xcb.header),
                            offset + sz,
                            FROM_BYTES_ERROR,
                        ));
                    }
                } else {
                    let _ = out.write_fmt(format_args!(
                        "{}: {}::from_bytes(bytes.get({offset}..{}).ok_or({FROM_BYTES_ERROR})?)?,\n",
                        f.name.to_rust_snake(),
                        f.kind.use_field().deref().borrow().import_name(&xcb.header),
                        offset + sz,
                    ));
                }

                offset += sz;
            }
            EntityMember::Pad(p) => {
                offset += *p;
            }
            EntityMember::List(el) => {
                let count = el.fixed_count.unwrap();
                if el.field.kind.use_field().is_builtin()
                    || el.field.kind.use_field().is_builtin_alias()
                {
                    let member_size = el.field.kind.use_field().byte_size().unwrap();
                    if member_size == 1 {
                        dump!(out, "// SAFETY: We know we can try into exact size slice\n{}: unsafe {{bytes.get({offset}..{offset} + {count}).ok_or({})?.try_into().unwrap_unchecked()}},\n", el.field.name.to_rust_snake(), FROM_BYTES_ERROR);
                    } else {
                        dump!(out, "// SAFETY: We know we have enough bytes, then the transmute is safe as well\n");
                        dump!(out, "{}: unsafe {{ let raw_bytes: [u8; {}] = bytes.get({offset}..{offset} + {}).ok_or({})?.try_into().unwrap_unchecked();\ncore::mem::transmute(raw_bytes)\n}}", el.field.name.to_rust_snake(), count * member_size, count * member_size, FROM_BYTES_ERROR);
                    }
                } else {
                    unimplemented!("Not in spec afaik");
                }
            }
            _ => {}
        }
    }
    out.push_str("})\n");
    out
}

pub(crate) fn should_serialize(entity_field: &EntityField) -> bool {
    if let Some(ref_type) = &entity_field.kind.reference_type {
        let ty = &*ref_type.deref().borrow();
        type_needs_serialize(ty)
    } else {
        let ty = &*entity_field.kind.concrete.deref().borrow();
        type_needs_serialize(ty)
    }
}

fn type_needs_serialize(xcb_type: &XcbType) -> bool {
    match xcb_type {
        XcbType::Builtin(b) => match b {
            XcbBuiltin::Char
            | XcbBuiltin::Bool
            | XcbBuiltin::Void
            | XcbBuiltin::Byte
            | XcbBuiltin::Atom
            | XcbBuiltin::Fd
            | XcbBuiltin::Card8 => false,
            XcbBuiltin::Card16
            | XcbBuiltin::Card32
            | XcbBuiltin::Card64
            | XcbBuiltin::Int8
            | XcbBuiltin::Int16
            | XcbBuiltin::Int32
            | XcbBuiltin::Float32
            | XcbBuiltin::Float64 => true,
        },
        _ => true,
    }
}

#[derive(Debug)]
pub enum FixedChunk {
    Field(String, String, usize, bool),
    Pad(usize),
}

pub(crate) fn serialize_into_dump_fixed(
    out: &mut String,
    chunks: Vec<FixedChunk>,
    no_offset: bool,
    start_offset: usize,
) -> usize {
    if chunks.is_empty() {
        return 0;
    }
    let mut offset = start_offset;
    for chunk in chunks {
        match chunk {
            FixedChunk::Field(name, _, bs, self_ref) => {
                let prefix = if self_ref { "self." } else { "" };
                if no_offset {
                    dump!(
                        out,
                        "buf_ptr.get_mut({}..{}).ok_or({})?.copy_from_slice(&{prefix}{}.serialize_fixed());\n",
                        offset,
                        offset + bs,
                        SERIALIZE_ERROR,
                        name
                    );
                } else if offset == 0 {
                    dump!(out, "buf_ptr.get_mut(offset..offset + {}).ok_or({})?.copy_from_slice(&{prefix}{}.serialize_fixed());\n", bs, SERIALIZE_ERROR, name);
                } else {
                    dump!(out, "buf_ptr.get_mut(offset + {}..offset + {}).ok_or({})?.copy_from_slice(&{prefix}{}.serialize_fixed());\n", offset, offset + bs, SERIALIZE_ERROR, name);
                }
                offset += bs;
            }
            FixedChunk::Pad(bs) => {
                offset += bs;
            }
        }
    }
    if !no_offset {
        dump!(out, "offset += {};\n", offset);
    }
    offset
}

fn from_bytes_dump_fixed(out: &mut String, chunks: Vec<FixedChunk>, no_offset: bool) -> usize {
    if chunks.is_empty() {
        return 0;
    }
    let mut offset = 0;
    for chunk in chunks {
        match chunk {
            FixedChunk::Field(f, t, bs, _) => {
                if no_offset {
                    dump!(
                        out,
                        "let {} = {}::from_bytes(ptr.get({}..{}).ok_or({FROM_BYTES_ERROR})?)?;\n",
                        f,
                        t,
                        offset,
                        bs + offset,
                    );
                } else if offset == 0 {
                    dump!(
                    out,
                    "let {} = {}::from_bytes(ptr.get(offset..offset + {}).ok_or({FROM_BYTES_ERROR})?)?;\n",
                    f,
                    t,
                    bs,
                );
                } else {
                    dump!(
                    out,
                    "let {} = {}::from_bytes(ptr.get(offset + {}..offset + {}).ok_or({FROM_BYTES_ERROR})?)?;\n",
                    f,
                    t,
                    offset,
                    bs + offset,
                );
                }

                offset += bs;
            }
            FixedChunk::Pad(bs) => {
                offset += bs;
            }
        }
    }
    if !no_offset {
        dump!(out, "offset += {};\n", offset);
    }
    offset
}

fn var_member_from_bytes(
    _name: &str,
    variable: &[EntityMember],
    switches: &[WrappedType],
    req_args: &[EntityField],
    exclude: &HashSet<String>,
    xcb: &Xcb,
) -> MethodBuilder {
    let mut mb = MethodBuilder::new("from_bytes")
        .set_return_type(ComponentSignature::Signature(Signature::simple(
            RustType::in_scope(format!("{}<(Self, usize)>", RESULT)),
        )))
        .add_argument(Argument::new(
            Ownership::Ref,
            NamedComponentSignature::new(
                "bytes",
                ComponentSignature::Signature(Signature::simple(RustType::in_scope("[u8]"))),
            ),
        ));
    for arg in fields_as_args(req_args, &xcb.header) {
        mb = mb.add_argument(arg);
    }
    let mut out = "let ptr = bytes;\n".to_string();
    let mut set_fields = vec![];
    let mut fixed_chunks = vec![];
    let mut no_offset = true;
    for (ind, member) in variable.iter().enumerate() {
        let is_last = ind == variable.len() - 1 && switches.is_empty();
        let mut_offset_prefix = if is_last { "" } else { "mut " };
        match member {
            EntityMember::StartAlign(sa) => {
                dump!(out, "// Start align {} {:?}\n", sa.align, sa.offset);
            }
            EntityMember::Field(f) => {
                let fname = f.name.to_rust_snake();
                let use_type = f.kind.use_field().import_name(&xcb.header);
                if let Some(sz) = f.kind.concrete.byte_size() {
                    let type_name = f.kind.concrete.import_name(&xcb.header);
                    fixed_chunks.push(FixedChunk::Field(fname.clone(), type_name, sz, false));
                } else {
                    if no_offset {
                        let offset = from_bytes_dump_fixed(
                            &mut out,
                            core::mem::take(&mut fixed_chunks),
                            no_offset,
                        );
                        dump!(
                            out,
                            "let ({}, {}offset) = {}::from_bytes(ptr.get({}..).ok_or({FROM_BYTES_ERROR})?)?;\n",
                            fname,
                            mut_offset_prefix,
                            use_type,
                            offset
                        );
                    } else {
                        from_bytes_dump_fixed(
                            &mut out,
                            core::mem::take(&mut fixed_chunks),
                            no_offset,
                        );
                        dump!(
                            out,
                            "let ({}, new_offset) = {}::from_bytes(ptr.get(offset..).ok_or({FROM_BYTES_ERROR})?)?;\n",
                            fname,
                            use_type,
                        );
                        dump!(out, "offset += new_offset;\n");
                    }
                    no_offset = false;
                }
                if !exclude.contains(&f.name) {
                    set_fields.push((fname, f.kind.reference_type.is_some()));
                }
                //set_fields.push((fname, f.kind.reference_type.is_some()));
            }
            EntityMember::List(l) => {
                let fname = l.field.name.to_rust_snake();
                if let Some(count) = l.fixed_count {
                    let size = l.field.kind.use_field().byte_size().unwrap();
                    if size == 1 {
                        fixed_chunks.push(FixedChunk::Field(
                            fname.clone(),
                            format!("<[u8; {count}]>"),
                            count,
                            false,
                        ));
                        set_fields.push((fname, false));
                        continue;
                    } else {
                        unimplemented!("Not in spec afaik");
                    }
                }
                let offset =
                    from_bytes_dump_fixed(&mut out, core::mem::take(&mut fixed_chunks), no_offset);
                let use_type = l.field.kind.use_field().import_name(&xcb.header);
                if let Some(l_expr) = &l.length_expr {
                    let expr =
                        from_bytes_length_expr(l_expr, &l.field.name.to_rust_snake(), false, false);
                    if matches!(
                        &*l.field.kind.concrete.deref().borrow(),
                        XcbType::Builtin(XcbBuiltin::Fd)
                    ) {
                        dump!(out, "let {} = alloc::vec![];\n", fname);
                    } else if let Some(sz) = l.field.kind.concrete.byte_size() {
                        if let XcbExpression::Value(v) = l_expr {
                            dump!(out, "let length_expr = {}_usize;\n", v);
                        } else {
                            dump!(out, "let length_expr = {} as usize;\n", expr);
                        }
                        if no_offset {
                            if sz == 1 {
                                dump!(
                                    out,
                                    "let {}: alloc::vec::Vec<u8> = {}(ptr.get({offset}..).ok_or({FROM_BYTES_ERROR})?, length_expr)?;\n",
                                    l.field.name.to_rust_snake(),
                                    BYTE_VEC_FROM_BYTES,
                                );
                            } else {
                                dump!(
                                    out,
                                    "let {}: alloc::vec::Vec<{}> = {}(ptr.get({offset}..).ok_or({FROM_BYTES_ERROR})?, {use_type}, length_expr, {});\n",
                                    l.field.name.to_rust_snake(),
                                    use_type,
                                    FIX_LEN_FROM_BYTES_MACRO,
                                    sz
                                );
                            }
                        } else if sz == 1 {
                            dump!(
                                out,
                                "let {}: alloc::vec::Vec<u8> = {}(ptr.get(offset..).ok_or({FROM_BYTES_ERROR})?, length_expr)?;\n",
                                l.field.name.to_rust_snake(),
                                BYTE_VEC_FROM_BYTES,
                            );
                        } else {
                            dump!(
                                out,
                                "let {}: alloc::vec::Vec<{}> = {}(ptr.get(offset..).ok_or({FROM_BYTES_ERROR})?, {use_type}, length_expr, {});\n",
                                l.field.name.to_rust_snake(),
                                use_type,
                                FIX_LEN_FROM_BYTES_MACRO,
                                sz
                            );
                        }

                        if no_offset {
                            if sz == 1 {
                                if offset == 0 {
                                    dump!(out, "let {}offset = length_expr;\n", mut_offset_prefix);
                                } else {
                                    dump!(
                                        out,
                                        "let {}offset = length_expr + {};\n",
                                        mut_offset_prefix,
                                        offset
                                    );
                                }
                            } else if offset == 0 {
                                dump!(
                                    out,
                                    "let {}offset = length_expr * {};\n",
                                    mut_offset_prefix,
                                    sz
                                );
                            } else {
                                dump!(
                                    out,
                                    "let {}offset = length_expr * {} + {};\n",
                                    mut_offset_prefix,
                                    sz,
                                    offset
                                );
                            }
                        } else if sz == 1 {
                            dump!(out, "offset += length_expr;\n");
                        } else {
                            dump!(out, "offset += length_expr * {};\n", sz);
                        }
                    } else {
                        let member_req = member.required_args();
                        let pool = l.field.kind.concrete.members();
                        let others = l
                            .length_expr
                            .as_ref()
                            .unwrap()
                            .required_fields()
                            .into_iter()
                            .map(|f| f.name)
                            .collect::<Vec<String>>();
                        let req = reduce_required_from_container(&member_req, &pool)
                            .into_iter()
                            .filter(|r| !others.contains(&r.name))
                            .collect::<Vec<EntityField>>();
                        dump!(out, "let {}_length = {} as usize;\n", fname, expr);
                        if no_offset {
                            dump!(out, "let mut offset = {};\n", offset);
                            dump!(
                                out,
                                "let {} = {}(ptr, {}, offset, {}_length, \n",
                                fname,
                                VAR_LEN_FROM_BYTES_MACRO,
                                use_type,
                                fname,
                            );
                        } else {
                            dump!(
                                out,
                                "let {} = {}(ptr, {}, offset, {}_length, \n",
                                fname,
                                VAR_LEN_FROM_BYTES_MACRO,
                                use_type,
                                fname,
                            );
                        }

                        for (ind, ef) in req.iter().enumerate() {
                            dump!(out, "{}", ef.name.to_rust_snake());
                            if ind == req.len() - 1 {
                                dump!(out, "\n");
                            } else {
                                dump!(out, ",\n");
                            }
                        }
                        dump!(out, ");\n");
                    }
                    set_fields.push((fname, false));
                } else {
                    if let Some(bs) = l.field.kind.concrete.byte_size() {
                        if no_offset {
                            dump!(out, "let mut offset = {};\n", offset);
                        }
                        dump!(out, "let mut {} = alloc::vec::Vec::new();\n", fname);
                        dump!(out, "while let Some(buf) = ptr.get(offset..).filter(|buf| !buf.is_empty()) {\n");
                        dump!(out, "{}.push({}::from_bytes(buf)?);\n", fname, use_type);
                        dump!(out, "offset += {};\n", bs);
                        dump!(out, "}\n");
                    } else {
                        panic!("Var len list with unknown length!");
                    }
                    set_fields.push((fname, false));
                    // Run until bytes run out
                };
                no_offset = false;
            }
            EntityMember::Pad(p) => {
                dump!(out, "// Padding {} bytes \n", p);
                fixed_chunks.push(FixedChunk::Pad(*p));
            }
            EntityMember::Align(a) => {
                dump!(out, "// Align {} bytes \n", a);
                let offset =
                    from_bytes_dump_fixed(&mut out, core::mem::take(&mut fixed_chunks), no_offset);
                if no_offset {
                    dump!(
                        out,
                        "let mut offset = ({} - ({} % {})) % {};\n",
                        a,
                        offset,
                        a,
                        a
                    );
                } else {
                    dump!(out, "offset += ({} - (offset % {})) % {};\n", a, a, a);
                }
                no_offset = false;
            }
            EntityMember::Length(_l) => {
                let offset =
                    from_bytes_dump_fixed(&mut out, core::mem::take(&mut fixed_chunks), no_offset);
                if no_offset {
                    dump!(out, "let mut offset = {};\n", offset);
                }
                no_offset = false;
                dump!(out, "// Length \n");
            }
        }
    }
    let offset = from_bytes_dump_fixed(&mut out, core::mem::take(&mut fixed_chunks), no_offset);

    for switch in switches {
        let args = switch.required_args();
        let sorted = get_sorted_required_fields(&args)
            .into_iter()
            .map(|ef| ef.name.to_rust_snake())
            .collect::<Vec<String>>();
        if no_offset {
            let mut_prefix = if switches.len() == 1 { "" } else { "mut " };
            dump!(
                out,
                "let ({}, {}offset) = {}::from_bytes(ptr.get({offset}..).ok_or({FROM_BYTES_ERROR})?,\n",
                switch.rust_field_name(),
                mut_prefix,
                switch.rust_entity_name()
            );
        } else {
            dump!(
                out,
                "let ({}, new_offset) = {}::from_bytes(ptr.get(offset..).ok_or({FROM_BYTES_ERROR})?,\n",
                switch.rust_field_name(),
                switch.rust_entity_name()
            );
        }

        for req in sorted {
            dump!(out, "{},\n", req);
        }
        dump!(out, ")?;\n");
        if !no_offset {
            dump!(out, "offset += new_offset;\n");
        }
        no_offset = false;
        set_fields.push((switch.rust_field_name(), false));
    }
    dump!(out, "Ok((Self {\n");
    for (field, convert) in set_fields {
        if convert {
            dump!(out, "{}: {}.into(),\n", field, field);
        } else {
            dump!(out, "{},\n", field);
        }
    }
    dump!(out, "}, offset))\n");
    mb.set_body(out)
}

fn var_member_serialize(variable: &[EntityMember], switches: &[WrappedType]) -> MethodBuilder {
    let mb = MethodBuilder::new("serialize_into")
        .set_self_ownership(Ownership::Owned)
        .set_return_type(ComponentSignature::Signature(Signature::simple(
            RustType::in_scope(format!("{}<usize>", RESULT)),
        )))
        .add_argument(Argument::new(
            Ownership::MutRef,
            NamedComponentSignature::new(
                "buf",
                ComponentSignature::Signature(Signature::simple(RustType::in_scope("[u8]"))),
            ),
        ));

    let mut replace = HashMap::new();
    for member in variable {
        if let EntityMember::List(l) = member {
            if let Some(expr) = &l.length_expr {
                let (_, repl) = serialize_length_expr(expr, &l.field.name.to_rust_snake(), true);
                for (field, list) in repl {
                    replace.insert(
                        field,
                        (
                            list.clone(),
                            from_bytes_length_expr(expr, &list, true, false),
                        ),
                    );
                }
            }
        }
    }
    let mut out = String::new();
    out.push_str("let buf_ptr = buf;\n");
    let mut fixed_chunks = vec![];
    let mut no_offset = true;
    for (ind, member) in variable.iter().enumerate() {
        let is_last = ind == variable.len() - 1 && switches.is_empty();
        let mut_offset_prefix = if is_last { "" } else { "mut " };
        match member {
            EntityMember::StartAlign(sa) => {
                dump!(out, "// Start align {} {:?}\n", sa.align, sa.offset);
            }
            EntityMember::Field(f) => {
                let var_name = f.name.to_rust_snake();
                if let Some(sz) = f.kind.concrete.byte_size() {
                    let (name, self_ref) = if let Some((repl, expr)) = replace.get(&f.name) {
                        dump!(
                            out,
                            "let {} = u{}::try_from({}).map_err(|_| {})?;\n",
                            &f.name.to_rust_snake(),
                            sz * 8,
                            repl,
                            SERIALIZE_ERROR
                        );
                        let name = format!(
                            "(u{}::try_from({}).map_err(|_| {})?)",
                            sz * 8,
                            expr,
                            SERIALIZE_ERROR
                        );
                        (name, false)
                    } else {
                        (var_name, true)
                    };
                    fixed_chunks.push(FixedChunk::Field(name, "".to_string(), sz, self_ref));
                } else {
                    let offset = serialize_into_dump_fixed(
                        &mut out,
                        core::mem::take(&mut fixed_chunks),
                        no_offset,
                        0,
                    );
                    if no_offset {
                        dump!(
                            out,
                            "let {}offset = self.{}.serialize_into(buf_ptr.get_mut({offset}..).ok_or({})?)?;\n",
                            mut_offset_prefix,
                            var_name,
                            SERIALIZE_ERROR,
                        );
                    } else {
                        dump!(
                            out,
                            "offset += self.{}.serialize_into(buf_ptr.get_mut(offset..).ok_or({})?)?;\n",
                            var_name,
                            SERIALIZE_ERROR,
                        );
                    }
                    no_offset = false;
                }
            }
            EntityMember::List(l) => {
                let offset = serialize_into_dump_fixed(
                    &mut out,
                    core::mem::take(&mut fixed_chunks),
                    no_offset,
                    0,
                );
                let fname = l.field.name.to_rust_snake();
                if let Some(_expr) = l.length_expr.as_ref() {
                    if let Some(sz) = l.field.kind.concrete.byte_size() {
                        if sz != 0 {
                            dump!(out, "let list_len = self.{}.len();\n", fname);
                            if no_offset {
                                // Byte vec
                                if sz == 1 {
                                    dump!(
                                        out,
                                        "{}(buf_ptr.get_mut({offset}..).ok_or({})?, &self.{})?;\n",
                                        BYTES_VEC_SER_FUNCTION,
                                        SERIALIZE_ERROR,
                                        l.field.name.to_rust_snake()
                                    );
                                } else {
                                    dump!(
                                        out,
                                        "{}(buf_ptr.get_mut({offset}..).ok_or({})?, self.{})?;\n",
                                        FIX_LEN_SER_FUNCTION,
                                        SERIALIZE_ERROR,
                                        l.field.name.to_rust_snake()
                                    );
                                }

                                if offset == 0 {
                                    dump!(out, "let {}offset = list_len;\n", mut_offset_prefix);
                                } else {
                                    dump!(
                                        out,
                                        "let {}offset = list_len + {};\n",
                                        mut_offset_prefix,
                                        offset
                                    );
                                }
                            } else {
                                if sz == 1 {
                                    dump!(
                                        out,
                                        "{}(buf_ptr.get_mut(offset..).ok_or({})?, &self.{})?;\n",
                                        BYTES_VEC_SER_FUNCTION,
                                        SERIALIZE_ERROR,
                                        l.field.name.to_rust_snake()
                                    );
                                } else {
                                    dump!(
                                        out,
                                        "{}(buf_ptr.get_mut(offset..).ok_or({})?, self.{})?;\n",
                                        FIX_LEN_SER_FUNCTION,
                                        SERIALIZE_ERROR,
                                        l.field.name.to_rust_snake()
                                    );
                                }

                                dump!(out, "offset += list_len;\n");
                            }
                        }
                    } else if no_offset {
                        dump!(
                        out,
                        "let {}offset = {}(buf_ptr.get_mut({offset}..).ok_or({})?, self.{})?;\n",
                        mut_offset_prefix,
                        VAR_LEN_SER_FUNCTION,
                        SERIALIZE_ERROR,
                        l.field.name.to_rust_snake()
                    );
                    } else {
                        dump!(
                            out,
                            "offset += {}(buf_ptr.get_mut(offset..).ok_or({})?, self.{})?;\n",
                            VAR_LEN_SER_FUNCTION,
                            SERIALIZE_ERROR,
                            l.field.name.to_rust_snake()
                        );
                    }
                } else if let Some(_) = l.field.kind.concrete.byte_size() {
                    dump!(out, "let list_len = self.{}.len();\n", fname);
                    if no_offset {
                        dump!(
                            out,
                            "{}(buf_ptr.get_mut({offset}..).ok_or({})?, self.{})?;\n",
                            FIX_LEN_SER_FUNCTION,
                            SERIALIZE_ERROR,
                            l.field.name.to_rust_snake()
                        );
                        if offset == 0 {
                            dump!(out, "let {}offset = list_len;\n", mut_offset_prefix);
                        } else {
                            dump!(
                                out,
                                "let {}offset = list_len + {};\n",
                                mut_offset_prefix,
                                offset
                            );
                        }
                    } else {
                        dump!(
                            out,
                            "{}(buf_ptr.get_mut(offset..).ok_or({})?, self.{})?;\n",
                            FIX_LEN_SER_FUNCTION,
                            SERIALIZE_ERROR,
                            l.field.name.to_rust_snake()
                        );
                        dump!(out, "offset += list_len;\n");
                    }
                } else {
                    panic!("Var len list with unknown length!");
                }

                no_offset = false;
            }
            EntityMember::Pad(p) => {
                dump!(out, "// Padding {} bytes \n", p);
                fixed_chunks.push(FixedChunk::Pad(*p));
            }
            EntityMember::Align(a) => {
                let offset = serialize_into_dump_fixed(
                    &mut out,
                    core::mem::take(&mut fixed_chunks),
                    no_offset,
                    0,
                );
                dump!(out, "// Align {} bytes \n", a);
                if no_offset {
                    dump!(
                        out,
                        "let {}offset = ({} - ({} % {})) % {};\n",
                        mut_offset_prefix,
                        a,
                        offset,
                        a,
                        a
                    );
                } else {
                    dump!(out, "offset += ({} - (offset % {})) % {};\n", a, a, a);
                }
                no_offset = false;
            }
            EntityMember::Length(_) => {
                let offset = serialize_into_dump_fixed(
                    &mut out,
                    core::mem::take(&mut fixed_chunks),
                    no_offset,
                    0,
                );
                dump!(out, "// Length \n");
                if no_offset {
                    dump!(out, "let {}offset = {};\n", mut_offset_prefix, offset);
                }
                no_offset = false;
            }
        }
    }
    let offset =
        serialize_into_dump_fixed(&mut out, core::mem::take(&mut fixed_chunks), no_offset, 0);
    for (ind, switch) in switches.iter().enumerate() {
        let last = ind == switches.len() - 1;
        let mut_prefix = if last { "" } else { "mut " };
        if no_offset {
            dump!(
                out,
                "let {}offset = self.{}.serialize_into(buf_ptr.get_mut({offset}..).ok_or({SERIALIZE_ERROR})?)?;\n",
                mut_prefix,
                switch.rust_field_name()
            );
        } else {
            dump!(
                out,
                "offset += self.{}.serialize_into(buf_ptr.get_mut(offset..).ok_or({})?)?;\n",
                switch.rust_field_name(),
                SERIALIZE_ERROR,
            );
        }
        no_offset = false;
    }
    out.push_str("Ok(offset)");
    mb.set_body(out)
}
