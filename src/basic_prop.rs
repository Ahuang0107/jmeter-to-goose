use crate::Deserializer;
use xmltree::Element;

/// `<stringProp>`
///
/// # Examples
///
/// ```
/// use xmltree::Element;
/// use jmeter_to_goose::{Deserializer, StringProp};
///
/// let xml = Element::parse(r#"
/// <stringProp name="prop name">string prop value</stringProp>
/// "#.trim().as_bytes()).unwrap();
/// assert_eq!(
///     StringProp::parse(&xml),
///     StringProp {
///         name: String::from("prop name"),
///         value: String::from("string prop value")
///     }
/// )
/// ```
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
///
/// # Examples
///
/// ```
/// use xmltree::Element;
/// use jmeter_to_goose::{BoolProp, Deserializer};
///
/// let xml = Element::parse(r#"
/// <boolProp name="prop name">true</boolProp>
/// "#.trim().as_bytes()).unwrap();
/// assert_eq!(
///     BoolProp::parse(&xml),
///     BoolProp {
///         name: String::from("prop name"),
///         value: true
///     }
/// )
/// ```
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
