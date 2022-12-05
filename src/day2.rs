use std::collections::HashMap;
use aoc_runner_derive::aoc;
use itertools::Itertools;

#[aoc(day2, part1)]
pub fn solver(input: &str) -> u32 {
    let all: HashMap<&str, u32> = HashMap::from([
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

#[aoc(day2, part2, two)]
pub fn solver_part2(input: &str)-> u32 {
    let all: HashMap<&str, u32> = HashMap::from([
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

    let transf: HashMap<&str, &str> = HashMap::from([
        ("A X", "A Z"),
        ("A Y", "A X"),
        ("A Z", "A Y"),
        ("B X", "B X"),
        ("B Y", "B Y"),
        ("B Z", "B Z"),
        ("C X", "C Y"),
        ("C Y", "C Z"),
        ("C Z", "C X"),
    ]);

    input
        .lines()
        .map(|line| {
            all[transf[line]]
        })
        .sum1::<u32>()
        .unwrap()
}
