use aoc_runner_derive::{aoc, aoc_generator};
use itertools::{Itertools, max};

#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> Vec<u32> {
    let mut cal = 0;
    let mut cnt = 0;
    input
        .lines()
        .map(|l| {
            match l {
                "" => {
                    let cur = cal;
                    cal = 0;
                    cnt += 1;
                    (0, cur)
                },
                _ => {
                    cal += l.parse::<u32>().unwrap();
                    (cnt, cal)
                },
            }
        })
        .filter_map(|(n, sum)| if n == 0 { Some(sum)} else { None})
        .collect()
}

#[aoc(day1, part1, max)]
pub fn solve_part1(input: &[u32]) -> u32 {
    *input
        .iter()
        .max()
        .unwrap()
}

#[aoc(day1, part1, sort_last)]
pub fn solve_part1_sort(input: &[u32]) -> u32 {
    let list: Vec<_> = input.iter().sorted().collect();
    *list[input.len() - 1]
}

#[aoc(day1, part2, fold)]
pub fn solve_part2(input: &[u32]) -> u32 {
    let list: Vec<_> = input.iter().sorted().collect();
    let sum = [1, 2, 3]
        .iter()
        .map(|v| list[input.len() - *v])
        .fold(0, |sum, &x| sum + x);
    sum
}

#[aoc(day1, part2, sum)]
pub fn solve_part2_sum(input: &[u32]) -> u32 {
    let list: Vec<_> = input.iter().sorted().collect();
    let sum = [1, 2, 3]
        .iter()
        .map(|v| list[input.len() - *v])
        .sum();
    sum
}
