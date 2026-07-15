#let _plugin = plugin("./target/wasm32-unknown-unknown/release/xml_plugin.wasm")

#let xmls = xml(bytes(xmlt.text))
#xmls
#let s = _plugin.to_xml(cbor.encode((
  root: xmls.at(0),
  pretty: true,
)))

#raw(str(s), block: true, lang: "xml")
