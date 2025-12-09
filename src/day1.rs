use std::num::ParseIntError;

use aoc_runner_derive::{aoc, aoc_generator};
use eyre::Result;
use nom::character::complete::{one_of, usize};
use nom::combinator::map_res;
use nom::{IResult, Parser};

#[derive(Copy, Clone, Debug)]
enum Rotation {
    Left(usize),
    Right(usize),
}

#[derive(Clone, Debug)]
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
fn solve_part1(input: &Input1) -> usize {
    let mut cnt = 0usize;

    input.data.iter().fold(50, |acc, r| {
        let m = match r {
            Rotation::Left(amount) => -(*amount as i32),
            Rotation::Right(amount) => *amount as i32,
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
    let r = |(dir, amount): (char, std::primitive::usize)| -> Result<Rotation, ParseIntError> {
        match dir {
            'L' => Ok(Rotation::Left(amount)),
            'R' => Ok(Rotation::Right(amount)),
            _ => unreachable!(
                "Invalid rotation direction: '{}'. Expected 'L' or 'R'.",
                dir
            ),
        }
    };
    map_res((one_of("LR"), usize), r).parse(input)
}
