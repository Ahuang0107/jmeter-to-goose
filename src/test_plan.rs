use crate::{CollectionProp, Deserializer, ElementArgument, HashMappable};
use std::collections::HashMap;
use xmltree::Element;

/// TestPlan
///
/// # Examples
///
/// ```
/// use std::collections::HashMap;
/// use xmltree::Element;
/// use jmeter_to_goose::TestPlan;
///
/// let xml = Element::parse(r#"
/// <TestPlan guiclass="TestPlanGui" testclass="TestPlan" testname="Test Plan" enabled="true">
///     <stringProp name="TestPlan.comments"></stringProp>
///     <boolProp name="TestPlan.functional_mode">false</boolProp>
///     <boolProp name="TestPlan.tearDown_on_shutdown">true</boolProp>
///     <boolProp name="TestPlan.serialize_threadgroups">false</boolProp>
///     <elementProp name="TestPlan.user_defined_variables" elementType="Arguments" guiclass="ArgumentsPanel" testclass="Arguments" testname="User Defined Variables" enabled="true">
///         <collectionProp name="Arguments.arguments">
///             <elementProp name="protocol" elementType="Argument">
///                 <stringProp name="Argument.name">protocol</stringProp>
///                 <stringProp name="Argument.value">https</stringProp>
///                 <stringProp name="Argument.metadata">=</stringProp>
///             </elementProp>
///             <elementProp name="ip" elementType="Argument">
///                 <stringProp name="Argument.name">ip</stringProp>
///                 <stringProp name="Argument.value">example.github.com</stringProp>
///                 <stringProp name="Argument.metadata">=</stringProp>
///             </elementProp>
///         </collectionProp>
///     </elementProp>
///     <stringProp name="TestPlan.user_define_classpath"></stringProp>
/// </TestPlan>
/// "#.trim().as_bytes(),
/// )
/// .unwrap();
/// assert_eq!(
///     TestPlan::parse(&xml),
///     TestPlan {
///         test_name: String::from("Test Plan"),
///         enabled: true,
///         variables: HashMap::from([
///             (
///                 String::from("protocol"),
///                 String::from("https")
///             ),
///             (
///                 String::from("ip"),
///                 String::from("example.github.com")
///             )
///         ])
///     }
/// )
/// ```
#[derive(Debug, PartialEq)]
pub struct TestPlan {
    pub test_name: String,
    pub enabled: bool,
    pub variables: HashMap<String, String>,
}

impl TestPlan {
    pub fn parse(e: &Element) -> Self {
        assert_eq!(e.name.as_str(), "TestPlan");
        let test_name = e.attributes.get("testname").unwrap().clone();
        let enabled = e
            .attributes
            .get("enabled")
            .unwrap()
            .parse::<bool>()
            .unwrap();
        let variables = CollectionProp::<ElementArgument>::parse(
            e.children
                .iter()
                .find(|c| c.as_element().unwrap().name.as_str() == "elementProp")
                .unwrap()
                .as_element()
                .unwrap()
                .children[0]
                .as_element()
                .unwrap(),
        )
        .to_hash_map();
        Self {
            test_name,
            enabled,
            variables,
        }
    }
}
