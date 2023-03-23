use crate::{Deserializer, ElementArgument, ElementHeader, HashMappable};
use std::collections::HashMap;
use xmltree::Element;

/// `<collectionProp>`
#[derive(Debug, PartialEq)]
pub struct CollectionProp<T> {
    pub props: Vec<T>,
}

impl Deserializer for CollectionProp<ElementHeader> {
    fn parse(e: &Element) -> Self {
        assert_eq!(e.name.as_str(), "collectionProp");
        assert_eq!(
            e.attributes.get("name").unwrap().as_str(),
            "HeaderManager.headers"
        );
        let props = e
            .children
            .iter()
            .map(|x| x.as_element())
            .filter(|x| x.is_some())
            .map(|x| ElementHeader::parse(x.unwrap()))
            .collect::<Vec<ElementHeader>>();
        Self { props }
    }
}

impl HashMappable for CollectionProp<ElementHeader> {
    fn to_hash_map(&self) -> HashMap<String, String> {
        self.props
            .iter()
            .map(|a| (a.name.clone(), a.value.clone()))
            .collect::<HashMap<String, String>>()
    }
}

impl Deserializer for CollectionProp<ElementArgument> {
    fn parse(e: &Element) -> Self {
        assert_eq!(e.name.as_str(), "collectionProp");
        assert_eq!(
            e.attributes.get("name").unwrap().as_str(),
            "Arguments.arguments"
        );
        let props = e
            .children
            .iter()
            .map(|x| x.as_element())
            .filter(|x| x.is_some())
            .map(|x| ElementArgument::parse(x.unwrap()))
            .collect::<Vec<ElementArgument>>();
        Self { props }
    }
}

impl HashMappable for CollectionProp<ElementArgument> {
    fn to_hash_map(&self) -> HashMap<String, String> {
        self.props
            .iter()
            .map(|a| (a.name.clone(), a.value.clone()))
            .collect::<HashMap<String, String>>()
    }
}

#[cfg(test)]
mod test {
    use crate::{CollectionProp, Deserializer};
    use xmltree::Element;

    #[test]
    fn check_parse_1() {
        use crate::ElementArgument;

        let xml = Element::parse(
            std::fs::read_to_string("./unittests_data/collection_prop/collection_prop_1.xml")
                .unwrap()
                .trim()
                .as_bytes(),
        )
        .unwrap();
        assert_eq!(
            CollectionProp::parse(&xml),
            CollectionProp {
                props: vec![
                    ElementArgument {
                        name: String::from("protocol"),
                        value: String::from("https"),
                        metadata: String::from("=")
                    },
                    ElementArgument {
                        name: String::from("ip"),
                        value: String::from("example.github.com"),
                        metadata: String::from("=")
                    }
                ]
            }
        )
    }

    #[test]
    fn check_parse_2() {
        use crate::ElementHeader;

        let xml = Element::parse(
            std::fs::read_to_string("./unittests_data/collection_prop/collection_prop_2.xml")
                .unwrap()
                .trim()
                .as_bytes(),
        )
        .unwrap();
        assert_eq!(
            CollectionProp::parse(&xml),
            CollectionProp {
                props: vec![
                    ElementHeader {
                        name: String::from("Content-Type"),
                        value: String::from("application/json")
                    },
                    ElementHeader {
                        name: String::from("accept-encoding"),
                        value: String::from("gzip")
                    }
                ]
            }
        )
    }
}
