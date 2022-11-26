use std::collections::HashMap;
use std::fmt::Write;
use std::io::Read;
use std::str::FromStr;

use anyhow::{Error, Result};
use xml::reader::XmlEvent;
use xml::EventReader;

use crate::parse::const_tags::{MAX_OCCURS, MIN_OCCURS, NAME};

#[derive(Debug, Copy, Clone)]
pub struct TagSpec<'a> {
    name: &'a str,
}

impl<'a> TagSpec<'a> {
    #[must_use]
    pub fn new(name: &'a str) -> Self {
        Self { name }
    }

    pub fn churn<R: Read>(
        &mut self,
        parent: Option<Box<TagItemParentRef>>,
        reader: &mut EventReader<R>,
        me: XmlEvent,
    ) -> Result<TagItem> {
        let mut ti = TagItem {
            name: self.name.to_string(),
            attrs: HashMap::default(),
            parent,
            inner: Vec::default(),
            value: None,
        };
        if let XmlEvent::StartElement {
            name,
            attributes,
            namespace: _,
        } = me
        {
            assert_eq!(self.name, name.local_name);
            for attribute in &attributes {
                ti.attrs
                    .insert(attribute.name.local_name.clone(), attribute.value.clone());
            }
        }
        loop {
            let event = reader.next()?;
            match event {
                XmlEvent::StartElement {
                    name,
                    attributes,
                    namespace,
                } => {
                    let mut new_spec = TagSpec::new(name.local_name.as_str());
                    let me_ref = TagItemParentRef {
                        name: ti.name.clone(),
                        attrs: ti.attrs.clone(),
                        parent: ti.parent.clone(),
                    };
                    let parsed = new_spec.churn(
                        Some(Box::new(me_ref)),
                        reader,
                        XmlEvent::StartElement {
                            name: name.clone(),
                            attributes: attributes.clone(),
                            namespace: namespace.clone(),
                        },
                    )?;
                    ti.inner.push(parsed);
                }
                XmlEvent::Characters(ch) => {
                    ti.value = Some(ch);
                }
                XmlEvent::EndElement { name } => {
                    if name.local_name == self.name {
                        return Ok(ti);
                    }
                }
                XmlEvent::StartDocument { .. }
                | XmlEvent::ProcessingInstruction { .. }
                | XmlEvent::CData(_)
                | XmlEvent::Comment(_)
                | XmlEvent::Whitespace(_) => {}

                XmlEvent::EndDocument => {
                    break;
                }
            }
        }

        Ok(ti)
    }
}

pub fn parse(content: &[u8], mut spec: TagSpec) -> Result<TagItem> {
    let mut reader = EventReader::new(content);
    let first = loop {
        let evt = reader.next()?;
        if matches!(evt, XmlEvent::StartElement { .. }) {
            break evt;
        }
    };
    let ti = spec.churn(None, &mut reader, first)?;
    Ok(ti)
}

#[derive(Debug, Clone)]
pub struct TagItem {
    pub name: String,
    pub attrs: HashMap<String, String>,
    pub parent: Option<Box<TagItemParentRef>>,
    pub inner: Vec<TagItem>,
    pub value: Option<String>,
}

#[derive(Debug, Clone)]
pub struct TagItemParentRef {
    pub name: String,
    pub attrs: HashMap<String, String>,
    pub parent: Option<Box<TagItemParentRef>>,
}

impl TagItem {
    #[must_use]
    pub fn to_string(&self, depth: usize) -> String {
        let indent = "\t".repeat(depth);
        let mut base = format!(
            "{indent}TagItem {{ name: {}, attrs: {:?}, inner_count {}, value: {:?}}}",
            self.name,
            self.attrs,
            self.inner.len(),
            self.value
        );
        for inner in &self.inner {
            let _ = base.write_fmt(format_args!("\n{}", inner.to_string(depth + 1)));
        }
        base
    }

    #[must_use]
    pub fn level(&self) -> usize {
        let mut next_above = &self.parent;
        let mut level = 0;
        while let Some(next) = next_above {
            level += 1;
            next_above = &next.parent;
        }
        level
    }

    pub fn extract_attr(&self, attr: &str) -> Result<String> {
        Ok(self
            .attrs
            .get(attr)
            .ok_or_else(|| {
                Error::msg(format!("Failed to extract attr '{attr}' from tag {self:?}"))
            })?
            .clone())
    }

    pub fn extract_attr_as<T: FromStr>(&self, attr: &str) -> Result<T>
    where
        <T as FromStr>::Err: Send + Sync + 'static + std::error::Error,
    {
        Ok(self
            .attrs
            .get(attr)
            .ok_or_else(|| {
                Error::msg(format!("Failed to extract attr '{attr}' from tag {self:?}"))
            })?
            .parse::<T>()?)
    }

    pub fn extract_value(&self) -> Result<String> {
        Ok(self
            .value
            .as_ref()
            .ok_or_else(|| Error::msg(format!("Failed to extract value from tag {self:?}")))?
            .clone())
    }

    pub fn extract_value_as<T: FromStr>(&self) -> Result<T>
    where
        <T as FromStr>::Err: Send + Sync + 'static + std::error::Error,
    {
        Ok(self
            .value
            .as_ref()
            .ok_or_else(|| Error::msg(format!("Failed to extract value from tag {self:?}")))?
            .parse::<T>()?)
    }

    #[must_use]
    pub fn extract_first_named_parent(&self) -> Option<String> {
        let mut pp = self.parent.as_ref();
        loop {
            if let Some(p) = pp {
                if let Some(name) = p.attrs.get(NAME) {
                    break Some(name.clone());
                }
                pp = p.parent.as_ref();
            } else {
                break None;
            }
        }
    }

    #[must_use]
    pub fn extract_first_unnamed_parent_occurences(&self) -> Option<(usize, String)> {
        let mut pp = self.parent.as_ref();
        loop {
            if let Some(p) = pp {
                if let (Some(min), Some(max)) = (
                    p.attrs
                        .get(MIN_OCCURS)
                        .and_then(|min| min.parse::<usize>().ok()),
                    p.attrs.get(MAX_OCCURS),
                ) {
                    break Some((min, max.clone()));
                }
                pp = p.parent.as_ref();
            } else {
                break None;
            }
        }
    }
}
