use jmeter_to_goose::TestClass;
use std::fs::read_to_string;
use xmltree::Element;

fn main() -> anyhow::Result<()> {
    let xml = read_to_string("test.xml")?;
    let doc = Element::parse(xml.as_bytes()).unwrap();

    let root = TestClass::root(&doc);
    println!("{:#?}", root);

    Ok(())
}
