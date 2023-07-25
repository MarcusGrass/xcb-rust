extern crate core;

use std::fmt::Write;
use std::path::Path;

use anyhow::Result;
use codegen_rs::structures::visibility::Visibility;
use codegen_rs::structures::{
    Annotation, Annotations, ComponentSignature, Import, Ownership, RustType, Signature,
};
use codegen_rs::{FileBuilder, FunctionBuilder, ModuleBuilder};

use xcb_xsd::parse::raw_xml_parse::TagSpec;

use crate::generated_parse_template::{Macro, Xcb};
use crate::generator::codegen::event::EvtNameSpec;
use crate::generator::codegen::request::ReqNameSpec;
use crate::generator::codegen::{
    add_evt, implement_req_name, FIX_LEN_FROM_BYTES, FIX_LEN_SERIALIZE, VAR_LEN_SERIALIZE,
};
use crate::generator::relationship::XcbImportOrdered;
use crate::generator::type_resolve::TypeResolver;

#[allow(
    unused,
    clippy::large_enum_variant,
    clippy::needless_return,
    clippy::needless_borrow,
    clippy::vec_box,
    clippy::collapsible_else_if
)]
mod generated_parse_template;
mod generator;

fn main() -> Result<()> {
    let all = parse_all()?;
    let ordered = XcbImportOrdered::new(all);
    let mut type_resolver = TypeResolver::new();
    let mut main_mod = ModuleBuilder::new(FileBuilder::new("mod"));
    let mut con_mod = ModuleBuilder::new(FileBuilder::new("mod"));
    let mut wrapped = vec![];
    // Resolve all first, there are a lot of cross-references
    let mut all_features = Vec::new();
    for chunk in ordered {
        for xcb in chunk {
            let imports = xcb
                .r#macro
                .iter()
                .filter_map(|m| {
                    if let Macro::Import(i) = m {
                        Some(i.clone())
                    } else {
                        None
                    }
                })
                .collect::<Vec<String>>();
            all_features.push((xcb.header.clone(), imports));
            let all = type_resolver.resolve_all(&xcb);
            wrapped.push((all, xcb));
        }
    }

    let mut reqs = ReqNameSpec::default();
    let mut evts = EvtNameSpec::default();

    all_features.sort_by(|a, b| a.0.cmp(&b.0));
    let mut ext_fun_body = "match name {\n".to_string();
    for (feat, _) in &all_features {
        if feat != "xproto" {
            let rf = format!("{}::EXTENSION_NAME", feat);
            dump!(
                ext_fun_body,
                "#[cfg(feature = \"{}\")]\n{} => Some({}),\n",
                feat,
                rf,
                rf
            );
        }
    }
    for (chunk, xcb) in wrapped {
        let mut functions = vec![];
        let file = generator::codegen::generate_proto(
            &chunk,
            &mut reqs,
            &mut evts,
            &mut functions,
            xcb.clone(),
        );
        main_mod = main_mod.add_module_file(
            Visibility::Public,
            file,
            Annotations::new(vec![Annotation::new(format!(
                "cfg(feature = \"{}\")",
                xcb.header
            ))]),
        );
        let mut fb = FileBuilder::new(&xcb.header)
            .add_any("#[allow(unused_imports)]\n")
            .add_import(Import::FullType(RustType::from_package(
                "crate::cookie",
                "Cookie",
            )))
            .add_any("#[allow(unused_imports)]\n")
            .add_import(Import::FullType(RustType::from_package(
                "crate::cookie",
                "FixedCookie",
            )))
            .add_any("#[allow(unused_imports)]\n")
            .add_import(Import::FullType(RustType::from_package(
                "crate::cookie",
                "VoidCookie",
            )))
            .add_any("#[allow(unused_imports)]\n")
            .add_import(Import::FullType(RustType::from_package(
                "crate::util",
                FIX_LEN_SERIALIZE,
            )))
            .add_any("#[allow(unused_imports)]\n")
            .add_import(Import::FullType(RustType::from_package(
                "crate::util",
                VAR_LEN_SERIALIZE,
            )));
        for func in functions {
            fb = fb.add_function(func);
        }
        con_mod = con_mod.add_module_file(
            Visibility::Public,
            fb,
            Annotations::new(vec![Annotation::new(format!(
                "cfg(feature = \"{}\")",
                xcb.header
            ))]),
        );
    }
    dump!(ext_fun_body, "_ => None,\n}");

    let req_name_fn = implement_req_name(reqs);
    main_mod.mod_file = main_mod.mod_file.add_function(req_name_fn).add_function(
        FunctionBuilder::new("find_extension")
            .add_argument_in_scope_simple_type(Ownership::Ref, "name", "str")
            .add_simple_annotation("inline")
            .set_visibility(Visibility::Public)
            .set_return_type(ComponentSignature::Signature(Signature::simple(
                RustType::in_scope("Option<&'static str>"),
            )))
            .set_body(ext_fun_body),
    );
    main_mod.mod_file = add_evt(evts, main_mod.mod_file);

    main_mod.write_to_disk("xcb-rust-protocol/src/proto")?;
    con_mod.write_to_disk("xcb-rust-protocol/src/connection")?;
    write_cargo_toml("xcb-rust-protocol/Cargo.toml", all_features)?;
    Ok(())
}

fn parse_all() -> Result<Vec<Xcb>> {
    let proto_dir = std::fs::read_dir("xcb-code-generator/proto")?;
    let mut xcbs = vec![];
    for de in proto_dir {
        let file = de?;
        if file.metadata()?.is_file() {
            let content = std::fs::read(file.path())?;
            let top_tag = TagSpec::new("xcb");
            let parsed = xcb_xsd::parse::raw_xml_parse::parse(&content, top_tag)?;
            xcbs.push(Xcb::from_tag(&parsed));
        }
    }
    Ok(xcbs)
}

fn write_cargo_toml(path: impl AsRef<Path>, features: Vec<(String, Vec<String>)>) -> Result<()> {
    let mut base = "\
[package]
name = \"xcb-rust-protocol\"
version = \"0.1.0\"
edition = \"2021\"
license = \"MPL-2.0\"

[dependencies]
tiny-std = { workspace = true }
unix-print = { workspace = true }

[features]
debug = []
"
    .to_string();
    let mut all = "all = [".to_string();
    let mut specific = String::new();
    let len = features.len();
    for (ind, (feature, deps)) in features.into_iter().enumerate() {
        dump!(all, "\"{}\"", feature);
        if ind != len - 1 {
            dump!(all, ", ");
        }
        dump!(specific, "{} = [", feature);
        let deps_len = deps.len();
        for (d_ind, dep) in deps.into_iter().enumerate() {
            dump!(specific, "\"{}\"", dep);
            if d_ind != deps_len - 1 {
                dump!(specific, ", ");
            }
        }
        dump!(specific, "]\n");
    }
    dump!(all, "]\n");
    dump!(base, "{}{}", all, specific);
    std::fs::write(path, base)?;
    Ok(())
}
