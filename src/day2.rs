use std::collections::HashMap;
use std::ops::Add;
use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

#[aoc(day2, part1)]
pub fn solver<'a>(input: &str) -> u32 {
    let all: HashMap<&'a str, u32> = HashMap::from([
        ("A X", 3 + 1),
        ("A Y", 6 + 2),
        ("A Z", 0 + 3),
        ("B X", 0 + 1),
        ("B Y", 3 + 2),
        ("B Z", 6 + 3),
        ("C X", 6 + 1),
        ("C Y", 0 + 2),
        ("C Z", 3 + 3),
    ]);
    input
        .lines()
        .map(|line| {
            all[line]
        })
        .sum1::<u32>()
        .unwrap()
}
