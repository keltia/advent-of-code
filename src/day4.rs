use std::collections::HashSet;

use aoc_runner_derive::aoc;
use itertools::Itertools;

pub fn make_set(l: &str) -> HashSet<usize> {
    let tpl: Vec<&str> = l.split('-').collect();
    let (b, e) = (tpl[0].parse::<usize>().unwrap(), tpl[1].parse::<usize>().unwrap());
    let mut set: HashSet<usize> = HashSet::new();
    for i in b..=e {
        set.insert(i);
    }
    set
}

#[aoc(day4, part1, sum1)]
pub fn solve_part1(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let tpl: Vec<&str> = line.split(',').collect();
            let s1 = tpl[0];
            let s2 = tpl[1];

            let s1 = make_set(s1);
            let s2 = make_set(s2);
            if s1.is_subset(&s2) || s1.is_superset(&s2) {
                1
            } else {
                0
            }
        })
        .sum1()
        .unwrap()
}

#[aoc(day4, part1, counter)]
pub fn solve_part1_counter(input: &str) -> u32 {
    let mut cnt = 0;
    input
        .lines()
        .for_each(|line| {
            let tpl: Vec<&str> = line.split(',').collect();
            let s1 = tpl[0];
            let s2 = tpl[1];

            let s1 = make_set(s1);
            let s2 = make_set(s2);
            if s1.is_subset(&s2) || s1.is_superset(&s2) {
                cnt += 1;
            }
        });
    cnt
}

#[aoc(day4, part2)]
pub fn solve_part2(input: &str) -> u32 {
    let mut cnt = 0;
    input
        .lines()
        .for_each(|line| {
            let tpl: Vec<&str> = line.split(',').collect();
            let s1 = tpl[0];
            let s2 = tpl[1];

            let s1 = make_set(s1);
            let s2 = make_set(s2);
            let common: HashSet<_> = s1.intersection(&s2).copied().collect();
            if common.len() != 0 {
                cnt += 1;
            }
        });
    cnt
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_make_set() {
        let s1 = "2-4";

        let h = make_set(s1);
        dbg!(&h);
    }

    #[test]
    fn test_intersect() {
        let s1 = HashSet::from([2, 3, 4, 5, 6]);
        let s2 = HashSet::from([3, 4]);
        let s3 = HashSet::from([9, 10]);

        assert_eq!(s2, s1.intersection(&s2).copied().collect());
        assert_eq!(s2, s2.intersection(&s1).copied().collect());
        assert_eq!(HashSet::new(), s1.intersection(&s3).copied().collect());
    }
}