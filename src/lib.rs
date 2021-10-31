use nom::{branch::alt, multi::many0, IResult};
use std::fmt;

mod common;
mod productlist;
mod quote;
mod text;

pub use productlist::ProductList;
pub use quote::Quote;

/// Enum of all elements of a markdown text.
#[derive(Debug, PartialEq, Eq)]
pub enum Element {
    ProductList(ProductList),
    Quote(Quote),
    Text(String),
}

/// Parses a full markdown text into its list of elements.
pub fn parse(input: &str) -> IResult<&str, Vec<Element>> {
    many0(alt((parse_productlist, parse_quote, parse_text)))(input)
}

/// Parses a product list and wraps it in an `Element` variant.
fn parse_productlist(input: &str) -> IResult<&str, Element> {
    let (input, productlist) = productlist::parse(input)?;
    Ok((input, Element::ProductList(productlist)))
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

impl fmt::Display for Element {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Element::ProductList(productlist) => {
                let ids = productlist
                    .0
                    .iter()
                    .map(ToString::to_string)
                    .collect::<Vec<String>>()
                    .join("|");
                write!(f, "[[productlist:{}]]", ids)
            }
            Element::Quote(Quote { text, source }) => {
                write!(f, r#"[[quote:{}"{}"]]"#, text, source)
            }
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
            Ok(("", vec![Element::ProductList(ProductList(vec![1]))])),
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
                    Element::ProductList(ProductList(vec![1])),
                    Element::Text(" quick ".into()),
                    Element::ProductList(ProductList(vec![1, 2])),
                    Element::Text(" brown".into())
                ]
            )),
            parse(input)
        );
    }

    #[test]
    fn format_quote() {
        let element = Element::Quote(Quote {
            text: "The text".into(),
            source: "The source".into(),
        });
        assert_eq!(r#"[[quote:The text"The source"]]"#, element.to_string());
    }

    #[test]
    fn format_productlist() {
        let element = Element::ProductList(ProductList(vec![1, 12]));

        assert_eq!("[[productlist:1|12]]", element.to_string());
    }

    #[test]
    fn format_text() {
        let element = Element::Text("Hello, world!".into());
        assert_eq!("Hello, world!", element.to_string());
    }
}
