use crate::basic_prop::Collection;
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
/// let xml = Element::parse(
/// r#"
///       <HeaderManager guiclass="HeaderPanel" testclass="HeaderManager" testname="HTTP信息头管理器" enabled="true">
///         <collectionProp name="HeaderManager.headers">
///           <elementProp name="X-REQUEST-ID" elementType="Header">
///             <stringProp name="Header.name">X-REQUEST-ID</stringProp>
///             <stringProp name="Header.value">2O742E18Q9FQGDEL</stringProp>
///           </elementProp>
///           <elementProp name="X-SIGNATURE" elementType="Header">
///             <stringProp name="Header.name">X-SIGNATURE</stringProp>
///             <stringProp name="Header.value">8b7ce095925bbba59cb55ee1fdbf8656</stringProp>
///           </elementProp>
///           <elementProp name="X-TIMESTAMP" elementType="Header">
///             <stringProp name="Header.name">X-TIMESTAMP</stringProp>
///             <stringProp name="Header.value">1677817619025</stringProp>
///           </elementProp>
///         </collectionProp>
///       </HeaderManager>
///         "#.trim().as_bytes(),
/// )
/// .unwrap();
/// let result = HeaderManager::parse(&xml);
/// assert_eq!(
///     result,
///     HeaderManager {
///         test_name: String::from("HTTP信息头管理器"),
///         enabled: true,
///         headers: HashMap::from([
///             (
///                 String::from("X-REQUEST-ID"),
///                 String::from("2O742E18Q9FQGDEL")
///             ),
///             (
///                 String::from("X-SIGNATURE"),
///                 String::from("8b7ce095925bbba59cb55ee1fdbf8656")
///             ),
///             (String::from("X-TIMESTAMP"), String::from("1677817619025"))
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
    pub fn parse(ele: &Element) -> Self {
        let test_name = ele.attributes.get("testname").unwrap().clone();
        let enabled = ele
            .attributes
            .get("enabled")
            .unwrap()
            .parse::<bool>()
            .unwrap();
        let headers = Collection::parse(ele.children[0].as_element().unwrap()).to_hash_map();
        Self {
            test_name,
            enabled,
            headers,
        }
    }
}
