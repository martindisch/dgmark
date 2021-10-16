use nom::{branch::alt, multi::many0, IResult};

mod common;
mod productlist;
mod text;

pub use productlist::ProductList;

#[derive(Debug, PartialEq, Eq)]
pub enum Element<'a> {
    ProductList(ProductList),
    Text(&'a str),
}

pub fn parse(input: &str) -> IResult<&str, Vec<Element>> {
    many0(alt((parse_productlist, parse_text)))(input)
}

fn parse_text(input: &str) -> IResult<&str, Element> {
    let (input, text) = text::parse(input)?;
    Ok((input, Element::Text(text)))
}

fn parse_productlist(input: &str) -> IResult<&str, Element> {
    let (input, productlist) = productlist::parse(input)?;
    Ok((input, Element::ProductList(productlist)))
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
            Ok(("", vec![Element::Text("The quick brown fox")])),
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
                    Element::Text("The "),
                    Element::ProductList(ProductList(vec![1])),
                    Element::Text(" quick "),
                    Element::ProductList(ProductList(vec![1, 2])),
                    Element::Text(" brown")
                ]
            )),
            parse(input)
        );
    }
}
