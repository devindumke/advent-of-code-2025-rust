advent_of_code::solution!(3);

fn line_to_bank(line: &str) -> Vec<u8> {
    line.chars()
        .filter_map(|c| c.to_digit(10).map(|d| d as u8))
        .collect()
}

fn get_max_index_and_element(subview: &[u8]) -> (usize, &u8) {
    subview
        .iter()
        .enumerate()
        .rev()
        .max_by_key(|&(_, &val)| val)
        .expect("at least one element in slice")
}

fn combine_digits<const JOLTAGE_DIGITS: usize>(digits: &[u8; JOLTAGE_DIGITS]) -> u64 {
    digits
        .iter()
        .fold(0u64, |sum, digit| (sum * 10) + *digit as u64)
}

fn calculate_max_joltage<const JOLTAGE_DIGITS: usize>(bank: &[u8]) -> u64 {
    assert!(
        bank.len() >= JOLTAGE_DIGITS,
        "Bank doesn't have enough digits"
    );
    let mut left_bound = 0;
    let mut digits: [u8; JOLTAGE_DIGITS] = [0; JOLTAGE_DIGITS];
    for (i, value) in digits.iter_mut().enumerate() {
        let remaining_needed = JOLTAGE_DIGITS - i;
        let right_bound = bank.len() - (remaining_needed - 1);
        let window: &[u8] = &bank[left_bound..right_bound];
        let (next_digit_idx, next_digit) = get_max_index_and_element(window);
        left_bound += next_digit_idx + 1;
        *value = *next_digit;
    }
    combine_digits::<JOLTAGE_DIGITS>(&digits)
}

pub fn part_one(input: &str) -> Option<u64> {
    let sum = input
        .lines()
        .map(line_to_bank)
        .map(|bank| calculate_max_joltage::<2>(&bank))
        .sum();
    Some(sum)
}

pub fn part_two(input: &str) -> Option<u64> {
    let sum = input
        .lines()
        .map(line_to_bank)
        .map(|bank| calculate_max_joltage::<12>(&bank))
        .sum();
    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(357));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3121910778619));
    }
}
