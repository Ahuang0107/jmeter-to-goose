mod basic_prop;
mod collection_prop;
mod config_element;
mod element_prop;
mod prop;
mod test_plan;
mod thread;

use crate::thread::ThreadGroup;
pub use basic_prop::*;
pub use collection_prop::*;
pub use config_element::*;
pub use element_prop::*;
use std::collections::HashMap;
pub use test_plan::*;
use xmltree::Element;

pub trait Deserializer {
    fn parse(e: &Element) -> Self;
}

pub trait Pairable {
    fn to_pair(&self) -> (&str, &str);
}

pub trait HashMappable {
    fn to_hash_map(&self) -> HashMap<String, String>;
}

/// TestClass
///
/// # Examples
///
/// ```
/// use std::fs::read_to_string;
/// use xmltree::Element;
/// use jmeter_to_goose::TestClass;
///
/// # fn main() -> std::io::Result<()> {
/// let xml = read_to_string("test.xml")?;
/// let doc = Element::parse(xml.as_bytes()).unwrap();
///
/// let root = TestClass::root(&doc);
/// println!("{:?}", root);
/// #
/// #     Ok(())
/// # }
/// ```
#[derive(Debug)]
pub enum TestClass {
    TestPlan(TestPlan, Vec<TestClass>),
    ThreadGroup(ThreadGroup, Vec<TestClass>),
    HeaderManager(HeaderManager, Vec<TestClass>),
    Unknown,
}

impl TestClass {
    pub fn root(ele: &Element) -> Self {
        assert_eq!(ele.children.len(), 1);

        let ele = ele.children.first().unwrap().as_element().unwrap();
        assert_eq!(ele.children.len(), 2);

        let element = ele.children[0].as_element().unwrap();
        let sub = ele.children[1].as_element().unwrap();
        Self::parse(element, sub)
    }
    pub fn parse(e: &Element, hash_tree: &Element) -> Self {
        assert_eq!(hash_tree.name, String::from("hashTree"));
        assert_eq!(hash_tree.children.len() % 2, 0);
        let mut subs: Vec<TestClass> = vec![];
        for i in (0..hash_tree.children.len()).step_by(2) {
            let ele = hash_tree.children[i].as_element().unwrap();
            let sub = hash_tree.children[i + 1].as_element().unwrap();
            subs.push(TestClass::parse(ele, sub))
        }
        match e.name.as_str() {
            "TestPlan" => TestClass::TestPlan(TestPlan::parse(e), subs),
            "ThreadGroup" => TestClass::ThreadGroup(ThreadGroup::parse(e), subs),
            "HeaderManager" => TestClass::HeaderManager(HeaderManager::parse(e), subs),
            _ => TestClass::Unknown,
        }
    }
}
