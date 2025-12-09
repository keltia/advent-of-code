use std::num::ParseIntError;

use aoc_runner_derive::{aoc, aoc_generator};
use eyre::Result;
use nom::character::complete::{one_of, u32};
use nom::combinator::map_res;
use nom::{IResult, Parser};

#[derive(Copy, Clone, Debug)]
enum Move {
    Left(u32),
    Right(u32),
}

#[derive(Clone, Debug)]
struct Input1 {
    pub data: Vec<Move>,
}

// -----

#[aoc_generator(day1)]
fn input_generator(input: &str) -> Result<Input1> {
    fn parse_rotation(input: &str) -> IResult<&str, Move> {
        let r = |(dir, amount): (char, std::primitive::u32)| -> Result<Move, ParseIntError> {
            match dir {
                'L' => Ok(Move::Left(amount)),
                'R' => Ok(Move::Right(amount)),
                _ => unreachable!("Invalid Move direction: '{}'. Expected 'L' or 'R'.", dir),
            }
        };
        map_res((one_of("LR"), u32), r).parse(input)
    }

    let input = input
        .lines()
        .map(|l| parse_rotation(l).unwrap().1)
        .collect::<Vec<Move>>();
    Ok(Input1 { data: input })
}

// -----

#[aoc(day1, part1)]
fn solve_part1(input: &Input1) -> u32 {
    let mut cnt = 0u32;

    input.data.iter().fold(50, |acc, r| {
        let m = match r {
            Move::Left(amount) => -(*amount as i32),
            Move::Right(amount) => *amount as i32,
        };
        let res = (acc + m) % 100;
        if res == 0 {
            cnt += 1;
        }
        res
    });
    cnt
}

// -----

const SIZE: u32 = 100;

#[derive(Clone, Debug)]
struct Wheel(u32);

#[derive(Clone, Debug)]
struct Turn(u32);

impl Turn {
    pub fn val(self) -> u32 {
        self.0
    }
}

impl Wheel {
    pub fn new(initial: u32) -> Self {
        Self(initial)
    }

    pub fn left(&mut self, amount: u32) -> Turn {
        // Number of full rotations
        let mut fin = amount / SIZE;

        // Look at any possible remainder
        let q = amount % SIZE;

        // is this a full rotation (or multiple thereof)?
        if q == 0 {
            return Turn(fin);
        }

        let res = (self.0 as i32) - q as i32;

        // Final result is 0, count one more.
        if res == 0 {
            self.0 = res as u32;
            return Turn(fin + 1);
        }

        // Store result
        if res < 0 {
            fin = if self.0 == 0 { fin } else { fin + 1 };
            self.0 = (SIZE as i32 + res) as u32;
        } else {
            self.0 = res as u32;
        }
        Turn(fin)
    }

    pub fn right(&mut self, amount: u32) -> Turn {
        // Number of full rotations
        let fin = amount / SIZE;

        // Look at any possible remainder
        let q = amount % SIZE;

        // is this a full rotation (or multiple thereof)?
        if q == 0 {
            return Turn(fin);
        }

        let res = self.0 + q;

        // If we are at 0, we completed a full rotation
        if res == 0 || res >= SIZE {
            self.0 = res % SIZE;
            return Turn(fin + 1);
        }
        self.0 = res;

        Turn(fin)
    }
}

#[aoc(day1, part2)]
fn solve_part2(input: &Input1) -> u32 {
    let mut count = 0;

    let w = Wheel::new(50);
    input.data.iter().fold(w, |mut acc, r| {
        let res = match r {
            Move::Left(amount) => acc.left(*amount),
            Move::Right(amount) => acc.right(*amount),
        };
        count += res.val();
        acc
    });
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample_part1() {
        let s = "L68\nL30\nR48\nL5\nR60\nL55\nL1\nL99\nR14\nL82\n";
        let input = input_generator(s).unwrap();

        assert_eq!(solve_part1(&input), 3);
    }

    #[test]
    fn test_sample_part2() {
        let s = "L68\nL30\nR48\nL5\nR60\nL55\nL1\nL99\nR14\nL82\n";
        let input = input_generator(s).unwrap();

        assert_eq!(solve_part2(&input), 6);
    }

    #[test]
    fn test_turn_value() {
        let turn = Turn(42);
        assert_eq!(turn.val(), 42);
    }

    #[test]
    fn test_wheel_new() {
        let wheel = Wheel::new(50);
        assert_eq!(wheel.pos(), 50);
    }

    #[test]
    fn test_wheel_left_simple() {
        let mut wheel = Wheel::new(50);
        let turn = wheel.left(30);
        assert_eq!(wheel.pos(), 20);
        assert_eq!(turn.val(), 0);
    }

    #[test]
    fn test_wheel_right_simple() {
        let mut wheel = Wheel::new(50);
        let turn = wheel.right(30);
        assert_eq!(wheel.pos(), 80);
        assert_eq!(turn.val(), 0);
    }

    #[test]
    fn test_wheel_left_multiple() {
        let mut wheel = Wheel::new(50);
        let turn = wheel.left(130);
        assert_eq!(wheel.pos(), 20);
        assert_eq!(turn.val(), 1);
    }

    #[test]
    fn test_wheel_right_multiple() {
        let mut wheel = Wheel::new(50);
        let turn = wheel.right(150);
        assert_eq!(wheel.pos(), 0);
        assert_eq!(turn.val(), 2);
    }

    #[test]
    fn test_wheel_zero_movement() {
        let mut wheel = Wheel::new(50);
        let turn = wheel.right(0);
        assert_eq!(wheel.pos(), 50);
        assert_eq!(turn.val(), 0);
    }

    #[test]
    fn test_wheel_full_rotation_left() {
        let mut wheel = Wheel::new(0);
        let turn = wheel.left(100);
        assert_eq!(wheel.pos(), 0);
        assert_eq!(turn.val(), 1);
    }

    #[test]
    fn test_wheel_full_rotation_right() {
        let mut wheel = Wheel::new(0);
        let turn = wheel.right(100);
        assert_eq!(wheel.pos(), 0);
        assert_eq!(turn.val(), 1);
    }
}
