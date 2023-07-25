use std::fmt::Write;
use std::ops::Deref;

use codegen_rs::structures::generics::{Bound, Bounds};
use codegen_rs::structures::visibility::Visibility;
use codegen_rs::structures::{
    Annotation, Annotations, ComponentSignature, Import, Ownership, RustType, Signature,
};
use codegen_rs::{
    ConstantBuilder, ContainerStructBuilder, EnumBuilder, FileBuilder, FunctionBuilder,
    ImplBuilder, MethodBuilder, RustCase,
};

use crate::generator::codegen::enums::{implement_bitmask_enum, implement_value_enum};
use crate::generator::codegen::error::{implement_error, implement_error_copy};
use crate::generator::codegen::event::{
    implement_event, implement_event_copy, EvtNameSpec, EvtSpec,
};
use crate::generator::codegen::functions::clean_up_number_case;
use crate::generator::codegen::plain_struct::implement_plain_struct;
use crate::generator::codegen::reply::implement_reply;
use crate::generator::codegen::request::{implement_request, ReqSpec};
use crate::generator::codegen::simple_def_types::{
    implement_const_type, implement_typedef, implement_xid, implement_xid_union,
};
use crate::generator::codegen::switch::{implement_switch_enum, implement_switch_struct};
use crate::generator::types::{WrappedType, XcbType};
use crate::{dump, ReqNameSpec, Xcb};

pub(crate) mod enums;
mod error;
pub(crate) mod event;
mod format;
pub(crate) mod functions;
mod plain_struct;
mod reply;
pub(crate) mod request;
pub(crate) mod simple_def_types;
mod switch;
mod union;
mod xcb;

pub(crate) const FIX_LEN_SERIALIZE: &str = "FixedLengthSerialize";
pub(crate) const FIX_LEN_FROM_BYTES: &str = "FixedLengthFromBytes";
pub(crate) const VAR_LEN_SERIALIZE: &str = "VariableLengthSerialize";
pub(crate) const VAR_LEN_FROM_BYTES: &str = "VariableLengthFromBytes";

pub(crate) const FIX_LEN_SER_FUNCTION: &str = "crate::util::fixed_vec_serialize_into";
pub(crate) const VAR_LEN_SER_FUNCTION: &str = "crate::util::var_vec_serialize_into";
pub(crate) const BYTES_VEC_SER_FUNCTION: &str = "crate::util::u8_vec_serialize_into";

pub(crate) const FIX_LEN_FROM_BYTES_MACRO: &str = "crate::vec_from_bytes_fixed!";
pub(crate) const VAR_LEN_FROM_BYTES_MACRO: &str = "crate::vec_from_bytes_var!";
pub(crate) const BYTE_VEC_FROM_BYTES: &str = "crate::util::u8_vec_from_bytes";

pub(crate) const RESULT: &str = "crate::error::Result";
pub(crate) const SERIALIZE_ERROR: &str = "crate::error::Error::Serialize";
pub(crate) const FROM_BYTES_ERROR: &str = "crate::error::Error::FromBytes";
pub(crate) const TRY_FROM_INT_ERROR: &str = "crate::error::Error::TryFromInt";
pub(crate) const REQ_TO_BIG_ERROR: &str = "crate::error::Error::TooLargeRequest";

trait ProtoImported {
    fn rust_type_maybe_imported(&self, name: &str, current_pkg: &str) -> RustType;
}

pub(crate) trait AsRustFormatted {
    fn to_rust_valid_pascal(&self) -> String;
    fn to_rust_snake(&self) -> String;
}

impl AsRustFormatted for String {
    fn to_rust_valid_pascal(&self) -> String {
        if let Ok(name) = RustCase::convert_to_valid_rust(self, RustCase::Pascal) {
            name
        } else if let Ok(name) =
            RustCase::convert_to_valid_rust(&self.to_lowercase(), RustCase::Pascal)
        {
            name
        } else {
            RustCase::convert_to_valid_rust(&clean_up_number_case(self), RustCase::Pascal).unwrap()
        }
    }

    fn to_rust_snake(&self) -> String {
        if let Ok(name) = RustCase::convert_to_valid_rust(self, RustCase::Snake) {
            name
        } else {
            RustCase::convert_to_valid_rust(&self.to_lowercase(), RustCase::Snake).unwrap()
        }
    }
}

impl ProtoImported for Option<String> {
    fn rust_type_maybe_imported(&self, name: &str, current_pkg: &str) -> RustType {
        if let Some(pkg) = self {
            if pkg != current_pkg {
                return RustType::from_package(format!("crate::proto::{}", pkg), name);
            }
        }
        RustType::in_scope(name)
    }
}

pub(crate) fn generate_proto(
    resolved: &[WrappedType],
    req_name_spec: &mut ReqNameSpec,
    evt_name_spec: &mut EvtNameSpec,
    functions: &mut Vec<FunctionBuilder>,
    xcb: Xcb,
) -> FileBuilder {
    let mut fb = FileBuilder::new(&xcb.header);
    fb = fb
        .add_any("#[allow(unused_imports)]\n")
        .add_import(Import::FullType(RustType::from_package(
            "crate::util",
            FIX_LEN_SERIALIZE,
        )))
        .add_any("#[allow(unused_imports)]\n")
        .add_import(Import::FullType(RustType::from_package(
            "crate::util",
            FIX_LEN_FROM_BYTES,
        )))
        .add_any("#[allow(unused_imports)]\n")
        .add_import(Import::FullType(RustType::from_package(
            "crate::util",
            VAR_LEN_FROM_BYTES,
        )))
        .add_any("#[allow(unused_imports)]\n")
        .add_import(Import::FullType(RustType::from_package(
            "crate::util",
            VAR_LEN_SERIALIZE,
        )));
    if let Some(ext_name) = &xcb.extension_xname {
        fb = fb.add_const(
            ConstantBuilder::const_builder(
                "EXTENSION_NAME",
                RustType::in_scope("&str"),
                format!("\"{}\"", ext_name),
            )
            .set_visibility(Visibility::Public),
        );
    }
    for item in resolved {
        match &*item.deref().borrow() {
            XcbType::Xid(xid) => {
                fb = implement_xid(xid, fb);
            }
            XcbType::XidUnion(xu) => {
                fb = implement_xid_union(xu, fb);
            }
            XcbType::PlainStruct(s) => {
                fb = implement_plain_struct(s, &xcb, fb);
            }
            XcbType::UnionStruct(s) => {
                fb = union::implement_union(s, fb);
            }
            XcbType::BitmaskEnum(be) => {
                fb = implement_bitmask_enum(be, fb);
            }
            XcbType::ValueEnum(ve) => {
                fb = implement_value_enum(ve, &xcb, fb);
            }
            XcbType::Alias(ta) => {
                fb = implement_typedef(ta, &xcb, fb);
            }
            XcbType::Const(ct) => {
                fb = implement_const_type(ct, fb);
            }
            XcbType::SwitchStruct(ss) => {
                fb = implement_switch_struct(ss, &xcb, fb);
            }
            XcbType::SwitchEnum(se) => {
                fb = implement_switch_enum(se, &xcb, fb);
            }
            XcbType::Request(req) => {
                let (func_b, new_fb) = implement_request(req, &xcb, req_name_spec, fb);
                fb = new_fb;
                functions.push(func_b);
            }
            // Trivial wrapper over an event that's always 32 bytes
            XcbType::EventStruct(es) => {
                let struct_name =
                    RustCase::convert_to_valid_rust(&es.name, RustCase::Pascal).unwrap();
                fb = fb.add_container_struct(
                    ContainerStructBuilder::new(&struct_name)
                        .set_visibility(Visibility::Public)
                        .add_derive_in_scope("Debug")
                        .add_derive_in_scope("Copy")
                        .add_derive_in_scope("Clone")
                        .add_contained(Visibility::Public, RustType::in_scope("[u8; 32]")),
                );
                fb = fb.add_impl(
                    ImplBuilder::new(Signature::simple(RustType::in_scope(&struct_name)))
                        .implement_for(Signature::simple(RustType::in_scope(format!(
                            "{}<32>",
                            FIX_LEN_SERIALIZE
                        ))))
                        .add_method(
                            MethodBuilder::new("serialize_fixed")
                                .set_self_ownership(Ownership::Owned)
                                .set_body("self.0")
                                .set_return_type(ComponentSignature::Signature(Signature::simple(
                                    RustType::in_scope("[u8; 32]"),
                                ))),
                        )
                    ,
                ).add_impl(ImplBuilder::new(Signature::simple(RustType::in_scope(&struct_name)))
                    .implement_for(Signature::simple(RustType::in_scope(format!(
                        "{}<32>",
                        FIX_LEN_FROM_BYTES
                    ))))

                    .add_method(
                        MethodBuilder::new("from_bytes")
                            .add_argument_in_scope_simple_type(Ownership::Ref, "buf", "[u8]")
                            .set_body("Ok(Self(buf.get(0..32).ok_or(crate::error::Error::FromBytes)?.try_into().map_err(|_| crate::error::Error::FromBytes)?))")
                            .set_return_type(ComponentSignature::Signature(Signature::simple(
                                RustType::in_scope(format!("{}<Self>", RESULT)),
                            ))),
                    ));
            }
            XcbType::Reply(r) => {
                fb = implement_reply(r, &xcb, fb);
            }
            XcbType::Event(e) => {
                fb = implement_event(e, evt_name_spec, &xcb, fb);
            }
            XcbType::EventCopy(ec) => {
                fb = implement_event_copy(ec, evt_name_spec, &xcb, fb);
            }
            XcbType::Error(e) => {
                fb = implement_error(e, fb);
            }
            XcbType::ErrorCopy(e) => {
                fb = implement_error_copy(e, fb);
            }
            // Not constructed at this stage
            XcbType::Builtin(_) | XcbType::Constructed(_) => {}
        }
    }
    fb
}

pub(crate) fn implement_req_name(mut req_name_spec: ReqNameSpec) -> FunctionBuilder {
    let req_name = FunctionBuilder::new("request_name")
        .set_visibility(Visibility::PublicCrate)
        .add_argument_in_scope_simple_type(Ownership::Owned, "extension", "Option<&str>")
        .add_argument_in_scope_simple_type(Ownership::Owned, "major_opcode", "u8")
        .add_argument_in_scope_simple_type(Ownership::Owned, "minor_opcode", "u16")
        .set_return_type(ComponentSignature::Signature(Signature::simple(
            RustType::in_scope("Option<&'static str>"),
        )));
    let mut req_name_body = "match major_opcode {\n".to_string();
    req_name_spec
        .xproto
        .sort_by(|a, b| a.major_opcode.cmp(&b.major_opcode));
    for req in req_name_spec.xproto {
        dump!(
            req_name_body,
            "#[cfg(feature = \"xproto\")]\n {} => return Some(\"{}\"),\n",
            req.major_opcode,
            req.req_name,
        );
    }
    dump!(req_name_body, "_ => {}\n}\n");
    let mut exts = req_name_spec
        .ext
        .into_iter()
        .collect::<Vec<(String, Vec<ReqSpec>)>>();
    exts.sort_by_key(|v| v.0.clone());

    dump!(req_name_body, "match (extension, minor_opcode) {\n");
    for (ext_name, mut req) in exts {
        req.sort_by_key(|r| r.major_opcode);
        for r in req {
            dump!(
                req_name_body,
                "#[cfg(feature = \"{}\")]\n (Some({}::EXTENSION_NAME), {}) => Some(\"{}\"),\n",
                ext_name,
                ext_name,
                r.major_opcode,
                r.req_name
            );
        }
    }
    dump!(req_name_body, "_ => None,\n}");

    req_name.set_body(req_name_body)
}

pub(crate) fn add_evt(mut evt_name_spec: EvtNameSpec, fb: FileBuilder) -> FileBuilder {
    let mut enum_builder = EnumBuilder::new("Event")
        .set_visibility(Visibility::Public)
        .add_derive_in_scope("Debug")
        .add_derive_in_scope("Clone")
        .add_type_member(
            "Unknown",
            Signature::simple(RustType::from_package("alloc::vec", "Vec<u8>")),
        );
    let evt_from_bytes = MethodBuilder::new("from_bytes")
        .set_visibility(Visibility::Public)
        .add_argument_in_scope_simple_type(Ownership::Ref, "raw", "[u8]")
        .add_argument_bounded_generic(
            Ownership::Ref,
            "ext_provider",
            "E",
            Bounds::single(Bound::new(
                RustType::from_package("crate::util", "ExtensionInfoProvider"),
                false,
            )),
        )
        .set_return_type(ComponentSignature::Signature(Signature::simple(
            RustType::in_scope(format!("{}<Event>", RESULT)),
        )));

    let mut parse_evt_body = "use crate::util::FixedLengthFromBytes;\n".to_string();
    dump!(
        parse_evt_body,
        "let opcode = crate::util::response_type(raw)?;\n"
    );
    dump!(parse_evt_body, "match opcode {\n");
    evt_name_spec
        .xproto
        .sort_by(|a, b| a.major_opcode.cmp(&b.major_opcode));
    for evt in evt_name_spec.xproto {
        if evt.generic {
            continue;
        }
        let from_bytes = if evt.fixed {
            "from_bytes(raw)?"
        } else {
            "from_bytes(raw)?.0"
        };
        enum_builder = enum_builder.add_type_member_with_annotations(
            &evt.evt_name,
            Signature::simple(RustType::from_package("xproto", &evt.evt_name)),
            Annotations::new(vec![Annotation::new(format!("cfg(feature = \"xproto\")",))]),
        );
        dump!(
            parse_evt_body,
            "#[cfg(feature = \"xproto\")]\n {} => return Ok(Self::{}(xproto::{}::{})),\n",
            evt.major_opcode,
            evt.evt_name,
            evt.evt_name,
            from_bytes,
        );
    }
    dump!(parse_evt_body, "_ => {}\n}\n");
    let mut exts = evt_name_spec
        .ext
        .into_iter()
        .collect::<Vec<(String, Vec<EvtSpec>)>>();
    exts.sort_by_key(|v| v.0.clone());

    dump!(
        parse_evt_body,
        "let ext_info = ext_provider.get_from_event_code(opcode);\n"
    );
    dump!(parse_evt_body, "match ext_info {\n");
    for (ext_name, mut req) in exts {
        req.sort_by_key(|r| r.major_opcode);
        dump!(
            parse_evt_body,
            "#[cfg(feature = \"{}\")]\n Some(({}::EXTENSION_NAME, info)) => {{\n",
            ext_name,
            ext_name,
        );
        dump!(parse_evt_body, "match opcode - info.first_event {\n");
        for r in req {
            if r.generic {
                continue;
            }
            let case_name = format!("{}{}", ext_name.to_rust_valid_pascal(), r.evt_name);
            enum_builder = enum_builder.add_type_member_with_annotations(
                &case_name,
                Signature::simple(RustType::from_package(&ext_name, &r.evt_name)),
                Annotations::new(vec![Annotation::new(format!(
                    "cfg(feature = \"{}\")",
                    ext_name
                ))]),
            );
            let from_bytes = if r.fixed {
                "from_bytes(raw)?"
            } else {
                "from_bytes(raw)?.0"
            };
            dump!(
                parse_evt_body,
                "{} => Ok(Self::{}({}::{}::{})),\n",
                r.major_opcode,
                case_name,
                ext_name,
                r.evt_name,
                from_bytes
            );
        }
        dump!(
            parse_evt_body,
            "_ => Ok(Event::Unknown(raw.to_vec())),\n}\n}\n"
        );
    }
    dump!(parse_evt_body, "_ => Ok(Event::Unknown(raw.to_vec())),\n}");
    let n = enum_builder.name.clone();
    fb.add_enum(enum_builder).add_impl(
        ImplBuilder::new(Signature::simple(RustType::in_scope(n)))
            .add_method(evt_from_bytes.set_body(parse_evt_body)),
    )
}
