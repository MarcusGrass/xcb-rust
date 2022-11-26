use std::cell::RefCell;
use std::rc::Rc;

use codegen_rs::FileBuilder;

use crate::generator::codegen::plain_struct::implement_plain;
use crate::generator::types::{
    EntityField, EntityMember, FieldNumberRef, TypeHelpers, WrappedType, XcbBuiltin, XcbReply,
    XcbType,
};
use crate::Xcb;

pub(crate) fn implement_reply(reply: &XcbReply, xcb: &Xcb, fb: FileBuilder) -> FileBuilder {
    let switches = reply
        .switch
        .clone()
        .into_iter()
        .collect::<Vec<WrappedType>>();
    let mut members = vec![];
    members.push(EntityMember::Field(EntityField {
        name: "response_type".to_string(),
        kind: FieldNumberRef {
            concrete: Rc::new(RefCell::new(XcbType::Builtin(XcbBuiltin::Card8))),
            reference_type: None,
        },
    }));
    let start_ind =
        if !reply.fields.is_empty() && reply.fields[0].byte_size().filter(|b| b == &1).is_some() {
            members.push(reply.fields[0].clone());
            1
        } else {
            0
        };
    members.push(EntityMember::Field(EntityField {
        name: "sequence".to_string(),
        kind: FieldNumberRef {
            concrete: Rc::new(RefCell::new(XcbType::Builtin(XcbBuiltin::Card16))),
            reference_type: None,
        },
    }));
    members.push(EntityMember::Field(EntityField {
        name: "length".to_string(),
        kind: FieldNumberRef {
            concrete: Rc::new(RefCell::new(XcbType::Builtin(XcbBuiltin::Card32))),
            reference_type: None,
        },
    }));
    members.extend_from_slice(&reply.fields[start_ind..]);

    implement_plain(
        reply.rust_entity_name(),
        &members,
        &switches,
        reply.source_type(),
        false,
        xcb,
        fb,
    )
}
