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

/// Find the intersection of all three sets
///
pub fn find_common(t: (&str, &str, &str)) -> u8 {
    // Given (A, B, C), compute D = A⏀B then D⏀C
    //
    let (r1, r2, r3) = (t.0, t.1, t.2);
    let r1: HashSet<char> = r1.chars().collect();
    let r2: HashSet<char> = r2.chars().collect();
    let r3: HashSet<char> = r3.chars().collect();

    let res: HashSet<char> = r1.intersection(&r2).copied().collect();

    let d: HashSet<_> = res.intersection(&r3).collect();
    let sum = **d.iter().nth(0).unwrap();
    sum as u8
}

#[aoc(day3, part2, references)]
pub fn solve_part2_str(input: &str) -> u32 {
    let mut bundles = vec![];

    // Read every 3 lines, create a list of tuples
    //
    let mut lines = input.lines();
    loop {
        match lines.next() {
            None => break,
            Some(r1) => {
                let r2 = lines.next().unwrap();
                let r3 = lines.next().unwrap();
                bundles.push((r1, r2, r3));
            }
        }
    }
    // Now act upon the list
    //
    let res: Vec<u8> = bundles.iter().map(|item| {
        let (r1, r2, r3) = item.clone();
        find_common((r1, r2, r3))
    }).collect();
    add_all(&res)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_common() {
        let a = "abcd";
        let b = "cgjh";
        let c= "iuycyt";

        assert_eq!('c' as u8, find_common((a, b, c)))
    }
}