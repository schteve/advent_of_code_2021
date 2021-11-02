use nom::{
    character::complete::{digit1, multispace0, one_of},
    combinator::{map, map_res, opt, recognize},
    error::ParseError,
    sequence::{delimited, pair, preceded, terminated},
    IResult, Parser,
};
use std::str::FromStr;

pub fn unsigned<T: FromStr>(input: &str) -> IResult<&str, T> {
    map_res(digit1, |x: &str| x.parse::<T>())(input)
}

pub fn signed<T: FromStr>(input: &str) -> IResult<&str, T> {
    map_res(recognize(pair(opt(one_of("+-")), digit1)), |x: &str| {
        x.parse::<T>()
    })(input)
}

pub fn to_owned<'a, E, I, P>(parser: P) -> impl FnMut(&'a I) -> IResult<&'a I, I::Owned, E>
where
    I: 'a + ToOwned + ?Sized,
    P: Parser<&'a I, &'a I, E>,
{
    map(parser, |x: &I| x.to_owned())
}

pub fn trim<'a, E, O, P>(parser: P) -> impl FnMut(&'a str) -> IResult<&'a str, O, E>
where
    E: ParseError<&'a str>,
    P: Parser<&'a str, O, E>,
{
    delimited(multispace0, parser, multispace0)
}

pub fn trim_start<'a, E, O, P>(parser: P) -> impl FnMut(&'a str) -> IResult<&'a str, O, E>
where
    E: ParseError<&'a str>,
    P: Parser<&'a str, O, E>,
{
    preceded(multispace0, parser)
}

pub fn trim_end<'a, E, O, P>(parser: P) -> impl FnMut(&'a str) -> IResult<&'a str, O, E>
where
    E: ParseError<&'a str>,
    P: Parser<&'a str, O, E>,
{
    terminated(parser, multispace0)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_unsigned() {
        let input = "1234";
        let (remain, num) = unsigned::<u32>(input).unwrap();
        assert_eq!(remain, "");
        assert_eq!(num, 1234);

        let input = "1234abc123";
        let (remain, num) = unsigned::<u32>(input).unwrap();
        assert_eq!(remain, "abc123");
        assert_eq!(num, 1234);

        let input = "1234.5678";
        let (remain, num) = unsigned::<u32>(input).unwrap();
        assert_eq!(remain, ".5678");
        assert_eq!(num, 1234);

        let input = " 1234";
        let result = unsigned::<u32>(input);
        assert!(result.is_err());

        let input = "-1234";
        let result = unsigned::<u8>(input);
        assert!(result.is_err());

        let input = "1234";
        let result = unsigned::<u8>(input);
        assert!(result.is_err());

        let input = "12345678901234567890";
        let (remain, num) = unsigned::<u64>(input).unwrap();
        assert_eq!(remain, "");
        assert_eq!(num, 12345678901234567890);
    }

    #[test]
    fn test_signed() {
        let input = "1234";
        let (remain, num) = signed::<i32>(input).unwrap();
        assert_eq!(remain, "");
        assert_eq!(num, 1234);

        let input = "-1234";
        let (remain, num) = signed::<i32>(input).unwrap();
        assert_eq!(remain, "");
        assert_eq!(num, -1234);

        let input = "+1234";
        let (remain, num) = signed::<i32>(input).unwrap();
        assert_eq!(remain, "");
        assert_eq!(num, 1234);

        let input = "1234abc123";
        let (remain, num) = signed::<i32>(input).unwrap();
        assert_eq!(remain, "abc123");
        assert_eq!(num, 1234);

        let input = "1234.5678";
        let (remain, num) = signed::<i32>(input).unwrap();
        assert_eq!(remain, ".5678");
        assert_eq!(num, 1234);

        let input = "1234-5678";
        let (remain, num) = signed::<i32>(input).unwrap();
        assert_eq!(remain, "-5678");
        assert_eq!(num, 1234);

        let input = " 1234";
        let result = signed::<i32>(input);
        assert!(result.is_err());

        let input = "1234";
        let result = signed::<i8>(input);
        assert!(result.is_err());

        let input = "1234567890123456789";
        let (remain, num) = signed::<i64>(input).unwrap();
        assert_eq!(remain, "");
        assert_eq!(num, 1234567890123456789);
    }

    #[test]
    fn test_to_owned() {
        use nom::character::complete::{alpha1, alphanumeric1};

        let input = "abcd";
        let result: IResult<&str, String> = to_owned(alpha1)(input);
        let (remain, s) = result.unwrap();
        assert_eq!(remain, "");
        assert_eq!(s, "abcd");

        let input = "abcd123;";
        let result: IResult<&str, String> = to_owned(alphanumeric1)(input);
        let (remain, s) = result.unwrap();
        assert_eq!(remain, ";");
        assert_eq!(s, "abcd123");

        let input = b"abcd123";
        let result: IResult<&[u8], Vec<u8>> = to_owned(alpha1)(input);
        let (remain, s) = result.unwrap();
        assert_eq!(remain, b"123");
        assert_eq!(s, b"abcd".to_vec());
    }

    #[test]
    fn test_trim() {
        let input = "1234";
        let result: IResult<&str, &str> = trim(digit1)(input);
        let (remain, num) = result.unwrap();
        assert_eq!(remain, "");
        assert_eq!(num, "1234");

        let input = "\n1234";
        let result: IResult<&str, &str> = trim(digit1)(input);
        let (remain, num) = result.unwrap();
        assert_eq!(remain, "");
        assert_eq!(num, "1234");

        let input = "1234\r";
        let result: IResult<&str, &str> = trim(digit1)(input);
        let (remain, num) = result.unwrap();
        assert_eq!(remain, "");
        assert_eq!(num, "1234");

        let input = " 1234\t";
        let result: IResult<&str, &str> = trim(digit1)(input);
        let (remain, num) = result.unwrap();
        assert_eq!(remain, "");
        assert_eq!(num, "1234");

        let input = "          1234            ";
        let result: IResult<&str, &str> = trim(digit1)(input);
        let (remain, num) = result.unwrap();
        assert_eq!(remain, "");
        assert_eq!(num, "1234");
    }

    #[test]
    fn test_trim_start() {
        let input = "1234";
        let result: IResult<&str, &str> = trim_start(digit1)(input);
        let (remain, num) = result.unwrap();
        assert_eq!(remain, "");
        assert_eq!(num, "1234");

        let input = "\n1234";
        let result: IResult<&str, &str> = trim_start(digit1)(input);
        let (remain, num) = result.unwrap();
        assert_eq!(remain, "");
        assert_eq!(num, "1234");

        let input = "1234\r";
        let result: IResult<&str, &str> = trim_start(digit1)(input);
        let (remain, num) = result.unwrap();
        assert_eq!(remain, "\r");
        assert_eq!(num, "1234");

        let input = " 1234\t";
        let result: IResult<&str, &str> = trim_start(digit1)(input);
        let (remain, num) = result.unwrap();
        assert_eq!(remain, "\t");
        assert_eq!(num, "1234");

        let input = "          1234";
        let result: IResult<&str, &str> = trim_start(digit1)(input);
        let (remain, num) = result.unwrap();
        assert_eq!(remain, "");
        assert_eq!(num, "1234");
    }

    #[test]
    fn test_trim_end() {
        let input = "1234";
        let result: IResult<&str, &str> = trim_end(digit1)(input);
        let (remain, num) = result.unwrap();
        assert_eq!(remain, "");
        assert_eq!(num, "1234");

        let input = "\n1234";
        let result: IResult<&str, &str> = trim_end(digit1)(input);
        assert!(result.is_err());

        let input = "1234\r";
        let result: IResult<&str, &str> = trim_end(digit1)(input);
        let (remain, num) = result.unwrap();
        assert_eq!(remain, "");
        assert_eq!(num, "1234");

        let input = " 1234\t";
        let result: IResult<&str, &str> = trim_end(digit1)(input);
        assert!(result.is_err());

        let input = "1234            ";
        let result: IResult<&str, &str> = trim_end(digit1)(input);
        let (remain, num) = result.unwrap();
        assert_eq!(remain, "");
        assert_eq!(num, "1234");
    }
}
