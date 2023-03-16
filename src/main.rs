use std::fs::read_to_string;
use xmltree::{Element, XMLNode};

mod basic_prop;

fn main() -> anyhow::Result<()> {
    let xml = read_to_string("test.xml")?;
    let doc = Element::parse(xml.as_bytes()).unwrap();

    assert_eq!(doc.children.len(), 1);

    if let Some(x) = doc.children.first() {
        match x {
            XMLNode::Element(data) => {
                assert_eq!(data.children.len(), 2);
                let element = data.children[0].as_element().unwrap();
                let children = data.children[1]
                    .as_element()
                    .unwrap()
                    .children
                    .iter()
                    .map(|c| c.as_element().unwrap())
                    .collect::<Vec<&Element>>();
                assert_eq!(&element.name, "TestPlan");
                assert!(children.len() > 1);
            }
            _ => {}
        }
    }

    Ok(())
}
