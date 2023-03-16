use quick_xml::events::Event;
use quick_xml::Reader;

enum Element {
    Text(String),
    Element {
        _name: String,
        _properties: Vec<(String, String)>,
        _children: Vec<Element>,
    },
}

impl Element {
    fn el(name: &str) -> Self {
        Self::Element {
            _name: String::from(name),
            _properties: vec![],
            _children: vec![],
        }
    }
    fn text(content: &str) -> Self {
        Self::Text(String::from(content))
    }
}

fn main() -> anyhow::Result<()> {
    let xml = r#"
<?xml version="1.0" encoding="UTF-8"?>
<jmeterTestPlan version="1.2" properties="3.2" jmeter="3.3-SNAPSHOT.20170917">
  <hashTree>
    <TestPlan guiclass="TestPlanGui" testclass="TestPlan" testname="CSVSample" enabled="true">
    </TestPlan>
    <hashTree>
      <ThreadGroup guiclass="ThreadGroupGui" testclass="ThreadGroup" testname="Thread Group" enabled="true">
      </ThreadGroup>
      <hashTree>
      </hashTree>
      <ResultCollector guiclass="TableVisualizer" testclass="ResultCollector" testname="View Results in Table" enabled="true">
      </ResultCollector>
      <hashTree/>
      <ResultCollector guiclass="ViewResultsFullVisualizer" testclass="ResultCollector" testname="View Results Tree" enabled="true">
      </ResultCollector>
      <hashTree/>
    </hashTree>
  </hashTree>
</jmeterTestPlan>
    "#;
    let mut reader = Reader::from_str(xml);
    reader.trim_text(true);

    loop {
        match reader.read_event()? {
            Event::Start(data) => {
                println!("{:?}", String::from_utf8_lossy(data.name().as_ref()))
            }
            Event::End(data) => {
                println!("{:?}", String::from_utf8_lossy(data.name().as_ref()))
            }
            Event::Empty(_) => {}
            Event::Text(_) => {}
            Event::Eof => break,
            _ => {}
        }
    }

    Ok(())
}
