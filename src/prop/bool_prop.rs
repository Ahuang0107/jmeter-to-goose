/// `<stringProp>`
#[derive(Debug, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct BoolProp {
    #[serde(rename = "@name")]
    pub name: String,
    #[serde(rename = "$text")]
    pub value: bool,
}

#[cfg(test)]
mod test {
    use crate::prop::BoolProp;

    #[test]
    fn check_deserialize() {
        assert_eq!(
            quick_xml::de::from_str::<BoolProp>(
                &std::fs::read_to_string("./unittests_data/basic_prop/bool_prop.xml").unwrap(),
            )
            .unwrap(),
            BoolProp {
                name: String::from("prop name"),
                value: true
            }
        )
    }
}
