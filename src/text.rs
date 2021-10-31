use nom::{
    error::{ErrorKind, ParseError},
    lib::std::ops::RangeFrom,
    Err, FindSubstring, IResult, InputLength, InputTake, Slice,
};

use crate::traits::Element;

/// Parses non-empty text.
pub fn parse(input: &str) -> IResult<&str, &str> {
    take_until1_relaxed("[[")(input)
}

/// Custom implementation that doesn't error if the pattern is not found.
fn take_until1_relaxed<T, Input, Error: ParseError<Input>>(
    tag: T,
) -> impl Fn(Input) -> IResult<Input, Input, Error>
where
    Input:
        Slice<RangeFrom<usize>> + InputLength + InputTake + FindSubstring<T>,
    T: InputLength + Clone,
{
    move |i: Input| {
        let t = tag.clone();
        match i.find_substring(t) {
            Some(0) => Err(Err::Error(Error::from_error_kind(
                i,
                ErrorKind::TakeUntil,
            ))),
            Some(index) => Ok(i.take_split(index)),
            None if i.input_len() > 0 => Ok((i.slice(i.input_len()..), i)),
            None => Err(Err::Error(Error::from_error_kind(
                i,
                ErrorKind::TakeUntil,
            ))),
        }
    }
}

impl<'a> Element<'a> for &'a str {
    fn texts(&self) -> Vec<&'a str> {
        vec![self]
    }
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
