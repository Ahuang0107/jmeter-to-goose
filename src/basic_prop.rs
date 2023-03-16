use std::fmt::Debug;
use std::str::FromStr;
use xmltree::Element;

pub enum BasicProp {
    String(TypeProp<String>),
    Bool(TypeProp<bool>),
    Int(TypeProp<usize>),
    Long(TypeProp<i64>),
}

#[derive(Debug, PartialEq)]
pub struct TypeProp<T>(String, T);

pub fn parse<F: FromStr + Debug>(ele: &Element) -> Result<TypeProp<F>, F::Err> {
    let name = ele.attributes.get("name").unwrap().clone();
    let value = ele
        .children
        .first()
        .unwrap()
        .as_text()
        .unwrap()
        .parse::<F>()?;
    Ok(TypeProp(name, value))
}

#[cfg(test)]
mod test {
    use crate::basic_prop::{parse, TypeProp};
    use xmltree::Element;

    #[test]
    fn check_parse_string() {
        let xml =
            Element::parse(r#"<stringProp name="num_threads">1</stringProp>"#.as_bytes()).unwrap();
        let result = parse::<String>(&xml).unwrap();
        assert_eq!(
            result,
            TypeProp(String::from("num_threads"), String::from("1"))
        )
    }

    #[test]
    fn check_parse_bool() {
        let xml =
            Element::parse(r#"<boolProp name="scheduler">false</boolProp>"#.as_bytes()).unwrap();
        let result = parse::<bool>(&xml).unwrap();
        assert_eq!(result, TypeProp(String::from("scheduler"), false))
    }

    #[test]
    fn check_parse_int() {
        let xml = Element::parse(r#"<intProp name="test_type">2</intProp>"#.as_bytes()).unwrap();
        let result = parse::<usize>(&xml).unwrap();
        assert_eq!(result, TypeProp(String::from("test_type"), 2))
    }
}
