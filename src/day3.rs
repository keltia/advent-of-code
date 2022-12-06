use aoc_runner_derive::aoc;
use itertools::{Either, Itertools};
use std::collections::HashSet;

/// Add all accumulated u8 into a u32
///
fn add_all(list: &[u8]) -> u32 {
    list.iter()
        .map(|&c| {
            let r = match c as char {
                // 97..122 => 1..16
                'a'..='z' => (c as u8) - 96,
                // 65..90 => 27..52
                'A'..='Z' => (c as u8) - 38,
                _ => panic!("impossible"),
            };
            r as u32
        })
        .sum1()
        .unwrap()
}

#[aoc(day3, part1, partition_map)]
pub fn solve_part1(input: &str) -> u32 {
    let list = input
        .lines()
        //
        // split at middle
        //
        .map(|line| {
            let splt = line.split_at(line.len() / 2);
            (splt.0.to_string(), splt.1.to_string())
        })
        //
        // Check which character is present in both
        //
        .map(|(l, r)| {
            let (uniq, _not): (Vec<_>, Vec<_>) = l.chars().partition_map(|c| {
                if r.contains(c) {
                    Either::Left(c)
                } else {
                    Either::Right(c)
                }
            });
            uniq[0] as u8
        })
        .collect::<Vec<u8>>();
    //
    // Now for each character, find value and sum everything.
    //
    add_all(&list)
}

/// Alternate version using a set and .any().  Cleaner but much slower.
///
#[aoc(day3, part1, any)]
pub fn solve_part1_any(input: &str) -> u32 {
    let list = input
        .lines()
        //
        // split at middle
        //
        .map(|line| {
            let splt = line.split_at(line.len() / 2);
            (splt.0.to_string(), splt.1.to_string())
        })
        //
        // Check which character is present in both
        //
        .map(|(l, r)| {
            let set: HashSet<char> = l.chars().collect();
            let v = r.chars().find(|c| set.contains(c));
            v.unwrap() as u8
        })
        .collect::<Vec<u8>>();
    //
    // Now for each character, find value and sum everything.
    //
    add_all(&list)
}

// #[aoc(day3,part2)]
// pub fn solve_part2(input: &[u8]) -> u32 {
//     unimplemented!()
// }
