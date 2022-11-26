use std::cell::RefCell;
use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::fmt::Debug;
use std::ops::Deref;
use std::rc::Rc;

use codegen_rs::RustCase;

use crate::generated_parse_template::{
    Caseexpr, Expression, Fields, ItemChoice, Macro, MacroEnum, MacroEvent, MacroRequest,
    RequestChoice, RequestReply, Struct, Switch, SwitchexprChoice,
};
use crate::generator::types::{
    Binop, BitmaskEnum, ConstType, EntityField, EntityList, EntityMember, FieldNumberRef,
    IdTypeAlias, IdTypeUnion, ManySources, NumKV, OpType, SourceType, Sumof, SwitchEnum,
    SwitchStruct, TypeAlias, Unop, ValueEnum, WrappedType, XcbBuiltin, XcbError, XcbErrorCopy,
    XcbEvent, XcbEventCopy, XcbEventStruct, XcbExpression, XcbReply, XcbRequest, XcbStruct,
    XcbSwitchCaseExpr, XcbType,
};
use crate::Xcb;

#[derive(Debug)]
pub(crate) struct TypeResolver {
    pub(crate) resolved: HashMap<Option<String>, HashMap<String, WrappedType>>,
}

macro_rules! continue_if_not_present {
    ($search_name: expr, $namespace: expr, $imports: expr, $source: expr, $retry_buf: expr, $macro_ref: expr) => {
        if let Some(val) = $source.get($search_name, $namespace, $imports).clone() {
            val
        } else {
            $retry_buf.push($macro_ref.clone());
            continue;
        }
    };
}

impl TypeResolver {
    pub(crate) fn new() -> Self {
        let base = vec![
            XcbBuiltin::Card8,
            XcbBuiltin::Card16,
            XcbBuiltin::Card32,
            XcbBuiltin::Card64,
            XcbBuiltin::Int8,
            XcbBuiltin::Int16,
            XcbBuiltin::Int32,
            XcbBuiltin::Float32,
            XcbBuiltin::Float64,
            XcbBuiltin::Atom,
            XcbBuiltin::Bool,
            XcbBuiltin::Byte,
            XcbBuiltin::Char,
            XcbBuiltin::Void,
            XcbBuiltin::Fd,
        ];
        let resolved: HashMap<Option<String>, HashMap<String, WrappedType>> = HashMap::new();
        let mut slf = Self { resolved };
        for builtin in base {
            slf.insert(builtin.name(), None, new_rc(XcbType::Builtin(builtin)));
        }
        slf
    }

    pub(crate) fn resolve_all(&mut self, xcb: &Xcb) -> Vec<WrappedType> {
        let mut try_again = xcb.r#macro.clone();
        let mut resolved = vec![];
        while !try_again.is_empty() {
            try_again = self.resolve_macros(&try_again, &mut resolved, xcb);
        }
        resolved
    }

    pub(crate) fn resolved_owned(
        &self,
        name: &str,
        namespace: Option<String>,
        imports: &[String],
    ) -> WrappedType {
        self.get(name, namespace, imports).unwrap()
    }

    pub(crate) fn get(
        &self,
        name: &str,
        namespace: Option<String>,
        imports: &[String],
    ) -> Option<WrappedType> {
        let (namespace, search) = if name.contains(':') {
            let (namespace, name) = name.split_once(':').unwrap();
            (Some(namespace.to_string()), name)
        } else {
            (namespace, name)
        };
        if let Some(found) = self
            .resolved
            .get(&namespace)
            .and_then(|inner| inner.get(search))
        {
            Some(found.clone())
        } else if let Some(found) = self.resolved.get(&None).and_then(|inner| inner.get(search)) {
            Some(found.clone())
        } else {
            for import in imports {
                if let Some(found) = self
                    .resolved
                    .get(&Some(import.to_string()))
                    .and_then(|inner| inner.get(search))
                {
                    return Some(found.clone());
                }
            }
            //eprintln!("{self:?}");
            // eprintln!("Failed to find {namespace:?}::{search}");
            None
        }
    }

    pub(crate) fn insert(&mut self, name: String, namespace: Option<String>, item: WrappedType) {
        match self.resolved.entry(namespace) {
            Entry::Occupied(mut o) => {
                //eprintln!("Inserted {namespace:?}::{name}");
                o.get_mut().insert(name, item);
            }
            Entry::Vacant(v) => {
                //eprintln!("Inserted {namespace:?}::{name}");
                let mut inner = HashMap::new();
                inner.insert(name, item);
                v.insert(inner);
            }
        }
    }

    pub(crate) fn resolve_macros(
        &mut self,
        m: &[Macro],
        resolved: &mut Vec<WrappedType>,
        xcb: &Xcb,
    ) -> Vec<Macro> {
        let mut try_again = vec![];
        let namespace = Some(xcb.header.clone());
        let imports = xcb
            .r#macro
            .iter()
            .filter_map(|m| match m {
                Macro::Import(i) => Some(i.clone()),
                _ => None,
            })
            .collect::<Vec<String>>();
        for m in m {
            match m {
                Macro::Xidtype(t) => {
                    let alias = IdTypeAlias {
                        name: t.name.clone(),
                        source: xcb.header.clone(),
                    };
                    let ty = new_rc(XcbType::Xid(alias));
                    resolved.push(ty.clone());
                    self.insert(t.name.clone(), namespace.clone(), ty);
                }
                Macro::Xidunion(u) => {
                    // Since all xids are just aliased u32's we can just
                    // direct alias them agains u32 instead of aliasing them with other aliases
                    // since the type system won't do anything for us on type aliases anyway.
                    let mut union_cases = vec![];
                    for t in &u.r#type {
                        let val = continue_if_not_present!(
                            t,
                            namespace.clone(),
                            &imports,
                            self,
                            try_again,
                            m
                        );
                        union_cases.push(val);
                    }
                    let ty = IdTypeUnion {
                        name: u.name.clone(),
                        _cases: union_cases,
                        source: xcb.header.clone(),
                    };
                    let ty = new_rc(XcbType::XidUnion(ty));
                    resolved.push(ty.clone());
                    self.insert(u.name.clone(), namespace.clone(), ty.clone());
                }
                Macro::Typedef(mtd) => {
                    let old = continue_if_not_present!(
                        &mtd.oldname,
                        namespace.clone(),
                        &imports,
                        self,
                        try_again,
                        m
                    );
                    if matches!(&*old.deref().borrow(), XcbType::Builtin(_)) {
                        let c = ConstType {
                            raw_name: mtd.newname.clone(),
                            builtin: old,
                            source: Some(xcb.header.clone()),
                        };
                        let ty = new_rc(XcbType::Const(c));
                        self.insert(mtd.newname.clone(), namespace.clone(), ty.clone());
                        resolved.push(ty);
                    } else {
                        let a = TypeAlias {
                            new_name: mtd.newname.clone(),
                            aliased: old,
                            source: namespace.clone(),
                        };
                        let ty = new_rc(XcbType::Alias(a));
                        self.insert(mtd.newname.clone(), namespace.clone(), ty.clone());
                        resolved.push(ty);
                    }
                }
                Macro::Struct(s) => {
                    if let Some((s, residual)) =
                        resolve_struct(s, SourceType::Other, namespace.clone(), &imports, xcb, self)
                    {
                        let name = s.name.clone();
                        let ty = new_rc(XcbType::PlainStruct(s));
                        self.insert(name, namespace.clone(), ty.clone());
                        resolved.push(ty);
                        resolved.extend(residual);
                    } else {
                        try_again.push(m.clone());
                    }
                }
                Macro::Union(s) => {
                    if let Some((s, residual)) =
                        resolve_struct(s, SourceType::Other, namespace.clone(), &imports, xcb, self)
                    {
                        let name = s.name.clone();
                        let ty = new_rc(XcbType::UnionStruct(s.into()));
                        self.insert(name, namespace.clone(), ty.clone());
                        resolved.push(ty);
                        resolved.extend(residual);
                    } else {
                        try_again.push(m.clone());
                    }
                }
                Macro::Enum(e) => {
                    let e = resolve_enum(e, xcb);
                    let name = e.name();
                    let ty = new_rc(e);
                    self.insert(name, namespace.clone(), ty.clone());
                    resolved.push(ty);
                }
                Macro::Request(r) => {
                    if let Some((req, res)) =
                        resolve_request(r, namespace.clone(), &imports, xcb, self)
                    {
                        let name = req.name.clone();
                        let ty = new_rc(XcbType::Request(req));
                        self.insert(name, namespace.clone(), ty.clone());
                        resolved.push(ty);
                        resolved.extend(res);
                    } else {
                        try_again.push(m.clone());
                    }
                }
                Macro::Eventstruct(es) => {
                    let es = XcbEventStruct {
                        name: es.name.clone(),
                        source: xcb.header.clone(),
                    };
                    let name = es.name.clone();
                    let ty = new_rc(XcbType::EventStruct(es));
                    self.insert(name, namespace.clone(), ty.clone());
                    resolved.push(ty);
                }
                Macro::Event(e) => {
                    if let Some((e, res)) = resolve_event(e, namespace.clone(), &imports, xcb, self)
                    {
                        let name = e.name.clone();
                        let ty = new_rc(XcbType::Event(e));
                        self.insert(name, namespace.clone(), ty.clone());
                        resolved.push(ty);
                        resolved.extend(res);
                    } else {
                        try_again.push(m.clone());
                    }
                }
                Macro::Eventcopy(ec) => {
                    if let Some(parent) = self.get(&ec.r#ref, namespace.clone(), &imports) {
                        let name = ec.name.clone();
                        let ty = new_rc(XcbType::EventCopy(XcbEventCopy {
                            name: name.clone(),
                            opcode: ec.number as u8,
                            copy_type: parent.clone(),
                            source: xcb.header.to_string(),
                        }));
                        self.insert(name, namespace.clone(), ty.clone());
                        resolved.push(ty);
                    } else {
                        try_again.push(m.clone());
                    }
                }
                Macro::Error(e) => {
                    let mut avail = HashMap::new();
                    if let Some((members, res)) = resolve_fields(
                        &e.fields,
                        SourceType::Error,
                        &mut avail,
                        namespace.clone(),
                        &imports,
                        xcb,
                        self,
                    ) {
                        let name = format!(
                            "{}Error",
                            RustCase::convert_to_valid_rust(&e.name, RustCase::Pascal).unwrap()
                        );
                        let ty = new_rc(XcbType::Error(XcbError {
                            name: name.clone(),
                            opcode: e.number as u8,
                            members,
                            source: xcb.header.to_string(),
                        }));
                        self.insert(name, namespace.clone(), ty.clone());
                        resolved.push(ty);
                        resolved.extend(res);
                    } else {
                        try_again.push(m.clone());
                    }
                }
                Macro::Errorcopy(ec) => {
                    let refname = format!(
                        "{}Error",
                        RustCase::convert_to_valid_rust(&ec.r#ref, RustCase::Pascal).unwrap()
                    );
                    if let Some(parent) = self.get(&refname, namespace.clone(), &imports) {
                        let name = format!(
                            "{}Error",
                            RustCase::convert_to_valid_rust(&ec.name, RustCase::Pascal).unwrap()
                        );
                        let ty = new_rc(XcbType::ErrorCopy(XcbErrorCopy {
                            name: name.clone(),
                            opcode: ec.number as u8,
                            copy_type: parent.clone(),
                            source: xcb.header.to_string(),
                        }));
                        self.insert(name, namespace.clone(), ty.clone());
                        resolved.push(ty);
                    } else {
                        eprintln!("Failed to resolve {refname}");
                        try_again.push(m.clone());
                    }
                }
                Macro::Import(_) => {}
            }
        }

        try_again
    }
}

fn new_rc(ty: XcbType) -> Rc<RefCell<XcbType>> {
    Rc::new(RefCell::new(ty))
}

fn resolve_struct(
    s: &Struct,
    source_type: SourceType,
    namespace: Option<String>,
    imports: &[String],
    xcb: &Xcb,
    resolver: &mut TypeResolver,
) -> Option<(XcbStruct, Vec<WrappedType>)> {
    let mut avail = HashMap::new();
    let (members, mut residuals) = resolve_fields(
        &s.fields,
        source_type,
        &mut avail,
        namespace.clone(),
        imports,
        xcb,
        resolver,
    )?;
    let switch: Option<WrappedType> = if let Some(sw) = &s.switch {
        let (switch, res) = resolve_switch(
            &s.name,
            source_type,
            sw,
            &mut avail,
            namespace,
            imports,
            xcb,
            resolver,
        )?;
        let ty = new_rc(switch);
        residuals.push(ty.clone());
        residuals.extend(res);
        Some(ty)
    } else {
        None
    };
    Some((
        XcbStruct {
            name: s.name.clone(),
            source: xcb.header.clone(),
            source_type,
            used_by: ManySources::default(),
            members,
            switch,
        },
        residuals,
    ))
}

fn resolve_enum(e: &MacroEnum, xcb: &Xcb) -> XcbType {
    let mut bits = vec![];
    let mut values = vec![];
    for item in &e.enum_item {
        match &item.item_choice {
            ItemChoice::Value(v) => {
                values.push(NumKV {
                    key: item.name.clone(),
                    value: *v as usize,
                });
            }
            ItemChoice::Bit(b) => bits.push(NumKV {
                key: item.name.clone(),
                value: b.integer as usize,
            }),
        }
    }
    if !bits.is_empty() && values.len() <= 1 {
        let head = if values.len() == 1 {
            Some(values[0].clone())
        } else {
            None
        };
        XcbType::BitmaskEnum(BitmaskEnum {
            name: e.name.clone(),
            source: xcb.header.clone(),
            source_type: SourceType::Other,
            used_by: ManySources::default(),
            concrete_number_types: Rc::new(RefCell::new(vec![])),
            default: head,
            kvs: bits,
        })
    } else if bits.is_empty() && !values.is_empty() {
        XcbType::ValueEnum(ValueEnum {
            name: e.name.clone(),
            source: xcb.header.clone(),
            source_type: SourceType::Other,
            used_by: ManySources::default(),
            concrete_number_types: Rc::new(RefCell::new(vec![])),
            kvs: values,
        })
    } else {
        panic!("Bad enum {e:?}");
    }
}

fn resolve_request(
    req: &MacroRequest,
    namespace: Option<String>,
    imports: &[String],
    xcb: &Xcb,
    resolver: &TypeResolver,
) -> Option<(XcbRequest, Vec<WrappedType>)> {
    let mut avail = HashMap::new();
    let fields = req
        .request_choice
        .iter()
        .filter_map(|rc| match rc {
            RequestChoice::Fields(f) => Some(f.clone()),
            RequestChoice::Exprfield(_) => None,
        })
        .collect::<Vec<Fields>>();
    let (members, mut residuals) = resolve_fields(
        &fields,
        SourceType::Request,
        &mut avail,
        namespace.clone(),
        imports,
        xcb,
        resolver,
    )?;
    let switch: Option<WrappedType> = if let Some(s) = &req.switch {
        let (resolved, res) = resolve_switch(
            &req.name,
            SourceType::Request,
            s,
            &mut avail,
            namespace.clone(),
            imports,
            xcb,
            resolver,
        )?;
        let switch_type = new_rc(resolved);
        residuals.push(switch_type.clone());
        residuals.extend(res);
        Some(switch_type)
    } else {
        None
    };
    let reply = if let Some(reply) = &req.request_reply {
        let (resolved_repl, res) =
            resolve_reply(reply, req.name.clone(), namespace, imports, xcb, resolver)?;
        let reply_type = new_rc(XcbType::Reply(resolved_repl));
        residuals.push(reply_type.clone());
        residuals.extend(res);
        Some(reply_type)
    } else {
        None
    };
    Some((
        XcbRequest {
            name: req.name.clone(),
            source: xcb.header.clone(),
            opcode: req.opcode as u8,
            members,
            switch,
            reply,
        },
        residuals,
    ))
}

fn resolve_reply(
    reply: &RequestReply,
    req_name: String,
    namespace: Option<String>,
    imports: &[String],
    xcb: &Xcb,
    resolver: &TypeResolver,
) -> Option<(XcbReply, Vec<WrappedType>)> {
    let mut avail = HashMap::new();
    let (members, mut residuals) = resolve_fields(
        &reply.fields,
        SourceType::Reply,
        &mut avail,
        namespace.clone(),
        imports,
        xcb,
        resolver,
    )?;
    let switch: Option<WrappedType> = if let Some(switch) = &reply.switch {
        let (switch, res) = resolve_switch(
            &req_name,
            SourceType::Reply,
            switch,
            &mut avail,
            namespace,
            imports,
            xcb,
            resolver,
        )?;
        let switch_type = new_rc(switch);
        residuals.push(switch_type.clone());
        residuals.extend(res);
        Some(switch_type)
    } else {
        None
    };
    Some((
        XcbReply {
            name: req_name,
            source: xcb.header.clone(),
            fields: members,
            switch,
        },
        residuals,
    ))
}

fn resolve_event(
    event: &MacroEvent,
    namespace: Option<String>,
    imports: &[String],
    xcb: &Xcb,
    resolver: &TypeResolver,
) -> Option<(XcbEvent, Vec<WrappedType>)> {
    let mut avail = HashMap::new();
    let (members, residuals) = resolve_fields(
        &event.packet_struct.fields,
        SourceType::Event,
        &mut avail,
        namespace,
        imports,
        xcb,
        resolver,
    )?;
    Some((
        XcbEvent {
            name: event.packet_struct.name.clone(),
            source: xcb.header.clone(),
            has_sequence: event.no_sequence_number.map_or(true, |n| !n),
            opcode: event.packet_struct.number as u8,
            members,
            generic: event.xge.unwrap_or_default(),
        },
        residuals,
    ))
}

fn resolve_fields(
    fields: &[Fields],
    source: SourceType,
    avail_fields: &mut HashMap<String, WrappedType>,
    namespace: Option<String>,
    imports: &[String],
    xcb: &Xcb,
    resolver: &TypeResolver,
) -> Option<(Vec<EntityMember>, Vec<WrappedType>)> {
    let mut members = vec![];
    let mut late = vec![];
    let residuals = vec![];
    for f in fields {
        // Spec writers sometimes put dependent fields above dependants making life difficult.
        if let Fields::Length(l) = f {
            late.push(l.clone());
            continue;
        }
        let resolved = resolve_field(
            f,
            source,
            avail_fields,
            namespace.clone(),
            imports,
            xcb,
            resolver,
        )?;
        match &resolved {
            EntityMember::Field(f) => {
                avail_fields.insert(f.name.clone(), f.kind.concrete.clone());
            }
            EntityMember::List(l) => {
                avail_fields.insert(l.field.name.clone(), l.field.kind.concrete.clone());
            }
            _ => {}
        }
        members.push(resolved);
    }
    for f in late {
        let resolved = resolve_field(
            &Fields::Length(f),
            source,
            avail_fields,
            namespace.clone(),
            imports,
            xcb,
            resolver,
        )?;
        match &resolved {
            EntityMember::Field(f) => {
                avail_fields.insert(f.name.clone(), f.kind.concrete.clone());
            }
            EntityMember::List(l) => {
                avail_fields.insert(l.field.name.clone(), l.field.kind.concrete.clone());
            }
            _ => {}
        }
        members.push(resolved);
    }
    Some((members, residuals))
}

fn resolve_field(
    fields: &Fields,
    source_type: SourceType,
    avail_fields: &HashMap<String, WrappedType>,
    namespace: Option<String>,
    imports: &[String],
    xcb: &Xcb,
    resolver: &TypeResolver,
) -> Option<EntityMember> {
    match fields {
        Fields::Pad(p) => {
            if let Some(align) = p.align {
                Some(EntityMember::Align(align as usize))
            } else if let Some(pad) = p.bytes {
                Some(EntityMember::Pad(pad as usize))
            } else {
                panic!("Pad without bytes or align");
            }
        }
        Fields::Field(f) => Some(EntityMember::Field(EntityField {
            name: f.name.clone(),
            kind: resolve_substitute(
                &f.r#type,
                source_type,
                f.r#enum.as_ref(),
                f.altenum.as_ref(),
                f.mask.as_ref(),
                f.altmask.as_ref(),
                namespace,
                imports,
                resolver,
            )?,
        })),
        Fields::List(l) => {
            let expr = if let Some(e) = l.expression.as_ref() {
                Some(resolve_expr(
                    e.as_ref(),
                    source_type,
                    avail_fields,
                    namespace.clone(),
                    imports,
                    xcb,
                    resolver,
                )?)
            } else {
                None
            };
            let fixed_count = expr.as_ref().and_then(|e| {
                if let XcbExpression::Value(n) = e {
                    Some(*n as usize)
                } else {
                    None
                }
            });
            let field = EntityField {
                name: l.var.name.clone(),
                kind: resolve_substitute(
                    &l.var.r#type,
                    source_type,
                    l.var.r#enum.as_ref(),
                    l.var.altenum.as_ref(),
                    l.var.mask.as_ref(),
                    l.var.altmask.as_ref(),
                    namespace,
                    imports,
                    resolver,
                )?,
            };
            Some(EntityMember::List(EntityList {
                field,
                fixed_count,
                length_expr: expr,
            }))
        }
        Fields::Fd(fd) => Some(EntityMember::Field(EntityField {
            name: fd.name.clone(),
            kind: FieldNumberRef {
                concrete: resolver.get("fd", namespace, imports)?,
                reference_type: None,
            },
        })),
        Fields::RequiredStartAlign(rsa) => Some(EntityMember::StartAlign(rsa.clone())),
        Fields::Length(l) => Some(EntityMember::Length(resolve_expr(
            l.expression.as_ref(),
            source_type,
            avail_fields,
            namespace,
            imports,
            xcb,
            resolver,
        )?)),
    }
}

fn resolve_substitute(
    kind: &str,
    source: SourceType,
    enum_ref: Option<&String>,
    altenum_ref: Option<&String>,
    mask_ref: Option<&String>,
    altmask_ref: Option<&String>,
    namespace: Option<String>,
    imports: &[String],
    resolver: &TypeResolver,
) -> Option<FieldNumberRef> {
    let concrete = resolver.resolved_owned(kind, namespace.clone(), imports);
    if let Some(ref_type) = enum_ref.or(altenum_ref).or(mask_ref).or(altmask_ref) {
        let found = resolver.get(ref_type, namespace, imports)?;
        found
            .deref()
            .borrow()
            .ping_update_concrete_type(concrete.clone());
        found.deref().borrow().ping_update_used_by(source);

        Some(FieldNumberRef {
            concrete,
            reference_type: Some(found),
        })
    } else {
        Some(FieldNumberRef {
            concrete,
            reference_type: None,
        })
    }
}

fn resolve_switch(
    parent_name: &str,
    source_type: SourceType,
    switch: &Switch,
    avail_fields: &mut HashMap<String, WrappedType>,
    namespace: Option<String>,
    imports: &[String],
    xcb: &Xcb,
    resolver: &TypeResolver,
) -> Option<(XcbType, Vec<WrappedType>)> {
    let mut residuals = vec![];
    let expr = resolve_expr(
        switch.expression.as_ref(),
        source_type,
        avail_fields,
        namespace.clone(),
        imports,
        xcb,
        resolver,
    )?;
    match &switch.switchexpr_choice {
        SwitchexprChoice::Bitcase(bc) => {
            let mut cases = vec![];
            for bc in bc {
                let (case, res) = resolve_case_expr(
                    parent_name,
                    source_type,
                    bc,
                    avail_fields,
                    namespace.clone(),
                    imports,
                    xcb,
                    resolver,
                )?;
                cases.push(case);
                residuals.extend(res);
            }
            Some((
                XcbType::SwitchStruct(SwitchStruct {
                    name: switch.name.clone(),
                    source_type,
                    used_by: ManySources::default(),
                    parent_name: parent_name.to_string(),
                    source: xcb.header.clone(),
                    start_align: switch.required_start_align.clone(),
                    switch_expr: expr,
                    bit_cases: cases,
                }),
                residuals,
            ))
        }
        SwitchexprChoice::Case(c) => {
            let mut cases = vec![];
            for c in c {
                let (case, res) = resolve_case_expr(
                    parent_name,
                    source_type,
                    c,
                    avail_fields,
                    namespace.clone(),
                    imports,
                    xcb,
                    resolver,
                )?;
                cases.push(case);
                residuals.extend(res);
            }
            Some((
                XcbType::SwitchEnum(SwitchEnum {
                    name: switch.name.clone(),
                    source_type,
                    used_by: ManySources::default(),
                    parent_name: parent_name.to_string(),
                    source: xcb.header.clone(),
                    start_align: switch.required_start_align.clone(),
                    switch_expr: expr,
                    enum_cases: cases,
                }),
                residuals,
            ))
        }
    }
}

fn resolve_case_expr(
    parent_name: &str,
    source_type: SourceType,
    case_expr: &Caseexpr,
    avail_fields: &mut HashMap<String, WrappedType>,
    namespace: Option<String>,
    imports: &[String],
    xcb: &Xcb,
    resolver: &TypeResolver,
) -> Option<(XcbSwitchCaseExpr, Vec<WrappedType>)> {
    let mut expressions = vec![];
    let (fields, resid) = resolve_fields(
        &case_expr.fields,
        source_type,
        avail_fields,
        namespace.clone(),
        imports,
        xcb,
        resolver,
    )?;
    for expr in &case_expr.expression {
        expressions.push(resolve_expr(
            expr.as_ref(),
            source_type,
            avail_fields,
            namespace.clone(),
            imports,
            xcb,
            resolver,
        )?);
    }
    let mut switches: Vec<WrappedType> = vec![];
    let mut residuals: Vec<WrappedType> = vec![];
    for switch in &case_expr.switch {
        let (switch, res) = resolve_switch(
            parent_name,
            source_type,
            switch,
            avail_fields,
            namespace.clone(),
            imports,
            xcb,
            resolver,
        )?;
        let as_type = new_rc(switch);
        switches.push(as_type.clone());
        residuals.push(as_type);
        residuals.extend(res);
    }

    residuals.extend(resid);
    Some((
        XcbSwitchCaseExpr {
            name: case_expr.name.clone(),
            expressions,
            fields,
            switches,
        },
        residuals,
    ))
}

fn resolve_expr(
    expression: &Expression,
    source: SourceType,
    avail_fields: &HashMap<String, WrappedType>,
    namespace: Option<String>,
    imports: &[String],
    xcb: &Xcb,
    resolver: &TypeResolver,
) -> Option<XcbExpression> {
    match expression {
        Expression::Op(op) => {
            let kind = OpType::from_str(&op.op);
            let left = resolve_expr(
                op.expression.as_ref(),
                source,
                avail_fields,
                namespace.clone(),
                imports,
                xcb,
                resolver,
            )?;
            let right = resolve_expr(
                op.expression_1.as_ref(),
                source,
                avail_fields,
                namespace,
                imports,
                xcb,
                resolver,
            )?;
            Some(XcbExpression::Binop(Binop {
                op: kind,
                left: Box::new(left),
                right: Box::new(right),
            }))
        }
        Expression::Unop(unop) => {
            let kind = OpType::from_str(&unop.op);
            let target = resolve_expr(
                unop.expression.as_ref(),
                source,
                avail_fields,
                namespace,
                imports,
                xcb,
                resolver,
            )?;
            Some(XcbExpression::Unop(Unop {
                op: kind,
                tgt: Box::new(target),
            }))
        }
        Expression::Fieldref(fr) => {
            if let Some(kind) = avail_fields.get(fr) {
                Some(XcbExpression::Fieldref(EntityField {
                    name: fr.clone(),
                    kind: FieldNumberRef {
                        concrete: kind.clone(),
                        reference_type: None,
                    },
                }))
            } else if fr == "length" {
                Some(XcbExpression::ListLengthExpr)
            } else {
                Some(XcbExpression::ListForeignFieldRef(fr.clone()))
            }
        }
        Expression::Paramref(pr) => {
            let concrete = resolver.resolved_owned(&pr.r#type, namespace.clone(), imports);
            let fr = if let Some(rt) = avail_fields.get(&pr.string) {
                rt.deref().borrow().ping_update_concrete_type(
                    resolver.resolved_owned(&pr.r#type, namespace, imports),
                );
                rt.deref().borrow().ping_update_used_by(source);
                Some(rt.clone())
            } else {
                None
            };

            Some(XcbExpression::Paramref(EntityField {
                name: pr.string.clone(),
                kind: FieldNumberRef {
                    concrete,
                    reference_type: fr,
                },
            }))
        }
        Expression::Enumref(er) => {
            let kind = resolver.get(&er.r#ref, namespace, imports)?;
            Some(XcbExpression::Enumref(EntityField {
                name: er.string.clone(),
                kind: FieldNumberRef {
                    concrete: kind,
                    reference_type: None,
                },
            }))
        }
        Expression::Popcount(pc) => Some(XcbExpression::Popcount(Box::new(resolve_expr(
            pc.expression.as_ref(),
            source,
            avail_fields,
            namespace,
            imports,
            xcb,
            resolver,
        )?))),
        Expression::Sumof(so) => {
            let kind = avail_fields
                .get(&so.r#ref)
                .unwrap_or_else(|| panic!("Failed to find field {}", so.r#ref))
                .clone();
            let expr = if let Some(so) = &so.expression {
                Some(Box::new(resolve_expr(
                    so.as_ref(),
                    source,
                    avail_fields,
                    namespace,
                    imports,
                    xcb,
                    resolver,
                )?))
            } else {
                None
            };
            Some(XcbExpression::Sumof(Sumof {
                entity: EntityField {
                    name: so.r#ref.clone(),
                    kind: FieldNumberRef {
                        concrete: kind,
                        reference_type: None,
                    },
                },
                expr,
            }))
        }
        Expression::ListelementRef(_) => Some(XcbExpression::ListelementRef),
        Expression::Value(v) => Some(XcbExpression::Value(*v)),
        Expression::Bit(bt) => Some(XcbExpression::Bit(bt.integer as u8)),
    }
}
