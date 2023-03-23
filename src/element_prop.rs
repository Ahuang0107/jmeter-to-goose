use crate::{Deserializer, Pairable, StringProp};
use xmltree::Element;

/// `<elementProp elementType="Header">`
#[derive(Debug, PartialEq)]
pub struct ElementHeader {
    pub name: String,
    pub value: String,
}

impl Pairable for ElementHeader {
    fn to_pair(&self) -> (&str, &str) {
        (self.name.as_str(), self.value.as_str())
    }
}

impl Deserializer for ElementHeader {
    fn parse(e: &Element) -> Self {
        assert_eq!(e.name.as_str(), "elementProp");
        assert_eq!(e.attributes.get("elementType").unwrap().as_str(), "Header");
        assert_eq!(e.children.len(), 2);
        let props = e
            .children
            .iter()
            .map(|x| x.as_element())
            .filter(|x| x.is_some())
            .map(|x| StringProp::parse(x.unwrap()))
            .collect::<Vec<StringProp>>();
        let name = props
            .iter()
            .find(|p| p.name == "Header.name")
            .unwrap()
            .value
            .clone();
        let value = props
            .iter()
            .find(|p| p.name == "Header.value")
            .unwrap()
            .value
            .clone();
        Self { name, value }
    }
}

/// `<elementProp elementType="Argument">`
#[derive(Debug, PartialEq)]
pub struct ElementArgument {
    pub name: String,
    pub value: String,
    pub metadata: String,
}

impl Pairable for ElementArgument {
    fn to_pair(&self) -> (&str, &str) {
        (self.name.as_str(), self.value.as_str())
    }
}

impl Deserializer for ElementArgument {
    fn parse(e: &Element) -> Self {
        assert_eq!(e.name.as_str(), "elementProp");
        assert_eq!(
            e.attributes.get("elementType").unwrap().as_str(),
            "Argument"
        );
        assert_eq!(e.children.len(), 3);
        let props = e
            .children
            .iter()
            .map(|x| x.as_element())
            .filter(|x| x.is_some())
            .map(|x| StringProp::parse(x.unwrap()))
            .collect::<Vec<StringProp>>();
        let name = props
            .iter()
            .find(|p| p.name == "Argument.name")
            .unwrap()
            .value
            .clone();
        let value = props
            .iter()
            .find(|p| p.name == "Argument.value")
            .unwrap()
            .value
            .clone();
        let metadata = props
            .iter()
            .find(|p| p.name == "Argument.metadata")
            .unwrap()
            .value
            .clone();
        Self {
            name,
            value,
            metadata,
        }
    }
}

/// `<elementProp elementType="LoopController">`
#[derive(Debug, PartialEq)]
pub struct ElementLoopController {
    pub loops: i32,
}

impl Deserializer for ElementLoopController {
    fn parse(e: &Element) -> Self {
        assert_eq!(e.name.as_str(), "elementProp");
        assert_eq!(
            e.attributes.get("elementType").unwrap().as_str(),
            "LoopController"
        );
        let children = e
            .children
            .iter()
            .map(|c| c.as_element())
            .filter(|c| c.is_some())
            .map(|c| c.unwrap())
            .collect::<Vec<&Element>>();
        let loops = children
            .iter()
            .find(|c| c.attributes.get("name").unwrap() == "LoopController.loops")
            .unwrap()
            .children
            .first()
            .unwrap()
            .as_text()
            .unwrap()
            .parse::<i32>()
            .unwrap();
        Self { loops }
    }
}

#[cfg(test)]
mod test {
    use crate::Deserializer;
    use xmltree::Element;

    #[test]
    fn check_parse_element_header() {
        use crate::ElementHeader;

        let xml = Element::parse(
            std::fs::read_to_string("./unittests_data/element_prop/element_header.xml")
                .unwrap()
                .trim()
                .as_bytes(),
        )
        .unwrap();
        assert_eq!(
            ElementHeader::parse(&xml),
            ElementHeader {
                name: String::from("header name"),
                value: String::from("header value")
            }
        )
    }

    #[test]
    fn check_parse_element_argument() {
        use crate::ElementArgument;

        let xml = Element::parse(
            std::fs::read_to_string("./unittests_data/element_prop/element_argument.xml")
                .unwrap()
                .trim()
                .as_bytes(),
        )
        .unwrap();
        assert_eq!(
            ElementArgument::parse(&xml),
            ElementArgument {
                name: String::from("protocol"),
                value: String::from("https"),
                metadata: String::from("="),
            }
        )
    }

    #[test]
    fn check_parse_element_loop_controller() {
        use crate::ElementLoopController;

        let xml = Element::parse(
            std::fs::read_to_string("./unittests_data/element_prop/element_loop_controller.xml")
                .unwrap()
                .trim()
                .as_bytes(),
        )
        .unwrap();
        assert_eq!(
            ElementLoopController::parse(&xml),
            ElementLoopController { loops: 1 }
        )
    }
}
