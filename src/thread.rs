use xmltree::Element;

#[derive(Debug, PartialEq)]
pub struct ThreadGroup {
    pub test_name: String,
    pub enabled: bool,
}

impl ThreadGroup {
    pub fn parse(ele: &Element) -> Self {
        assert_eq!(ele.name.as_str(), "ThreadGroup");
        let test_name = ele.attributes.get("testname").unwrap().clone();
        let enabled = ele
            .attributes
            .get("enabled")
            .unwrap()
            .parse::<bool>()
            .unwrap();
        Self { test_name, enabled }
    }
}
