use std::collections::hash_map::Entry;
use std::collections::{HashMap, HashSet};
use std::fmt::Write;
use std::ops::Deref;

use codegen_rs::structures::gen_enum::NamedComponentSignature;
use codegen_rs::structures::method::Argument;
use codegen_rs::structures::{ComponentSignature, Ownership, RustType, Signature};
use codegen_rs::{FileBuilder, ImplBuilder, MethodBuilder, RustCase, TraitBuilder};

use crate::generator::codegen::functions::{
    find_req_derive_fields, from_bytes_length_expr, get_unsorted_required_fields, is_finite_size,
    is_finite_switch_type, member_size, serialize_length_expr,
};
use crate::generator::codegen::plain_struct::{
    serialize_into_dump_fixed, should_serialize, FixedChunk, FixedField, VarRef,
};
use crate::generator::codegen::switch::find_switch_enum_ref_type;
use crate::generator::codegen::{
    AsRustFormatted, BYTES_VEC_SER_FUNCTION, FIX_LEN_SER_FUNCTION, REQ_TO_BIG_ERROR, RESULT,
    SERIALIZE_ERROR, VAR_LEN_SER_FUNCTION,
};
use crate::generator::types::{
    EntityField, EntityMember, TypeHelpers, WrappedType, XcbRequest, XcbType,
};
use crate::{dump, Xcb};

#[derive(Debug, Default)]
pub(crate) struct ReqNameSpec {
    pub(crate) xproto: Vec<ReqSpec>,
    pub(crate) ext: HashMap<String, Vec<ReqSpec>>,
}

#[derive(Debug, Eq, PartialEq, Hash)]
pub(crate) struct ReqSpec {
    pub(crate) major_opcode: u8,
    pub(crate) req_name: String,
}

pub(crate) fn implement_request(
    req: &XcbRequest,
    xcb: &Xcb,
    req_name_spec: &mut ReqNameSpec,
    mut con_trait_bilder: TraitBuilder,
    mut con_impl_builder: ImplBuilder,
    fb: FileBuilder,
) -> (TraitBuilder, ImplBuilder, FileBuilder) {
    if xcb.header == "xproto" {
        req_name_spec.xproto.push(ReqSpec {
            major_opcode: req.opcode,
            req_name: req.name.clone(),
        });
    } else {
        match req_name_spec.ext.entry(xcb.header.clone()) {
            Entry::Vacant(v) => {
                v.insert(vec![ReqSpec {
                    major_opcode: req.opcode,
                    req_name: req.name.clone(),
                }]);
            }
            Entry::Occupied(mut o) => {
                o.get_mut().push(ReqSpec {
                    major_opcode: req.opcode,
                    req_name: req.name.clone(),
                });
            }
        }
    }
    let fixed_size = req
        .switch
        .is_none()
        .then(|| member_size(&req.members))
        .flatten();
    let mut mb = if fixed_size.is_some() {
        implement_fixed_req_serialize(
            req.opcode,
            req.rust_field_name().replace("_request", ""),
            &req.members,
            req.reply.clone(),
            xcb,
        )
    } else {
        implement_var_len_req_serialize(
            req.opcode,
            req.rust_field_name().replace("_request", ""),
            &req.members,
            req.switch.clone(),
            req.reply.clone(),
            xcb,
        )
    };
    mb = mb.add_argument_in_scope_simple_type(Ownership::Owned, "forget", "bool");
    let marker = mb.clone();

    con_trait_bilder = con_trait_bilder.add_method(marker.set_trait_no_body());
    con_impl_builder = con_impl_builder.add_method(mb);

    (con_trait_bilder, con_impl_builder, fb)
}

pub(crate) fn implement_fixed_req_serialize(
    opcode: u8,
    name: String,
    members: &[EntityMember],
    reply: Option<WrappedType>,
    xcb: &Xcb,
) -> MethodBuilder {
    let xproto = xcb.header == "xproto";
    let bs = member_size(members).unwrap();
    let mut body = String::new();
    let mut mb = MethodBuilder::new(&name).set_self_ownership(Ownership::MutRef);
    add_major_opcode(xproto, &mut body, xcb);
    // We don't need to do anything on an empty-body request essentially
    if bs == 0 {
        // We measure length in words here
        dump!(
            body,
            "let buf = self.write_buf().get_mut(..4).ok_or({})?\n;",
            SERIALIZE_ERROR
        );
        if xproto {
            dump!(body, "buf[0] = {};\nbuf[1] = 0;\n", opcode);
        } else {
            dump!(body, "buf[0] = major_opcode;\nbuf[1] = {};\n", opcode);
        }
        dump!(body, "buf[2..4].copy_from_slice(&(1u16).to_ne_bytes());\n");
        dump!(body, "self.advance_writer(4);\n");
    } else {
        let ff = create_fixed_fields(members);

        let first_byte_sized = ff[0].byte_size == 1;
        let mut bs = if xproto && first_byte_sized {
            bs + 3
        } else {
            bs + 4
        };
        let diff = (4 - (bs % 4)) % 4;
        bs += diff;
        dump!(
            body,
            "let length: [u8; 2] = ({}u16).to_ne_bytes();\n",
            bs / 4
        );
        assert_eq!(bs % 4, 0, "Bad size on serialize request {bs} {diff}");
        for member in &ff {
            if let Some(field) = member.put_in_var() {
                let wrap = if let VarRef::RefValue(_, w) = member.var_ref {
                    w
                } else {
                    false
                };
                if wrap {
                    let _ = body.write_fmt(format_args!("let {}_bytes = ({}.0", &field, &field));
                    let _ = body.write_fmt(format_args!(
                        " as u{}).serialize_fixed();\n",
                        member.byte_size * 8
                    ));
                } else {
                    let _ = body.write_fmt(format_args!(
                        "let {}_bytes = {}.serialize_fixed();\n",
                        &field, &field
                    ));
                }
            }
        }
        dump!(body, "let buf = self.write_buf();\n");
        dump!(
            body,
            "buf.get_mut(..{}).ok_or({})?.copy_from_slice(&[\n",
            bs,
            SERIALIZE_ERROR
        );
        let mut ff_iter = ff.iter();
        if xproto {
            if first_byte_sized {
                dump!(body, "{},\n", opcode);
                ff_iter.next().unwrap().push_insert_expr(&mut body, false);
            } else {
                dump!(body, "{},\n0,\n", opcode);
            }
        } else {
            dump!(body, "major_opcode,\n{},\n", opcode);
        }
        dump!(body, "length[0],\nlength[1],\n");

        for member in ff_iter {
            member.push_insert_expr(&mut body, false);
        }
        for _ in 0..diff {
            dump!(body, "0\n,");
        }
        dump!(body, "]);\n");
        dump!(body, "self.advance_writer({});\n", bs);
    }
    mb = set_return_type(mb, &mut body, reply);
    let mut list_field_names = HashSet::new();
    let fields = members
        .iter()
        .filter_map(|m| match m {
            EntityMember::Field(f) => Some(f.clone()),
            EntityMember::List(el) => {
                list_field_names.insert(el.field.name.clone());
                Some(el.field.clone())
            }
            _ => None,
        })
        .collect::<Vec<EntityField>>();
    let sorted = get_unsorted_required_fields(&fields);
    let mut args = vec![];
    for field in sorted {
        let use_type = field.kind.use_field().import_name("NONE");
        let t = if list_field_names.contains(&field.name) {
            format!("&[{}]", use_type)
        } else {
            use_type
        };
        args.push(Argument::new(
            Ownership::Owned,
            NamedComponentSignature::new(
                field.name.to_rust_snake(),
                ComponentSignature::Signature(Signature::simple(RustType::in_scope(t))),
            ),
        ));
    }
    for arg in args {
        mb = mb.add_argument(arg);
    }
    mb.set_body(body)
}

fn add_major_opcode(xproto: bool, body: &mut String, xcb: &Xcb) {
    if !xproto {
        let ext_name_source = format!("crate::proto::{}::EXTENSION_NAME", xcb.header);
        let _ = body.write_fmt(format_args!("let major_opcode = self.major_opcode({}).ok_or(crate::error::Error::MissingExtension({}))?;\n", ext_name_source, ext_name_source));
    }
}

fn set_return_type(
    mut mb: MethodBuilder,
    body: &mut String,
    reply: Option<WrappedType>,
) -> MethodBuilder {
    body.push_str(
        "let seq = if forget { \nself.next_seq()\n} else {\n self.keep_and_return_next_seq()\n};\n",
    );
    if let Some(reply) = reply {
        let (cookie, constructor) = if let Some(sz) = reply.byte_size() {
            (
                format!(
                    "{}<FixedCookie<{}, {}>>",
                    RESULT,
                    reply.import_name("none"),
                    sz + 7
                ),
                "Ok(FixedCookie::new(seq))",
            )
        } else {
            (
                format!("{}<Cookie<{}>>", RESULT, reply.import_name("none"),),
                "Ok(Cookie::new(seq))",
            )
        };
        mb = mb.set_return_type(ComponentSignature::Signature(Signature::simple(
            RustType::in_scope(cookie),
        )));
        let _ = body.write_str(constructor);
    } else {
        mb = mb.set_return_type(ComponentSignature::Signature(Signature::simple(
            RustType::in_scope(format!("{}<VoidCookie>", RESULT)),
        )));
        let _ = body.write_str("Ok(VoidCookie::new(seq))");
    }
    mb
}

fn create_fixed_fields(members: &[EntityMember]) -> Vec<FixedField> {
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
                let sz = el.fixed_count.unwrap() * el.field.kind.concrete.byte_size().unwrap();
                ff.push(FixedField {
                    var_ref: VarRef::FixedList(fname, el.field.kind.concrete.byte_size().unwrap()),
                    byte_size: sz,
                    needs_serialize: false,
                });
            }
            EntityMember::StartAlign(_) => {}
            _ => panic!("Bad member in fixed struct {member:?}"),
        }
    }
    ff
}

pub(crate) fn implement_var_len_req_serialize(
    opcode: u8,
    name: String,
    members: &[EntityMember],
    switch: Option<WrappedType>,
    reply: Option<WrappedType>,
    xcb: &Xcb,
) -> MethodBuilder {
    let finite =
        is_finite_size(&members) && switch.as_ref().map(is_finite_switch_type).unwrap_or(true);

    let xproto = xcb.header == "xproto";
    let switch_ref_type = if let Some(switch) = &switch {
        if let XcbType::SwitchStruct(ss) = &*switch.deref().borrow() {
            Some(find_switch_enum_ref_type(ss))
        } else {
            None
        }
    } else {
        None
    };
    let (excl, repl_switch) = find_req_derive_fields(members, switch_ref_type);
    let excl_names = excl
        .iter()
        .map(|e| e.name.clone())
        .collect::<HashSet<String>>();
    let mut body = String::new();
    let name = RustCase::convert_to_valid_rust(&name, RustCase::Snake).unwrap();
    let mut mb = MethodBuilder::new(&name).set_self_ownership(Ownership::MutRef);
    add_major_opcode(xproto, &mut body, xcb);
    dump!(body, "let buf_ptr = self.write_buf();\n");
    let mut known_offset = 4;
    let mut header = String::new();
    let mut skip_first = false;
    if xproto {
        let first_member_fixed = members
            .first()
            .filter(|m| m.byte_size().filter(|bs| *bs == 1).is_some())
            .map(|m| match m {
                EntityMember::Field(f) => {
                    if f.kind.use_field().is_builtin() {
                        f.name.to_rust_snake()
                    } else {
                        format!("{}.0", f.name.to_rust_snake())
                    }
                }
                EntityMember::Pad(_) => "0".to_string(),
                _ => panic!("Bad filter"),
            });
        if let Some(first) = first_member_fixed {
            dump!(
                header,
                "buf_ptr.get_mut(0..2).ok_or({})?.copy_from_slice(&[{}, {first}]);\n",
                SERIALIZE_ERROR,
                opcode
            );
            known_offset -= 1;
            skip_first = true;
        } else {
            dump!(
                header,
                "buf_ptr.get_mut(0..2).ok_or({})?.copy_from_slice(&[{}, 0]);\n",
                SERIALIZE_ERROR,
                opcode
            );
        }
    } else {
        dump!(
            header,
            "buf_ptr.get_mut(0..2).ok_or({})?.copy_from_slice(&[major_opcode, {}]);\n",
            SERIALIZE_ERROR,
            opcode
        );
    }

    let mut fixed = vec![];
    let mut no_offset = true;
    let mut args = vec![];
    let mut replace = HashMap::new();

    for member in members {
        if let EntityMember::List(l) = member {
            if let Some(expr) = &l.length_expr {
                let (_, repl) = serialize_length_expr(expr, &l.field.name.to_rust_snake(), false);
                if repl.len() > 1 {
                    continue;
                }
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
    for (ind, field) in members.iter().enumerate() {
        let is_last = ind == members.len() - 1;
        let should_skip = ind == 0 && skip_first;
        match field {
            EntityMember::StartAlign(sa) => {
                dump!(
                    body,
                    "// Start align {}, offset {:?}\n",
                    sa.align,
                    sa.offset
                );
            }
            EntityMember::Field(field) => {
                let use_type = field.kind.use_field();
                let use_type_name = use_type.import_name("NONE");
                let field_name = field.name.to_rust_snake();
                if !excl_names.contains(&field.name) {
                    args.push(Argument::new(
                        Ownership::Owned,
                        NamedComponentSignature::new(
                            &field_name,
                            ComponentSignature::Signature(Signature::simple(RustType::in_scope(
                                &use_type_name,
                            ))),
                        ),
                    ));
                }

                if let Some(bs) = field.kind.concrete.byte_size() {
                    let (name, self_ref) =
                        if repl_switch.as_ref().filter(|r| *r == &field_name).is_some() {
                            let sw = switch.as_ref().unwrap().rust_field_name();
                            (format!("{sw}.switch_expr()"), false)
                        } else if let Some((repl, expr)) = replace.get(&field.name) {
                            let name_expr = if excl_names.contains(&field.name) {
                                repl.clone()
                            } else {
                                if field.kind.reference_type.is_some() {
                                    format!("{}.0", field.name.to_rust_snake())
                                } else {
                                    field.name.to_rust_snake()
                                }
                            };
                            dump!(
                                body,
                                "let {} = u{}::try_from({}).map_err(|_| {})?;\n",
                                &field.name.to_rust_snake(),
                                bs * 8,
                                name_expr,
                                SERIALIZE_ERROR
                            );
                            let name = format!(
                                "(u{}::try_from({}).map_err(|_| {})?)",
                                bs * 8,
                                expr,
                                SERIALIZE_ERROR
                            );
                            if should_skip {
                                continue;
                            }
                            (name, false)
                        } else {
                            if should_skip {
                                known_offset += 1;
                                continue;
                            }
                            (field.name.to_rust_snake(), false)
                        };
                    fixed.push(FixedChunk::Field(name, use_type_name, bs, self_ref));
                } else {
                    let offset = serialize_into_dump_fixed(
                        &mut body,
                        core::mem::take(&mut fixed),
                        no_offset,
                        known_offset,
                    );
                    if no_offset {
                        dump!(
                            body,
                            "let mut offset = {}.serialize_into(buf_ptr.get_mut({offset}..).ok_or({})?)?;\n",
                            field_name,
                            SERIALIZE_ERROR
                        );
                    } else {
                        dump!(
                            body,
                            "offset += {}.serialize_into(buf_ptr.get_mut(offset..).ok_or({})?)?;\n",
                            field_name,
                            SERIALIZE_ERROR
                        );
                    }
                    no_offset = false;
                    known_offset = 0;
                }
            }
            EntityMember::List(l) => {
                let offset = serialize_into_dump_fixed(
                    &mut body,
                    core::mem::take(&mut fixed),
                    no_offset,
                    known_offset,
                );
                let use_type = if l.field.kind.concrete.byte_size().is_some() {
                    format!("&[{}]", l.field.kind.use_field().import_name("NONE"))
                } else {
                    format!(
                        "alloc::vec::Vec<{}>",
                        l.field.kind.use_field().import_name("NONE")
                    )
                };
                args.push(Argument::new(
                    Ownership::Owned,
                    NamedComponentSignature::new(
                        l.field.name.to_rust_snake(),
                        ComponentSignature::Signature(Signature::simple(RustType::in_scope(
                            use_type,
                        ))),
                    ),
                ));
                if let Some(sz) = l.field.kind.concrete.byte_size() {
                    if sz == 0 {
                        if no_offset {
                            dump!(body, "let offset = {};\n", offset);
                        }
                    } else {
                        let length_mult = if sz == 1 {
                            "".to_string()
                        } else {
                            format!("* {sz}")
                        };
                        dump!(
                            body,
                            "let list_len = {}.len(){};\n",
                            l.field.name.to_rust_snake(),
                            length_mult
                        );
                        if no_offset {
                            if sz == 1 {
                                dump!(
                                    body,
                                    "{}(buf_ptr.get_mut({offset}..).ok_or({})?, {})?;\n",
                                    BYTES_VEC_SER_FUNCTION,
                                    SERIALIZE_ERROR,
                                    l.field.name.to_rust_snake()
                                );
                            } else {
                                dump!(
                                    body,
                                    "{}(buf_ptr.get_mut({offset}..).ok_or({})?, {})?;\n",
                                    FIX_LEN_SER_FUNCTION,
                                    SERIALIZE_ERROR,
                                    l.field.name.to_rust_snake()
                                );
                            }

                            if offset == 0 {
                                dump!(body, "let mut offset = list_len;\n");
                            } else {
                                dump!(body, "let mut offset = list_len + {};\n", offset);
                            }
                        } else {
                            if sz == 1 {
                                dump!(
                                    body,
                                    "{}(buf_ptr.get_mut(offset..).ok_or({})?, {})?;\n",
                                    BYTES_VEC_SER_FUNCTION,
                                    SERIALIZE_ERROR,
                                    l.field.name.to_rust_snake()
                                );
                            } else {
                                dump!(
                                    body,
                                    "{}(buf_ptr.get_mut(offset..).ok_or({})?, {})?;\n",
                                    FIX_LEN_SER_FUNCTION,
                                    SERIALIZE_ERROR,
                                    l.field.name.to_rust_snake()
                                );
                            }
                            dump!(body, "offset += list_len;\n");
                        }
                    }
                } else if no_offset {
                    dump!(
                        body,
                        "let mut offset = {}(buf_ptr.get_mut({offset}..).ok_or({})?, {})?;\n",
                        VAR_LEN_SER_FUNCTION,
                        SERIALIZE_ERROR,
                        l.field.name.to_rust_snake()
                    );
                } else {
                    dump!(
                        body,
                        "offset += {}(buf_ptr.get_mut(offset..).ok_or({})?, {})?;\n",
                        VAR_LEN_SER_FUNCTION,
                        SERIALIZE_ERROR,
                        l.field.name.to_rust_snake()
                    );
                }
                if is_last {
                    dump!(body, "let mut offset = offset + (4 - (offset % 4)) % 4;\n");
                }
                no_offset = false;
                known_offset = 0;
            }
            EntityMember::Pad(p) => {
                dump!(body, "// Pad {} bytes\n", p);
                fixed.push(FixedChunk::Pad(*p));
            }
            EntityMember::Align(a) => {
                let offset = serialize_into_dump_fixed(
                    &mut body,
                    core::mem::take(&mut fixed),
                    no_offset,
                    known_offset,
                );
                if no_offset {
                    dump!(
                        body,
                        "let mut offset = ({} - ({} % {})) % {};\n",
                        a,
                        offset,
                        a,
                        a
                    );
                } else {
                    dump!(body, "offset += ({} - (offset % {})) % {};\n", a, a, a);
                }
                no_offset = false;
                known_offset = 0;
            }
            EntityMember::Length(_) => {
                dump!(body, "// Length");
                let offset = serialize_into_dump_fixed(
                    &mut body,
                    core::mem::take(&mut fixed),
                    no_offset,
                    known_offset,
                );
                if no_offset {
                    dump!(body, "let mut offset = {}", offset);
                }
                no_offset = false;
                known_offset = 0;
            }
        }
    }
    dump!(body, "{}", header);
    for arg in args {
        mb = mb.add_argument(arg);
    }
    if let Some(switch) = switch {
        mb = mb.add_argument_in_scope_simple_type(
            Ownership::Owned,
            switch.rust_field_name(),
            switch.import_name("NONE"),
        );
        if no_offset {
            let offset = serialize_into_dump_fixed(
                &mut body,
                core::mem::take(&mut fixed),
                no_offset,
                known_offset,
            );
            dump!(
                body,
                "let mut offset = {offset} + {}.serialize_into(buf_ptr.get_mut({offset}..).ok_or({})?)?;\n",
                switch.rust_field_name(),
                SERIALIZE_ERROR
            );
        } else {
            dump!(
                body,
                "offset += {}.serialize_into(buf_ptr.get_mut(offset..).ok_or({})?)?;\n",
                switch.rust_field_name(),
                SERIALIZE_ERROR
            );
        }
    }
    dump!(
        body,
        "debug_assert!(offset % 4 == 0, \"Bad request length offset {offset}\");\n"
    );
    dump!(body, "let word_len = offset / 4;\n");
    if finite {
        dump!(
            body,
            "let len = u16::try_from(word_len).map_err(|_| {})?;\n",
            SERIALIZE_ERROR
        );
        dump!(body, "let length: [u8; 2] = len.to_ne_bytes();\n");
        dump!(
            body,
            "buf_ptr.get_mut(2..4).ok_or({})?.copy_from_slice(&length);\n",
            SERIALIZE_ERROR
        );
    } else {
        dump!(body, "if let Ok(len) = u16::try_from(word_len) {\n");
        dump!(body, "let length: [u8; 2] = len.to_ne_bytes();\n");
        dump!(
            body,
            "buf_ptr.get_mut(2..4).ok_or({})?.copy_from_slice(&length);\n",
            SERIALIZE_ERROR
        );
        dump!(body, "} else {\n");
        dump!(
            body,
            "if word_len > self.max_request_size() {{\n return Err({}); \n}}\n",
            REQ_TO_BIG_ERROR
        );

        dump!(body, "let buf_ptr = self.write_buf();\n");
        dump!(
            body,
            "buf_ptr.get_mut(2..4).ok_or({})?.copy_from_slice(&[0, 0]);\n",
            SERIALIZE_ERROR
        );
        dump!(
        body,
        "let length: [u8; 4] = u32::try_from(word_len).map_err(|_| {})?.checked_add(1).ok_or({})?.to_ne_bytes();\n",
        REQ_TO_BIG_ERROR,
        REQ_TO_BIG_ERROR,
    );
        dump!(body, "buf_ptr.copy_within(4..offset, 8);\n");
        dump!(
            body,
            "buf_ptr.get_mut(4..8).ok_or({})?.copy_from_slice(&length);\n",
            SERIALIZE_ERROR
        );
        dump!(body, "offset += 4;\n");
        dump!(body, "}\n");
    }
    dump!(body, "self.advance_writer(offset);\n");

    mb = set_return_type(mb, &mut body, reply);
    mb.set_body(body)
}
