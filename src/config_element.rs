use crate::{CollectionProp, Deserializer, ElementHeader, HashMappable};
use std::collections::HashMap;
use xmltree::Element;

/// HTTP Header Manager
/// `<HeaderManager>`
#[derive(Debug, PartialEq)]
pub struct HeaderManager {
    pub test_name: String,
    pub enabled: bool,
    pub headers: HashMap<String, String>,
}

impl HeaderManager {
    pub fn parse(e: &Element) -> Self {
        assert_eq!(e.name.as_str(), "HeaderManager");
        let test_name = e.attributes.get("testname").unwrap().clone();
        let enabled = e
            .attributes
            .get("enabled")
            .unwrap()
            .parse::<bool>()
            .unwrap();
        let headers = CollectionProp::<ElementHeader>::parse(e.children[0].as_element().unwrap())
            .to_hash_map();
        Self {
            test_name,
            enabled,
            headers,
        }
    }
}

#[cfg(test)]
mod test {
    use std::collections::HashMap;
    use xmltree::Element;

    #[test]
    fn check_parse() {
        use crate::HeaderManager;

        let xml = Element::parse(
            std::fs::read_to_string("./unittests_data/config_element/header_manager.xml")
                .unwrap()
                .trim()
                .as_bytes(),
        )
        .unwrap();
        assert_eq!(
            HeaderManager::parse(&xml),
            HeaderManager {
                test_name: String::from("HTTP Header Manager"),
                enabled: true,
                headers: HashMap::from([
                    (
                        String::from("Content-Type"),
                        String::from("application/json")
                    ),
                    (String::from("accept-encoding"), String::from("gzip"))
                ])
            }
        )
    }
}
