use std::num::ParseIntError;

use aoc_runner_derive::{aoc, aoc_generator};
use eyre::Result;
use nom::character::complete::{i16, one_of};
use nom::combinator::map_res;
use nom::{IResult, Parser};

#[derive(Copy, Clone, Debug)]
enum Rotation {
    Left(i16),
    Right(i16),
}

struct Input1 {
    pub data: Vec<Rotation>,
}

#[aoc_generator(day1)]
fn input_generator(input: &str) -> Result<Input1> {
    let input = input
        .lines()
        .map(|l| parse_rotation(l).unwrap().1)
        .collect::<Vec<Rotation>>();
    Ok(Input1 { data: input })
}

#[aoc(day1, part1)]
fn solve_part1(input: &Input1) -> u32 {
    let mut cnt = 0;

    input.data.iter().fold(50, |acc, r| {
        let m = match r {
            Rotation::Left(amount) => -(*amount),
            Rotation::Right(amount) => *amount,
        };
        let res = (acc + m) % 100;
        if res == 0 {
            cnt += 1;
        }
        res
    });
    cnt
}

fn parse_rotation(input: &str) -> IResult<&str, Rotation> {
    let r = |(dir, amount): (char, std::primitive::i16)| -> Result<Rotation, ParseIntError> {
        match dir {
            'L' => Ok(Rotation::Left(amount)),
            'R' => Ok(Rotation::Right(amount)),
            _ => unreachable!(
                "Invalid rotation direction: '{}'. Expected 'L' or 'R'.",
                dir
            ),
        }
    };
    map_res((one_of("LR"), i16), r).parse(input)
}
