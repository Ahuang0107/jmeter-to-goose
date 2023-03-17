use jmeter_to_goose::TestClass;
use std::fs::read_to_string;
use xmltree::{Element, XMLNode};

fn main() -> anyhow::Result<()> {
    let xml = read_to_string("test.xml")?;
    let doc = Element::parse(xml.as_bytes()).unwrap();

    assert_eq!(doc.children.len(), 1);

    if let Some(x) = doc.children.first() {
        match x {
            XMLNode::Element(data) => {
                assert_eq!(data.children.len(), 2);
                let element = data.children[0].as_element().unwrap();
                let sub = data.children[1].as_element().unwrap();
                println!("{:?}", TestClass::parse(element, sub));
            }
            _ => {}
        }
    }

    Ok(())
}
