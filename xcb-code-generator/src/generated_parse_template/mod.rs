use xcb_xsd::parse::raw_xml_parse::TagItem;
#[derive(Debug, Clone)]
pub struct Xcb {
    pub r#macro: Vec<Macro>,
    pub header: String,
    pub extension_xname: Option<String>,
    pub extension_name: Option<String>,
    pub extension_multiword: Option<bool>,
    pub major_version: Option<i32>,
    pub minor_version: Option<i32>,
}
impl Xcb {
    pub fn valid_tag(tag_name: &str) -> bool {
        tag_name == "xcb"
    }

    pub fn from_tag(tag: &TagItem) -> Self {
        let mut tag_index = 0;
        let mut r#macro = vec![];
        while tag.inner.len() > tag_index && Macro::valid_tag(&tag.inner[tag_index].name) {
            r#macro.push(Macro::from_tag(&tag.inner[tag_index]));
            tag_index += 1;
        }
        let header = tag.extract_attr_as::<String>("header").unwrap();
        let extension_xname = tag.extract_attr_as::<String>("extension-xname").ok();
        let extension_name = tag.extract_attr_as::<String>("extension-name").ok();
        let extension_multiword = tag.extract_attr_as::<bool>("extension-multiword").ok();
        let major_version = tag.extract_attr_as::<i32>("major-version").ok();
        let minor_version = tag.extract_attr_as::<i32>("minor-version").ok();
        Self {
            r#macro,
            header,
            extension_xname,
            extension_name,
            extension_multiword,
            major_version,
            minor_version,
        }
    }
}
#[derive(Debug, Clone)]
pub struct Pad {
    pub bytes: Option<i32>,
    pub align: Option<i32>,
    pub serialize: Option<bool>,
}
impl Pad {
    pub fn valid_tag(tag_name: &str) -> bool {
        tag_name == "pad"
    }

    pub fn from_tag(tag: &TagItem) -> Self {
        let mut tag_index = 0;
        let bytes = tag.extract_attr_as::<i32>("bytes").ok();
        let align = tag.extract_attr_as::<i32>("align").ok();
        let serialize = tag.extract_attr_as::<bool>("serialize").ok();
        Self {
            bytes,
            align,
            serialize,
        }
    }
}
#[derive(Debug, Clone)]
pub struct RequiredStartAlign {
    pub align: i32,
    pub offset: Option<i32>,
}
impl RequiredStartAlign {
    pub fn valid_tag(tag_name: &str) -> bool {
        tag_name == "required_start_align"
    }

    pub fn from_tag(tag: &TagItem) -> Self {
        let mut tag_index = 0;
        let align = tag.extract_attr_as::<i32>("align").unwrap();
        let offset = tag.extract_attr_as::<i32>("offset").ok();
        Self { align, offset }
    }
}
#[derive(Debug, Clone)]
pub struct Field {
    pub name: String,
    pub r#type: String,
    pub r#enum: Option<String>,
    pub altenum: Option<String>,
    pub mask: Option<String>,
    pub altmask: Option<String>,
}
impl Field {
    pub fn valid_tag(tag_name: &str) -> bool {
        tag_name == "field" || tag_name == "var"
    }

    pub fn from_tag(tag: &TagItem) -> Self {
        let mut tag_index = 0;
        let name = tag.extract_attr_as::<String>("name").unwrap();
        let r#type = tag.extract_attr_as::<String>("type").unwrap();
        let r#enum = tag.extract_attr_as::<String>("enum").ok();
        let altenum = tag.extract_attr_as::<String>("altenum").ok();
        let mask = tag.extract_attr_as::<String>("mask").ok();
        let altmask = tag.extract_attr_as::<String>("altmask").ok();
        Self {
            name,
            r#type,
            r#enum,
            altenum,
            mask,
            altmask,
        }
    }
}
#[derive(Debug, Clone)]
pub struct Var {
    pub name: String,
    pub r#type: String,
    pub r#enum: Option<String>,
    pub altenum: Option<String>,
    pub mask: Option<String>,
    pub altmask: Option<String>,
}
impl Var {
    pub fn valid_tag(tag_name: &str) -> bool {
        tag_name == "field" || tag_name == "var"
    }

    pub fn from_tag(tag: &TagItem) -> Self {
        let mut tag_index = 0;
        let name = tag.extract_attr_as::<String>("name").unwrap();
        let r#type = tag.extract_attr_as::<String>("type").unwrap();
        let r#enum = tag.extract_attr_as::<String>("enum").ok();
        let altenum = tag.extract_attr_as::<String>("altenum").ok();
        let mask = tag.extract_attr_as::<String>("mask").ok();
        let altmask = tag.extract_attr_as::<String>("altmask").ok();
        Self {
            name,
            r#type,
            r#enum,
            altenum,
            mask,
            altmask,
        }
    }
}
#[derive(Debug, Clone)]
pub struct Caseexpr {
    pub expression: Vec<Box<Expression>>,
    pub fields: Vec<Fields>,
    pub switch: Vec<Switch>,
    pub name: Option<String>,
}
impl Caseexpr {
    pub fn valid_tag(tag_name: &str) -> bool {
        tag_name == "caseexpr"
    }

    pub fn from_tag(tag: &TagItem) -> Self {
        let mut tag_index = 0;
        let mut expression = vec![];
        while tag.inner.len() > tag_index && Expression::valid_tag(&tag.inner[tag_index].name) {
            expression.push(Box::new(Expression::from_tag(&tag.inner[tag_index])));
            tag_index += 1;
        }
        let mut fields = vec![];
        while tag.inner.len() > tag_index && Fields::valid_tag(&tag.inner[tag_index].name) {
            fields.push(Fields::from_tag(&tag.inner[tag_index]));
            tag_index += 1;
        }
        let mut switch = vec![];
        while tag.inner.len() > tag_index && Switch::valid_tag(&tag.inner[tag_index].name) {
            switch.push(Switch::from_tag(&tag.inner[tag_index]));
            tag_index += 1;
        }
        let name = tag.extract_attr_as::<String>("name").ok();
        Self {
            expression,
            fields,
            switch,
            name,
        }
    }
}
#[derive(Debug, Clone)]
pub struct Switch {
    pub expression: Box<Expression>,
    pub required_start_align: Option<RequiredStartAlign>,
    pub switchexpr_choice: SwitchexprChoice,
    pub name: String,
}
impl Switch {
    pub fn valid_tag(tag_name: &str) -> bool {
        tag_name == "switch" || tag_name == "switchexpr"
    }

    pub fn from_tag(tag: &TagItem) -> Self {
        let mut tag_index = 0;
        let expression = Box::new(Expression::from_tag(&tag.inner[tag_index]));
        tag_index += 1;
        let required_start_align = if tag.inner.len() > tag_index
            && RequiredStartAlign::valid_tag(&tag.inner[tag_index].name)
        {
            let tmp = RequiredStartAlign::from_tag(&tag.inner[tag_index]);
            tag_index += 1;
            Some(tmp)
        } else {
            None
        };
        let switchexpr_choice = SwitchexprChoice::from_tag(tag, tag_index);
        tag_index += 1;
        let name = tag.extract_attr_as::<String>("name").unwrap();
        Self {
            expression,
            required_start_align,
            switchexpr_choice,
            name,
        }
    }
}
#[derive(Debug, Clone)]
pub struct Switchexpr {
    pub expression: Box<Expression>,
    pub required_start_align: Option<RequiredStartAlign>,
    pub switchexpr_choice: SwitchexprChoice,
    pub name: String,
}
impl Switchexpr {
    pub fn valid_tag(tag_name: &str) -> bool {
        tag_name == "switch" || tag_name == "switchexpr"
    }

    pub fn from_tag(tag: &TagItem) -> Self {
        let mut tag_index = 0;
        let expression = Box::new(Expression::from_tag(&tag.inner[tag_index]));
        tag_index += 1;
        let required_start_align = if tag.inner.len() > tag_index
            && RequiredStartAlign::valid_tag(&tag.inner[tag_index].name)
        {
            let tmp = RequiredStartAlign::from_tag(&tag.inner[tag_index]);
            tag_index += 1;
            Some(tmp)
        } else {
            None
        };
        let switchexpr_choice = SwitchexprChoice::from_tag(tag, tag_index);
        tag_index += 1;
        let name = tag.extract_attr_as::<String>("name").unwrap();
        Self {
            expression,
            required_start_align,
            switchexpr_choice,
            name,
        }
    }
}
#[derive(Debug, Clone)]
pub struct Length {
    pub expression: Box<Expression>,
}
impl Length {
    pub fn valid_tag(tag_name: &str) -> bool {
        tag_name == "length"
    }

    pub fn from_tag(tag: &TagItem) -> Self {
        let mut tag_index = 0;
        let expression = Box::new(Expression::from_tag(&tag.inner[tag_index]));
        tag_index += 1;
        Self { expression }
    }
}
#[derive(Debug, Clone)]
pub struct Fd {
    pub name: String,
}
impl Fd {
    pub fn valid_tag(tag_name: &str) -> bool {
        tag_name == "fd"
    }

    pub fn from_tag(tag: &TagItem) -> Self {
        let mut tag_index = 0;
        let name = tag.extract_attr_as::<String>("name").unwrap();
        Self { name }
    }
}
#[derive(Debug, Clone)]
pub struct List {
    pub var: Var,
    pub expression: Option<Box<Expression>>,
}
impl List {
    pub fn valid_tag(tag_name: &str) -> bool {
        tag_name == "list"
    }

    pub fn from_tag(tag: &TagItem) -> Self {
        let mut tag_index = 0;
        let var = Var::from_tag(&tag);
        let expression =
            if tag.inner.len() > tag_index && Expression::valid_tag(&tag.inner[tag_index].name) {
                let tmp = Box::new(Expression::from_tag(&tag.inner[tag_index]));
                tag_index += 1;
                Some(tmp)
            } else {
                None
            };
        Self { var, expression }
    }
}
#[derive(Debug, Clone)]
pub struct ExpressionOp {
    pub expression: Box<Expression>,
    pub expression_1: Box<Expression>,
    pub op: String,
}
impl ExpressionOp {
    pub fn valid_tag(tag_name: &str) -> bool {
        tag_name == "op"
    }

    pub fn from_tag(tag: &TagItem) -> Self {
        let mut tag_index = 0;
        let expression = Box::new(Expression::from_tag(&tag.inner[tag_index]));
        tag_index += 1;
        let expression_1 = Box::new(Expression::from_tag(&tag.inner[tag_index]));
        tag_index += 1;
        let op = tag.extract_attr_as::<String>("op").unwrap();
        Self {
            expression,
            expression_1,
            op,
        }
    }
}
#[derive(Debug, Clone)]
pub struct ExpressionUnop {
    pub expression: Box<Expression>,
    pub op: String,
}
impl ExpressionUnop {
    pub fn valid_tag(tag_name: &str) -> bool {
        tag_name == "unop"
    }

    pub fn from_tag(tag: &TagItem) -> Self {
        let mut tag_index = 0;
        let expression = Box::new(Expression::from_tag(&tag.inner[tag_index]));
        tag_index += 1;
        let op = tag.extract_attr_as::<String>("op").unwrap();
        Self { expression, op }
    }
}
#[derive(Debug, Clone)]
pub struct ExpressionParamref {
    pub string: String,
    pub r#type: String,
}
impl ExpressionParamref {
    pub fn valid_tag(tag_name: &str) -> bool {
        tag_name == "paramref"
    }

    pub fn from_tag(tag: &TagItem) -> Self {
        let mut tag_index = 0;
        let string = tag.extract_value_as::<String>().unwrap();
        let r#type = tag.extract_attr_as::<String>("type").unwrap();
        Self { string, r#type }
    }
}
#[derive(Debug, Clone)]
pub struct ExpressionEnumref {
    pub string: String,
    pub r#ref: String,
}
impl ExpressionEnumref {
    pub fn valid_tag(tag_name: &str) -> bool {
        tag_name == "enumref"
    }

    pub fn from_tag(tag: &TagItem) -> Self {
        let mut tag_index = 0;
        let string = tag.extract_value_as::<String>().unwrap();
        let r#ref = tag.extract_attr_as::<String>("ref").unwrap();
        Self { string, r#ref }
    }
}
#[derive(Debug, Clone)]
pub struct ExpressionPopcount {
    pub expression: Box<Expression>,
}
impl ExpressionPopcount {
    pub fn valid_tag(tag_name: &str) -> bool {
        tag_name == "popcount"
    }

    pub fn from_tag(tag: &TagItem) -> Self {
        let mut tag_index = 0;
        let expression = Box::new(Expression::from_tag(&tag.inner[tag_index]));
        tag_index += 1;
        Self { expression }
    }
}
#[derive(Debug, Clone)]
pub struct ExpressionSumof {
    pub expression: Option<Box<Expression>>,
    pub r#ref: String,
}
impl ExpressionSumof {
    pub fn valid_tag(tag_name: &str) -> bool {
        tag_name == "sumof"
    }

    pub fn from_tag(tag: &TagItem) -> Self {
        let mut tag_index = 0;
        let expression =
            if tag.inner.len() > tag_index && Expression::valid_tag(&tag.inner[tag_index].name) {
                let tmp = Box::new(Expression::from_tag(&tag.inner[tag_index]));
                tag_index += 1;
                Some(tmp)
            } else {
                None
            };
        let r#ref = tag.extract_attr_as::<String>("ref").unwrap();
        Self { expression, r#ref }
    }
}
#[derive(Debug, Clone)]
pub struct ExpressionListelementRef {}
impl ExpressionListelementRef {
    pub fn valid_tag(tag_name: &str) -> bool {
        tag_name == "listelement-ref"
    }

    pub fn from_tag(tag: &TagItem) -> Self {
        let mut tag_index = 0;
        Self {}
    }
}
#[derive(Debug, Clone)]
pub struct Exprfield {
    pub var: Var,
    pub expression: Box<Expression>,
}
impl Exprfield {
    pub fn valid_tag(tag_name: &str) -> bool {
        tag_name == "exprfield"
    }

    pub fn from_tag(tag: &TagItem) -> Self {
        let mut tag_index = 0;
        let var = Var::from_tag(&tag);
        let expression = Box::new(Expression::from_tag(&tag.inner[tag_index]));
        tag_index += 1;
        Self { var, expression }
    }
}
#[derive(Debug, Clone)]
pub struct Struct {
    pub fields: Vec<Fields>,
    pub switch: Option<Switch>,
    pub name: String,
}
impl Struct {
    pub fn valid_tag(tag_name: &str) -> bool {
        tag_name == "struct"
    }

    pub fn from_tag(tag: &TagItem) -> Self {
        let mut tag_index = 0;
        let mut fields = vec![];
        while tag.inner.len() > tag_index && Fields::valid_tag(&tag.inner[tag_index].name) {
            fields.push(Fields::from_tag(&tag.inner[tag_index]));
            tag_index += 1;
        }
        let switch = if tag.inner.len() > tag_index && Switch::valid_tag(&tag.inner[tag_index].name)
        {
            let tmp = Switch::from_tag(&tag.inner[tag_index]);
            tag_index += 1;
            Some(tmp)
        } else {
            None
        };
        let name = tag.extract_attr_as::<String>("name").unwrap();
        Self {
            fields,
            switch,
            name,
        }
    }
}
#[derive(Debug, Clone)]
pub struct PacketStruct {
    pub fields: Vec<Fields>,
    pub name: String,
    pub number: i32,
}
impl PacketStruct {
    pub fn valid_tag(tag_name: &str) -> bool {
        tag_name == "packet-struct"
    }

    pub fn from_tag(tag: &TagItem) -> Self {
        let mut tag_index = 0;
        let mut fields = vec![];
        while tag.inner.len() > tag_index && Fields::valid_tag(&tag.inner[tag_index].name) {
            fields.push(Fields::from_tag(&tag.inner[tag_index]));
            tag_index += 1;
        }
        let name = tag.extract_attr_as::<String>("name").unwrap();
        let number = tag.extract_attr_as::<i32>("number").unwrap();
        Self {
            fields,
            name,
            number,
        }
    }
}
#[derive(Debug, Clone)]
pub struct PacketStructCopy {
    pub name: String,
    pub number: i32,
    pub r#ref: String,
}
impl PacketStructCopy {
    pub fn valid_tag(tag_name: &str) -> bool {
        tag_name == "packet-struct-copy"
    }

    pub fn from_tag(tag: &TagItem) -> Self {
        let mut tag_index = 0;
        let name = tag.extract_attr_as::<String>("name").unwrap();
        let number = tag.extract_attr_as::<i32>("number").unwrap();
        let r#ref = tag.extract_attr_as::<String>("ref").unwrap();
        Self {
            name,
            number,
            r#ref,
        }
    }
}
#[derive(Debug, Clone)]
pub struct EventstructAllowed {
    pub extension: String,
    pub xge: bool,
    pub opcode_min: i32,
    pub opcode_max: i32,
}
impl EventstructAllowed {
    pub fn valid_tag(tag_name: &str) -> bool {
        tag_name == "allowed"
    }

    pub fn from_tag(tag: &TagItem) -> Self {
        let mut tag_index = 0;
        let extension = tag.extract_attr_as::<String>("extension").unwrap();
        let xge = tag.extract_attr_as::<bool>("xge").unwrap();
        let opcode_min = tag.extract_attr_as::<i32>("opcode-min").unwrap();
        let opcode_max = tag.extract_attr_as::<i32>("opcode-max").unwrap();
        Self {
            extension,
            xge,
            opcode_min,
            opcode_max,
        }
    }
}
#[derive(Debug, Clone)]
pub struct Eventstruct {
    pub eventstruct_allowed: EventstructAllowed,
    pub name: String,
}
impl Eventstruct {
    pub fn valid_tag(tag_name: &str) -> bool {
        tag_name == "eventstruct"
    }

    pub fn from_tag(tag: &TagItem) -> Self {
        let mut tag_index = 0;
        let eventstruct_allowed = EventstructAllowed::from_tag(&tag.inner[tag_index]);
        tag_index += 1;
        let name = tag.extract_attr_as::<String>("name").unwrap();
        Self {
            eventstruct_allowed,
            name,
        }
    }
}
#[derive(Debug, Clone)]
pub struct BitType {
    pub integer: i32,
}
impl BitType {
    pub fn valid_tag(tag_name: &str) -> bool {
        tag_name == "bitType"
    }

    pub fn from_tag(tag: &TagItem) -> Self {
        let mut tag_index = 0;
        let integer = tag.extract_value_as::<i32>().unwrap();
        Self { integer }
    }
}
#[derive(Debug, Clone)]
pub struct DocFieldsField {
    pub string: String,
    pub name: Option<String>,
}
impl DocFieldsField {
    pub fn valid_tag(tag_name: &str) -> bool {
        tag_name == "field"
    }

    pub fn from_tag(tag: &TagItem) -> Self {
        let mut tag_index = 0;
        let string = tag.extract_value_as::<String>().unwrap();
        let name = tag.extract_attr_as::<String>("name").ok();
        Self { string, name }
    }
}
#[derive(Debug, Clone)]
pub struct DocFields {
    pub doc_fields_field: DocFieldsField,
}
impl DocFields {
    pub fn valid_tag(tag_name: &str) -> bool {
        tag_name == "doc-fields"
    }

    pub fn from_tag(tag: &TagItem) -> Self {
        let mut tag_index = 0;
        let doc_fields_field = DocFieldsField::from_tag(&tag.inner[tag_index]);
        tag_index += 1;
        Self { doc_fields_field }
    }
}
#[derive(Debug, Clone)]
pub struct ErrorFieldsError {
    pub string: String,
    pub r#type: Option<String>,
}
impl ErrorFieldsError {
    pub fn valid_tag(tag_name: &str) -> bool {
        tag_name == "error"
    }

    pub fn from_tag(tag: &TagItem) -> Self {
        let mut tag_index = 0;
        let string = tag.extract_value_as::<String>().unwrap();
        let r#type = tag.extract_attr_as::<String>("type").ok();
        Self { string, r#type }
    }
}
#[derive(Debug, Clone)]
pub struct ErrorFields {
    pub error_fields_error: ErrorFieldsError,
}
impl ErrorFields {
    pub fn valid_tag(tag_name: &str) -> bool {
        tag_name == "error-fields"
    }

    pub fn from_tag(tag: &TagItem) -> Self {
        let mut tag_index = 0;
        let error_fields_error = ErrorFieldsError::from_tag(&tag.inner[tag_index]);
        tag_index += 1;
        Self { error_fields_error }
    }
}
#[derive(Debug, Clone)]
pub struct SeeFieldsSee {
    pub name: Option<String>,
    pub r#type: Option<String>,
}
impl SeeFieldsSee {
    pub fn valid_tag(tag_name: &str) -> bool {
        tag_name == "see"
    }

    pub fn from_tag(tag: &TagItem) -> Self {
        let mut tag_index = 0;
        let name = tag.extract_attr_as::<String>("name").ok();
        let r#type = tag.extract_attr_as::<String>("type").ok();
        Self { name, r#type }
    }
}
#[derive(Debug, Clone)]
pub struct SeeFields {
    pub see_fields_see: SeeFieldsSee,
}
impl SeeFields {
    pub fn valid_tag(tag_name: &str) -> bool {
        tag_name == "see-fields"
    }

    pub fn from_tag(tag: &TagItem) -> Self {
        let mut tag_index = 0;
        let see_fields_see = SeeFieldsSee::from_tag(&tag.inner[tag_index]);
        tag_index += 1;
        Self { see_fields_see }
    }
}
#[derive(Debug, Clone)]
pub struct Doc {
    pub brief: Option<String>,
    pub description: Option<String>,
    pub example: Option<String>,
    pub doc_fields: Vec<DocFields>,
    pub error_fields: Vec<ErrorFields>,
    pub see_fields: Vec<SeeFields>,
}
impl Doc {
    pub fn valid_tag(tag_name: &str) -> bool {
        tag_name == "doc"
    }

    pub fn from_tag(tag: &TagItem) -> Self {
        let mut tag_index = 0;
        let brief = if tag.inner.len() > tag_index && "brief" == tag.name {
            let tmp = tag.extract_value_as::<String>().unwrap();
            Some(tmp)
        } else {
            None
        };
        let description = if tag.inner.len() > tag_index && "description" == tag.name {
            let tmp = tag.extract_value_as::<String>().unwrap();
            Some(tmp)
        } else {
            None
        };
        let example = if tag.inner.len() > tag_index && "example" == tag.name {
            let tmp = tag.extract_value_as::<String>().unwrap();
            Some(tmp)
        } else {
            None
        };
        let mut doc_fields = vec![];
        while tag.inner.len() > tag_index && DocFields::valid_tag(&tag.inner[tag_index].name) {
            doc_fields.push(DocFields::from_tag(&tag.inner[tag_index]));
            tag_index += 1;
        }
        let mut error_fields = vec![];
        while tag.inner.len() > tag_index && ErrorFields::valid_tag(&tag.inner[tag_index].name) {
            error_fields.push(ErrorFields::from_tag(&tag.inner[tag_index]));
            tag_index += 1;
        }
        let mut see_fields = vec![];
        while tag.inner.len() > tag_index && SeeFields::valid_tag(&tag.inner[tag_index].name) {
            see_fields.push(SeeFields::from_tag(&tag.inner[tag_index]));
            tag_index += 1;
        }
        Self {
            brief,
            description,
            example,
            doc_fields,
            error_fields,
            see_fields,
        }
    }
}
#[derive(Debug, Clone)]
pub struct RequestReply {
    pub fields: Vec<Fields>,
    pub switch: Option<Switch>,
    pub doc: Option<Doc>,
}
impl RequestReply {
    pub fn valid_tag(tag_name: &str) -> bool {
        tag_name == "reply"
    }

    pub fn from_tag(tag: &TagItem) -> Self {
        let mut tag_index = 0;
        let mut fields = vec![];
        while tag.inner.len() > tag_index && Fields::valid_tag(&tag.inner[tag_index].name) {
            fields.push(Fields::from_tag(&tag.inner[tag_index]));
            tag_index += 1;
        }
        let switch = if tag.inner.len() > tag_index && Switch::valid_tag(&tag.inner[tag_index].name)
        {
            let tmp = Switch::from_tag(&tag.inner[tag_index]);
            tag_index += 1;
            Some(tmp)
        } else {
            None
        };
        let doc = if tag.inner.len() > tag_index && Doc::valid_tag(&tag.inner[tag_index].name) {
            let tmp = Doc::from_tag(&tag.inner[tag_index]);
            tag_index += 1;
            Some(tmp)
        } else {
            None
        };
        Self {
            fields,
            switch,
            doc,
        }
    }
}
#[derive(Debug, Clone)]
pub struct MacroRequest {
    pub request_choice: Vec<RequestChoice>,
    pub switch: Option<Switch>,
    pub request_reply: Option<RequestReply>,
    pub doc: Option<Doc>,
    pub name: String,
    pub opcode: i32,
    pub combine_adjacent: Option<bool>,
}
impl MacroRequest {
    pub fn valid_tag(tag_name: &str) -> bool {
        tag_name == "request"
    }

    pub fn from_tag(tag: &TagItem) -> Self {
        let mut tag_index = 0;
        let mut request_choice = vec![];
        while tag.inner.len() > tag_index && RequestChoice::valid_tag(&tag.inner[tag_index].name) {
            request_choice.push(RequestChoice::from_tag(&tag.inner[tag_index]));
            tag_index += 1;
        }
        let switch = if tag.inner.len() > tag_index && Switch::valid_tag(&tag.inner[tag_index].name)
        {
            let tmp = Switch::from_tag(&tag.inner[tag_index]);
            tag_index += 1;
            Some(tmp)
        } else {
            None
        };
        let request_reply =
            if tag.inner.len() > tag_index && RequestReply::valid_tag(&tag.inner[tag_index].name) {
                let tmp = RequestReply::from_tag(&tag.inner[tag_index]);
                tag_index += 1;
                Some(tmp)
            } else {
                None
            };
        let doc = if tag.inner.len() > tag_index && Doc::valid_tag(&tag.inner[tag_index].name) {
            let tmp = Doc::from_tag(&tag.inner[tag_index]);
            tag_index += 1;
            Some(tmp)
        } else {
            None
        };
        let name = tag.extract_attr_as::<String>("name").unwrap();
        let opcode = tag.extract_attr_as::<i32>("opcode").unwrap();
        let combine_adjacent = tag.extract_attr_as::<bool>("combine-adjacent").ok();
        Self {
            request_choice,
            switch,
            request_reply,
            doc,
            name,
            opcode,
            combine_adjacent,
        }
    }
}
#[derive(Debug, Clone)]
pub struct MacroEvent {
    pub packet_struct: PacketStruct,
    pub doc: Option<Doc>,
    pub no_sequence_number: Option<bool>,
    pub xge: Option<bool>,
}
impl MacroEvent {
    pub fn valid_tag(tag_name: &str) -> bool {
        tag_name == "event"
    }

    pub fn from_tag(tag: &TagItem) -> Self {
        let mut tag_index = 0;
        let packet_struct = PacketStruct::from_tag(&tag);
        let doc = if tag.inner.len() > tag_index && Doc::valid_tag(&tag.inner[tag_index].name) {
            let tmp = Doc::from_tag(&tag.inner[tag_index]);
            tag_index += 1;
            Some(tmp)
        } else {
            None
        };
        let no_sequence_number = tag.extract_attr_as::<bool>("no-sequence-number").ok();
        let xge = tag.extract_attr_as::<bool>("xge").ok();
        Self {
            packet_struct,
            doc,
            no_sequence_number,
            xge,
        }
    }
}
#[derive(Debug, Clone)]
pub struct MacroXidtype {
    pub name: String,
}
impl MacroXidtype {
    pub fn valid_tag(tag_name: &str) -> bool {
        tag_name == "xidtype"
    }

    pub fn from_tag(tag: &TagItem) -> Self {
        let mut tag_index = 0;
        let name = tag.extract_attr_as::<String>("name").unwrap();
        Self { name }
    }
}
#[derive(Debug, Clone)]
pub struct MacroXidunion {
    pub r#type: Vec<String>,
    pub name: String,
}
impl MacroXidunion {
    pub fn valid_tag(tag_name: &str) -> bool {
        tag_name == "xidunion"
    }

    pub fn from_tag(tag: &TagItem) -> Self {
        let mut tag_index = 0;
        let mut r#type = vec![];
        while tag.inner.len() > tag_index && "type" == tag.inner[tag_index].name {
            r#type.push(tag.inner[tag_index].extract_value_as::<String>().unwrap());
            tag_index += 1;
        }
        let name = tag.extract_attr_as::<String>("name").unwrap();
        Self { r#type, name }
    }
}
#[derive(Debug, Clone)]
pub struct EnumItem {
    pub item_choice: ItemChoice,
    pub name: String,
}
impl EnumItem {
    pub fn valid_tag(tag_name: &str) -> bool {
        tag_name == "item"
    }

    pub fn from_tag(tag: &TagItem) -> Self {
        let mut tag_index = 0;
        let item_choice = ItemChoice::from_tag(&tag.inner[tag_index]);
        tag_index += 1;
        let name = tag.extract_attr_as::<String>("name").unwrap();
        Self { item_choice, name }
    }
}
#[derive(Debug, Clone)]
pub struct MacroEnum {
    pub enum_item: Vec<EnumItem>,
    pub doc: Option<Doc>,
    pub name: String,
}
impl MacroEnum {
    pub fn valid_tag(tag_name: &str) -> bool {
        tag_name == "enum"
    }

    pub fn from_tag(tag: &TagItem) -> Self {
        let mut tag_index = 0;
        let mut enum_item = vec![];
        while tag.inner.len() > tag_index && EnumItem::valid_tag(&tag.inner[tag_index].name) {
            enum_item.push(EnumItem::from_tag(&tag.inner[tag_index]));
            tag_index += 1;
        }
        let doc = if tag.inner.len() > tag_index && Doc::valid_tag(&tag.inner[tag_index].name) {
            let tmp = Doc::from_tag(&tag.inner[tag_index]);
            tag_index += 1;
            Some(tmp)
        } else {
            None
        };
        let name = tag.extract_attr_as::<String>("name").unwrap();
        Self {
            enum_item,
            doc,
            name,
        }
    }
}
#[derive(Debug, Clone)]
pub struct MacroTypedef {
    pub oldname: String,
    pub newname: String,
}
impl MacroTypedef {
    pub fn valid_tag(tag_name: &str) -> bool {
        tag_name == "typedef"
    }

    pub fn from_tag(tag: &TagItem) -> Self {
        let mut tag_index = 0;
        let oldname = tag.extract_attr_as::<String>("oldname").unwrap();
        let newname = tag.extract_attr_as::<String>("newname").unwrap();
        Self { oldname, newname }
    }
}
#[derive(Debug, Clone)]
pub enum SwitchexprChoice {
    Bitcase(Vec<Caseexpr>),

    Case(Vec<Caseexpr>),
}
impl SwitchexprChoice {
    pub fn valid_tag(tag_name: &str) -> bool {
        if tag_name == "bitcase" {
            return true;
        }
        if tag_name == "case" {
            return true;
        }
        return false;
    }

    pub fn from_tag(tag: &TagItem, mut tag_index: usize) -> Self {
        if tag.inner.len() <= tag_index {
            return Self::Bitcase(vec![]);
        }
        if tag.inner[tag_index].name == "bitcase" {
            let mut bitcase = vec![];
            while tag.inner.len() > tag_index && tag.inner[tag_index].name == "bitcase" {
                bitcase.push(Caseexpr::from_tag(&tag.inner[tag_index]));
                tag_index += 1;
            }
            return Self::Bitcase(bitcase);
        }
        if tag.inner.len() <= tag_index {
            return Self::Case(vec![]);
        }
        if tag.inner[tag_index].name == "case" {
            let mut case = vec![];
            while tag.inner.len() > tag_index && tag.inner[tag_index].name == "case" {
                case.push(Caseexpr::from_tag(&tag.inner[tag_index]));
                tag_index += 1;
            }
            return Self::Case(case);
        }
        panic!("Unrecognized element name {}", tag.name);
    }
}
#[derive(Debug, Clone)]
pub enum Expression {
    Op(ExpressionOp),

    Unop(ExpressionUnop),

    Fieldref(String),

    Paramref(ExpressionParamref),

    Enumref(ExpressionEnumref),

    Popcount(ExpressionPopcount),

    Sumof(ExpressionSumof),

    ListelementRef(ExpressionListelementRef),

    Value(i32),

    Bit(BitType),
}
impl Expression {
    pub fn valid_tag(tag_name: &str) -> bool {
        if ExpressionOp::valid_tag(tag_name) {
            return true;
        }
        if ExpressionUnop::valid_tag(tag_name) {
            return true;
        }
        if tag_name == "fieldref" {
            return true;
        }
        if ExpressionParamref::valid_tag(tag_name) {
            return true;
        }
        if ExpressionEnumref::valid_tag(tag_name) {
            return true;
        }
        if ExpressionPopcount::valid_tag(tag_name) {
            return true;
        }
        if ExpressionSumof::valid_tag(tag_name) {
            return true;
        }
        if ExpressionListelementRef::valid_tag(tag_name) {
            return true;
        }
        if tag_name == "value" {
            return true;
        }
        if tag_name == "bit" {
            return true;
        }
        return false;
    }

    pub fn from_tag(tag: &TagItem) -> Self {
        let mut tag_index = 0;
        let tag_name = tag.name.as_str();
        if ExpressionOp::valid_tag(tag_name) {
            return Self::Op(ExpressionOp::from_tag(tag));
        }
        let tag_name = tag.name.as_str();
        if ExpressionUnop::valid_tag(tag_name) {
            return Self::Unop(ExpressionUnop::from_tag(tag));
        }
        let tag_name = tag.name.as_str();
        if tag_name == "fieldref" {
            return Self::Fieldref(tag.extract_value_as::<String>().unwrap());
        }
        let tag_name = tag.name.as_str();
        if ExpressionParamref::valid_tag(tag_name) {
            return Self::Paramref(ExpressionParamref::from_tag(tag));
        }
        let tag_name = tag.name.as_str();
        if ExpressionEnumref::valid_tag(tag_name) {
            return Self::Enumref(ExpressionEnumref::from_tag(tag));
        }
        let tag_name = tag.name.as_str();
        if ExpressionPopcount::valid_tag(tag_name) {
            return Self::Popcount(ExpressionPopcount::from_tag(tag));
        }
        let tag_name = tag.name.as_str();
        if ExpressionSumof::valid_tag(tag_name) {
            return Self::Sumof(ExpressionSumof::from_tag(tag));
        }
        let tag_name = tag.name.as_str();
        if ExpressionListelementRef::valid_tag(tag_name) {
            return Self::ListelementRef(ExpressionListelementRef::from_tag(tag));
        }
        let tag_name = tag.name.as_str();
        if tag_name == "value" {
            return Self::Value(tag.extract_value_as::<i32>().unwrap());
        }
        let tag_name = tag.name.as_str();
        if tag_name == "bit" {
            return Self::Bit(BitType::from_tag(tag));
        }
        panic!("Unrecognized element name {}", tag.name);
    }
}
#[derive(Debug, Clone)]
pub enum Fields {
    Pad(Pad),

    Field(Field),

    List(List),

    Fd(Fd),

    RequiredStartAlign(RequiredStartAlign),

    Length(Length),
}
impl Fields {
    pub fn valid_tag(tag_name: &str) -> bool {
        if Pad::valid_tag(tag_name) {
            return true;
        }
        if Field::valid_tag(tag_name) {
            return true;
        }
        if List::valid_tag(tag_name) {
            return true;
        }
        if Fd::valid_tag(tag_name) {
            return true;
        }
        if RequiredStartAlign::valid_tag(tag_name) {
            return true;
        }
        if Length::valid_tag(tag_name) {
            return true;
        }
        return false;
    }

    pub fn from_tag(tag: &TagItem) -> Self {
        let mut tag_index = 0;
        let tag_name = tag.name.as_str();
        if Pad::valid_tag(tag_name) {
            return Self::Pad(Pad::from_tag(tag));
        }
        let tag_name = tag.name.as_str();
        if Field::valid_tag(tag_name) {
            return Self::Field(Field::from_tag(tag));
        }
        let tag_name = tag.name.as_str();
        if List::valid_tag(tag_name) {
            return Self::List(List::from_tag(tag));
        }
        let tag_name = tag.name.as_str();
        if Fd::valid_tag(tag_name) {
            return Self::Fd(Fd::from_tag(tag));
        }
        let tag_name = tag.name.as_str();
        if RequiredStartAlign::valid_tag(tag_name) {
            return Self::RequiredStartAlign(RequiredStartAlign::from_tag(tag));
        }
        let tag_name = tag.name.as_str();
        if Length::valid_tag(tag_name) {
            return Self::Length(Length::from_tag(tag));
        }
        panic!("Unrecognized element name {}", tag.name);
    }
}
#[derive(Debug, Clone)]
pub enum RequestChoice {
    Fields(Fields),

    Exprfield(Exprfield),
}
impl RequestChoice {
    pub fn valid_tag(tag_name: &str) -> bool {
        if Fields::valid_tag(tag_name) {
            return true;
        }
        if Exprfield::valid_tag(tag_name) {
            return true;
        }
        return false;
    }

    pub fn from_tag(tag: &TagItem) -> Self {
        let mut tag_index = 0;
        let tag_name = tag.name.as_str();
        if Fields::valid_tag(tag_name) {
            return Self::Fields(Fields::from_tag(tag));
        }
        let tag_name = tag.name.as_str();
        if Exprfield::valid_tag(tag_name) {
            return Self::Exprfield(Exprfield::from_tag(tag));
        }
        panic!("Unrecognized element name {}", tag.name);
    }
}
#[derive(Debug, Clone)]
pub enum ItemChoice {
    Value(u32),

    Bit(BitType),
}
impl ItemChoice {
    pub fn valid_tag(tag_name: &str) -> bool {
        if tag_name == "value" {
            return true;
        }
        if tag_name == "bit" {
            return true;
        }
        return false;
    }

    pub fn from_tag(tag: &TagItem) -> Self {
        let mut tag_index = 0;
        let tag_name = tag.name.as_str();
        if tag_name == "value" {
            return Self::Value(tag.extract_value_as::<u32>().unwrap());
        }
        let tag_name = tag.name.as_str();
        if tag_name == "bit" {
            return Self::Bit(BitType::from_tag(tag));
        }
        panic!("Unrecognized element name {}", tag.name);
    }
}
#[derive(Debug, Clone)]
pub enum Macro {
    Request(MacroRequest),

    Event(MacroEvent),

    Eventcopy(PacketStructCopy),

    Error(PacketStruct),

    Errorcopy(PacketStructCopy),

    Struct(Struct),

    Union(Struct),

    Eventstruct(Eventstruct),

    Xidtype(MacroXidtype),

    Xidunion(MacroXidunion),

    Enum(MacroEnum),

    Typedef(MacroTypedef),

    Import(String),
}
impl Macro {
    pub fn valid_tag(tag_name: &str) -> bool {
        if MacroRequest::valid_tag(tag_name) {
            return true;
        }
        if MacroEvent::valid_tag(tag_name) {
            return true;
        }
        if tag_name == "eventcopy" {
            return true;
        }
        if tag_name == "error" {
            return true;
        }
        if tag_name == "errorcopy" {
            return true;
        }
        if Struct::valid_tag(tag_name) {
            return true;
        }
        if tag_name == "union" {
            return true;
        }
        if Eventstruct::valid_tag(tag_name) {
            return true;
        }
        if MacroXidtype::valid_tag(tag_name) {
            return true;
        }
        if MacroXidunion::valid_tag(tag_name) {
            return true;
        }
        if MacroEnum::valid_tag(tag_name) {
            return true;
        }
        if MacroTypedef::valid_tag(tag_name) {
            return true;
        }
        if tag_name == "import" {
            return true;
        }
        return false;
    }

    pub fn from_tag(tag: &TagItem) -> Self {
        let mut tag_index = 0;
        let tag_name = tag.name.as_str();
        if MacroRequest::valid_tag(tag_name) {
            return Self::Request(MacroRequest::from_tag(tag));
        }
        let tag_name = tag.name.as_str();
        if MacroEvent::valid_tag(tag_name) {
            return Self::Event(MacroEvent::from_tag(tag));
        }
        let tag_name = tag.name.as_str();
        if tag_name == "eventcopy" {
            return Self::Eventcopy(PacketStructCopy::from_tag(tag));
        }
        let tag_name = tag.name.as_str();
        if tag_name == "error" {
            return Self::Error(PacketStruct::from_tag(tag));
        }
        let tag_name = tag.name.as_str();
        if tag_name == "errorcopy" {
            return Self::Errorcopy(PacketStructCopy::from_tag(tag));
        }
        let tag_name = tag.name.as_str();
        if Struct::valid_tag(tag_name) {
            return Self::Struct(Struct::from_tag(tag));
        }
        let tag_name = tag.name.as_str();
        if tag_name == "union" {
            return Self::Union(Struct::from_tag(tag));
        }
        let tag_name = tag.name.as_str();
        if Eventstruct::valid_tag(tag_name) {
            return Self::Eventstruct(Eventstruct::from_tag(tag));
        }
        let tag_name = tag.name.as_str();
        if MacroXidtype::valid_tag(tag_name) {
            return Self::Xidtype(MacroXidtype::from_tag(tag));
        }
        let tag_name = tag.name.as_str();
        if MacroXidunion::valid_tag(tag_name) {
            return Self::Xidunion(MacroXidunion::from_tag(tag));
        }
        let tag_name = tag.name.as_str();
        if MacroEnum::valid_tag(tag_name) {
            return Self::Enum(MacroEnum::from_tag(tag));
        }
        let tag_name = tag.name.as_str();
        if MacroTypedef::valid_tag(tag_name) {
            return Self::Typedef(MacroTypedef::from_tag(tag));
        }
        let tag_name = tag.name.as_str();
        if tag_name == "import" {
            return Self::Import(tag.extract_value_as::<String>().unwrap());
        }
        panic!("Unrecognized element name {}", tag.name);
    }
}
