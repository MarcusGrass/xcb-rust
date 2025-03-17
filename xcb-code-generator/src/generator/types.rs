use std::cell::RefCell;
use std::fmt::Debug;
use std::ops::Deref;
use std::rc::Rc;

use codegen_rs::RustCase;

use crate::generated_parse_template::RequiredStartAlign;
use crate::generator::codegen::enums::find_largest_concrete_builtins;
use crate::generator::codegen::functions::get_sorted_required_by_members;

pub(crate) type WrappedType = Rc<RefCell<XcbType>>;
pub(crate) type ManyWrappedTypes = Rc<RefCell<Vec<WrappedType>>>;
pub(crate) type ManySources = Rc<RefCell<Vec<SourceType>>>;

pub(crate) trait TypeHelpers {
    fn is_builtin_alias(&self) -> bool {
        false
    }
    fn is_builtin(&self) -> bool {
        false
    }
    fn byte_size(&self) -> Option<usize>;
    fn rust_entity_name(&self) -> String;
    fn rust_field_name(&self) -> String {
        RustCase::convert_to_valid_rust(&self.rust_entity_name(), RustCase::Snake).unwrap()
    }

    fn source(&self) -> Option<String>;
    fn import_name(&self, current_package: &str) -> String {
        self.source().filter(|s| current_package != s).map_or_else(
            || self.rust_entity_name(),
            |s| format!("crate::proto::{}::{}", s, self.rust_entity_name()),
        )
    }

    fn members(&self) -> Vec<EntityMember> {
        vec![]
    }

    fn required_args(&self) -> Vec<EntityField> {
        vec![]
    }
    fn source_type(&self) -> SourceType;
}

#[derive(Debug, Copy, Clone)]
pub(crate) enum SourceType {
    Request,
    Event,
    Error,
    Reply,
    Other,
}

impl SourceType {
    pub(crate) fn resolve_effective_source(self, others: ManySources) -> SourceType {
        let mut has_out = false;
        let mut has_in = false;
        for st in others.deref().borrow().iter().chain(std::iter::once(&self)) {
            match st {
                SourceType::Request => has_out = true,
                SourceType::Event | SourceType::Error | SourceType::Reply => has_in = true,
                SourceType::Other => {}
            }
        }
        if has_out && has_in {
            SourceType::Other
        } else if has_out {
            SourceType::Request
        } else if has_in {
            SourceType::Event
        } else {
            SourceType::Other
        }
    }

    pub(crate) fn should_serialize(self) -> bool {
        matches!(self, Self::Request | Self::Other)
    }
    pub(crate) fn should_deserialize(self) -> bool {
        !matches!(self, Self::Request)
    }
}

#[derive(Debug, Clone)]
pub(crate) enum XcbType {
    Request(XcbRequest),
    Reply(XcbReply),
    Error(XcbError),
    ErrorCopy(XcbErrorCopy),
    Event(XcbEvent),
    EventCopy(XcbEventCopy),
    EventStruct(XcbEventStruct),
    Builtin(XcbBuiltin),
    SwitchStruct(SwitchStruct),
    SwitchEnum(SwitchEnum),
    ValueEnum(ValueEnum),
    BitmaskEnum(BitmaskEnum),
    PlainStruct(XcbStruct),
    UnionStruct(XcbUnion),
    Xid(IdTypeAlias),
    XidUnion(IdTypeUnion),
    Alias(TypeAlias),
    Const(ConstType),
    // Wildcard for bitcase types
    Constructed(Constructed),
}

#[derive(Debug, Clone)]
pub(crate) struct XcbError {
    pub(crate) name: String,
    pub(crate) opcode: u8,
    pub(crate) members: Vec<EntityMember>,
    pub(crate) source: String,
}

#[derive(Debug, Clone)]
pub(crate) struct XcbErrorCopy {
    pub(crate) name: String,
    pub(crate) opcode: u8,
    pub(crate) copy_type: WrappedType,
    pub(crate) source: String,
}

#[derive(Debug, Clone)]
pub(crate) struct XcbEventCopy {
    pub(crate) name: String,
    pub(crate) opcode: u8,
    pub(crate) copy_type: WrappedType,
    pub(crate) source: String,
}

#[derive(Debug, Clone)]
pub(crate) struct Constructed {
    pub(crate) name: String,
    pub(crate) byte_size: Option<usize>,
    pub(crate) required_args: Vec<EntityField>,
    pub(crate) source_type: SourceType,
}

impl XcbType {
    pub(crate) fn name(&self) -> String {
        match self {
            XcbType::Request(r) => r.name.clone(),
            XcbType::Reply(r) => r.name.clone(),
            XcbType::Event(e) => e.name.clone(),
            XcbType::EventStruct(es) => es.name.clone(),
            XcbType::Builtin(b) => b.name(),
            XcbType::SwitchStruct(ss) => ss.name.clone(),
            XcbType::SwitchEnum(se) => se.name.clone(),
            XcbType::ValueEnum(ve) => ve.name.clone(),
            XcbType::BitmaskEnum(be) => be.name.clone(),
            XcbType::PlainStruct(s) => s.name.clone(),
            XcbType::UnionStruct(s) => s.name.clone(),
            XcbType::Xid(x) => x.name.clone(),
            XcbType::XidUnion(xu) => xu.name.clone(),
            XcbType::Alias(al) => al.new_name.clone(),
            XcbType::Const(c) => c.raw_name.clone(),
            XcbType::Constructed(c) => c.name.clone(),
            XcbType::Error(e) => e.name.clone(),
            XcbType::ErrorCopy(ec) => ec.name.clone(),
            XcbType::EventCopy(ec) => ec.name.clone(),
        }
    }

    pub(crate) fn import_name(&self, current_location: &str) -> String {
        self.source().filter(|s| current_location != s).map_or_else(
            || self.rust_entity_name(),
            |s| format!("crate::proto::{}::{}", s, self.rust_entity_name()),
        )
    }

    pub(crate) fn source(&self) -> Option<String> {
        match self {
            XcbType::Request(r) => Some(r.source.clone()),
            XcbType::Reply(r) => Some(r.source.clone()),
            XcbType::Event(o) => Some(o.source.clone()),
            XcbType::EventStruct(o) => Some(o.source.clone()),
            XcbType::SwitchStruct(o) => Some(o.source.clone()),
            XcbType::SwitchEnum(o) => Some(o.source.clone()),
            XcbType::ValueEnum(o) => Some(o.source.clone()),
            XcbType::BitmaskEnum(o) => Some(o.source.clone()),
            XcbType::PlainStruct(o) => Some(o.source.clone()),
            XcbType::UnionStruct(o) => Some(o.source.clone()),
            XcbType::Xid(o) => Some(o.source.clone()),
            XcbType::XidUnion(o) => Some(o.source.clone()),
            XcbType::Alias(o) => o.source.clone(),
            XcbType::Const(o) => o.source.clone(),
            XcbType::Error(e) => Some(e.source.clone()),
            XcbType::ErrorCopy(ec) => Some(ec.source.clone()),
            XcbType::EventCopy(ec) => Some(ec.source.clone()),
            XcbType::Builtin(_) | XcbType::Constructed(_) => None,
        }
    }

    pub(crate) fn ping_update_concrete_type(&self, new_type: WrappedType) {
        match self {
            XcbType::ValueEnum(ve) => ve.concrete_number_types.deref().borrow_mut().push(new_type),
            XcbType::BitmaskEnum(be) => {
                be.concrete_number_types.deref().borrow_mut().push(new_type);
            }
            XcbType::Error(_) | XcbType::ErrorCopy(_) | XcbType::EventCopy(_) => {}
            _ => panic!("Tried to update concrete type for something other than enum {self:?}"),
        }
    }

    pub(crate) fn ping_update_used_by(&self, used_by: SourceType) {
        match self {
            XcbType::Request(_)
            | XcbType::Reply(_)
            | XcbType::Error(_)
            | XcbType::ErrorCopy(_)
            | XcbType::Event(_)
            | XcbType::EventCopy(_)
            | XcbType::EventStruct(_)
            | XcbType::Xid(_)
            | XcbType::XidUnion(_)
            | XcbType::Alias(_)
            | XcbType::Const(_)
            | XcbType::Constructed(_)
            | XcbType::Builtin(_) => {}
            XcbType::SwitchStruct(ss) => {
                ss.used_by.deref().borrow_mut().push(used_by);
            }
            XcbType::SwitchEnum(ss) => {
                ss.used_by.deref().borrow_mut().push(used_by);
            }
            XcbType::ValueEnum(ve) => {
                ve.used_by.deref().borrow_mut().push(used_by);
            }
            XcbType::BitmaskEnum(be) => {
                be.used_by.deref().borrow_mut().push(used_by);
            }
            XcbType::PlainStruct(ps) => {
                ps.used_by.deref().borrow_mut().push(used_by);
            }
            XcbType::UnionStruct(ps) => {
                ps.used_by.deref().borrow_mut().push(used_by);
            }
        }
    }
}

impl TypeHelpers for XcbType {
    fn is_builtin_alias(&self) -> bool {
        match self {
            XcbType::Alias(a) => a.aliased.is_builtin(),
            XcbType::Const(_) => true,
            _ => false,
        }
    }
    fn is_builtin(&self) -> bool {
        matches!(self, XcbType::Builtin(_))
    }

    fn byte_size(&self) -> Option<usize> {
        match self {
            XcbType::Request(r) => r.byte_size(),
            XcbType::Reply(r) => r.byte_size(),
            XcbType::Builtin(b) => b.byte_size(),
            XcbType::ValueEnum(ve) => {
                find_largest_concrete_builtins(ve.concrete_number_types.clone())
                    .0
                    .byte_size()
            }
            XcbType::SwitchStruct(_) | XcbType::SwitchEnum(_) => None,
            // Both enum sizes are tagged on the containing entity
            XcbType::BitmaskEnum(be) => {
                find_largest_concrete_builtins(be.concrete_number_types.clone())
                    .0
                    .byte_size()
            }
            XcbType::PlainStruct(ps) => ps.byte_size(),
            XcbType::UnionStruct(ps) => ps.byte_size(),
            XcbType::Xid(_) | XcbType::XidUnion(_) => Some(4),
            XcbType::Alias(a) => a.aliased.byte_size(),
            XcbType::Const(c) => c.builtin.byte_size(),
            XcbType::Constructed(c) => c.byte_size,
            XcbType::Error(_)
            | XcbType::ErrorCopy(_)
            | XcbType::EventCopy(_)
            | XcbType::Event(_)
            | XcbType::EventStruct(_) => Some(32),
        }
    }

    fn rust_entity_name(&self) -> String {
        match self {
            XcbType::Request(r) => r.rust_entity_name(),
            XcbType::Reply(r) => r.rust_entity_name(),
            XcbType::Event(evt) => evt.rust_entity_name(),
            XcbType::Const(c) => c.rust_entity_name(),
            XcbType::Builtin(b) => b.rust_name().to_string(),
            XcbType::ValueEnum(ve) => ve.rust_name(),
            XcbType::SwitchStruct(ss) => ss.rust_entity_name(),
            XcbType::SwitchEnum(se) => se.rust_entity_name(),
            XcbType::Constructed(c) => c.name.clone(),
            XcbType::Error(e) => e.name.clone(),
            XcbType::ErrorCopy(e) => e.name.clone(),
            XcbType::EventCopy(e) => format!(
                "{}Event",
                RustCase::convert_to_valid_rust(&e.name, RustCase::Pascal).unwrap()
            ),
            XcbType::EventStruct(_)
            | XcbType::BitmaskEnum(_)
            | XcbType::PlainStruct(_)
            | XcbType::UnionStruct(_)
            | XcbType::Xid(_)
            | XcbType::XidUnion(_)
            | XcbType::Alias(_) => {
                RustCase::convert_to_valid_rust(&self.name(), RustCase::Pascal).unwrap()
            }
        }
    }

    fn source(&self) -> Option<String> {
        self.source()
    }

    fn members(&self) -> Vec<EntityMember> {
        match self {
            XcbType::Request(r) => r.members.clone(),
            XcbType::Reply(r) => r.fields.clone(),
            XcbType::Event(e) => e.members.clone(),
            XcbType::EventStruct(_)
            | XcbType::Builtin(_)
            | XcbType::BitmaskEnum(_)
            | XcbType::Xid(_)
            | XcbType::XidUnion(_)
            | XcbType::Constructed(_)
            | XcbType::ValueEnum(_) => vec![],
            XcbType::SwitchStruct(ss) => {
                let mut out = vec![];
                for case in &ss.bit_cases {
                    out.extend(case.fields.clone());
                }
                out
            }
            XcbType::SwitchEnum(se) => {
                let mut out = vec![];
                for case in &se.enum_cases {
                    out.extend(case.fields.clone());
                }
                out
            }
            XcbType::PlainStruct(ps) => ps.members.clone(),
            XcbType::UnionStruct(ps) => ps.members.clone(),
            XcbType::Alias(a) => a.aliased.members(),
            XcbType::Const(c) => c.builtin.members(),
            XcbType::Error(e) => e.members.clone(),
            XcbType::ErrorCopy(e) => e.copy_type.members(),
            XcbType::EventCopy(e) => e.copy_type.members(),
        }
    }

    fn required_args(&self) -> Vec<EntityField> {
        match self {
            XcbType::Request(r) => get_sorted_required_by_members(&r.members),
            XcbType::Reply(r) => get_sorted_required_by_members(&r.fields),
            XcbType::Event(e) => get_sorted_required_by_members(&e.members),
            XcbType::Error(_)
            | XcbType::ErrorCopy(_)
            | XcbType::EventCopy(_)
            | XcbType::EventStruct(_)
            | XcbType::Builtin(_)
            | XcbType::BitmaskEnum(_)
            | XcbType::Xid(_)
            | XcbType::XidUnion(_)
            | XcbType::ValueEnum(_) => vec![],
            XcbType::SwitchStruct(ss) => ss.required_args(),
            XcbType::SwitchEnum(se) => se.required_args(),
            XcbType::PlainStruct(ps) => get_sorted_required_by_members(&ps.members),
            XcbType::UnionStruct(ps) => get_sorted_required_by_members(&ps.members),
            XcbType::Alias(a) => a.aliased.required_args(),
            XcbType::Const(c) => c.builtin.required_args(),
            XcbType::Constructed(c) => c.required_args.clone(),
        }
    }

    fn source_type(&self) -> SourceType {
        match self {
            XcbType::Request(_) => SourceType::Request,
            XcbType::Reply(_) => SourceType::Reply,
            XcbType::Error(_) | XcbType::ErrorCopy(_) => SourceType::Error,
            XcbType::Event(_) | XcbType::EventCopy(_) | XcbType::EventStruct(_) => {
                SourceType::Event
            }
            XcbType::Builtin(_) => SourceType::Other,
            XcbType::SwitchStruct(e) => e.source_type.resolve_effective_source(e.used_by.clone()),
            XcbType::SwitchEnum(e) => e.source_type.resolve_effective_source(e.used_by.clone()),
            XcbType::ValueEnum(e) => e.source_type.resolve_effective_source(e.used_by.clone()),
            XcbType::BitmaskEnum(e) => e.source_type.resolve_effective_source(e.used_by.clone()),
            XcbType::PlainStruct(e) => e.source_type.resolve_effective_source(e.used_by.clone()),
            XcbType::UnionStruct(e) => e.source_type.resolve_effective_source(e.used_by.clone()),
            XcbType::Xid(_) | XcbType::XidUnion(_) | XcbType::Alias(_) | XcbType::Const(_) => {
                SourceType::Other
            }
            XcbType::Constructed(c) => c.source_type,
        }
    }
}

impl<B> TypeHelpers for Rc<RefCell<B>>
where
    B: TypeHelpers,
{
    fn is_builtin_alias(&self) -> bool {
        self.deref().borrow().is_builtin_alias()
    }
    fn is_builtin(&self) -> bool {
        self.deref().borrow().is_builtin()
    }
    fn byte_size(&self) -> Option<usize> {
        self.deref().borrow().byte_size()
    }

    fn rust_entity_name(&self) -> String {
        self.deref().borrow().rust_entity_name()
    }

    fn rust_field_name(&self) -> String {
        self.deref().borrow().rust_field_name()
    }

    fn source(&self) -> Option<String> {
        self.deref().borrow().source()
    }

    fn import_name(&self, current_package: &str) -> String {
        self.deref().borrow().import_name(current_package)
    }

    fn members(&self) -> Vec<EntityMember> {
        self.deref().borrow().members()
    }

    fn required_args(&self) -> Vec<EntityField> {
        self.deref().borrow().required_args()
    }

    fn source_type(&self) -> SourceType {
        self.deref().borrow().source_type()
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum XcbBuiltin {
    Atom,
    Card8,
    Card16,
    Card32,
    Card64,
    Int8,
    Int16,
    Int32,
    Float32,
    Float64,
    Bool,
    Byte,
    Char,
    Void,
    Fd,
}

impl XcbBuiltin {
    pub(crate) fn name(self) -> String {
        match self {
            XcbBuiltin::Atom => "ATOM".to_string(),
            XcbBuiltin::Card8 => "CARD8".to_string(),
            XcbBuiltin::Card16 => "CARD16".to_string(),
            XcbBuiltin::Card32 => "CARD32".to_string(),
            XcbBuiltin::Card64 => "CARD64".to_string(),
            XcbBuiltin::Int8 => "INT8".to_string(),
            XcbBuiltin::Int16 => "INT16".to_string(),
            XcbBuiltin::Int32 => "INT32".to_string(),
            XcbBuiltin::Float32 => "float".to_string(),
            XcbBuiltin::Float64 => "double".to_string(),
            XcbBuiltin::Bool => "BOOL".to_string(),
            XcbBuiltin::Byte => "BYTE".to_string(),
            XcbBuiltin::Char => "char".to_string(),
            XcbBuiltin::Void => "void".to_string(),
            XcbBuiltin::Fd => "fd".to_string(),
        }
    }

    pub(crate) fn rust_name(self) -> &'static str {
        match self {
            XcbBuiltin::Card32 | XcbBuiltin::Atom => "u32",
            XcbBuiltin::Bool
            | XcbBuiltin::Char
            | XcbBuiltin::Byte
            | XcbBuiltin::Void
            | XcbBuiltin::Card8 => "u8",
            XcbBuiltin::Card16 => "u16",
            XcbBuiltin::Card64 => "u64",
            XcbBuiltin::Int8 => "i8",
            XcbBuiltin::Int16 => "i16",
            XcbBuiltin::Int32 => "i32",
            XcbBuiltin::Float32 => "f32",
            XcbBuiltin::Float64 => "f64",
            XcbBuiltin::Fd => "()",
        }
    }
}

impl TypeHelpers for XcbBuiltin {
    fn byte_size(&self) -> Option<usize> {
        Some(match self {
            XcbBuiltin::Float32 | XcbBuiltin::Int32 | XcbBuiltin::Card32 | XcbBuiltin::Atom => 4,
            XcbBuiltin::Char
            | XcbBuiltin::Void
            | XcbBuiltin::Byte
            | XcbBuiltin::Bool
            | XcbBuiltin::Int8
            | XcbBuiltin::Card8 => 1,
            XcbBuiltin::Int16 | XcbBuiltin::Card16 => 2,
            XcbBuiltin::Card64 | XcbBuiltin::Float64 => 8,
            XcbBuiltin::Fd => 0,
        })
    }

    fn rust_entity_name(&self) -> String {
        XcbBuiltin::rust_name(*self).to_string()
    }

    fn source(&self) -> Option<String> {
        None
    }

    fn source_type(&self) -> SourceType {
        SourceType::Other
    }
}

#[derive(Debug, Clone)]
pub(crate) struct IdTypeAlias {
    pub(crate) name: String,
    pub(crate) source: String,
}

#[derive(Debug, Clone)]
pub(crate) struct IdTypeUnion {
    pub(crate) name: String,
    pub(crate) _cases: Vec<WrappedType>,
    pub(crate) source: String,
}

#[derive(Debug, Clone)]
pub(crate) struct ConstType {
    pub(crate) raw_name: String,
    pub(crate) builtin: WrappedType,
    pub(crate) source: Option<String>,
}

impl TypeHelpers for ConstType {
    fn byte_size(&self) -> Option<usize> {
        self.builtin.byte_size()
    }

    fn rust_entity_name(&self) -> String {
        RustCase::convert_to_valid_rust(&self.raw_name, RustCase::Pascal).unwrap()
    }

    fn source(&self) -> Option<String> {
        self.source.clone()
    }

    fn source_type(&self) -> SourceType {
        SourceType::Other
    }
}

#[derive(Debug, Clone)]
pub(crate) struct TypeAlias {
    pub(crate) new_name: String,
    pub(crate) aliased: WrappedType,
    pub(crate) source: Option<String>,
}

impl TypeHelpers for TypeAlias {
    fn byte_size(&self) -> Option<usize> {
        self.aliased.byte_size()
    }

    fn rust_entity_name(&self) -> String {
        RustCase::convert_to_valid_rust(&self.new_name, RustCase::Pascal).unwrap()
    }

    fn source(&self) -> Option<String> {
        self.source.clone()
    }

    fn source_type(&self) -> SourceType {
        SourceType::Other
    }
}

#[derive(Debug, Clone)]
pub(crate) struct XcbStruct {
    pub(crate) name: String,
    pub(crate) source: String,
    pub(crate) source_type: SourceType,
    pub(crate) used_by: ManySources,
    pub(crate) members: Vec<EntityMember>,
    pub(crate) switch: Option<WrappedType>,
}

impl TypeHelpers for XcbStruct {
    fn byte_size(&self) -> Option<usize> {
        let mut sum = 0;
        for member in &self.members {
            sum += member.byte_size()?;
        }
        if let Some(sw) = &self.switch {
            sum += sw.deref().borrow().byte_size()?;
        }
        Some(sum)
    }

    fn rust_entity_name(&self) -> String {
        RustCase::convert_to_valid_rust(&self.name, RustCase::Pascal).unwrap()
    }

    fn source(&self) -> Option<String> {
        Some(self.source.clone())
    }

    fn source_type(&self) -> SourceType {
        self.source_type
    }
}

#[derive(Debug, Clone)]
pub(crate) struct XcbUnion {
    pub(crate) name: String,
    pub(crate) source: String,
    pub(crate) source_type: SourceType,
    pub(crate) used_by: ManySources,
    pub(crate) members: Vec<EntityMember>,
}

impl From<XcbStruct> for XcbUnion {
    fn from(s: XcbStruct) -> Self {
        assert!(s.switch.is_none(), "Tried to convert XcbUnion with switch");
        Self {
            name: s.name.clone(),
            source: s.source.clone(),
            source_type: s.source_type,
            used_by: s.used_by.clone(),
            members: s.members.clone(),
        }
    }
}

impl TypeHelpers for XcbUnion {
    fn byte_size(&self) -> Option<usize> {
        let mut biggest = 0;
        for member in &self.members {
            let bs = member.byte_size().unwrap();
            if bs > biggest {
                biggest = bs;
            }
        }
        Some(biggest)
    }

    fn rust_entity_name(&self) -> String {
        RustCase::convert_to_valid_rust(&self.name, RustCase::Pascal).unwrap()
    }

    fn source(&self) -> Option<String> {
        Some(self.source.clone())
    }

    fn source_type(&self) -> SourceType {
        self.source_type
    }
}

#[derive(Debug, Clone)]
pub(crate) enum EntityMember {
    StartAlign(RequiredStartAlign),
    Field(EntityField),
    List(EntityList),
    Pad(usize),
    Align(usize),
    Length(XcbExpression),
}

impl EntityMember {
    pub(crate) fn is_constructable(&self) -> bool {
        matches!(self, EntityMember::Field(_) | EntityMember::List(_))
    }

    pub(crate) fn required_args(&self) -> Vec<EntityField> {
        match self {
            Self::Field(ef) => ef.kind.concrete.required_args(),
            Self::List(l) => {
                let mut base = l.field.kind.concrete.required_args();
                base.extend(
                    l.length_expr
                        .clone()
                        .into_iter()
                        .flat_map(|f| f.required_fields()),
                );
                base
            }
            _ => vec![],
        }
    }
}

impl TypeHelpers for EntityMember {
    fn byte_size(&self) -> Option<usize> {
        match self {
            EntityMember::StartAlign(_) => Some(0),
            EntityMember::Field(f) => f.kind.concrete.deref().borrow().byte_size(),
            EntityMember::Pad(p) => Some(*p),
            EntityMember::List(l) => l
                .fixed_count
                .and_then(|fc| l.field.kind.use_field().byte_size().map(|b| b * fc)),
            EntityMember::Align(_) | EntityMember::Length(_) => None,
        }
    }

    fn rust_field_name(&self) -> String {
        match self {
            EntityMember::Field(f) => {
                if let Ok(n) = RustCase::convert_to_valid_rust(&f.name, RustCase::Snake) {
                    n
                } else {
                    RustCase::convert_to_valid_rust(&f.name.to_lowercase(), RustCase::Snake)
                        .unwrap()
                }
            }
            EntityMember::List(l) => {
                if let Ok(n) = RustCase::convert_to_valid_rust(&l.field.name, RustCase::Snake) {
                    n
                } else {
                    RustCase::convert_to_valid_rust(&l.field.name.to_lowercase(), RustCase::Snake)
                        .unwrap()
                }
            }
            _ => panic!("Tried to get entity name of invalid member"),
        }
    }

    fn rust_entity_name(&self) -> String {
        match self {
            EntityMember::Field(f) => f
                .kind
                .reference_type
                .clone()
                .unwrap_or_else(|| f.kind.concrete.clone())
                .rust_entity_name(),
            EntityMember::List(l) => {
                let use_type = l
                    .field
                    .kind
                    .reference_type
                    .clone()
                    .unwrap_or_else(|| l.field.kind.concrete.clone())
                    .rust_entity_name();
                if let Some(count) = l.fixed_count {
                    format!("[{use_type}; {count}]")
                } else {
                    format!("alloc::vec::Vec<{use_type}>")
                }
            }
            _ => panic!("Tried to get entity name of invalid member"),
        }
    }

    fn import_name(&self, current_package: &str) -> String {
        match self {
            EntityMember::Field(f) => f
                .kind
                .reference_type
                .clone()
                .unwrap_or_else(|| f.kind.concrete.clone())
                .import_name(current_package),
            EntityMember::List(l) => {
                let use_type = l
                    .field
                    .kind
                    .reference_type
                    .clone()
                    .unwrap_or_else(|| l.field.kind.concrete.clone())
                    .import_name(current_package);
                format!("alloc::vec::Vec<{use_type}>")
            }
            _ => panic!("Tried to get entity name of invalid member"),
        }
    }

    fn source(&self) -> Option<String> {
        None
    }

    fn source_type(&self) -> SourceType {
        match self {
            EntityMember::Field(f) => f
                .kind
                .reference_type
                .clone()
                .unwrap_or_else(|| f.kind.concrete.clone())
                .source_type(),
            EntityMember::List(l) => l
                .field
                .kind
                .reference_type
                .clone()
                .unwrap_or_else(|| l.field.kind.concrete.clone())
                .source_type(),
            _ => panic!("Tried to get source_type of invalid member"),
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) struct EntityField {
    pub(crate) name: String,
    pub(crate) kind: FieldNumberRef,
}

#[derive(Debug, Clone)]
pub(crate) struct FieldNumberRef {
    pub(crate) concrete: WrappedType,
    pub(crate) reference_type: Option<WrappedType>,
}

impl FieldNumberRef {
    pub(crate) fn use_field(&self) -> WrappedType {
        self.reference_type
            .clone()
            .unwrap_or_else(|| self.concrete.clone())
    }
}

#[derive(Debug, Clone)]
pub(crate) struct EntityList {
    pub(crate) field: EntityField,
    pub(crate) fixed_count: Option<usize>,
    pub(crate) length_expr: Option<XcbExpression>,
}

#[derive(Debug, Clone)]
pub(crate) struct ValueEnum {
    pub(crate) name: String,
    pub(crate) source: String,
    pub(crate) source_type: SourceType,
    pub(crate) used_by: ManySources,
    pub(crate) concrete_number_types: ManyWrappedTypes,
    pub(crate) kvs: Vec<NumKV>,
}

impl ValueEnum {
    pub(crate) fn rust_name(&self) -> String {
        format!(
            "{}Enum",
            RustCase::convert_to_valid_rust(&self.name, RustCase::Pascal).unwrap()
        )
    }
}

#[derive(Debug, Clone)]
pub(crate) struct BitmaskEnum {
    pub(crate) name: String,
    pub(crate) source: String,
    pub(crate) source_type: SourceType,
    pub(crate) used_by: ManySources,
    pub(crate) concrete_number_types: Rc<RefCell<Vec<WrappedType>>>,
    pub(crate) default: Option<NumKV>,
    pub(crate) kvs: Vec<NumKV>,
}

#[derive(Debug, Clone)]
pub(crate) struct NumKV {
    pub(crate) key: String,
    pub(crate) value: usize,
}

#[derive(Debug, Clone)]
pub(crate) struct XcbRequest {
    pub(crate) name: String,
    pub(crate) source: String,
    pub(crate) opcode: u8,
    pub(crate) members: Vec<EntityMember>,
    pub(crate) switch: Option<WrappedType>,
    pub(crate) reply: Option<WrappedType>,
}

impl TypeHelpers for XcbRequest {
    fn byte_size(&self) -> Option<usize> {
        let mut sum = 0;
        for member in &self.members {
            sum += member.byte_size()?;
        }
        if let Some(sw) = &self.switch {
            sum += sw.deref().borrow().byte_size()?;
        }
        Some(sum)
    }

    fn rust_entity_name(&self) -> String {
        format!(
            "{}Request",
            RustCase::convert_to_valid_rust(&self.name, RustCase::Pascal).unwrap()
        )
    }

    fn source(&self) -> Option<String> {
        Some(self.source.clone())
    }

    fn source_type(&self) -> SourceType {
        SourceType::Request
    }
}

#[derive(Debug, Clone)]
pub(crate) struct XcbReply {
    pub(crate) name: String,
    pub(crate) source: String,
    pub(crate) fields: Vec<EntityMember>,
    pub(crate) switch: Option<WrappedType>,
}

impl TypeHelpers for XcbReply {
    fn byte_size(&self) -> Option<usize> {
        let mut sum = 0;
        for member in &self.fields {
            sum += member.byte_size()?;
        }
        if let Some(sw) = &self.switch {
            sum += sw.byte_size()?;
        }
        Some(sum)
    }

    fn rust_entity_name(&self) -> String {
        format!(
            "{}Reply",
            RustCase::convert_to_valid_rust(&self.name, RustCase::Pascal).unwrap()
        )
    }

    fn source(&self) -> Option<String> {
        Some(self.source.clone())
    }

    fn source_type(&self) -> SourceType {
        SourceType::Reply
    }
}

#[derive(Debug, Clone)]
pub(crate) struct XcbEvent {
    pub(crate) name: String,
    pub(crate) source: String,
    pub(crate) has_sequence: bool,
    pub(crate) opcode: u8,
    pub(crate) members: Vec<EntityMember>,
    pub(crate) generic: bool,
}

impl TypeHelpers for XcbEvent {
    fn byte_size(&self) -> Option<usize> {
        let mut sum = 0;
        for member in &self.members {
            sum += member.byte_size()?;
        }
        Some(sum)
    }

    fn rust_entity_name(&self) -> String {
        format!(
            "{}Event",
            RustCase::convert_to_valid_rust(&self.name, RustCase::Pascal).unwrap()
        )
    }

    fn source(&self) -> Option<String> {
        Some(self.source.clone())
    }

    fn source_type(&self) -> SourceType {
        SourceType::Event
    }
}

#[derive(Debug, Clone)]
pub(crate) struct SwitchStruct {
    pub(crate) name: String,
    pub(crate) parent_name: String,
    pub(crate) source: String,
    pub(crate) source_type: SourceType,
    pub(crate) used_by: ManySources,
    #[allow(unused)]
    pub(crate) start_align: Option<RequiredStartAlign>,
    pub(crate) switch_expr: XcbExpression,
    pub(crate) bit_cases: Vec<XcbSwitchCaseExpr>,
}

impl SwitchStruct {
    pub(crate) fn required_args(&self) -> Vec<EntityField> {
        // Todo: Filter out args in scope
        let mut v = vec![];
        v.extend(self.switch_expr.required_fields());
        for c in &self.bit_cases {
            v.extend(get_sorted_required_by_members(&c.fields));
        }
        v
    }
}

impl TypeHelpers for SwitchStruct {
    fn byte_size(&self) -> Option<usize> {
        None
    }

    fn rust_entity_name(&self) -> String {
        format!(
            "{}{}",
            RustCase::convert_to_valid_rust(&self.parent_name, RustCase::Pascal).unwrap(),
            RustCase::convert_to_valid_rust(&self.name, RustCase::Pascal).unwrap()
        )
    }

    fn source(&self) -> Option<String> {
        Some(self.source.clone())
    }

    fn source_type(&self) -> SourceType {
        self.source_type
    }
}

#[derive(Debug, Clone)]
pub(crate) struct SwitchEnum {
    pub(crate) name: String,
    pub(crate) parent_name: String,
    pub(crate) source: String,
    pub(crate) source_type: SourceType,
    pub(crate) used_by: ManySources,
    #[allow(unused)]
    pub(crate) start_align: Option<RequiredStartAlign>,
    pub(crate) switch_expr: XcbExpression,
    pub(crate) enum_cases: Vec<XcbSwitchCaseExpr>,
}

impl SwitchEnum {
    pub(crate) fn required_args(&self) -> Vec<EntityField> {
        let mut v = vec![];
        v.extend(self.switch_expr.required_fields());
        for c in &self.enum_cases {
            v.extend(get_sorted_required_by_members(&c.fields));
        }
        v
    }
}

impl TypeHelpers for SwitchEnum {
    fn byte_size(&self) -> Option<usize> {
        None
    }

    fn rust_entity_name(&self) -> String {
        format!(
            "{}SwitchEnum",
            RustCase::convert_to_valid_rust(&self.parent_name, RustCase::Pascal).unwrap()
        )
    }

    fn source(&self) -> Option<String> {
        Some(self.source.clone())
    }

    fn source_type(&self) -> SourceType {
        self.source_type
    }
}

#[derive(Debug, Clone)]
pub(crate) struct XcbSwitchCaseExpr {
    pub(crate) name: Option<String>,
    pub(crate) expressions: Vec<XcbExpression>,
    pub(crate) fields: Vec<EntityMember>,
    pub(crate) switches: Vec<WrappedType>,
}

#[derive(Debug, Clone)]
pub(crate) struct XcbEventStruct {
    pub(crate) name: String,
    pub(crate) source: String,
}

#[derive(Debug, Clone)]
pub(crate) enum XcbExpression {
    Binop(Binop),
    Unop(Unop),
    Fieldref(EntityField),
    Paramref(EntityField),
    Enumref(EntityField),
    Popcount(Box<XcbExpression>),
    Sumof(Sumof),
    ListelementRef,
    ListLengthExpr,
    ListForeignFieldRef(String),
    Value(i32),
    #[expect(dead_code)]
    Bit(u8),
}

impl XcbExpression {
    pub(crate) fn required_fields(&self) -> Vec<EntityField> {
        let mut set = vec![];
        self.recurse_required(&mut set);
        // Needs to be sorted to be reproducible
        set.sort_by(|a, b| a.name.cmp(&b.name));
        set
    }

    fn recurse_required(&self, input: &mut Vec<EntityField>) {
        match self {
            XcbExpression::Binop(bo) => {
                bo.right.recurse_required(input);
                bo.left.recurse_required(input);
            }
            XcbExpression::Unop(uo) => {
                uo.tgt.recurse_required(input);
            }
            XcbExpression::Fieldref(fr) => {
                input.push(fr.clone());
            }
            XcbExpression::Paramref(pr) => {
                input.push(pr.clone());
            }
            XcbExpression::Popcount(pc) => {
                pc.recurse_required(input);
            }
            XcbExpression::Sumof(so) => {
                if let Some(e) = &so.expr {
                    e.recurse_required(input);
                }
            }
            XcbExpression::Enumref(_)
            | XcbExpression::ListelementRef
            | XcbExpression::Value(_)
            | XcbExpression::Bit(_)
            | XcbExpression::ListLengthExpr
            | XcbExpression::ListForeignFieldRef(_) => {}
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) struct Binop {
    pub(crate) op: OpType,
    pub(crate) left: Box<XcbExpression>,
    pub(crate) right: Box<XcbExpression>,
}

#[derive(Debug, Clone)]
pub(crate) struct Unop {
    pub(crate) op: OpType,
    pub(crate) tgt: Box<XcbExpression>,
}

#[derive(Debug, Clone)]
pub(crate) struct Sumof {
    pub(crate) entity: EntityField,
    pub(crate) expr: Option<Box<XcbExpression>>,
}

#[derive(Debug, Copy, Clone)]
pub(crate) enum OpType {
    Sub,
    Add,
    Mul,
    Div,
    And,
    Not,
}

impl OpType {
    pub(crate) fn from_str(s: &str) -> Self {
        match s {
            "*" => OpType::Mul,
            "/" => OpType::Div,
            "-" => OpType::Sub,
            "+" => OpType::Add,
            "&" => OpType::And,
            "~" => OpType::Not,
            _ => panic!("Unrecognized op {s}"),
        }
    }
    pub(crate) fn into_op_expr(self, rev: bool) -> &'static str {
        if rev {
            self.reverse_of_expr()
        } else {
            match self {
                OpType::Sub => "core::ops::Sub::sub",
                OpType::Add => "core::ops::Add::add",
                OpType::Mul => "core::ops::Mul::mul",
                OpType::Div => "core::ops::Div::div",
                OpType::And => "core::ops::BitAnd::bitand",
                OpType::Not => "core::ops::Not::not",
            }
        }
    }
    pub(crate) fn reverse_of_expr(self) -> &'static str {
        match self {
            OpType::Sub => "core::ops::Add::add",
            OpType::Add => "core::ops::Sub::sub",
            OpType::Mul => "core::ops::Div::div",
            OpType::Div => "core::ops::Mul::mul",
            // TODO: check these
            OpType::And => "core::ops::BitAnd::bitand",
            OpType::Not => "core::ops::Not::not",
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn do_thing() {}
}
