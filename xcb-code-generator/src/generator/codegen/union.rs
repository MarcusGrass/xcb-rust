use crate::generator::codegen::{AsRustFormatted, FROM_BYTES_ERROR, RESULT};
use crate::generator::types::{EntityMember, TypeHelpers, XcbUnion};
use codegen_rs::structures::visibility::Visibility;
use codegen_rs::structures::{ComponentSignature, Ownership, RustType, Signature};
use codegen_rs::{ContainerStructBuilder, FileBuilder, ImplBuilder, MethodBuilder};

pub(crate) fn implement_union(s: &XcbUnion, fb: FileBuilder) -> FileBuilder {
    let mut biggest = 0;
    for member in &s.members {
        match member {
            EntityMember::Field(f) => {
                let field_size = f.kind.use_field().byte_size().unwrap();
                if biggest < field_size {
                    biggest = field_size;
                }
            }
            EntityMember::List(l) => {
                let list_size =
                    l.fixed_count.unwrap() * l.field.kind.use_field().byte_size().unwrap();
                if biggest < list_size {
                    biggest = list_size;
                }
            }
            EntityMember::Pad(p) => {
                if biggest < *p {
                    biggest = *p;
                }
            }
            EntityMember::Align(_) | EntityMember::StartAlign(_) | EntityMember::Length(_) => {
                panic!("Bad member in union {member:?}");
            }
        }
    }
    let rust_name = s.name.to_rust_valid_pascal();
    // eprintln!("Union {} size {}", s.name, biggest);
    let byte_type = format!("[u8; {}]", biggest);
    let builder = ContainerStructBuilder::new(&rust_name)
        .set_visibility(Visibility::Public)
        .add_derive_in_scope("Debug")
        .add_derive_in_scope("Copy")
        .add_derive_in_scope("Clone")
        .add_derive_in_scope("Eq")
        .add_derive_in_scope("PartialEq")
        .add_contained(Visibility::Public, RustType::in_scope(&byte_type));
    fb.add_container_struct(builder).add_impl(
        ImplBuilder::new(Signature::simple(RustType::in_scope(&rust_name)))
            .implement_for(Signature::simple(RustType::in_scope(format!(
                "FixedLengthFromBytes<{}>",
                biggest
            ))))
            .add_method(
                MethodBuilder::new("from_bytes")
                    .add_argument_in_scope_simple_type(Ownership::Ref, "bytes", "[u8]")
                    .set_return_type(ComponentSignature::Signature(Signature::simple(
                        RustType::in_scope(format!("{}<Self>", RESULT)),
                    )))
                    .set_body(format!("// SAFETY: Checked that the bytes are available\n Ok(Self(unsafe {{bytes.get(..{}).ok_or({})?.try_into().unwrap_unchecked()}}))", biggest, FROM_BYTES_ERROR)),
            ),
    )
        .add_impl(ImplBuilder::new(Signature::simple(RustType::in_scope(&rust_name)))
            .implement_for(Signature::simple(RustType::in_scope(
                format!("FixedLengthSerialize<{}>", biggest)
            )))
            .add_method(MethodBuilder::new("serialize_fixed")
                .set_self_ownership(Ownership::Owned)
                .add_simple_annotation("inline")
                .set_return_type(ComponentSignature::Signature(Signature::simple(
                    RustType::in_scope(&byte_type)
                )))
                .set_body("self.0")
            )
        )
}
