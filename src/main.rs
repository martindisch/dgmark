use dgmark::Element;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Container {
    #[serde(rename = "$value")]
    elements: Vec<Element>,
}

fn main() {
    let input = "Hi there with [[productlist: 1|2|20]] and [[productlist:20]]";
    let (_, parsed) = dgmark::parse(input).unwrap();

    let serialized =
        quick_xml::se::to_string(&Container { elements: parsed }).unwrap();
    println!("{}", serialized);

    let deserialized: Container =
        quick_xml::de::from_str(&serialized).unwrap();
    println!("{:?}", deserialized);
}
