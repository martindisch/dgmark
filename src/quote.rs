use nom::{
    bytes::complete::{tag, tag_no_case, take_till1},
    character::complete::char,
    combinator::value,
    sequence::{delimited, tuple},
    IResult,
};

use crate::common::*;

/// A quote's text.
#[derive(Debug, PartialEq, Eq)]
pub struct QuoteText(pub String);

/// A quote's source.
#[derive(Debug, PartialEq, Eq)]
pub struct QuoteSource(pub String);

/// Parses a `Quote`.
pub fn parse(input: &str) -> IResult<&str, (QuoteText, QuoteSource)> {
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

    Ok((input, (QuoteText(text.into()), QuoteSource(source.into()))))
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn full_tag() {
        let input = r#"[[quote:here's some text"And a source too"]]"#;
        assert_eq!(
            Ok((
                "",
                (
                    QuoteText("here's some text".into()),
                    QuoteSource("And a source too".into()),
                )
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
}
