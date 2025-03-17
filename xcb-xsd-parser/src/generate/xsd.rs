use std::collections::HashMap;
use std::fmt::Write;

use anyhow::Result;
use codegen_rs::structures::gen_enum::NamedComponentSignature;
use codegen_rs::structures::method::Argument;
use codegen_rs::structures::visibility::Visibility;
use codegen_rs::structures::{ComponentSignature, Import, Ownership, RustType, Signature};
use codegen_rs::{
    EnumBuilder, FileBuilder, ImplBuilder, MethodBuilder, ModuleBuilder, RustCase, StructBuilder,
};

#[allow(clippy::wildcard_imports)]
use xcb_xsd::parse::const_tags::*;
use xcb_xsd::parse::raw_xml_parse::{parse, TagItem, TagSpec};

const ATTRIBUTE: &str = "attribute";

pub(crate) fn generate_xsd_spec(bytes: &[u8]) -> Result<ModuleBuilder> {
    let document_tag = parse(bytes, TagSpec::new("schema"))?;
    //let res = recurse_generate(&document_tag);
    let pt = parse_tag(&document_tag);
    Ok(gen_document(pt))
}

fn parse_tag(tag: &TagItem) -> ParsedTag {
    match tag.name.as_str() {
        ATTRIBUTE => parse_attribute(tag),
        ELEMENT => parse_element(tag),
        GROUP => parse_group(tag),
        COMPLEX_TYPE => parse_complex_type(tag),
        CHOICE => parse_choice(tag),
        SEQUENCE => parse_sequence(tag),
        SIMPLE_TYPE => parse_simple_type(tag),
        SIMPLE_CONTENT => parse_simple_content(tag),
        COMPLEX_CONTENT => parse_complex_content(tag),
        SCHEMA | EXTENSION => parse_transparent(tag),
        _ => panic!("Unrecognized tag {}", tag.name),
    }
}

fn parse_attribute(tag: &TagItem) -> ParsedTag {
    // All attributes have a name param
    let name = tag.extract_attr("name").unwrap();
    let must_use_val = tag.extract_attr("use");
    let must_use = if let Ok(must_use_val) = must_use_val {
        match must_use_val.as_str() {
            "optional" => false,
            "required" => true,
            _ => unreachable!(),
        }
    } else {
        false
    };
    let r#type = if tag.inner.is_empty() {
        let (_, t) = base_type_to_rust_valid(&tag.extract_attr("type").unwrap()).unwrap();
        t.to_string()
    } else {
        assert_eq!(1, tag.inner.len());
        let simple = &tag.inner[0];
        assert_eq!("simpleType", &simple.name);
        assert_eq!(1, simple.inner.len());
        let restriction = &simple.inner[0];
        assert_eq!(1, restriction.inner.len());
        let pattern = &restriction.inner[0];
        assert_eq!("pattern", &pattern.name);
        "String".to_string()
    };
    let attr = ParsedAttribute {
        name: name.clone(),
        r#type,
        must_use,
    };
    ParsedTag {
        built_structs: vec![],
        built_type_defs: vec![],
        built_enums: vec![],
        fields: vec![FieldDef {
            suggested_name: RustCase::convert_to_valid_rust(&name, RustCase::Snake).unwrap(),
            df: DirectField::Attr(attr),
        }],
    }
}

fn parse_complex_content(tag: &TagItem) -> ParsedTag {
    // Wrapper type
    assert_eq!(1, tag.inner.len());
    let tgt = &tag.inner[0];

    let ext = tgt.extract_attr(BASE).unwrap();
    let mut pt = ParsedTag::default_from_tag(tag);
    let ext_f_name = RustCase::convert_to_valid_rust(&ext, RustCase::Snake).unwrap();
    let ext_t_name = RustCase::convert_to_valid_rust(&ext, RustCase::Pascal).unwrap();
    pt.fields.push(FieldDef {
        suggested_name: ext_f_name,
        df: DirectField::Element(ElementFieldDef {
            element_name: ElementNameRef::Me,
            field_type: ElementFieldType::Constructed(ext_t_name, false),
            occurrence: Occurrence::extract_from_tag(tag),
        }),
    });
    // Actually fields for the above type
    for inner in &tgt.inner {
        let parsed = parse_tag(inner);
        pt.merge_full(parsed);
    }
    pt
}

fn parse_simple_content(tag: &TagItem) -> ParsedTag {
    assert_eq!(1, tag.inner.len());
    let mut pt = ParsedTag::default_from_tag(tag);
    let tgt = &tag.inner[0];
    let ext = tgt.extract_attr(BASE).unwrap();
    assert_eq!(STRING, &ext);
    let elem_name = tag.extract_first_named_parent().unwrap();
    pt.fields.push(FieldDef {
        suggested_name: "string".to_string(),
        df: DirectField::Element(ElementFieldDef {
            element_name: ElementNameRef::Child(elem_name),
            field_type: ElementFieldType::Builtin("String".to_owned()),
            occurrence: Occurrence::extract_from_tag(tag),
        }),
    });
    for inner in &tgt.inner {
        // Get attributes out
        pt.merge_full(parse_tag(inner));
    }
    pt
}

fn parse_simple_type(tag: &TagItem) -> ParsedTag {
    let name = tag.extract_attr(NAME).unwrap();
    // Should only be a restriction
    assert_eq!(1, tag.inner.len());
    let mut pt = ParsedTag::default_from_tag(tag);
    let base = &tag.inner[0].extract_attr(BASE).unwrap();
    let struct_name = RustCase::convert_to_valid_rust(&name, RustCase::Pascal).unwrap();
    let (f, t) = base_type_to_rust_valid(base).unwrap();
    let field_def = FieldDef {
        suggested_name: f.to_string(),
        df: DirectField::Element(ElementFieldDef {
            element_name: ElementNameRef::Child(name.clone()),
            field_type: ElementFieldType::Builtin(t.to_string()),
            occurrence: Occurrence::extract_from_tag(tag),
        }),
    };
    pt.fields.push(field_def);
    pt.generate_and_push_struct(name, struct_name, Occurrence::extract_from_tag(tag));
    pt
}

fn parse_complex_type(tag: &TagItem) -> ParsedTag {
    if let Some(parent) = tag.parent.as_ref() {
        if parent.name == ELEMENT {
            // This tag is transparent
            parse_transparent(tag)
        } else {
            let mut pt = ParsedTag::default_from_tag(tag);
            let elem_name = tag.extract_attr(NAME).unwrap();
            unpack_struct(elem_name.as_str(), tag, &mut pt);
            pt
        }
    } else {
        unreachable!();
    }
}

fn parse_choice(tag: &TagItem) -> ParsedTag {
    let mut pt = ParsedTag::default_from_tag(tag);
    if let Some(parent) = tag.parent.as_ref() {
        if parent.name == GROUP {
            // Proper enum
            let elem_name = parent.attrs[NAME].clone();
            let enum_name = RustCase::convert_to_valid_rust(&elem_name, RustCase::Pascal).unwrap();
            let mut any_name = vec![];
            for inner in &tag.inner {
                let inner_pt = parse_tag(inner);
                pt.merge_full(inner_pt);
                let name = if let Ok(name) = inner.extract_attr(NAME) {
                    name
                } else if let Ok(r) = inner.extract_attr(REF) {
                    r
                } else {
                    panic!("Can't name sub-tag");
                };
                any_name.push(name);
            }

            pt.generate_and_push_enum(
                ElementNameRef::Either(enum_name.clone(), any_name),
                enum_name,
                Occurrence::extract_from_tag(tag),
            );
        } else if tag.inner.len() == 1 {
            let elem = parse_tag(&tag.inner[0]);
            pt.merge_full(elem);
            // If min/max exists on the choice, use it
            if tag.extract_attr(MIN_OCCURS).is_ok() {
                match &mut pt.fields[0].df {
                    DirectField::Attr(_) => unreachable!(),
                    DirectField::Element(e) => {
                        e.occurrence = Occurrence::extract_from_tag(tag);
                    }
                }
            }
        } else {
            // Element two steps above
            let mut p = tag.parent.as_ref().unwrap();
            let p = loop {
                if p.attrs.contains_key(NAME) {
                    break p;
                }
                p = p.parent.as_ref().unwrap();
            };
            let name = p.attrs[NAME].clone();
            let enum_name = format!("{name}Choice");
            let enum_name = RustCase::convert_to_valid_rust(&enum_name, RustCase::Pascal).unwrap();
            let mut any_name = vec![];
            for inner in &tag.inner {
                let inner_pt = parse_tag(inner);
                pt.merge_full(inner_pt);
                let name = if let Ok(name) = inner.extract_attr(NAME) {
                    name
                } else if let Ok(r) = inner.extract_attr(REF) {
                    r
                } else {
                    panic!("Can't name sub-tag");
                };
                any_name.push(name);
            }
            let occ = Occurrence::extract_from_tag(tag);
            pt.generate_and_push_enum(
                ElementNameRef::Either(enum_name.clone(), any_name),
                enum_name,
                occ,
            );
        }
        return pt;
    }
    unreachable!()
}

fn parse_element(tag: &TagItem) -> ParsedTag {
    let mut pt = ParsedTag::default_from_tag(tag);
    if let Ok(r#ref) = tag.extract_attr(REF) {
        let ref_field = RustCase::convert_to_valid_rust(&r#ref, RustCase::Snake).unwrap();
        let ref_type = RustCase::convert_to_valid_rust(&r#ref, RustCase::Pascal).unwrap();
        pt.fields.push(FieldDef {
            suggested_name: ref_field,
            df: DirectField::Element(ElementFieldDef {
                element_name: ElementNameRef::Child(r#ref),
                field_type: ElementFieldType::Constructed(ref_type, false),
                occurrence: Occurrence::extract_from_tag(tag),
            }),
        });
        return pt;
    }
    let name = tag.extract_attr(NAME).unwrap();
    if let Ok(r#type) = tag.extract_attr(TYPE) {
        // Type defs
        if tag.inner.is_empty() && tag.level() == 1 {
            let type_a = RustCase::convert_to_valid_rust(&name, RustCase::Pascal).unwrap();
            let type_b = RustCase::convert_to_valid_rust(&r#type, RustCase::Pascal).unwrap();
            pt.built_type_defs = vec![TypeRef {
                target_type: type_b,
                name: type_a,
            }];
            return pt;
        }
        let field_name = RustCase::convert_to_valid_rust(&name, RustCase::Snake).unwrap();
        if let Some((_f, t)) = base_type_to_rust_valid(&r#type) {
            pt.fields.push(FieldDef {
                suggested_name: field_name,
                df: DirectField::Element(ElementFieldDef {
                    element_name: ElementNameRef::Child(name),
                    field_type: ElementFieldType::Builtin(t.to_string()),
                    occurrence: Occurrence::extract_from_tag(tag),
                }),
            });
        } else {
            let ref_type = RustCase::convert_to_valid_rust(&r#type, RustCase::Pascal).unwrap();
            pt.fields.push(FieldDef {
                suggested_name: field_name,
                df: DirectField::Element(ElementFieldDef {
                    element_name: ElementNameRef::Child(name),
                    field_type: ElementFieldType::Constructed(ref_type, false),
                    occurrence: Occurrence::extract_from_tag(tag),
                }),
            });
        }
        pt
    } else {
        unpack_struct(&name, tag, &mut pt);
        pt
    }
}

fn parse_group(tag: &TagItem) -> ParsedTag {
    let mut pt = ParsedTag::default_from_tag(tag);
    if tag.inner.len() == 1 && tag.inner[0].name == CHOICE {
        // enum
        for inner in &tag.inner {
            let inner_pt = parse_tag(inner);
            pt.merge_full(inner_pt);
        }
    } else if let Ok(name) = tag.extract_attr(NAME) {
        // struct
        unpack_struct(&name, tag, &mut pt);
    } else if let Ok(r#ref) = tag.extract_attr(REF) {
        assert!(tag.inner.is_empty());
        let suggest = RustCase::convert_to_valid_rust(&r#ref, RustCase::Snake).unwrap();
        let ft = RustCase::convert_to_valid_rust(&r#ref, RustCase::Pascal).unwrap();
        pt.fields.push(FieldDef {
            suggested_name: suggest,
            df: DirectField::Element(ElementFieldDef {
                element_name: ElementNameRef::Child(r#ref),
                field_type: ElementFieldType::Constructed(ft, false),
                occurrence: Occurrence::extract_from_tag(tag),
            }),
        });
    } else {
        panic!("Bad group");
    }
    pt
}

fn parse_sequence(tag: &TagItem) -> ParsedTag {
    let mut pt = ParsedTag::default_from_tag(tag);
    for inner in &tag.inner {
        let inner_pt = parse_tag(inner);
        pt.merge_full(inner_pt);
    }
    pt
}

fn parse_transparent(tag: &TagItem) -> ParsedTag {
    let mut pt = ParsedTag::default_from_tag(tag);
    for inner in &tag.inner {
        let downstream = parse_tag(inner);
        pt.merge_full(downstream);
    }
    pt
}

fn unpack_struct(elem_name: &str, tag: &TagItem, pt: &mut ParsedTag) {
    let name = RustCase::convert_to_valid_rust(elem_name, RustCase::Pascal).unwrap();
    let (name, occ) = if let Some(parent) = tag.extract_first_named_parent() {
        if tag.level() == 1 {
            (name, None)
        } else {
            let prefix = RustCase::convert_to_valid_rust(&parent, RustCase::Pascal).unwrap();
            let occ = if let Some((min, max)) = tag.extract_first_unnamed_parent_occurences() {
                Some(Occurrence::from_min_max(min, max))
            } else {
                None
            };
            (format!("{prefix}{name}"), occ)
        }
    } else {
        (name, None)
    };
    for inner in &tag.inner {
        let inner_pt = parse_tag(inner);
        pt.merge_full(inner_pt);
    }

    pt.generate_and_push_struct(
        elem_name.to_string(),
        name,
        occ.unwrap_or_else(|| Occurrence::extract_from_tag(tag)),
    );
}

fn base_type_to_rust_valid(input: &str) -> Option<(&'static str, &'static str)> {
    match input {
        STRING => Some(("string", "String")),
        INTEGER => Some(("integer", "i32")),
        BOOLEAN => Some(("boolean", "bool")),
        UINT => Some(("uint", "u32")),
        _ => None,
    }
}

#[derive(Debug, Clone)]
pub(crate) struct ParsedTag {
    built_structs: Vec<StructBuild>,
    built_type_defs: Vec<TypeRef>,
    built_enums: Vec<EnumBuild>,
    fields: Vec<FieldDef>,
}

#[derive(Debug, Clone)]
struct TypeRef {
    target_type: String,
    name: String,
}

impl ParsedTag {
    fn default_from_tag(_tag: &TagItem) -> Self {
        Self {
            built_structs: vec![],
            built_type_defs: vec![],
            built_enums: vec![],
            fields: vec![],
        }
    }

    fn merge_full(&mut self, other: Self) {
        self.built_structs.extend(other.built_structs);
        self.built_enums.extend(other.built_enums);
        self.built_type_defs.extend(other.built_type_defs);
        self.fields.extend(other.fields);
    }

    fn generate_and_push_struct(
        &mut self,
        element_name: String,
        struct_name: String,
        occurrence: Occurrence,
    ) {
        let gen = generate_struct(element_name.clone(), struct_name.clone(), &mut self.fields);
        self.built_structs.push(gen);
        let s_name = RustCase::convert_to_valid_rust(&struct_name, RustCase::Snake).unwrap();
        let fd = FieldDef {
            suggested_name: s_name,
            df: DirectField::Element(ElementFieldDef {
                element_name: ElementNameRef::Child(element_name),
                field_type: ElementFieldType::Constructed(struct_name, false),
                occurrence,
            }),
        };
        self.fields.clear();
        self.fields.push(fd);
    }

    fn generate_and_push_enum(
        &mut self,
        element_name: ElementNameRef,
        enum_name: String,
        occurrence: Occurrence,
    ) {
        let (gen, inherits_tag_index) = generate_enum(enum_name.clone(), &self.fields);
        self.built_enums.push(gen);
        let e_name = RustCase::convert_to_valid_rust(&enum_name, RustCase::Snake).unwrap();
        let fd = FieldDef {
            suggested_name: e_name,
            df: DirectField::Element(ElementFieldDef {
                element_name,
                field_type: ElementFieldType::Constructed(enum_name, inherits_tag_index),
                occurrence,
            }),
        };
        self.fields.clear();
        self.fields.push(fd);
    }
}

#[derive(Debug, Clone)]
pub(crate) struct StructBuild {
    builder: StructBuilder,
    from_tag_impl: String,
    valid_tag_impl: String,
}

#[derive(Debug, Clone)]
pub(crate) struct EnumBuild {
    builder: EnumBuilder,
    from_tag_impl: String,
    valid_tag_impl: String,
    inherits_tag_index: bool,
}

#[derive(Debug, Clone)]
pub(crate) struct ParsedAttribute {
    name: String,
    r#type: String,
    must_use: bool,
}

fn merge_struct_valid_tag(a: &str, b: &str) -> String {
    let elem_a = RustCase::convert_to_valid_rust(a, RustCase::Snake).unwrap();
    let elem_b = RustCase::convert_to_valid_rust(b, RustCase::Snake).unwrap();
    format!("tag_name == \"{elem_a}\" || tag_name == \"{elem_b}\"")
}

fn gen_document(main: ParsedTag) -> ModuleBuilder {
    let mut fb = FileBuilder::new("mod").add_import(Import::Spec(
        "xcb_xsd::parse::raw_xml_parse::TagItem".to_string(),
    ));
    let mut dup_type_defs = HashMap::new();
    for tr in main.built_type_defs {
        dup_type_defs.insert(tr.target_type.clone(), tr);
    }
    for inner in main.built_structs {
        let struct_valid_tag_body = if let Some(t) = dup_type_defs.get(&inner.builder.name) {
            merge_struct_valid_tag(&t.name, &t.target_type)
        } else {
            inner.valid_tag_impl.clone()
        };
        if let Some(tr) = dup_type_defs.get(&inner.builder.name) {
            let mut cloned_struct = inner.builder.clone();
            cloned_struct.name.clone_from(&tr.name);
            fb = fb.add_struct(cloned_struct).add_impl(struct_impl_builder(
                &tr.name,
                &struct_valid_tag_body,
                &inner.from_tag_impl,
            ));
        }
        fb = fb
            .add_struct(inner.builder.clone())
            .add_impl(struct_impl_builder(
                &inner.builder.name,
                &struct_valid_tag_body,
                &inner.from_tag_impl,
            ));
    }
    for inner in main.built_enums {
        let mut from_tag_builder = MethodBuilder::new("from_tag")
            .set_visibility(Visibility::Public)
            .add_argument(Argument::new(
                Ownership::Ref,
                NamedComponentSignature::new_simple_type("tag", RustType::in_scope("TagItem")),
            ));
        if inner.inherits_tag_index {
            from_tag_builder = from_tag_builder.add_argument(Argument::new(
                Ownership::OwnedMut,
                NamedComponentSignature::new_simple_type("tag_index", RustType::in_scope("usize")),
            ));
        }
        from_tag_builder = from_tag_builder
            .set_body(inner.from_tag_impl)
            .set_return_type(ComponentSignature::Signature(Signature::simple(
                RustType::in_scope("Self"),
            )));

        let valid_tag_builder =
            ImplBuilder::new(Signature::simple(RustType::in_scope(&inner.builder.name)))
                .add_method(
                    MethodBuilder::new("valid_tag")
                        .set_visibility(Visibility::Public)
                        .add_argument(Argument::new(
                            Ownership::Ref,
                            NamedComponentSignature::new_simple_type(
                                "tag_name",
                                RustType::in_scope("str"),
                            ),
                        ))
                        .set_body(inner.valid_tag_impl)
                        .set_return_type(ComponentSignature::Signature(Signature::simple(
                            RustType::in_scope("bool"),
                        ))),
                )
                .add_method(from_tag_builder);
        fb = fb.add_enum(inner.builder).add_impl(valid_tag_builder);
    }

    ModuleBuilder::new(fb)
}

fn struct_impl_builder(name: &str, valid_tag_impl: &str, from_tag_impl: &str) -> ImplBuilder {
    ImplBuilder::new(Signature::simple(RustType::in_scope(name)))
        .add_method(
            MethodBuilder::new("valid_tag")
                .set_visibility(Visibility::Public)
                .add_argument(Argument::new(
                    Ownership::Ref,
                    NamedComponentSignature::new_simple_type("tag_name", RustType::in_scope("str")),
                ))
                .set_body(valid_tag_impl)
                .set_return_type(ComponentSignature::Signature(Signature::simple(
                    RustType::in_scope("bool"),
                ))),
        )
        .add_method(
            MethodBuilder::new("from_tag")
                .set_visibility(Visibility::Public)
                .add_argument(Argument::new(
                    Ownership::Ref,
                    NamedComponentSignature::new_simple_type("tag", RustType::in_scope("TagItem")),
                ))
                .set_body(from_tag_impl)
                .set_return_type(ComponentSignature::Signature(Signature::simple(
                    RustType::in_scope("Self"),
                ))),
        )
}

#[derive(Debug, Clone)]
struct FieldDef {
    suggested_name: String,
    df: DirectField,
}

#[derive(Debug, Clone)]
enum DirectField {
    Attr(ParsedAttribute),
    Element(ElementFieldDef),
}

#[derive(Debug, Clone)]
struct ElementFieldDef {
    element_name: ElementNameRef,
    field_type: ElementFieldType,
    occurrence: Occurrence,
}

#[derive(Debug, Clone)]
enum ElementNameRef {
    Child(String),
    Me,
    Either(String, Vec<String>),
}

#[derive(Debug, Clone)]
enum ElementFieldType {
    Builtin(String),
    Constructed(String, bool),
}

#[derive(Debug, Copy, Clone)]
enum Occurrence {
    Once,
    Optional,
    Any,
    AtLeastOne,
}

impl Occurrence {
    fn extract_from_tag(tag: &TagItem) -> Self {
        let min = tag.extract_attr_as::<usize>(MIN_OCCURS);
        if let Ok(min) = min {
            if let Ok(max_str) = tag.extract_attr(MAX_OCCURS) {
                Self::from_min_max(min, max_str)
            } else {
                panic!("No max for tag {tag:?}");
            }
        } else {
            Self::Once
        }
    }

    fn from_min_max(min: usize, max_str: String) -> Self {
        if min == 0 && max_str == "unbounded" {
            Self::Any
        } else if min == 0 && max_str == "1" {
            Self::Optional
        } else if min == 1 && max_str == "unbounded" {
            Self::AtLeastOne
        } else if min == 1 && max_str == "1" {
            Self::Once
        } else {
            panic!("Unrecognized min/max {min}/{max_str}");
        }
    }
}

fn generate_struct(elem_name: String, name: String, fields: &mut [FieldDef]) -> StructBuild {
    let mut sb = StructBuilder::new(&name)
        .set_visibility(Visibility::Public)
        .add_derive(RustType::in_scope("Debug"))
        .add_derive(RustType::in_scope("Clone"));
    // Only happens in one case, annoying as hell
    let mut ident_field_count = 0;
    let mut last_field_name = None;
    for field in fields.iter_mut() {
        let tp = match &field.df {
            DirectField::Attr(parsed_attr) => {
                if parsed_attr.must_use {
                    parsed_attr.r#type.clone()
                } else {
                    format!("Option<{}>", parsed_attr.r#type)
                }
            }
            DirectField::Element(e) => {
                let wrapper = match e.occurrence {
                    Occurrence::Once => None,
                    Occurrence::Optional => Some(RustType::in_scope("Option")),
                    Occurrence::Any | Occurrence::AtLeastOne => Some(RustType::in_scope("Vec")),
                };
                let tp = match &e.field_type {
                    ElementFieldType::Builtin(c) | ElementFieldType::Constructed(c, _) => {
                        c.as_str()
                    }
                };
                // I hate this spec, have to do this for stack allocation reasons, expression recurses
                if tp == "Expression" {
                    if let Some(wrapper) = wrapper {
                        wrapper
                            .wrap(RustType::in_scope("Box").wrap(RustType::in_scope(tp)))
                            .format()
                    } else {
                        RustType::in_scope("Box")
                            .wrap(RustType::in_scope(tp))
                            .format()
                    }
                } else if let Some(wrapper) = wrapper {
                    wrapper.wrap(RustType::in_scope(tp)).format()
                } else {
                    tp.to_string()
                }
            }
        };
        if let Some(s) = &last_field_name {
            if s == &field.suggested_name {
                ident_field_count += 1;
                let n = format!("{s}_{ident_field_count}");
                field.suggested_name = n;
            } else {
                last_field_name = Some(field.suggested_name.clone());
            }
        } else {
            last_field_name = Some(field.suggested_name.clone());
        }
        sb = sb.add_field_in_scope_simple_type(Visibility::Public, &field.suggested_name, tp);
    }

    let (from_tag_impl, valid_tag_impl) = implement_struct(elem_name, fields);
    StructBuild {
        builder: sb,
        from_tag_impl,
        valid_tag_impl,
    }
}

fn generate_enum(name: String, fields: &[FieldDef]) -> (EnumBuild, bool) {
    let mut eb = EnumBuilder::new(&name)
        .set_visibility(Visibility::Public)
        .add_derive(RustType::in_scope("Debug"))
        .add_derive(RustType::in_scope("Clone"));

    let mut num_vecs = 0;
    for field in fields {
        let (type_name, tag_name) = match &field.df {
            DirectField::Attr(parsed_attr) => {
                panic!("Attr in enum {parsed_attr:?}");
            }
            DirectField::Element(e) => {
                let elem_ref = match &e.element_name {
                    ElementNameRef::Child(ch) => ch.clone(),
                    ElementNameRef::Me => panic!("Me ref in enum"),
                    ElementNameRef::Either(_, _) => panic!("Either in enum"),
                };
                if matches!(e.occurrence, Occurrence::AtLeastOne | Occurrence::Any) {
                    num_vecs += 1;
                }
                let tag_name =
                    RustCase::convert_to_valid_rust(&elem_ref, RustCase::Pascal).unwrap();
                let wrapper = match e.occurrence {
                    Occurrence::Once => None,
                    Occurrence::Optional => Some(RustType::in_scope("Option")),
                    Occurrence::Any | Occurrence::AtLeastOne => Some(RustType::in_scope("Vec")),
                };
                match &e.field_type {
                    ElementFieldType::Builtin(c) | ElementFieldType::Constructed(c, _) => {
                        if let Some(wrap) = wrapper {
                            (wrap.wrap(RustType::in_scope(c)).format(), tag_name)
                        } else {
                            (c.clone(), tag_name)
                        }
                    }
                }
            }
        };

        eb = eb.add_type_member(tag_name, Signature::simple(RustType::in_scope(type_name)));
    }
    let inherits_tag_index = num_vecs > 0 && num_vecs == fields.len();
    let (from_tag, valid_tag) = implement_enum(fields, inherits_tag_index);
    (
        EnumBuild {
            builder: eb,
            from_tag_impl: from_tag,
            valid_tag_impl: valid_tag,
            inherits_tag_index,
        },
        inherits_tag_index,
    )
}

fn impl_struct_from_tag(fields: &[FieldDef]) -> String {
    let mut body = String::new();
    body.push_str("let mut tag_index = 0;\n");
    let mut ret = "Self {\n".to_owned();
    let mut struct_valid_tag_names = vec![];
    for field in fields {
        match &field.df {
            DirectField::Attr(attr) => {
                let unwrap_or_ok = if attr.must_use { "unwrap()" } else { "ok()" };
                let _ = body.write_fmt(format_args!(
                    "let {} = tag.extract_attr_as::<{}>(\"{}\").{};\n",
                    field.suggested_name, attr.r#type, attr.name, unwrap_or_ok
                ));
                let _ = ret.write_fmt(format_args!("{},\n", field.suggested_name));
            }
            DirectField::Element(e) => {
                let (tgt, name_comparison, increment) = match &e.element_name {
                    ElementNameRef::Child(ch) => {
                        struct_valid_tag_names.push(ch.clone());
                        let (tgt, increment) =
                            if matches!(e.field_type, ElementFieldType::Builtin(_)) {
                                ("tag", false)
                            } else {
                                ("tag.inner[tag_index]", true)
                            };
                        (
                            tgt.to_string(),
                            ElementNameComparisonTarget::SingleNamed(ch.clone()),
                            increment,
                        )
                    }
                    ElementNameRef::Me => {
                        ("tag".to_string(), ElementNameComparisonTarget::None, false)
                    }
                    ElementNameRef::Either(c, e) => {
                        struct_valid_tag_names.extend(e.clone().into_iter());
                        (
                            "tag.inner[tag_index]".to_string(),
                            ElementNameComparisonTarget::MultiNamed(c.clone(), e.clone()),
                            true,
                        )
                    }
                };

                match e.occurrence {
                    Occurrence::Once => {
                        let _ = body.write_fmt(format_args!(
                            "let {} = {};\n",
                            field.suggested_name,
                            convert(&tgt, &e.field_type)
                        ));
                        if increment {
                            body.push_str("tag_index += 1;\n");
                        }
                    }
                    Occurrence::Optional => {
                        let _ = body.write_fmt(format_args!(
                            "let {} = if {} {{\n",
                            field.suggested_name,
                            name_comparison.create_option_cmp_expr(&tgt, &e.field_type),
                        ));
                        let _ = body.write_fmt(format_args!(
                            "let tmp = {};\n",
                            convert(&tgt, &e.field_type)
                        ));
                        if increment {
                            body.push_str("tag_index += 1;\n");
                        }
                        body.push_str("Some(tmp)\n");
                        body.push_str("} else {\nNone\n};\n");
                    }
                    Occurrence::Any | Occurrence::AtLeastOne => {
                        let _ = body.write_fmt(format_args!(
                            "let mut {} = vec![];\n",
                            field.suggested_name
                        ));
                        let _ = body.write_fmt(format_args!(
                            "while {} {{\n",
                            name_comparison
                                .create_option_cmp_expr("tag.inner[tag_index]", &e.field_type),
                        ));
                        let _ = body.write_fmt(format_args!(
                            "{}.push({});\n",
                            field.suggested_name,
                            convert("tag.inner[tag_index]", &e.field_type)
                        ));

                        body.push_str("tag_index += 1;\n}\n");
                    }
                }
                let _ = ret.write_fmt(format_args!("{},\n", field.suggested_name));
            }
        }
    }
    ret.push_str("}\n");
    body.push_str(&ret);
    body
}

fn convert(tgt: &str, e: &ElementFieldType) -> String {
    match &e {
        ElementFieldType::Builtin(b) => {
            format!("{tgt}.extract_value_as::<{b}>().unwrap()")
        }
        ElementFieldType::Constructed(c, inherits_tag_index) => {
            if *inherits_tag_index {
                if c == "Expression" {
                    format!("Box::new({c}::from_tag(tag, tax_index))")
                } else {
                    format!("{c}::from_tag(tag, tag_index)")
                }
            } else if c == "Expression" {
                format!("Box::new({c}::from_tag(&{tgt}))")
            } else {
                format!("{c}::from_tag(&{tgt})")
            }
        }
    }
}

fn implement_struct_valid_tag(elem_name: String) -> String {
    format!("tag_name == \"{elem_name}\"")
}

fn implement_struct(elem_name: String, fields: &[FieldDef]) -> (String, String) {
    (
        impl_struct_from_tag(fields),
        implement_struct_valid_tag(elem_name),
    )
}

#[derive(Debug, Clone)]
enum ElementNameComparisonTarget {
    None,
    SingleNamed(String),
    MultiNamed(String, Vec<String>),
}

impl ElementNameComparisonTarget {
    fn create_option_cmp_expr(&self, tgt: &str, e: &ElementFieldType) -> String {
        match self {
            ElementNameComparisonTarget::None => {
                panic!("No cmp target!");
            }
            ElementNameComparisonTarget::SingleNamed(n) => match e {
                ElementFieldType::Builtin(_) => {
                    format!("tag.inner.len() > tag_index && \"{n}\" == {tgt}.name")
                }
                ElementFieldType::Constructed(c, _) => {
                    format!("tag.inner.len() > tag_index && {c}::valid_tag(&{tgt}.name)")
                }
            },
            ElementNameComparisonTarget::MultiNamed(n, _mn) => match e {
                ElementFieldType::Builtin(_) => {
                    format!("tag.inner.len() > tag_index && \"{n}\" == {tgt}.name")
                }
                ElementFieldType::Constructed(c, _) => {
                    format!("tag.inner.len() > tag_index && {c}::valid_tag(&{tgt}.name)")
                }
            },
        }
    }
}

fn implement_enum(fields: &[FieldDef], inherits_tag_index: bool) -> (String, String) {
    let mut ret = String::new();
    if !inherits_tag_index {
        ret.push_str("let mut tag_index = 0;\n");
    }
    let mut elem_names = Vec::new();
    for field in fields {
        match &field.df {
            DirectField::Attr(_) => {
                panic!("Got attr field for enum");
            }
            DirectField::Element(e) => {
                let element_name = match &e.element_name {
                    ElementNameRef::Child(ch) => ch.clone(),
                    ElementNameRef::Me => panic!("Enum with ext"),
                    ElementNameRef::Either(_, e) => panic!("Either in enum {e:?}"),
                };
                let maybe_struct_name = match &e.field_type {
                    ElementFieldType::Builtin(_) => None,
                    ElementFieldType::Constructed(c, _) => Some(c.clone()),
                };
                elem_names.push((element_name.clone(), maybe_struct_name));
                let is_vec = match e.occurrence {
                    Occurrence::Once => false,
                    Occurrence::Optional => panic!("Option in enum"),
                    Occurrence::Any | Occurrence::AtLeastOne => true,
                };
                match &e.field_type {
                    ElementFieldType::Builtin(b) => {
                        ret.push_str("let tag_name = tag.name.as_str();\n");
                        let kind_name =
                            RustCase::convert_to_valid_rust(&element_name, RustCase::Pascal)
                                .unwrap();
                        if is_vec {
                            let _ = ret
                                .write_fmt(format_args!("if tag_name == \"{element_name}\" {{\n"));
                            let _ = ret.write_fmt(format_args!(
                                "let mut {} = vec![];\n",
                                field.suggested_name
                            ));
                            let _ = ret.write_fmt(format_args!(
                                "while tag.inner.len() > tag_index && tag.inner[tag_index].name == \"{element_name}\"{{\n"
                            ));
                            let _ = ret.write_fmt(format_args!(
                                "{}.push(tag.inner[tag_index].extract_value_as::<{}>().unwrap());\n",
                                field.suggested_name, b
                            ));
                            ret.push_str("tag_index += 1;\n");
                            let _ = ret.write_fmt(format_args!("}}\n"));
                            let _ = ret.write_fmt(format_args!(
                                "return Self::{}({})\n",
                                kind_name, field.suggested_name
                            ));
                            ret.push_str("}\n");
                        } else {
                            let _ = ret
                                .write_fmt(format_args!("if tag_name == \"{element_name}\" {{\n"));
                            let _ = ret.write_fmt(format_args!(
                                "return Self::{kind_name}(tag.extract_value_as::<{b}>().unwrap());\n"
                            ));
                            ret.push_str("}\n");
                        }
                    }
                    ElementFieldType::Constructed(c, _) => {
                        let kind_name =
                            RustCase::convert_to_valid_rust(&element_name, RustCase::Pascal)
                                .unwrap();

                        if is_vec {
                            ret.push_str("if tag.inner.len() <= tag_index {\n");
                            let _ =
                                ret.write_fmt(format_args!("return Self::{kind_name}(vec![]);"));
                            ret.push_str("}\n");
                            // TODO: Actually not always valid if constructed, but w/e it's valid for
                            // x11
                            let _ = ret.write_fmt(format_args!(
                                "if tag.inner[tag_index].name == \"{element_name}\"{{\n"
                            ));
                            let _ = ret.write_fmt(format_args!(
                                "let mut {} = vec![];\n",
                                field.suggested_name
                            ));
                            let _ = ret.write_fmt(format_args!(
                                "while tag.inner.len() > tag_index && tag.inner[tag_index].name == \"{element_name}\"{{\n"
                            ));
                            if c == "Expression" {
                                let _ = ret.write_fmt(format_args!(
                                    "{}.push(Box::new({}::from_tag(&tag.inner[tag_index])));\n",
                                    field.suggested_name, c
                                ));
                            } else {
                                let _ = ret.write_fmt(format_args!(
                                    "{}.push({}::from_tag(&tag.inner[tag_index]));\n",
                                    field.suggested_name, c
                                ));
                            }

                            ret.push_str("tag_index += 1;\n");
                            ret.push_str("}\n");
                            let _ = ret.write_fmt(format_args!(
                                "return Self::{}({});\n",
                                kind_name, field.suggested_name
                            ));
                            ret.push_str("}\n");
                        } else if c == "Expression" {
                            ret.push_str("let tag_name = tag.name.as_str();\n");
                            let elem_rust_name =
                                RustCase::convert_to_valid_rust(&element_name, RustCase::Pascal)
                                    .unwrap();
                            if c.ends_with(&elem_rust_name) {
                                let _ = ret.write_fmt(format_args!(
                                    "if {c}::valid_tag(tag_name) {{ return Self::{kind_name}(Box::new({c}::from_tag(tag))); }}\n"
                                ));
                            } else {
                                let _ = ret.write_fmt(format_args!(
                                    "if tag_name == \"{element_name}\" {{ return Self::{kind_name}(Box::new({c}::from_tag(tag))); }}\n"
                                ));
                            }
                        } else {
                            ret.push_str("let tag_name = tag.name.as_str();\n");
                            let elem_rust_name =
                                RustCase::convert_to_valid_rust(&element_name, RustCase::Pascal)
                                    .unwrap();
                            if c.ends_with(&elem_rust_name) {
                                let _ = ret.write_fmt(format_args!(
                                    "if {c}::valid_tag(tag_name) {{ return Self::{kind_name}({c}::from_tag(tag)); }}\n"
                                ));
                            } else {
                                let _ = ret.write_fmt(format_args!(
                                    "if tag_name == \"{element_name}\" {{ return Self::{kind_name}({c}::from_tag(tag)); }}\n"
                                ));
                            }
                        }
                    }
                }
            }
        }
    }
    ret.push_str("panic!(\"Unrecognized element name {}\", tag.name);\n");

    let mut is_valid_tag_name_for_enum = String::new();
    for (element, struct_build) in elem_names {
        if let Some(struct_name) = struct_build {
            let elem_rust_name =
                RustCase::convert_to_valid_rust(&element, RustCase::Pascal).unwrap();
            if struct_name.ends_with(&elem_rust_name) {
                let _ = is_valid_tag_name_for_enum.write_fmt(format_args!(
                    "if {struct_name}::valid_tag(tag_name) {{return true;}}"
                ));
            } else {
                let _ = is_valid_tag_name_for_enum.write_fmt(format_args!(
                    "if tag_name == \"{element}\" {{return true;}}"
                ));
            }
        } else {
            let _ = is_valid_tag_name_for_enum.write_fmt(format_args!(
                "if tag_name == \"{element}\" {{return true;}}"
            ));
        }
    }
    is_valid_tag_name_for_enum.push_str("return false;");
    (ret, is_valid_tag_name_for_enum)
}
