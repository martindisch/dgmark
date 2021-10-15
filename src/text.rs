use nom::{
    branch::alt,
    bytes::complete::{take_until1, take_while1},
    error::{ErrorKind, ParseError},
    Err, FindSubstring, IResult, InputLength, InputTake,
};

pub fn parse(input: &str) -> IResult<&str, &str> {
    // TODO: if take_until1 fails (e.g. when tag is first thing encountered),
    // take_while1 will eat everything. To prevent it, this parser needs to be
    // the last one evaluated, which is not necessarily intuitive.
    take_until1_relaxed("[[")(input)
}

fn take_until1_relaxed<T, Input, Error: ParseError<Input>>(
    tag: T,
) -> impl Fn(Input) -> IResult<Input, Input, Error>
where
    Input: InputTake + FindSubstring<T>,
    T: InputLength + Clone,
{
    move |i: Input| {
        let t = tag.clone();
        let res = match i.find_substring(t) {
            None => Err(Err::Error(Error::from_error_kind(
                i,
                ErrorKind::TakeUntil,
            ))),
            Some(0) => Err(Err::Error(Error::from_error_kind(
                i,
                ErrorKind::TakeUntil,
            ))),
            Some(index) => Ok(i.take_split(index)),
        };
        res
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
