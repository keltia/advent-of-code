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
    input
        .data
        .iter()
        .fold(0u64, |acc, intv| acc + check_invalid_gemini3(intv))
}

fn check_invalid(intv: &Intv) -> u64 {
    let mut sum = 0;
    let low = intv.0;
    let high = intv.1;

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

// Alternate version proposed by Gemini Pro 3
//
fn check_invalid_gemini3(intv: &Intv) -> u64 {
    let low = intv.0;
    let high = intv.1;
    let mut sum = 0;

    // Valid numbers must have even length. We construct them mathematically.
    // A number "xyxy" is effectively "xy" * 100 + "xy" = "xy" * 101.
    // We iterate over the "base" magnitude (10, 100, 1000...).
    let mut base = 10u64;

    loop {
        // The multiplier to create the repeated number (e.g., 10^1 + 1 = 11)
        let multiplier = base + 1;

        // If the smallest possible valid number (1 * multiplier) is larger than high, we're done.
        if multiplier > high {
            break;
        }

        // The "half" number range for this base (e.g., for base 10, halves are 1..9)
        let min_half = base / 10;
        let max_half = base - 1;

        // Calculate the subset of halves that produce numbers within [low, high]
        // 1. half * multiplier >= low  =>  half >= ceil(low / multiplier)
        let start_half = (low + multiplier - 1) / multiplier;
        // 2. half * multiplier <= high =>  half <= floor(high / multiplier)
        let end_half = high / multiplier;

        // Intersect with the valid range for the current digit length
        let actual_start = min_half.max(start_half);
        let actual_end = max_half.min(end_half);

        if actual_start <= actual_end {
            // We need to sum: (h * multiplier) for h in [actual_start, actual_end]
            // Sum = multiplier * (sum of integers from actual_start to actual_end)
            // Sum of range [A, B] = (A + B) * count / 2

            // Use u128 to strictly avoid overflow during intermediate calculation
            let count = (actual_end - actual_start + 1) as u128;
            let sum_halves = (actual_start as u128 + actual_end as u128) * count / 2;

            sum += (sum_halves * multiplier as u128) as u64;
        }

        // Move to next even length (e.g., 2 digits -> 4 digits)
        // base goes 10 -> 100 -> 1000 ...
        match base.checked_mul(10) {
            Some(next) => base = next,
            None => break,
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
        assert_eq!(check_invalid(&Intv(1, 9)), 0);
    }

    #[test]
    fn test_check_invalid_with_valid() {
        // 1111 has 4 digits, splits into "11" and "11" which match
        // Sum should be 1111
        assert_eq!(check_invalid(&Intv(1111, 1111)), 1111);
    }

    #[test]
    fn test_check_invalid_range_mixed() {
        // Range 10-12: all 2 digits
        // 10: "1" != "0" - invalid
        // 11: "1" == "1" - valid, sum = 11
        // 12: "1" != "2" - invalid
        assert_eq!(check_invalid(&Intv(10, 12)), 11);
    }

    #[test]
    fn test_check_invalid_four_digits() {
        // 1212: "12" != "12" would be equal - sum = 1212
        // 1221: "12" != "21" - invalid
        assert_eq!(check_invalid(&Intv(1212, 1212)), 1212);
    }

    #[test]
    fn test_check_invalid_zero() {
        // 0 has 1 digit (odd), should be skipped
        assert_eq!(check_invalid(&Intv(0, 0)), 0);
    }

    #[test]
    fn test_check_invalid_larger_range() {
        // Testing range with multiple valid numbers
        // Valid: 11, 22, 33, 44, 55, 66, 77, 88, 99
        // Sum = 11+22+33+44+55+66+77+88+99 = 495
        assert_eq!(check_invalid(&Intv(10, 99)), 495);
    }
}
