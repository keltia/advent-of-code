use aoc_runner_derive::{aoc, aoc_generator};
use itertools::{Itertools, Either};

#[aoc_generator(day3)]
pub fn generator(input: &str) -> Vec<u8> {
    input
        .lines()
        //
        // split at middle
        //
        .map(|line| {
            let splt = line
                .split_at(line.len() / 2);
            (splt.0.to_string(), splt.1.to_string())
        })
        //
        // Check which character is present in both
        //
        .map(|(l,r)| {
            let (uniq, _not): (Vec<_>, Vec<_>) = l
                .chars()
                .partition_map(|c| {
                    if r.contains(c) {
                        Either::Left(c)
                    } else {
                        Either::Right(c)
                    }
                });
            uniq[0] as u8
        })
        .collect()
}

#[aoc(day3,part1)]
pub fn solve_part1(input: &[u8]) -> u32 {
    input
        .iter()
        .map(|c| {
            let c = *c as char;
            let r = match c {
                // 97..122 => 1..16
                'a'..='z' => {
                    (c as u8) - 96
                },
                // 65..90 => 27..52
                'A'..='Z' => {
                    (c as u8) - 38
                }
                _ => panic!("impossible")
            };
            r as u32
        }).sum::<u32>()
}

// #[aoc(day3,part2)]
// pub fn solve_part2(input: &[u8]) -> u32 {
//     unimplemented!()
// }

