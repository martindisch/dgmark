use nom::{
    character::complete::{char, digit1, multispace0},
    combinator::{map_res, value},
    multi::separated_list1,
    sequence::tuple,
    IResult,
};
use std::str::FromStr;

pub fn colon_with_whitespace(input: &str) -> IResult<&str, ()> {
    value((), tuple((char(':'), multispace0)))(input)
}

pub fn ids(input: &str) -> IResult<&str, Vec<u64>> {
    separated_list1(char('|'), id)(input)
}

pub fn id(input: &str) -> IResult<&str, u64> {
    map_res(digit1, FromStr::from_str)(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn colon_alone() {
        let input = ":a";
        assert_eq!(Ok(("a", ())), colon_with_whitespace(input));
    }

    #[test]
    fn colon_spaces() {
        let input = ":  a";
        assert_eq!(Ok(("a", ())), colon_with_whitespace(input));
    }

    #[test]
    fn colon_tab() {
        let input = ":\ta";
        assert_eq!(Ok(("a", ())), colon_with_whitespace(input));
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
