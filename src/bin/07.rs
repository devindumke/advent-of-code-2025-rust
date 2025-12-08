use std::cmp::PartialEq;
use itertools::Itertools;

advent_of_code::solution!(7);

#[derive(Copy, Clone, PartialEq)]
enum Space
{
    Source,
    Empty,
    Splitter,
}

impl Space {
    fn from(c: char) -> Option<Self> {
        match c {
            'S' => Some(Space::Source),
            '.' => Some(Space::Empty),
            '^' => Some(Space::Splitter),
            _ => None,
        }
    }
}

fn read_line(input: &str) -> Vec<Space> {
    input.trim().chars().filter_map(Space::from).collect()
}

fn read_map(input: &str) -> Vec<Vec<Space>> {
    input.lines().map(read_line).collect()
}

fn apply_new_line(current: &[u64], line: &[Space], next: &mut [u64]) -> u64 {
    debug_assert_eq!(line.len(), current.len());
    debug_assert_eq!(line.len(), next.len());
    next.fill(0);
    let mut split_count = 0;
    for (pos, beam_count) in current.iter().enumerate() {
        if *beam_count == 0 {
            continue;
        }
        match line[pos] {
            Space::Source => panic!(),
            Space::Empty => next[pos] += beam_count,
            Space::Splitter => {
                split_count += 1;
                debug_assert_ne!(pos, 0);
                next[pos - 1] += beam_count;
                debug_assert_ne!(pos, next.len() - 1);
                next[pos + 1] += beam_count;
            }
        }
    }
    split_count
}

fn track_beam_progress(map: &[Vec<Space>]) -> (u64, u64) {
    let width = map[0].len();
    let mut current_line = vec![0; width];
    let mut next = vec![0; width];
    debug_assert_eq!(map[0].iter().filter(|s| **s == Space::Source).count(), 1);
    let source_idx = map[0].iter().find_position(|s| **s == Space::Source).unwrap().0;
    current_line[source_idx] = 1;
    let mut split_beam_count = 0;
    for line in map.iter().skip(1) {
        let splits = apply_new_line(&current_line, line, &mut next);
        split_beam_count += splits;
        // Reuse buffer
        std::mem::swap(&mut current_line, &mut next);
    }
    (split_beam_count, current_line.iter().sum::<u64>())
}

pub fn part_one(input: &str) -> Option<u64> {
    let map = read_map(input);
    debug_assert!(map.iter().map(|l| l.len()).all_equal());
    Some(track_beam_progress(&map).0)
}

pub fn part_two(input: &str) -> Option<u64> {
    let map = read_map(input);
    debug_assert!(map.iter().map(|l| l.len()).all_equal());
    Some(track_beam_progress(&map).1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(21));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(40));
    }
}
