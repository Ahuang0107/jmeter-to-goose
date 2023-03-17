use crate::{CollectionProp, Deserializer, ElementHeader, HashMappable};
use std::collections::HashMap;
use xmltree::Element;

/// HTTP Header Manager
///
/// # Examples
///
/// ```
/// use std::collections::HashMap;
/// use xmltree::Element;
/// use jmeter_to_goose::HeaderManager;
///
/// let xml = Element::parse(r#"
/// <HeaderManager guiclass="HeaderPanel" testclass="HeaderManager" testname="HTTP Header Manager" enabled="true">
///     <collectionProp name="HeaderManager.headers">
///         <elementProp name="Content-Type" elementType="Header">
///             <stringProp name="Header.name">Content-Type</stringProp>
///             <stringProp name="Header.value">application/json</stringProp>
///         </elementProp>
///         <elementProp name="accept-encoding" elementType="Header">
///             <stringProp name="Header.name">accept-encoding</stringProp>
///             <stringProp name="Header.value">gzip</stringProp>
///         </elementProp>
///     </collectionProp>
/// </HeaderManager>
/// "#.trim().as_bytes(),
/// )
/// .unwrap();
/// let result = HeaderManager::parse(&xml);
/// assert_eq!(
///     result,
///     HeaderManager {
///         test_name: String::from("HTTP Header Manager"),
///         enabled: true,
///         headers: HashMap::from([
///             (
///                 String::from("Content-Type"),
///                 String::from("application/json")
///             ),
///             (
///                 String::from("accept-encoding"),
///                 String::from("gzip")
///             )
///         ])
///     }
/// )
/// ```
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
