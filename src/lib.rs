mod basic_prop;
mod config_element;
mod test_plan;

pub use basic_prop::*;
pub use config_element::*;
pub use test_plan::*;
use xmltree::Element;

#[derive(Debug)]
pub enum TestClass {
    TestPlan(TestPlan, Vec<TestClass>),
    Unknown,
}

impl TestClass {
    pub fn parse(ele: &Element, hash_tree: &Element) -> Self {
        assert_eq!(hash_tree.name, String::from("hashTree"));
        assert_eq!(hash_tree.children.len() % 2, 0);
        let mut subs: Vec<TestClass> = vec![];
        for i in (0..hash_tree.children.len()).step_by(2) {
            let ele = hash_tree.children[i].as_element().unwrap();
            let sub = hash_tree.children[i + 1].as_element().unwrap();
            subs.push(TestClass::parse(ele, sub))
        }
        match ele.attributes.get("testclass").unwrap().as_str() {
            "TestPlan" => TestClass::TestPlan(TestPlan::parse(ele), subs),
            _ => TestClass::Unknown,
        }
    }
}
