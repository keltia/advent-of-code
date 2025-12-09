use std::num::ParseIntError;

use aoc_runner_derive::{aoc, aoc_generator};
use eyre::Result;
use nom::character::complete::{one_of, usize};
use nom::combinator::map_res;
use nom::{IResult, Parser};

#[derive(Copy, Clone, Debug)]
enum Move {
    Left(usize),
    Right(usize),
}

#[derive(Clone, Debug)]
struct Input1 {
    pub data: Vec<Move>,
}

// -----

#[aoc_generator(day1)]
fn input_generator(input: &str) -> Result<Input1> {
    fn parse_rotation(input: &str) -> IResult<&str, Move> {
        let r = |(dir, amount): (char, std::primitive::usize)| -> Result<Move, ParseIntError> {
            match dir {
                'L' => Ok(Move::Left(amount)),
                'R' => Ok(Move::Right(amount)),
                _ => unreachable!("Invalid Move direction: '{}'. Expected 'L' or 'R'.", dir),
            }
        };
        map_res((one_of("LR"), usize), r).parse(input)
    }

    let input = input
        .lines()
        .map(|l| parse_rotation(l).unwrap().1)
        .collect::<Vec<Move>>();
    Ok(Input1 { data: input })
}

// -----

#[aoc(day1, part1)]
fn solve_part1(input: &Input1) -> usize {
    let mut cnt = 0usize;

    input.data.iter().fold(50, |acc, r| {
        let m = match r {
            Move::Left(amount) => -(*amount as i32),
            Move::Right(amount) => *amount as i32,
        };
        let res = (acc + m) % 100;
        if res == 0 {
            cnt += 1;
        }
        res
    });
    cnt
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample_1() {
        let s = "L68\nL30\nR48\nL5\nR60\nL55\nL1\nL99\nR14\nL82\n";
        let input = input_generator(s).unwrap();

        assert_eq!(solve_part1(&input), 3);
    }
}
