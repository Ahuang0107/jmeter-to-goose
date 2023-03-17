use crate::{BoolProp, Deserializer, ElementLoopController, StringProp};
use xmltree::Element;

#[derive(Debug, PartialEq)]
pub struct ThreadGroup {
    pub test_name: String,
    pub enabled: bool,
    pub on_sample_error: String,
    pub loops: i32,
    pub num_threads: i32,
    pub ramp_time: i32,
    pub scheduler: bool,
    pub duration: Option<i32>,
    pub delay: Option<i32>,
}

impl ThreadGroup {
    pub fn parse(e: &Element) -> Self {
        assert_eq!(e.name.as_str(), "ThreadGroup");
        let test_name = e.attributes.get("testname").unwrap().clone();
        let enabled = e
            .attributes
            .get("enabled")
            .unwrap()
            .parse::<bool>()
            .unwrap();
        let children = e
            .children
            .iter()
            .map(|x| x.as_element())
            .filter(|x| x.is_some())
            .map(|x| x.unwrap())
            .collect::<Vec<&Element>>();
        let on_sample_error = StringProp::parse(
            children
                .iter()
                .find(|x| x.attributes.get("name").unwrap() == "ThreadGroup.on_sample_error")
                .unwrap(),
        )
        .value;
        let loop_controller = ElementLoopController::parse(
            children
                .iter()
                .find(|x| x.attributes.get("name").unwrap() == "ThreadGroup.main_controller")
                .unwrap(),
        );
        let num_threads = StringProp::parse(
            children
                .iter()
                .find(|x| x.attributes.get("name").unwrap() == "ThreadGroup.num_threads")
                .unwrap(),
        )
        .value
        .parse::<i32>()
        .unwrap();
        let ramp_time = StringProp::parse(
            children
                .iter()
                .find(|x| x.attributes.get("name").unwrap() == "ThreadGroup.ramp_time")
                .unwrap(),
        )
        .value
        .parse::<i32>()
        .unwrap();
        let scheduler = BoolProp::parse(
            children
                .iter()
                .find(|x| x.attributes.get("name").unwrap() == "ThreadGroup.scheduler")
                .unwrap(),
        )
        .value;
        let duration = StringProp::parse(
            children
                .iter()
                .find(|x| x.attributes.get("name").unwrap() == "ThreadGroup.duration")
                .unwrap(),
        )
        .value
        .parse::<i32>()
        .ok();
        let delay = StringProp::parse(
            children
                .iter()
                .find(|x| x.attributes.get("name").unwrap() == "ThreadGroup.delay")
                .unwrap(),
        )
        .value
        .parse::<i32>()
        .ok();
        Self {
            test_name,
            enabled,
            on_sample_error,
            loops: loop_controller.loops,
            num_threads,
            ramp_time,
            scheduler,
            duration,
            delay,
        }
    }
}
