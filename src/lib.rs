use aoc_runner_derive::{aoc, aoc_generator, aoc_lib};
use itertools::{Itertools,max};

struct Elf {
    id: u32,
    cal: u32,
}

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
        .inspect(|(n, sum)| println!("{}", sum))
        .filter_map(|(n, sum)| if n == 0 { Some(sum)} else { None})
        .inspect(|sum| println!("{}", sum))
        .collect()
}

#[aoc(day1, part1)]
pub fn solve_part1(input: &[u32]) -> u32 {
    *input
        .iter()
        .max()
        .unwrap()
}

aoc_lib!{ year = 2022 }
