#[derive(Debug, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum ElementType {
    Header,
    Argument,
    Arguments,
    LoopController,
}

/// `<elementProp>`
#[derive(Debug, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ElementProp {
    #[serde(rename = "@name")]
    pub name: String,
    #[serde(rename = "@elementType")]
    pub element_type: ElementType,
}

#[cfg(test)]
mod test {
    use crate::prop::{ElementProp, ElementType};

    #[test]
    fn check_deserialize() {
        assert_eq!(
            quick_xml::de::from_str::<ElementProp>(
                &std::fs::read_to_string("./unittests_data/element_prop/element_argument.xml")
                    .unwrap(),
            )
            .unwrap(),
            ElementProp {
                name: String::from("protocol"),
                element_type: ElementType::Argument
            }
        )
    }
}
