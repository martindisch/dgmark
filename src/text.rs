use nom::{
    branch::alt,
    bytes::complete::{take_until1, take_while1},
    IResult,
};

pub fn parse(input: &str) -> IResult<&str, &str> {
    // TODO: if take_until1 fails (e.g. when tag is first thing encountered),
    // take_while1 will eat everything. To prevent it, this parser needs to be
    // the last one evaluated, which is not necessarily intuitive.
    alt((take_until1("[["), take_while1(|_| true)))(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn no_tag() {
        let input = "The [ quick]] brown fox jumps";
        assert_eq!(Ok(("", "The [ quick]] brown fox jumps")), parse(input));
    }

    #[test]
    fn single_tag() {
        let input = "The [ quick]] brown [[fox]]jumps";
        assert_eq!(Ok(("[[fox]]jumps", "The [ quick]] brown ")), parse(input));
    }
}
