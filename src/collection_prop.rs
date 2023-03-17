use crate::{Deserializer, ElementArgument, ElementHeader, HashMappable};
use std::collections::HashMap;
use xmltree::Element;

/// `<collectionProp>`
///
/// # Examples
///
/// ```
/// use xmltree::Element;
/// use jmeter_to_goose::{Deserializer, ElementArgument, CollectionProp};
///
/// let xml = Element::parse(r#"
/// <collectionProp name="Arguments.arguments">
///     <elementProp name="protocol" elementType="Argument">
///         <stringProp name="Argument.name">protocol</stringProp>
///         <stringProp name="Argument.value">https</stringProp>
///         <stringProp name="Argument.metadata">=</stringProp>
///     </elementProp>
///     <elementProp name="ip" elementType="Argument">
///         <stringProp name="Argument.name">ip</stringProp>
///         <stringProp name="Argument.value">example.github.com</stringProp>
///         <stringProp name="Argument.metadata">=</stringProp>
///     </elementProp>
/// </collectionProp>
/// "#.trim().as_bytes()).unwrap();
/// assert_eq!(
///     CollectionProp::parse(&xml),
///     CollectionProp {
///         props: vec![
///             ElementArgument{
///                 name : String::from("protocol"),
///                 value : String::from("https"),
///                 metadata : String::from("=")
///             },
///             ElementArgument{
///                 name : String::from("ip"),
///                 value : String::from("example.github.com"),
///                 metadata : String::from("=")
///             }
///         ]
///     }
/// )
/// ```
///
/// ```
/// use xmltree::Element;
/// use jmeter_to_goose::{Deserializer, CollectionProp, ElementHeader};
///
/// let xml = Element::parse(r#"
/// <collectionProp name="HeaderManager.headers">
///     <elementProp name="Content-Type" elementType="Header">
///         <stringProp name="Header.name">Content-Type</stringProp>
///         <stringProp name="Header.value">application/json</stringProp>
///     </elementProp>
///     <elementProp name="accept-encoding" elementType="Header">
///         <stringProp name="Header.name">accept-encoding</stringProp>
///         <stringProp name="Header.value">gzip</stringProp>
///     </elementProp>
/// </collectionProp>
/// "#.trim().as_bytes()).unwrap();
/// assert_eq!(
///     CollectionProp::parse(&xml),
///     CollectionProp {
///         props: vec![
///             ElementHeader{
///                 name : String::from("Content-Type"),
///                 value : String::from("application/json")
///             },
///             ElementHeader{
///                 name : String::from("accept-encoding"),
///                 value : String::from("gzip")
///             }
///         ]
///     }
/// )
/// ```
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
