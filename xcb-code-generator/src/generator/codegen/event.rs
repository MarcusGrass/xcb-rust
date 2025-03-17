use std::cell::RefCell;
use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::rc::Rc;

use codegen_rs::structures::visibility::Visibility;
use codegen_rs::structures::RustType;
use codegen_rs::{ConstantBuilder, FileBuilder, RustCase};

use crate::generator::codegen::plain_struct::implement_plain;
use crate::generator::types::{
    EntityField, EntityMember, FieldNumberRef, TypeHelpers, XcbBuiltin, XcbEvent, XcbEventCopy,
    XcbType,
};
use crate::Xcb;

#[derive(Debug, Default)]
pub(crate) struct EvtNameSpec {
    pub(crate) xproto: Vec<EvtSpec>,
    pub(crate) ext: HashMap<String, Vec<EvtSpec>>,
}

#[derive(Debug, Eq, PartialEq, Hash)]
pub(crate) struct EvtSpec {
    pub(crate) major_opcode: u8,
    pub(crate) evt_name: String,
    pub(crate) fixed: bool,
    pub(crate) generic: bool,
}

pub(crate) fn implement_event(
    event: &XcbEvent,
    evt_spec: &mut EvtNameSpec,
    xcb: &Xcb,
    mut fb: FileBuilder,
) -> FileBuilder {
    let rust_name = event.rust_entity_name();
    let evt_s = EvtSpec {
        major_opcode: event.opcode,
        evt_name: rust_name.clone(),
        fixed: event.byte_size().is_some(),
        generic: event.generic,
    };
    if xcb.header == "xproto" {
        evt_spec.xproto.push(evt_s);
    } else {
        match evt_spec.ext.entry(xcb.header.clone()) {
            Entry::Occupied(mut o) => {
                o.get_mut().push(evt_s);
            }
            Entry::Vacant(v) => {
                v.insert(vec![evt_s]);
            }
        }
    }
    let mut modified_members = vec![];
    modified_members.push(EntityMember::Field(EntityField {
        name: "opcode".to_string(),
        kind: FieldNumberRef {
            concrete: Rc::new(RefCell::new(XcbType::Builtin(XcbBuiltin::Card8))),
            reference_type: None,
        },
    }));
    let start_ind = if !event.members.is_empty()
        && event.members[0].byte_size().filter(|bs| bs == &1).is_some()
    {
        modified_members.push(event.members[0].clone());
        1
    } else {
        0
    };
    if event.has_sequence {
        modified_members.push(EntityMember::Field(EntityField {
            name: "sequence".to_string(),
            kind: FieldNumberRef {
                concrete: Rc::new(RefCell::new(XcbType::Builtin(XcbBuiltin::Card16))),
                reference_type: None,
            },
        }));
    }
    modified_members.extend_from_slice(&event.members[start_ind..]);

    fb = fb.add_const(
        ConstantBuilder::const_builder(
            RustCase::convert_to_valid_rust(&rust_name, RustCase::Scream).unwrap(),
            RustType::in_scope("u8"),
            event.opcode.to_string(),
        )
        .set_visibility(Visibility::Public),
    );
    fb = implement_plain(
        rust_name,
        &modified_members,
        &[],
        event.source_type(),
        false,
        xcb,
        fb,
    );
    fb
}

pub(crate) fn implement_event_copy(
    event: &XcbEventCopy,
    event_name_spec: &mut EvtNameSpec,
    xcb: &Xcb,
    mut fb: FileBuilder,
) -> FileBuilder {
    let rust_name = format!(
        "{}Event",
        RustCase::convert_to_valid_rust(&event.name, RustCase::Pascal).unwrap()
    );
    if let XcbType::Event(evt) = &*event.copy_type.borrow() {
        let evt_s = EvtSpec {
            major_opcode: event.opcode,
            evt_name: rust_name.clone(),
            fixed: evt.byte_size().is_some(),
            generic: evt.generic,
        };
        if xcb.header == "xproto" {
            event_name_spec.xproto.push(evt_s);
        } else {
            match event_name_spec.ext.entry(xcb.header.clone()) {
                Entry::Occupied(mut o) => {
                    o.get_mut().push(evt_s);
                }
                Entry::Vacant(v) => {
                    v.insert(vec![evt_s]);
                }
            }
        }
    } else {
        panic!("Event copy not copying event");
    };

    fb = fb.add_const(
        ConstantBuilder::const_builder(
            RustCase::convert_to_valid_rust(&rust_name, RustCase::Scream).unwrap(),
            RustType::in_scope("u8"),
            event.opcode.to_string(),
        )
        .set_visibility(Visibility::Public),
    );
    fb = fb.add_type_def_simple_type(
        Visibility::Public,
        rust_name,
        RustType::in_scope(event.copy_type.import_name(&xcb.header)),
    );
    fb
}
