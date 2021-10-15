use nom::{
    branch::alt, bytes::complete::take_until, combinator::rest, IResult,
};

pub fn parse(input: &str) -> IResult<&str, &str> {
    alt((take_until("[["), rest))(input)
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
