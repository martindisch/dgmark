use nom::{
    bytes::complete::{tag, tag_no_case},
    combinator::value,
    sequence::{delimited, tuple},
    IResult,
};
use serde::{Deserialize, Serialize};

use crate::common::*;

/// A single product.
#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Product {
    pub id: u64,
}

/// Parses a product list into a `Vec` of `Product`.
pub fn parse(input: &str) -> IResult<&str, Vec<Product>> {
    let (input, (_element_name, _whitespace, ids)) = delimited(
        tag("[["),
        tuple((element_name, colon_with_whitespace, ids)),
        tag("]]"),
    )(input)?;

    Ok((input, ids.into_iter().map(|id| Product { id }).collect()))
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
        assert_eq!(
            Ok((
                "",
                vec![
                    Product { id: 1 },
                    Product { id: 20 },
                    Product { id: 31 }
                ]
            )),
            parse(input)
        );
    }

    #[test]
    fn simple_tag() {
        let input = "[[productlist:1]]";
        assert_eq!(Ok(("", vec![Product { id: 1 }])), parse(input));
    }
}
