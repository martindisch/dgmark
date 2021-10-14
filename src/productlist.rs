use nom::{
    bytes::complete::tag_no_case,
    character::complete::{char, multispace0},
    sequence::tuple,
    IResult,
};

#[derive(Debug, PartialEq, Eq)]
struct ProductList {
    products: Vec<u64>,
}

fn parse(input: &str) -> IResult<&str, ProductList> {
    let tag_name = tag_no_case("productlist");
    let (input, tag) = tag_name(input)?;
    Ok((input, ProductList { products: vec![] }))
}

fn colon_with_whitespace(input: &str) -> IResult<&str, (char, &str)> {
    let colon = char(':');
    let whitespace = multispace0;
    tuple((colon, whitespace))(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn multiple_products() {
        let input = "productlist: 1|20|31";
        assert_eq!(
            Ok((": 1|20|31", ProductList { products: vec![] })),
            parse(input)
        );
    }

    fn finalized() {
        let input = "Some text [[productlist: 1|20|31]] and more text";
    }

    fn colon_whitespace() {
        let input = ": ";
        assert_eq!(Ok(("", (':', " "))), colon_with_whitespace(input));
    }
}
