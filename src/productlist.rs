use nom::{
    bytes::complete::{tag, tag_no_case},
    sequence::{delimited, tuple},
    IResult,
};

use crate::common::*;

#[derive(Debug, PartialEq, Eq)]
pub struct ProductList(pub Vec<u64>);

pub fn parse(input: &str) -> IResult<&str, ProductList> {
    let (input, (_element_name, _whitespace, ids)) = delimited(
        tag("[["),
        tuple((element_name, colon_with_whitespace, ids)),
        tag("]]"),
    )(input)?;

    Ok((input, ProductList(ids)))
}

fn element_name(input: &str) -> IResult<&str, &str> {
    tag_no_case("productlist")(input)
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
