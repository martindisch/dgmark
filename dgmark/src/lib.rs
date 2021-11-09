use nom::{branch::alt, multi::many0, IResult};

mod common;
mod productlist;
mod quote;
mod text;
mod traits;

pub use productlist::ProductList;
pub use quote::Quote;
pub use traits::Element;

/// Parses markdown and returns the list of translatable texts.
pub fn texts(input: &str) -> Result<Vec<&str>, &str> {
    match parse(input) {
        Ok(("", elements)) => {
            Ok(elements.into_iter().flat_map(|e| e.texts()).collect())
        }
        Ok(_) => Err("Input could not be fully parsed"),
        Err(_) => Err("Parser encountered an error"),
    }
}

/// Parses markdown into its list of elements.
pub fn parse<'a>(input: &'a str) -> IResult<&str, Vec<Box<dyn Element + 'a>>> {
    many0(alt((parse_productlist, parse_quote, parse_text)))(input)
}

/// Parses a product list and wraps it in an `Element` box.
fn parse_productlist<'a>(
    input: &'a str,
) -> IResult<&str, Box<dyn Element + 'a>> {
    let (input, productlist) = productlist::parse(input)?;
    Ok((input, Box::new(productlist)))
}

/// Parses a quote and wraps it in an `Element` box.
fn parse_quote<'a>(input: &'a str) -> IResult<&str, Box<dyn Element + 'a>> {
    let (input, quote) = quote::parse(input)?;
    Ok((input, Box::new(quote)))
}

/// Parses text and wraps it in an `Element` box.
fn parse_text<'a>(input: &'a str) -> IResult<&str, Box<dyn Element + 'a>> {
    let (input, text) = text::parse(input)?;
    Ok((input, Box::new(text)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn no_elements() {
        let input = "";
        let (_, parsed) = parse(input).unwrap();

        assert_eq!(0, parsed.len());
    }

    #[test]
    fn just_text() {
        let input = "The quick brown fox";
        let (_, parsed) = parse(input).unwrap();

        assert_eq!(1, parsed.len());
    }

    #[test]
    fn just_productlist() {
        let input = "[[productlist:1]]";
        let (_, parsed) = parse(input).unwrap();

        assert_eq!(1, parsed.len());
    }

    #[test]
    fn texts_and_productlists() {
        let input = "The [[productlist:1]] quick [[productlist:1|2]] brown";
        let (_, parsed) = parse(input).unwrap();

        assert_eq!(5, parsed.len());
    }
}
