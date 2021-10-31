use nom::{
    bytes::complete::{tag, tag_no_case, take_till1},
    character::complete::char,
    combinator::value,
    sequence::{delimited, tuple},
    IResult,
};
use std::fmt;

use crate::{common::*, traits::Element};

/// A quoted text with source.
#[derive(Debug, PartialEq, Eq)]
pub struct Quote<'a> {
    pub text: &'a str,
    pub source: &'a str,
}

/// Parses a quote.
pub fn parse(input: &str) -> IResult<&str, Quote> {
    let (input, (_element_name, _whitespace, text, source)) = delimited(
        tag("[["),
        tuple((
            element_name,
            colon_with_whitespace,
            quote_text,
            quote_source,
        )),
        tag("]]"),
    )(input)?;

    Ok((input, Quote { text, source }))
}

/// Parses the text of a quote.
fn quote_text(input: &str) -> IResult<&str, &str> {
    take_till1(|c| c == '"')(input)
}

/// Parses the source of a quote.
fn quote_source(input: &str) -> IResult<&str, &str> {
    delimited(char('"'), take_till1(|c| c == '"'), char('"'))(input)
}

/// Matches the quote's tag, discarding it.
fn element_name(input: &str) -> IResult<&str, ()> {
    value((), tag_no_case("quote"))(input)
}

impl fmt::Display for Quote<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, r#"[[quote:{}"{}"]]"#, self.text, self.source)
    }
}

impl Element for Quote<'_> {
    fn texts(&self) -> Vec<&str> {
        vec![self.text, self.source]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn full_tag() {
        let input = r#"[[quote:here's some text"And a source too"]]"#;
        assert_eq!(
            Ok((
                "",
                Quote {
                    text: "here's some text",
                    source: "And a source too",
                }
            )),
            parse(input)
        );
    }

    #[test]
    fn no_text() {
        let input = r#"[[quote:"source"]]"#;
        assert!(parse(input).is_err());
    }

    #[test]
    fn no_source() {
        let input = r#"[[quote:text]]"#;
        assert!(parse(input).is_err());
    }

    #[test]
    fn empty_source() {
        let input = r#"[[quote:text""]]"#;
        assert!(parse(input).is_err());
    }

    #[test]
    fn invalid_source() {
        let input = r#"[[quote:text"invalid_source]]"#;
        assert!(parse(input).is_err());

        let input = r#"[[quote:textinvalid_source"]]"#;
        assert!(parse(input).is_err());
    }

    #[test]
    fn format() {
        let quote = Quote {
            text: "The text",
            source: "The source",
        };
        assert_eq!(r#"[[quote:The text"The source"]]"#, quote.to_string());
    }
}
