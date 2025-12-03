advent_of_code::solution!(2);

struct Range {
    start: u64,
    end: u64,
}

impl Range {
    fn from(input: &str) -> Option<Self> {
        let values: Vec<&str> = input.split('-').collect();
        if values.len() != 2 {
            return None;
        }
        let start = values[0].parse::<u64>().ok()?;
        let end = values[1].parse::<u64>().ok()?;
        Some(Range { start, end })
    }

    fn iter(&self) -> RangeIter {
        RangeIter {
            current: self.start,
            end: self.end,
        }
    }
}

struct RangeIter {
    current: u64,
    end: u64,
}

impl Iterator for RangeIter {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current > self.end {
            return None;
        }
        let value = self.current;
        self.current += 1;
        Some(value)
    }
}

struct UniqueFactors {
    n: usize,
    current: usize,
}

impl UniqueFactors {
    fn new(n: usize) -> Self {
        UniqueFactors { n, current: 1 }
    }
}

impl Iterator for UniqueFactors {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if self.current > self.n {
                return None;
            }
            self.current += 1;
            if self.n.is_multiple_of(self.current) {
                return Some(self.current);
            }
        }
    }
}

fn get_invalid_ids<F>(range: &Range, pred: F) -> Vec<u64>
where
    F: Fn(&u64) -> bool,
{
    range.iter().filter(pred).collect()
}

fn is_n_repeating_slices(str: &str, slice_count: usize) -> bool {
    debug_assert_eq!(str.len() % slice_count, 0);
    let slice_len = str.len() / slice_count;
    let slices: Vec<&str> = (0..slice_count)
        .map(|i| {
            let start_idx = i * slice_len;
            let end_idx = start_idx + slice_len;
            &str[start_idx..end_idx]
        })
        .collect();
    slices.iter().all(|s| *s == slices[0])
}
fn is_two_repeating_slices(id: &u64) -> bool {
    let string = id.to_string();
    string.len() % 2 == 0 && is_n_repeating_slices(&string, 2)
}

fn has_any_repeating_slices(id: &u64) -> bool {
    let string = id.to_string();
    UniqueFactors::new(string.len()).any(|n| is_n_repeating_slices(&string, n))
}

fn parse_ranges(input: &str) -> Vec<Range> {
    input
        .split(',')
        .map(|range| {
            Range::from(range).expect("can split input by ',' then parse into a valid range")
        })
        .collect()
}
pub fn part_one(input: &str) -> Option<u64> {
    let ranges = parse_ranges(input);
    let invalid_id_sum = ranges
        .iter()
        .flat_map(|range| get_invalid_ids(range, is_two_repeating_slices))
        .sum();
    Some(invalid_id_sum)
}

pub fn part_two(input: &str) -> Option<u64> {
    let ranges = parse_ranges(input);
    let invalid_id_sum = ranges
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
