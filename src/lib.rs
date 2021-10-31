use nom::{branch::alt, multi::many0, IResult};
use std::fmt;

mod common;
mod productlist;
mod quote;
mod text;

pub use productlist::Product;
pub use quote::{QuoteSource, QuoteText};

/// Enum of all elements of a markdown text.
#[derive(Debug, PartialEq, Eq)]
pub enum Element {
    ProductList {
        products: Vec<Product>,
    },
    Quote {
        text: QuoteText,
        source: QuoteSource,
    },
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
    let (input, (text, source)) = quote::parse(input)?;
    Ok((input, Element::Quote { text, source }))
}

/// Parses text and wraps it in an `Element` variant.
fn parse_text(input: &str) -> IResult<&str, Element> {
    let (input, text) = text::parse(input)?;
    Ok((input, Element::Text(text.into())))
}

impl fmt::Display for Element {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Element::ProductList { products } => {
                let ids = products
                    .iter()
                    .map(|p| p.id.to_string())
                    .collect::<Vec<String>>()
                    .join("|");
                write!(f, "[[productlist:{}]]", ids)
            }
            Element::Quote {
                text: QuoteText(text),
                source: QuoteSource(source),
            } => write!(f, r#"[[quote:{}"{}"]]"#, text, source),
            Element::Text(text) => write!(f, "{}", text),
        }
    }
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

    #[test]
    fn format_quote() {
        let element = Element::Quote {
            text: QuoteText("The text".into()),
            source: QuoteSource("The source".into()),
        };
        assert_eq!(r#"[[quote:The text"The source"]]"#, element.to_string());
    }

    #[test]
    fn format_productlist() {
        let element = Element::ProductList {
            products: vec![Product { id: 1 }, Product { id: 12 }],
        };

        assert_eq!("[[productlist:1|12]]", element.to_string());
    }

    #[test]
    fn format_text() {
        let element = Element::Text("Hello, world!".into());
        assert_eq!("Hello, world!", element.to_string());
    }
}
