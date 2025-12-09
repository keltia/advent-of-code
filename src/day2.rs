use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Debug)]
struct Intv(u64, u64);

#[derive(Debug)]
struct Input {
    pub data: Vec<Intv>,
}

// -----

#[aoc_generator(day2)]
fn input_generator(input: &str) -> eyre::Result<Input> {
    let input = input
        .split(',')
        .collect::<Vec<&str>>()
        .iter()
        .map(|&s| {
            let intv = s.split('-').collect::<Vec<_>>();
            let low = intv[0].parse::<u64>().unwrap();
            let high = intv[1].parse::<u64>().unwrap();
            Intv(low, high)
        })
        .collect::<Vec<Intv>>();
    Ok(Input { data: input })
}

// -----

#[aoc(day2, part1)]
fn solver_part1(input: &Input) -> u64 {
    let mut sum = 0;

    for intv in &input.data {
        let low = intv.0;
        let high = intv.1;
        sum += check_invalid(low, high);
    }
    sum
}

fn check_invalid(low: u64, high: u64) -> u64 {
    let mut sum = 0;

    for i in low..=high {
        let str = i.to_string();

        // we look for an odd number of digits, if yes, it is valid.
        if str.len() % 2 == 1 {
            continue;
        }

        // now, split in two
        let (a, b) = str.split_at(str.len() / 2);
        if a == b {
            sum += i;
        }
    }
    sum
}

// -----

#[aoc(day2, part2)]
fn solver_part2(input: &Input) -> u64 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check_invalid_no_valid() {
        // Range 1-9 all have odd number of digits (1), so none should be valid
        assert_eq!(check_invalid(1, 9), 0);
    }

    #[test]
    fn test_check_invalid_with_valid() {
        // 1111 has 4 digits, splits into "11" and "11" which match
        // Sum should be 1111
        assert_eq!(check_invalid(1111, 1111), 1111);
    }

    #[test]
    fn test_check_invalid_range_mixed() {
        // Range 10-12: all 2 digits
        // 10: "1" != "0" - invalid
        // 11: "1" == "1" - valid, sum = 11
        // 12: "1" != "2" - invalid
        assert_eq!(check_invalid(10, 12), 11);
    }

    #[test]
    fn test_check_invalid_four_digits() {
        // 1212: "12" != "12" would be equal - sum = 1212
        // 1221: "12" != "21" - invalid
        assert_eq!(check_invalid(1212, 1212), 1212);
    }

    #[test]
    fn test_check_invalid_zero() {
        // 0 has 1 digit (odd), should be skipped
        assert_eq!(check_invalid(0, 0), 0);
    }

    #[test]
    fn test_check_invalid_larger_range() {
        // Testing range with multiple valid numbers
        // Valid: 11, 22, 33, 44, 55, 66, 77, 88, 99
        // Sum = 11+22+33+44+55+66+77+88+99 = 495
        assert_eq!(check_invalid(10, 99), 495);
    }
}
