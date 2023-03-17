use std::collections::HashMap;
use std::fmt::Debug;
use std::str::FromStr;
use xmltree::Element;

pub enum BasicProp {
    String(TypeProp<String>),
    Bool(TypeProp<bool>),
    Int(TypeProp<usize>),
    Long(TypeProp<i64>),
}

/// stringProp | boolProp | intProp
///
/// # Examples
///
/// ```
/// use xmltree::Element;
/// use jmeter_to_goose::{parse, TypeProp};
///
/// let xml =Element::parse(r#"
/// <stringProp name="Header.name">X-REQUEST-ID</stringProp>
/// "#.trim().as_bytes()).unwrap();
/// assert_eq!(
///     parse::<String>(&xml).unwrap(),
///     TypeProp(String::from("Header.name"), String::from("X-REQUEST-ID"))
/// )
/// ```
/// ```
/// use xmltree::Element;
/// use jmeter_to_goose::{parse, TypeProp};
///
/// let xml =Element::parse(r#"
/// <boolProp name="TestPlan.functional_mode">false</boolProp>
/// "#.trim().as_bytes()).unwrap();
/// assert_eq!(
///     parse::<bool>(&xml).unwrap(),
///     TypeProp(String::from("TestPlan.functional_mode"), false)
/// )
/// ```
/// ```
/// use xmltree::Element;
/// use jmeter_to_goose::{parse, TypeProp};
///
/// let xml =Element::parse(r#"
/// <intProp name="Assertion.test_type">2</intProp>
/// "#.trim().as_bytes()).unwrap();
/// assert_eq!(
///     parse::<usize>(&xml).unwrap(),
///     TypeProp(String::from("Assertion.test_type"), 2)
/// )
/// ```
#[derive(Debug, PartialEq)]
pub struct TypeProp<T>(pub String, pub T);

pub fn parse<F: FromStr + Debug>(ele: &Element) -> Result<TypeProp<F>, F::Err> {
    let name = ele.attributes.get("name").unwrap().clone();
    let value = ele
        .children
        .first()
        .unwrap()
        .as_text()
        .unwrap()
        .parse::<F>()?;
    Ok(TypeProp(name, value))
}

/// elementProp elementType=Argument
///
/// # Examples
///
/// ```
/// use xmltree::Element;
/// use jmeter_to_goose::Argument;
///
/// let xml =Element::parse(r#"
/// <elementProp name="X-REQUEST-ID" elementType="Header">
///   <stringProp name="Header.name">X-REQUEST-ID</stringProp>
///   <stringProp name="Header.value">2O742E18Q9FQGDEL</stringProp>
/// </elementProp>
/// "#.trim().as_bytes()).unwrap();
/// assert_eq!(
///     Argument::parse(&xml),
///     Argument(
///         String::from("X-REQUEST-ID"),
///         String::from("2O742E18Q9FQGDEL")
///     )
/// )
/// ```
#[derive(Debug, PartialEq)]
pub struct Argument(pub String, pub String);

impl Argument {
    pub fn parse(ele: &Element) -> Self {
        let name = ele.attributes.get("name").unwrap().clone();
        let value = ele.children[1].as_element().unwrap().children[0]
            .as_text()
            .unwrap();
        Self(name, String::from(value))
    }
}

/// collectionProp
///
/// # Examples
///
/// ```
/// use xmltree::Element;
/// use jmeter_to_goose::{Argument, Collection};
///
/// let xml =Element::parse(r#"
/// <collectionProp name="HeaderManager.headers">
///   <elementProp name="X-REQUEST-ID" elementType="Header">
///     <stringProp name="Header.name">X-REQUEST-ID</stringProp>
///     <stringProp name="Header.value">2O742E18Q9FQGDEL</stringProp>
///   </elementProp>
///   <elementProp name="X-SIGNATURE" elementType="Header">
///     <stringProp name="Header.name">X-SIGNATURE</stringProp>
///     <stringProp name="Header.value">8b7ce095925bbba59cb55ee1fdbf8656</stringProp>
///   </elementProp>
///   <elementProp name="X-TIMESTAMP" elementType="Header">
///     <stringProp name="Header.name">X-TIMESTAMP</stringProp>
///     <stringProp name="Header.value">1677817619025</stringProp>
///   </elementProp>
/// </collectionProp>
/// "#.trim().as_bytes()).unwrap();
/// assert_eq!(
///     Collection::parse(&xml),
///     Collection(vec!(
///         Argument(
///             String::from("X-REQUEST-ID"),
///             String::from("2O742E18Q9FQGDEL")
///         ),
///         Argument(
///             String::from("X-SIGNATURE"),
///             String::from("8b7ce095925bbba59cb55ee1fdbf8656")
///         ),
///         Argument(String::from("X-TIMESTAMP"), String::from("1677817619025"))
///     ))
/// )
/// ```
#[derive(Debug, PartialEq)]
pub struct Collection(pub Vec<Argument>);

impl Collection {
    pub fn parse(ele: &Element) -> Self {
        Self(
            ele.children
                .iter()
                .map(|c| Argument::parse(c.as_element().unwrap()))
                .collect::<Vec<Argument>>(),
        )
    }
    pub fn to_hash_map(self) -> HashMap<String, String> {
        self.0
            .iter()
            .map(|a| (a.0.clone(), a.1.clone()))
            .collect::<HashMap<String, String>>()
    }
}
