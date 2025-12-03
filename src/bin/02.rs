advent_of_code::solution!(2);

fn parse_range(input: &str) -> Option<std::ops::RangeInclusive<u64>> {
    let (lhs, rhs) = input.split_once('-')?;
    let lhs = lhs.parse::<u64>().ok()?;
    let rhs = rhs.parse::<u64>().ok()?;
    Some(lhs..=rhs)
}

fn divisors_excluding_one(n: usize) -> impl Iterator<Item = usize> {
    (2..=n).filter(move |&i| n.is_multiple_of(i))
}

fn get_invalid_ids<F>(range: &std::ops::RangeInclusive<u64>, pred: F) -> Vec<u64>
where
    F: Fn(&u64) -> bool,
{
    range.clone().filter(pred).collect()
}

fn is_n_repeating_slices(str: &str, slice_count: usize) -> bool {
    debug_assert!(str.len().is_multiple_of(slice_count));
    let slice_len = str.len() / slice_count;
    let first_slice = &str[..slice_len];
    (1..slice_count).all(|i| {
        let start_idx = i * slice_len;
        let end_idx = start_idx + slice_len;
        &str[start_idx..end_idx] == first_slice
    })
}
fn is_two_repeating_slices(id: &u64) -> bool {
    let string = id.to_string();
    string.len().is_multiple_of(2) && is_n_repeating_slices(&string, 2)
}

fn has_any_repeating_slices(id: &u64) -> bool {
    let string = id.to_string();
    divisors_excluding_one(string.len()).any(|n| is_n_repeating_slices(&string, n))
}

fn parse_ranges(input: &str) -> Vec<std::ops::RangeInclusive<u64>> {
    input
        .split(',')
        .map(|range| {
            parse_range(range).expect("can split input by ',' then parse into a valid range")
        })
        .collect()
}
pub fn part_one(input: &str) -> Option<u64> {
    let invalid_id_sum = parse_ranges(input)
        .iter()
        .flat_map(|range| get_invalid_ids(range, is_two_repeating_slices))
        .sum();
    Some(invalid_id_sum)
}

pub fn part_two(input: &str) -> Option<u64> {
    let invalid_id_sum = parse_ranges(input)
        .iter()
        .flat_map(|range| get_invalid_ids(range, has_any_repeating_slices))
        .sum();
    Some(invalid_id_sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1227775554));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4174379265));
    }
}
