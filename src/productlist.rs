use nom::{
    bytes::complete::{tag, tag_no_case},
    combinator::value,
    sequence::{delimited, tuple},
    IResult,
};

use crate::common::*;

/// A list of products.
#[derive(Debug, PartialEq, Eq)]
pub struct ProductList(pub Vec<u64>);

/// Parses a `ProductList`.
pub fn parse(input: &str) -> IResult<&str, ProductList> {
    let (input, (_element_name, _whitespace, ids)) = delimited(
        tag("[["),
        tuple((element_name, colon_with_whitespace, ids)),
        tag("]]"),
    )(input)?;

    Ok((input, ProductList(ids)))
}

/// Matches the product list's tag, discarding it.
fn element_name(input: &str) -> IResult<&str, ()> {
    value((), tag_no_case("productlist"))(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn full_tag() {
        let input = "[[productlist: 1|20|31]]";
        assert_eq!(Ok(("", ProductList(vec![1, 20, 31]))), parse(input));
    }

    #[test]
    fn simple_tag() {
        let input = "[[productlist:1]]";
        assert_eq!(Ok(("", ProductList(vec![1]))), parse(input));
    }
}
