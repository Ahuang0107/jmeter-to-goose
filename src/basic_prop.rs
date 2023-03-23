use crate::Deserializer;
use xmltree::Element;

/// `<stringProp>`
#[derive(Debug, PartialEq)]
pub struct StringProp {
    pub name: String,
    pub value: String,
}

impl Deserializer for StringProp {
    fn parse(e: &Element) -> Self {
        assert_eq!(e.name.as_str(), "stringProp");
        let name = e.attributes.get("name").unwrap().clone();
        let value = if let Some(child) = e.children.first() {
            child.as_text().unwrap()
        } else {
            ""
        }
        .to_string();
        Self { name, value }
    }
}

/// `<boolProp>`
#[derive(Debug, PartialEq)]
pub struct BoolProp {
    pub name: String,
    pub value: bool,
}

impl Deserializer for BoolProp {
    fn parse(e: &Element) -> Self {
        assert_eq!(e.name.as_str(), "boolProp");
        let name = e.attributes.get("name").unwrap().clone();
        let value = e
            .children
            .first()
            .unwrap()
            .as_text()
            .unwrap()
            .parse::<bool>()
            .unwrap();
        Self { name, value }
    }
}

#[cfg(test)]
mod test {
    use crate::Deserializer;
    use xmltree::Element;

    #[test]
    fn string_prop_check_parse() {
        use crate::StringProp;

        let xml = Element::parse(
            std::fs::read_to_string("./unittests_data/basic_prop/string_prop.xml")
                .unwrap()
                .trim()
                .as_bytes(),
        )
        .unwrap();
        assert_eq!(
            StringProp::parse(&xml),
            StringProp {
                name: String::from("prop name"),
                value: String::from("string prop value")
            }
        )
    }

    #[test]
    fn bool_prop_check_parse() {
        use crate::BoolProp;

        let xml = Element::parse(
            std::fs::read_to_string("./unittests_data/basic_prop/bool_prop.xml")
                .unwrap()
                .trim()
                .as_bytes(),
        )
        .unwrap();
        assert_eq!(
            BoolProp::parse(&xml),
            BoolProp {
                name: String::from("prop name"),
                value: true
            }
        )
    }
}
