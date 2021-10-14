use nom::{
    bytes::complete::{tag, tag_no_case},
    character::complete::{char, digit1, multispace0},
    combinator::{map_res, recognize},
    multi::separated_list1,
    sequence::{delimited, tuple},
    IResult,
};
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq)]
pub struct ProductList {
    products: Vec<u64>,
}

pub fn parse(input: &str) -> IResult<&str, ProductList> {
    let (input, (_tagname, _whitespace, ids)) = delimited(
        tag("[["),
        tuple((tag_name, colon_with_whitespace, ids)),
        tag("]]"),
    )(input)?;

    Ok((input, ProductList { products: ids }))
}

fn tag_name(input: &str) -> IResult<&str, &str> {
    tag_no_case("productlist")(input)
}

fn colon_with_whitespace(input: &str) -> IResult<&str, &str> {
    recognize(tuple((char(':'), multispace0)))(input)
}

fn ids(input: &str) -> IResult<&str, Vec<u64>> {
    separated_list1(char('|'), id)(input)
}

fn id(input: &str) -> IResult<&str, u64> {
    map_res(digit1, FromStr::from_str)(input)
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
                ProductList {
                    products: vec![1, 20, 31]
                }
            )),
            parse(input)
        );
    }

    #[test]
    fn simple_tag() {
        let input = "[[productlist:1]]";
        assert_eq!(Ok(("", ProductList { products: vec![1] })), parse(input));
    }

    #[test]
    fn colon_alone() {
        let input = ":";
        assert_eq!(Ok(("", ":")), colon_with_whitespace(input));
    }

    #[test]
    fn colon_spaces() {
        let input = ":  ";
        assert_eq!(Ok(("", ":  ")), colon_with_whitespace(input));
    }

    #[test]
    fn colon_tab() {
        let input = ":\t";
        assert_eq!(Ok(("", ":\t")), colon_with_whitespace(input));
    }

    #[test]
    fn ids_single() {
        let input = "1";
        assert_eq!(Ok(("", vec![1])), ids(input))
    }

    #[test]
    fn ids_multiple() {
        let input = "1|2|10";
        assert_eq!(Ok(("", vec![1, 2, 10])), ids(input))
    }

    #[test]
    fn id_single_digit() {
        let input = "1";
        assert_eq!(Ok(("", 1)), id(input));
    }

    #[test]
    fn id_multiple_digits() {
        let input = "10";
        assert_eq!(Ok(("", 10)), id(input));
    }
}
