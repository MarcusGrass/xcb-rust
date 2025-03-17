use std::collections::{HashMap, HashSet};
use std::fmt::Write;
use std::ops::Deref;

use codegen_rs::structures::gen_enum::NamedComponentSignature;
use codegen_rs::structures::method::Argument;
use codegen_rs::structures::visibility::Visibility;
use codegen_rs::structures::{ComponentSignature, Ownership, RustType, Signature};
use codegen_rs::{RustCase, StructBuilder};

use crate::generator::codegen::{AsRustFormatted, TRY_FROM_INT_ERROR};
use crate::generator::types::{
    EntityField, EntityMember, SwitchEnum, SwitchStruct, TypeHelpers, WrappedType, XcbBuiltin,
    XcbExpression, XcbType,
};
use crate::{dump, Xcb};

pub(crate) fn add_fields(
    mut sb: StructBuilder,
    members: &[EntityMember],
    wrapper: Option<RustType>,
    exclude: &HashSet<String>,
    xcb: &Xcb,
) -> StructBuilder {
    for member in members {
        match member {
            EntityMember::Field(f) => {
                if exclude.contains(&f.name) {
                    continue;
                }
                let name =
                    if let Ok(name) = RustCase::convert_to_valid_rust(&f.name, RustCase::Snake) {
                        name
                    } else {
                        RustCase::convert_to_valid_rust(&f.name.to_lowercase(), RustCase::Snake)
                            .unwrap()
                    };

                if let Some(eref) = &f.kind.reference_type {
                    if let (Some(a), Some(b)) = (eref.byte_size(), f.kind.concrete.byte_size()) {
                        if a < b {
                            let rt = RustType::in_scope(
                                f.kind.concrete.deref().borrow().import_name(&xcb.header),
                            );
                            let t = if let Some(w) = wrapper.clone() {
                                w.wrap(rt)
                            } else {
                                rt
                            };
                            sb = sb.add_field(
                                Visibility::Public,
                                NamedComponentSignature::new_simple_type(name, t),
                            );
                            continue;
                        }
                    }
                    let rt = RustType::in_scope(eref.deref().borrow().import_name(&xcb.header));
                    let t = if let Some(w) = wrapper.clone() {
                        w.wrap(rt)
                    } else {
                        rt
                    };
                    sb = sb.add_field(
                        Visibility::Public,
                        NamedComponentSignature::new_simple_type(name, t),
                    );
                } else {
                    let rt = RustType::in_scope(
                        f.kind.concrete.deref().borrow().import_name(&xcb.header),
                    );
                    let t = if let Some(w) = wrapper.clone() {
                        w.wrap(rt)
                    } else {
                        rt
                    };
                    sb = sb.add_field(
                        Visibility::Public,
                        NamedComponentSignature::new_simple_type(name, t),
                    );
                }
            }
            EntityMember::List(l) => {
                let name = RustCase::convert_to_valid_rust(&l.field.name, RustCase::Snake).unwrap();
                let wrap_with = RustType::from_package("alloc::vec", "Vec");

                if let Some(eref) = &l.field.kind.reference_type {
                    if let Some(fixed_size) = l.fixed_count {
                        sb = sb.add_field(
                            Visibility::Public,
                            NamedComponentSignature::new_simple_type(
                                name,
                                RustType::in_scope(format!(
                                    "[{}; {}]",
                                    eref.borrow().import_name(&xcb.header),
                                    fixed_size
                                )),
                            ),
                        );
                        continue;
                    }
                    let rt = wrap_with.wrap(RustType::in_scope(
                        eref.deref().borrow().import_name(&xcb.header),
                    ));
                    let t = if let Some(w) = wrapper.clone() {
                        w.wrap(rt)
                    } else {
                        rt
                    };
                    sb = sb.add_field(
                        Visibility::Public,
                        NamedComponentSignature::new_simple_type(name, t),
                    );
                } else {
                    if let Some(fixed_size) = l.fixed_count {
                        sb = sb.add_field(
                            Visibility::Public,
                            NamedComponentSignature::new_simple_type(
                                name,
                                RustType::in_scope(format!(
                                    "[{}; {}]",
                                    l.field.kind.concrete.borrow().import_name(&xcb.header),
                                    fixed_size
                                )),
                            ),
                        );
                        continue;
                    }
                    let rt = wrap_with.wrap(RustType::in_scope(
                        l.field
                            .kind
                            .concrete
                            .deref()
                            .borrow()
                            .import_name(&xcb.header),
                    ));
                    let t = if let Some(w) = wrapper.clone() {
                        w.wrap(rt)
                    } else {
                        rt
                    };
                    sb = sb.add_field(
                        Visibility::Public,
                        NamedComponentSignature::new_simple_type(name, t),
                    );
                }
            }
            EntityMember::StartAlign(_)
            | EntityMember::Pad(_)
            | EntityMember::Length(_)
            | EntityMember::Align(_) => {}
        }
    }
    sb
}

pub(crate) fn find_req_derive_fields(
    members: &[EntityMember],
    switch_enum_ref: Option<WrappedType>,
) -> (Vec<EntityField>, Option<String>) {
    let mut exclude = vec![];
    let mut replace = None;
    for member in members {
        if let EntityMember::Field(ef) = member {
            if let Some(ref_type) = &switch_enum_ref {
                if let Some(field_ref) = &ef.kind.reference_type {
                    if ref_type.rust_entity_name() == field_ref.rust_entity_name() {
                        // The spec is insane, specifically xkb, can't streamline by removing derived args
                        // because some are multi-input functions that can't be cleanly derived
                        // without more effort than it's worth to derive it for a single function in xkb.
                        // If we find that, just give up
                        if replace.is_some() {
                            replace = None;
                            exclude.clear();
                            break;
                        }
                        exclude.push(ef.clone());
                        replace = Some(ef.name.to_rust_snake());
                    }
                }
            }
        }
    }
    for member in members {
        if let EntityMember::List(el) = member {
            if let Some(XcbExpression::Fieldref(f)) = &el.length_expr {
                if !exclude.is_empty() {
                    return (vec![], replace);
                }
                exclude.push(f.clone());
            }
        }
    }
    (exclude, replace)
}

pub(crate) fn find_derive_fields(members: &[EntityMember]) -> Vec<EntityField> {
    let mut exclude = vec![];
    for member in members {
        if let EntityMember::List(el) = member {
            if let Some(expr) = &el.length_expr {
                let req = expr.required_fields();
                if req.len() == 1 {
                    exclude.extend(req);
                } else {
                    for r in req {
                        if r.name.ends_with("len") {
                            exclude.push(r.clone());
                        }
                    }
                }
            }
        }
    }
    exclude
}

#[inline]
pub(crate) fn member_size(members: &[EntityMember]) -> Option<usize> {
    let mut sum = 0;
    for m in members {
        sum += m.byte_size()?;
    }
    Some(sum)
}

#[inline]
pub(crate) fn is_finite_size(members: &[EntityMember]) -> bool {
    for member in members {
        match member {
            EntityMember::Field(f) => {
                if f.kind.concrete.byte_size().is_none() {
                    return false;
                }
            }
            EntityMember::List(l) => {
                if l.fixed_count.is_none() {
                    return false;
                }
            }
            EntityMember::StartAlign(_)
            | EntityMember::Pad(_)
            | EntityMember::Align(_)
            | EntityMember::Length(_) => {}
        }
    }
    true
}

#[inline]
pub(crate) fn is_finite_switch_struct(switch: &SwitchStruct) -> bool {
    for case in &switch.bit_cases {
        for sw in &case.switches {
            if !is_finite_switch_type(sw) {
                return false;
            }
        }
        if !is_finite_size(&case.fields) {
            return false;
        }
    }
    true
}

#[inline]
pub(crate) fn is_finite_switch_enum(switch: &SwitchEnum) -> bool {
    for case in &switch.enum_cases {
        for sw in &case.switches {
            if !is_finite_switch_type(sw) {
                return false;
            }
        }
        if !is_finite_size(&case.fields) {
            return false;
        }
    }
    true
}

#[inline]
pub(crate) fn is_finite_switch_type(expect_switch: &WrappedType) -> bool {
    if let XcbType::SwitchStruct(ss) = &*expect_switch.borrow() {
        if !is_finite_switch_struct(ss) {
            return false;
        }
    } else if let XcbType::SwitchEnum(se) = &*expect_switch.borrow() {
        if !is_finite_switch_enum(se) {
            return false;
        }
    } else {
        panic!("Expected switch");
    }
    true
}

#[inline]
pub(crate) fn bitmask_enum_case_name(input: &str) -> String {
    if let Ok(clean_convert) = RustCase::convert_to_valid_rust(input, RustCase::Scream) {
        clean_convert
    } else if input.chars().next().unwrap().is_numeric() {
        let converted = clean_up_number_case(input);
        if let Ok(num_convert) = RustCase::convert_to_valid_rust(&converted, RustCase::Scream) {
            num_convert
        } else {
            let lc = converted.to_lowercase();
            RustCase::convert_to_valid_rust(&lc, RustCase::Scream).unwrap()
        }
    } else {
        let lc = input.to_lowercase();
        RustCase::convert_to_valid_rust(&lc, RustCase::Scream).unwrap()
    }
}

#[inline]
pub(crate) fn clean_up_number_case(input: &str) -> String {
    let mut new_str = String::new();
    for (ind, ch) in input.chars().enumerate() {
        if ind == 0 && ch.is_numeric() {
            let repl = match ch {
                '0' => "Zero".to_string(),
                '1' => "One".to_string(),
                '2' => "Two".to_string(),
                '3' => "Three".to_string(),
                '4' => "Four".to_string(),
                '5' => "Five".to_string(),
                '6' => "Six".to_string(),
                '7' => "Seven".to_string(),
                '8' => "Eight".to_string(),
                '9' => "Nine".to_string(),
                _ => ch.to_string(),
            };
            new_str.push_str(&repl);
        } else {
            new_str.push(ch);
        }
    }
    new_str
}

pub(crate) fn probe_num_types(expr: &XcbExpression) -> Vec<EntityField> {
    let mut v = vec![];
    match expr {
        XcbExpression::Binop(b) => {
            v.extend(probe_num_types(b.left.as_ref()));
            v.extend(probe_num_types(b.right.as_ref()));
        }
        XcbExpression::Unop(u) => {
            v.extend(probe_num_types(u.tgt.as_ref()));
        }
        XcbExpression::Fieldref(f) => {
            v.push(f.clone());
        }
        XcbExpression::Paramref(p) => {
            v.push(p.clone());
        }
        XcbExpression::Popcount(pc) => {
            v.extend(probe_num_types(pc.as_ref()));
        }
        XcbExpression::Sumof(so) => {
            v.push(so.entity.clone());
            if let Some(expr) = so.expr.as_ref() {
                v.extend(probe_num_types(expr.as_ref()));
            }
        }
        XcbExpression::Enumref(_)
        | XcbExpression::ListelementRef
        | XcbExpression::ListLengthExpr
        | XcbExpression::ListForeignFieldRef(_)
        | XcbExpression::Value(_)
        | XcbExpression::Bit(_) => {}
    }
    v
}

pub(crate) fn from_bytes_length_expr(
    expr: &XcbExpression,
    rev: bool,
    ref_self_fields: bool,
) -> String {
    let num_types = probe_num_types(expr);
    let mut largest_builtin: Option<XcbBuiltin> = None;
    for t in num_types {
        if let XcbType::Builtin(b) = &*t.kind.concrete.deref().borrow() {
            if let Some(lb) = &mut largest_builtin {
                if lb.byte_size().unwrap() < b.byte_size().unwrap() {
                    largest_builtin = Some(*b);
                }
            } else {
                largest_builtin = Some(*b);
            }
        }
    }
    let short_ty = largest_builtin
        .map(|lb| lb.rust_entity_name())
        .unwrap_or_default();
    let ty = largest_builtin
        .map(|lb| format!(" as {}", lb.rust_entity_name()))
        .unwrap_or_default();
    do_write_length_expr(expr, rev, ref_self_fields, &ty, &short_ty)
}

pub(crate) fn do_write_length_expr(
    expr: &XcbExpression,
    rev: bool,
    ref_self_fields: bool,
    ty: &str,
    short_ty: &str,
) -> String {
    let mut out = String::new();
    match expr {
        XcbExpression::Binop(b) => {
            let _ = out.write_fmt(format_args!(
                "{}({}{ty}, {}{ty})",
                b.op.into_op_expr(rev),
                do_write_length_expr(b.left.as_ref(), rev, ref_self_fields, ty, short_ty,),
                do_write_length_expr(b.right.as_ref(), rev, ref_self_fields, ty, short_ty,),
            ));
        }
        XcbExpression::Unop(u) => {
            let _ = out.write_fmt(format_args!(
                "{}({})",
                u.op.into_op_expr(rev),
                do_write_length_expr(u.tgt.as_ref(), rev, ref_self_fields, ty, short_ty,)
            ));
        }
        XcbExpression::Fieldref(f) => {
            dump!(
                out,
                "{}",
                maybe_self_ref(ref_self_fields, &f.name.to_rust_snake())
            );
        }
        XcbExpression::Paramref(pr) => {
            dump!(out, "{}", pr.name.to_rust_snake());
        }
        XcbExpression::Popcount(pc) => {
            let _ = out.write_fmt(format_args!(
                "{}.count_ones()",
                maybe_self_ref(
                    ref_self_fields,
                    &do_write_length_expr(pc.as_ref(), rev, ref_self_fields, ty, short_ty,),
                )
            ));
        }
        XcbExpression::Sumof(so) => {
            if let Some(expr) = so.expr.as_ref() {
                let le = do_write_length_expr(expr.as_ref(), rev, ref_self_fields, ty, short_ty);
                let dot = if le.starts_with('.') { "" } else { "." };
                dump!(
                    out,
                    "{}.iter().try_fold(0u32, |start, val| start.checked_add(u32::try_from(val{dot}{}).map_err(|_| {})?).ok_or({}))?",
                    maybe_self_ref(ref_self_fields, &so.entity.name.to_rust_snake()),
                    le,
                    TRY_FROM_INT_ERROR,
                    TRY_FROM_INT_ERROR
                );
            } else {
                dump!(
                    out,
                    "{}.iter().try_fold(0usize, |start, val| start.checked_add(usize::try_from(*val).map_err(|_| {})?).ok_or({}))?",
                    maybe_self_ref(ref_self_fields, &so.entity.name.to_rust_snake()),
                    TRY_FROM_INT_ERROR,
                    TRY_FROM_INT_ERROR
                );
            }
        }
        XcbExpression::ListLengthExpr => {
            dump!(out, "length");
        }
        XcbExpression::ListForeignFieldRef(ff) => {
            dump!(out, "{} as usize", ff);
        }
        XcbExpression::Value(v) => {
            let _ = out.write_fmt(format_args!("{v}{short_ty}"));
        }
        XcbExpression::ListelementRef | XcbExpression::Bit(_) | XcbExpression::Enumref(_) => {
            // panic!("Illegal expr in list length expr {expr:?}");
        }
    }
    out
}

pub(crate) fn serialize_length_expr(
    expr: &XcbExpression,
    list_name: &str,
    ref_self_fields: bool,
) -> (String, HashMap<String, String>) {
    let mut replace = HashMap::new();
    let num_types = probe_num_types(expr);
    let mut largest_builtin: Option<XcbBuiltin> = None;
    for t in num_types {
        if let XcbType::Builtin(b) = &*t.kind.concrete.deref().borrow() {
            if let Some(lb) = &mut largest_builtin {
                if lb.byte_size().unwrap() < b.byte_size().unwrap() {
                    largest_builtin = Some(*b);
                }
            } else {
                largest_builtin = Some(*b);
            }
        }
    }
    let short_ty = largest_builtin
        .map(|lb| lb.rust_entity_name())
        .unwrap_or_default();
    let ty = largest_builtin
        .map(|lb| format!(" as {}", lb.rust_entity_name()))
        .unwrap_or_default();
    let expr = do_write_ser_length_expr(
        expr,
        list_name,
        ref_self_fields,
        &mut replace,
        &ty,
        &short_ty,
    );
    (expr, replace)
}

pub(crate) fn do_write_ser_length_expr(
    expr: &XcbExpression,
    list_name: &str,
    ref_self_fields: bool,
    replace_with_expr: &mut HashMap<String, String>,
    ty: &str,
    short_ty: &str,
) -> String {
    let mut out = String::new();
    match expr {
        XcbExpression::Binop(b) => {
            let _ = out.write_fmt(format_args!(
                "{}({}{ty}, {})",
                b.op.reverse_of_expr(),
                do_write_ser_length_expr(
                    b.left.as_ref(),
                    list_name,
                    ref_self_fields,
                    replace_with_expr,
                    ty,
                    short_ty,
                ),
                do_write_ser_length_expr(
                    b.right.as_ref(),
                    list_name,
                    ref_self_fields,
                    replace_with_expr,
                    ty,
                    short_ty,
                ),
            ));
        }
        XcbExpression::Unop(u) => {
            let _ = out.write_fmt(format_args!(
                "{}({})",
                u.op.reverse_of_expr(),
                do_write_ser_length_expr(
                    u.tgt.as_ref(),
                    list_name,
                    ref_self_fields,
                    replace_with_expr,
                    ty,
                    short_ty,
                )
            ));
        }
        XcbExpression::Fieldref(f) => {
            let prefix = if ref_self_fields { "self." } else { "" };
            replace_with_expr.insert(f.name.clone(), format!("{prefix}{list_name}.len()"));
            dump!(out, "{}{}.len()", prefix, list_name);
        }
        XcbExpression::Paramref(pr) => {
            dump!(out, "paramref!{}", pr.name.to_rust_snake());
        }
        XcbExpression::Popcount(pc) => {
            let expr = &do_write_ser_length_expr(
                pc.as_ref(),
                list_name,
                ref_self_fields,
                replace_with_expr,
                ty,
                short_ty,
            );
            let ins = if expr.is_empty() {
                String::new()
            } else {
                format!("{}.", maybe_self_ref(ref_self_fields, expr))
            };
            let _ = out.write_fmt(format_args!("{ins}count_ones()",));
        }
        XcbExpression::Sumof(so) => {
            if let Some(expr) = so.expr.as_ref() {
                let le = do_write_ser_length_expr(
                    expr.as_ref(),
                    list_name,
                    ref_self_fields,
                    replace_with_expr,
                    ty,
                    short_ty,
                );
                let dot = if le.starts_with('.') { "" } else { "." };
                let ins = if le.is_empty() {
                    String::new()
                } else {
                    format!("{dot}{le}")
                };
                dump!(
                    out,
                    "{}.iter().try_fold(0u32, |start, val| start.checked_add(u32::try_from(val{ins}).map_err(|_| {})?).ok_or({}))?",
                    maybe_self_ref(ref_self_fields, &so.entity.name.to_rust_snake()),
                    TRY_FROM_INT_ERROR,
                    TRY_FROM_INT_ERROR
                );
            } else {
                dump!(
                    out,
                    "{}.iter().try_fold(0usize, |start, val| start.checked_add(usize::try_from(*val).map_err(|_| {})?).ok_or({}))?",
                    maybe_self_ref(ref_self_fields, &so.entity.name.to_rust_snake()),
                    TRY_FROM_INT_ERROR,
                    TRY_FROM_INT_ERROR
                );
            }
        }
        XcbExpression::ListLengthExpr => {
            dump!(out, "length");
        }
        XcbExpression::ListForeignFieldRef(ff) => {
            dump!(out, "{} as usize", ff);
        }
        XcbExpression::Value(v) => {
            let _ = out.write_fmt(format_args!("{v}{short_ty}"));
        }
        XcbExpression::ListelementRef | XcbExpression::Bit(_) | XcbExpression::Enumref(_) => {
            // panic!("Illegal expr in list length expr {expr:?}");
        }
    }
    out
}

fn maybe_self_ref(ref_self: bool, input: &str) -> String {
    if ref_self {
        format!("self.{input}")
    } else {
        input.to_string()
    }
}

pub(crate) fn reduce_required_from_container(
    req: &[EntityField],
    members: &[EntityMember],
) -> Vec<EntityField> {
    req.iter()
        .filter(|r| {
            let mut in_scope = false;
            for member in members.iter().filter(|f| f.is_constructable()) {
                if r.name.to_rust_snake() == member.rust_field_name() {
                    in_scope = true;
                    break;
                }
            }
            !in_scope
        })
        .cloned()
        .collect()
}

pub(crate) fn get_sorted_required_by_members(members: &[EntityMember]) -> Vec<EntityField> {
    let mut required = vec![];
    for member in members {
        for arg in member.required_args() {
            let mut in_scope = false;
            for m in members.iter().filter(|f| f.is_constructable()) {
                if m.rust_field_name() == arg.name.to_rust_snake() {
                    in_scope = true;
                    break;
                }
            }
            if !in_scope {
                required.push(arg);
            }
        }
    }
    get_sorted_required_fields(&required)
}

pub(crate) fn get_unsorted_required_fields(field: &[EntityField]) -> Vec<EntityField> {
    let mut seen = HashSet::new();
    let mut out = vec![];
    for val in field {
        if !seen.contains(&val.name) {
            seen.insert(val.name.clone());
            out.push(val.clone());
        }
    }
    out
}

pub(crate) fn get_sorted_required_fields(field: &[EntityField]) -> Vec<EntityField> {
    let mut to_sort = field
        .iter()
        .cloned()
        .map(|ef| (ef.name.to_rust_snake(), ef))
        .collect::<HashMap<String, EntityField>>()
        .into_values()
        .collect::<Vec<EntityField>>();
    to_sort.sort_by(|a, b| a.name.cmp(&b.name));
    to_sort
}

pub(crate) fn fields_as_args(field: &[EntityField], current_package: &str) -> Vec<Argument> {
    let sorted = get_sorted_required_fields(field);
    let mut args = vec![];
    for field in sorted {
        args.push(Argument::new(
            Ownership::Owned,
            NamedComponentSignature::new(
                field.name.to_rust_snake(),
                ComponentSignature::Signature(Signature::simple(RustType::in_scope(
                    field.kind.concrete.import_name(current_package),
                ))),
            ),
        ));
    }
    args
}
