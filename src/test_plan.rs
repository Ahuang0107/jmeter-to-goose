use crate::{CollectionProp, Deserializer, ElementArgument, HashMappable};
use std::collections::HashMap;
use xmltree::Element;

/// TestPlan
#[derive(Debug, PartialEq)]
pub struct TestPlan {
    pub test_name: String,
    pub enabled: bool,
    pub variables: HashMap<String, String>,
}

impl TestPlan {
    pub fn parse(e: &Element) -> Self {
        assert_eq!(e.name.as_str(), "TestPlan");
        let test_name = e.attributes.get("testname").unwrap().clone();
        let enabled = e
            .attributes
            .get("enabled")
            .unwrap()
            .parse::<bool>()
            .unwrap();
        let variables = CollectionProp::<ElementArgument>::parse(
            e.children
                .iter()
                .find(|c| c.as_element().unwrap().name.as_str() == "elementProp")
                .unwrap()
                .as_element()
                .unwrap()
                .children[0]
                .as_element()
                .unwrap(),
        )
        .to_hash_map();
        Self {
            test_name,
            enabled,
            variables,
        }
    }
}

#[cfg(test)]
mod test {

    #[test]
    fn check_parse() {
        use crate::TestPlan;
        use std::collections::HashMap;
        use xmltree::Element;

        let xml = Element::parse(
            std::fs::read_to_string("./unittests_data/test_plan/test_plan.xml")
                .unwrap()
                .trim()
                .as_bytes(),
        )
        .unwrap();
        assert_eq!(
            TestPlan::parse(&xml),
            TestPlan {
                test_name: String::from("Test Plan"),
                enabled: true,
                variables: HashMap::from([
                    (String::from("protocol"), String::from("https")),
                    (String::from("ip"), String::from("example.github.com"))
                ])
            }
        )
    }
}
