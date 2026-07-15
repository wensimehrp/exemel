use std::io::Cursor;

use quick_xml::events::{BytesEnd, BytesStart, BytesText, Event};
use quick_xml::writer::Writer;
use serde::Deserialize;
use wasm_minimal_protocol::*;

initiate_protocol!();

#[derive(Deserialize)]
struct Input<'a> {
    #[serde(borrow)]
    root: XmlValue<'a>,
    pretty: bool,
}

#[derive(Deserialize)]
#[serde(untagged)]
enum XmlValue<'a> {
    Element {
        #[serde(default)]
        namespace: Option<&'a str>,
        tag: &'a str,
        #[serde(borrow, default)]
        attrs: std::collections::BTreeMap<&'a str, &'a str>,
        #[serde(borrow, default)]
        children: Box<[XmlValue<'a>]>,
    },
    Text(&'a str),
}

impl<'a> XmlValue<'a> {
    fn qualified_tag(&self) -> Option<String> {
        match self {
            XmlValue::Element { namespace, tag, .. } => Some(match namespace {
                Some(ns) => format!("{}:{}", ns, tag),
                None => tag.to_string(),
            }),
            XmlValue::Text(_) => None,
        }
    }
}

fn write_node<W: std::io::Write>(writer: &mut Writer<W>, node: &XmlValue) -> quick_xml::Result<()> {
    match node {
        XmlValue::Text(s) => {
            writer.write_event(Event::Text(BytesText::new(s)))?;
        }
        XmlValue::Element {
            attrs, children, ..
        } => {
            let name = node.qualified_tag().unwrap();
            let mut start = BytesStart::new(name.as_str());
            for (k, v) in attrs.iter() {
                start.push_attribute((*k, *v));
            }

            if children.is_empty() {
                writer.write_event(Event::Empty(start))?;
            } else {
                writer.write_event(Event::Start(start))?;
                for child in children.iter() {
                    write_node(writer, child)?;
                }
                writer.write_event(Event::End(BytesEnd::new(name.as_str())))?;
            }
        }
    }
    Ok(())
}

fn xml_from_cbor(cbor: &[u8]) -> Result<String, String> {
    let input: Input =
        cbor4ii::serde::from_slice(cbor).map_err(|e| format!("failed to decode input: {e}"))?;

    let buf = Cursor::new(Vec::new());
    let mut writer = if input.pretty {
        Writer::new_with_indent(buf, b' ', 2)
    } else {
        Writer::new(buf)
    };

    write_node(&mut writer, &input.root).map_err(|e| format!("failed to write xml: {e}"))?;

    let bytes = writer.into_inner().into_inner();
    String::from_utf8(bytes).map_err(|e| format!("invalid utf-8 produced: {e}"))
}

#[wasm_func]
pub fn to_xml(cbor: &[u8]) -> Result<Vec<u8>, String> {
    xml_from_cbor(cbor).map(|s| s.into_bytes())
}
