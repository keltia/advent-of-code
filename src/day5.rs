use std::collections::VecDeque;

use aoc_runner_derive::{aoc, aoc_generator};
use array_macro::array;
use itertools::{enumerate, Itertools};
use nom::{
    branch::alt,
    bytes::complete::{tag, take},
    combinator::map,
    multi::many0,
    sequence::delimited,
    IResult,
};
use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct Line(VecDeque<char>);

#[derive(Debug)]
pub struct Start {
    lines: [Line; 9],
}

#[derive(Debug, Eq, PartialEq)]
struct Bucket(char);

/// Parse a bucket with a char inside
///
fn parse_bucket(input: &str) -> IResult<&str, Bucket> {
    let first = |s: &str| Bucket(s.chars().next().unwrap());
    let f = delimited(tag("["), take(1 as usize), tag("]"));
    map(f, first)(input)
}

/// Parse nonexistent bucket aka 3 spaces
///
fn parse_none(input: &str) -> IResult<&str, ()> {
    map(tag("   "), drop)(input)
}

/// Parse whether we have a bucket or not
///
fn parse_bucket_or_not(input: &str) -> IResult<&str, Option<Bucket>> {
    alt((map(parse_bucket, Some), map(parse_none, |_| None)))(input)
}

/// Parse one line
///
fn parse_one_line(input: &str) -> IResult<&str, Vec<Option<Bucket>>> {
    many0(parse_bucket_or_not)(input)
}

//#[aoc_generator(day5)]

#[aoc(day5, part1)]
fn solver_part1(input: &str) -> u32 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_bucket_ok() {
        let a = "[P]";
        let r = parse_bucket(a).unwrap();
        assert_eq!(Bucket('P'), r.1);
    }

    #[test]
    fn test_parse_bucket_nok() {
        let a = "<P>";
        let r = parse_bucket(a);
        assert!(r.is_err());
    }

    #[test]
    fn test_parse_none_ok() {
        let a = "    ";
        let r = parse_none(a);
        assert!(r.is_ok());
        let r = r.unwrap();
        assert_eq!(" ", r.0);
    }

    #[test]
    fn test_parse_none_nok() {
        let a = "[P]";
        let r = parse_none(a);
        assert!(r.is_err());
    }

    #[test]
    fn test_parse_alt_ok() {
        let a = "[P]";
        let r = parse_bucket_or_not(a);
        assert!(r.is_ok());
        let r = r.unwrap();
        assert!(r.0.is_empty());
        assert_eq!(Some(Bucket('P')), r.1);
    }

    #[test]
    fn test_parse_alt_none_ok() {
        let a = "    ";
        let r = parse_bucket_or_not(a);
        assert!(r.is_ok());
        let r = r.unwrap();
        assert_eq!(" ", r.0);
        assert_eq!(None, r.1);
    }

    #[test]
    fn test_parse_one_line() {
        let a = "[P]   [Z]";
        let r = parse_one_line(a);
        assert!(r.is_ok());
        let r = r.unwrap();
        dbg!(&r);
        assert!(r.0.is_empty());
        assert_eq!(vec![Some(Bucket('P')), None, Some(Bucket('Z'))], r.1);
    }
}
