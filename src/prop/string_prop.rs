/// `<stringProp>`
#[derive(Debug, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct StringProp {
    #[serde(rename = "@name")]
    pub name: String,
    #[serde(rename = "$text")]
    pub value: String,
}

#[cfg(test)]
mod test {
    use crate::prop::StringProp;

    #[test]
    fn check_deserialize() {
        assert_eq!(
            quick_xml::de::from_str::<StringProp>(
                &std::fs::read_to_string("./unittests_data/basic_prop/string_prop.xml").unwrap(),
            )
            .unwrap(),
            StringProp {
                name: String::from("prop name"),
                value: String::from("string prop value")
            }
        )
    }
}
