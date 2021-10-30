use dgmark::Element;
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Serialize, Deserialize)]
struct Container {
    #[serde(rename = "$value")]
    elements: Vec<Element>,
}

impl fmt::Display for Container {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for element in &self.elements {
            element.fmt(f)?;
        }

        Ok(())
    }
}

fn main() {
    let input = r#"Hi there with [[productlist: 1|2|20]] and [[productlist:20]]. We have a quote [[quote:Some text is nice"The source"]] too."#;

    let (_, parsed) = dgmark::parse(input).unwrap();
    println!("{:?}", parsed);

    let serialized =
        quick_xml::se::to_string(&Container { elements: parsed }).unwrap();
    println!("{}", serialized);

    let deserialized: Container =
        quick_xml::de::from_str(&serialized).unwrap();
    println!("{:?}", deserialized);

    let formatted = deserialized.to_string();
    assert_eq!(input, formatted);
    println!("{}", formatted);
}
