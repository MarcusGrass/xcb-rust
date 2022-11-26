use codegen_rs::structures::visibility::Visibility;
use codegen_rs::structures::RustType;
use codegen_rs::{ConstantBuilder, FileBuilder, RustCase};

use crate::generator::types::{XcbError, XcbErrorCopy};

pub(crate) fn implement_error(event: &XcbError, mut fb: FileBuilder) -> FileBuilder {
    let rust_name = event.name.clone();
    fb = fb.add_const(
        ConstantBuilder::const_builder(
            RustCase::convert_to_valid_rust(&rust_name, RustCase::Scream).unwrap(),
            RustType::in_scope("u8"),
            event.opcode.to_string(),
        )
        .set_visibility(Visibility::Public),
    );
    fb
}

pub(crate) fn implement_error_copy(event: &XcbErrorCopy, mut fb: FileBuilder) -> FileBuilder {
    let rust_name = event.name.clone();
    fb = fb.add_const(
        ConstantBuilder::const_builder(
            RustCase::convert_to_valid_rust(&rust_name, RustCase::Scream).unwrap(),
            RustType::in_scope("u8"),
            event.opcode.to_string(),
        )
        .set_visibility(Visibility::Public),
    );
    fb
}
