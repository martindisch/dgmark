use nom::{
    bytes::complete::{tag, tag_no_case},
    combinator::value,
    sequence::{delimited, tuple},
    IResult,
};
use std::fmt;

use crate::{common::*, traits::Element};

/// A product list.
#[derive(Debug, PartialEq, Eq)]
pub struct ProductList(pub Vec<u64>);

/// Parses a product list.
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

impl fmt::Display for ProductList {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let ids = self
            .0
            .iter()
            .map(ToString::to_string)
            .collect::<Vec<String>>()
            .join("|");
        write!(f, "[[productlist:{}]]", ids)
    }
}

impl Element for ProductList {}

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

    #[test]
    fn format() {
        let productlist = ProductList(vec![1, 12]);
        assert_eq!("[[productlist:1|12]]", productlist.to_string());
    }
}
