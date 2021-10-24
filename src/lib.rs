use nom::{branch::alt, multi::many0, IResult};
use serde::{Deserialize, Serialize};

mod common;
mod productlist;
mod quote;
mod text;

pub use productlist::Product;
pub use quote::Quote;

/// Enum of all elements of a markdown text.
#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Element {
    ProductList {
        #[serde(rename = "Product")]
        products: Vec<Product>,
    },
    Quote(Quote),
    Text(String),
}

/// Parses a full markdown text into its list of elements.
pub fn parse(input: &str) -> IResult<&str, Vec<Element>> {
    many0(alt((parse_productlist, parse_quote, parse_text)))(input)
}

/// Parses a product list and wraps it in an `Element` variant.
fn parse_productlist(input: &str) -> IResult<&str, Element> {
    let (input, products) = productlist::parse(input)?;
    Ok((input, Element::ProductList { products }))
}

/// Parses a quote and wraps it in an `Element` variant.
fn parse_quote(input: &str) -> IResult<&str, Element> {
    let (input, quote) = quote::parse(input)?;
    Ok((input, Element::Quote(quote)))
}

/// Parses text and wraps it in an `Element` variant.
fn parse_text(input: &str) -> IResult<&str, Element> {
    let (input, text) = text::parse(input)?;
    Ok((input, Element::Text(text.into())))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn no_elements() {
        let input = "";
        assert_eq!(Ok(("", vec![])), parse(input));
    }

    #[test]
    fn just_text() {
        let input = "The quick brown fox";
        assert_eq!(
            Ok(("", vec![Element::Text("The quick brown fox".into())])),
            parse(input)
        );
    }

    #[test]
    fn just_productlist() {
        let input = "[[productlist:1]]";
        assert_eq!(
            Ok((
                "",
                vec![Element::ProductList {
                    products: vec![Product { id: 1 }]
                }]
            )),
            parse(input)
        );
    }

    #[test]
    fn texts_and_productlists() {
        let input = "The [[productlist:1]] quick [[productlist:1|2]] brown";
        assert_eq!(
            Ok((
                "",
                vec![
                    Element::Text("The ".into()),
                    Element::ProductList {
                        products: vec![Product { id: 1 }]
                    },
                    Element::Text(" quick ".into()),
                    Element::ProductList {
                        products: vec![Product { id: 1 }, Product { id: 2 }]
                    },
                    Element::Text(" brown".into())
                ]
            )),
            parse(input)
        );
    }
}
