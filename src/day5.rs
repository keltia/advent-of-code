use std::collections::VecDeque;

use aoc_runner_derive::{aoc, aoc_generator};
use array_macro::array;
use itertools::{enumerate, Itertools};
use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct Line(VecDeque<char>);

#[derive(Debug)]
pub struct Start {
    lines: [Line; 9],
}

/// next letter position is n + 4 for "[x] "
///
const POS: [usize; 9] = [1, 5, 9, 13, 17, 21, 25, 29, 33];

#[aoc_generator(day5)]
pub fn parse_data(input: &str) -> String {
    let mut lines = input.lines();
    let slines = array![Line(VecDeque::<char>::new()); 9];
    let mut start = Start { lines: slines };
    dbg!(&start);

    loop {
        // Stop at first empty line
        //
        let line = match lines.next() {
            Some("") => break,
            Some(line) => {
                // Special exit line
                //
                if line.starts_with(" 1   2   3") {
                    break;
                }
                line
            }
            _ => panic!("bad input"),
        };

        // No check at each interesting position if we have something.
        //
        POS.iter().enumerate().map(|(n, &i)| match line.get(i) {
            ' ' => (),
            _ => {
                let c = line.get(i).unwrap();
                start.lines[n].0.push_front(c);
            }
        });
        dbg!(&start);
    }
    0.to_string()
}

#[aoc(day5, part1)]
fn solver_part1(input: &str) -> u32 {
    0
}
