use crate::{Deserializer, Pairable, StringProp};
use xmltree::Element;

/// `<elementProp elementType="Header">`
///
/// # Examples
///
/// ```
/// use xmltree::Element;
/// use jmeter_to_goose::{ElementHeader, Deserializer};
///
/// let xml = Element::parse(r#"
/// <elementProp name="header name" elementType="Header">
///     <stringProp name="Header.name">header name</stringProp>
///     <stringProp name="Header.value">header value</stringProp>
/// </elementProp>
/// "#.trim().as_bytes()).unwrap();
/// assert_eq!(
///     ElementHeader::parse(&xml),
///     ElementHeader {
///         name: String::from("header name"),
///         value: String::from("header value")
///     }
/// )
/// ```
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
///
/// # Examples
///
/// ```
/// use xmltree::Element;
/// use jmeter_to_goose::{ElementArgument, Deserializer};
///
/// let xml = Element::parse(r#"
/// <elementProp name="protocol" elementType="Argument">
///     <stringProp name="Argument.name">protocol</stringProp>
///     <stringProp name="Argument.value">https</stringProp>
///     <stringProp name="Argument.metadata">=</stringProp>
/// </elementProp>
/// "#.trim().as_bytes()).unwrap();
/// assert_eq!(
///     ElementArgument::parse(&xml),
///     ElementArgument {
///         name: String::from("protocol"),
///         value: String::from("https"),
///         metadata: String::from("="),
///     }
/// )
/// ```
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
///
/// # Examples
///
/// ```
/// use xmltree::Element;
/// use jmeter_to_goose::{Deserializer, ElementLoopController};
///
/// let xml = Element::parse(r#"
/// <elementProp name="ThreadGroup.main_controller" elementType="LoopController" guiclass="LoopControlPanel" testclass="LoopController" testname="Loop Controller" enabled="true">
///     <boolProp name="LoopController.continue_forever">false</boolProp>
///     <stringProp name="LoopController.loops">1</stringProp>
/// </elementProp>
/// "#.trim().as_bytes()).unwrap();
/// assert_eq!(
///     ElementLoopController::parse(&xml),
///     ElementLoopController {
///         loops: 1
///     }
/// )
/// ```
#[derive(Debug, PartialEq)]
pub struct ElementLoopController {
    // pub continue_forever: bool,
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
