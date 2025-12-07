use std::str::FromStr;

advent_of_code::solution!(5);

struct Ingredient {
    id: u64,
}

impl FromStr for Ingredient {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let id = s.trim().parse().map_err(|_| ())?;
        Ok(Self { id })
    }
}

#[derive(Copy, Clone)]
struct Range {
    start: u64,
    end: u64,
}

impl FromStr for Range {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (lhs, rhs) = s.split_once('-').ok_or(())?;
        let start = lhs.parse().map_err(|_| ())?;
        let end = rhs.parse().map_err(|_| ())?;
        if start > end {
            return Err(());
        }
        Ok(Self { start, end })
    }
}

impl Range {
    fn contains(&self, x: u64) -> bool {
        self.start <= x && x <= self.end
    }

    fn get_count(&self) -> u64 {
        self.end - self.start + 1
    }

    fn can_join(lhs: &Range, rhs: &Range) -> bool {
        lhs.start <= rhs.end && rhs.start <= lhs.end
    }
}

fn get_ingredients(input: &str) -> Vec<Ingredient> {
    input
        .split("\n\n")
        .nth(1)
        .expect("A double new line separating the ranges from the ingredient IDs")
        .lines()
        .filter_map(|l| l.parse().ok())
        .collect()
}

fn get_fresh_ranges(input: &str) -> Vec<Range> {
    input
        .split("\n\n")
        .next()
        .expect("A double new line separating the ranges from the ingredient IDs")
        .lines()
        .filter_map(|l| l.parse().ok())
        .collect()
}

fn is_fresh(ingredient: &Ingredient, ranges: &[Range]) -> bool {
    ranges.iter().any(|r| r.contains(ingredient.id))
}

fn sort_ranges(ranges: &mut [Range]) {
    ranges.sort_by(|r1, r2| r1.start.cmp(&r2.start));
}

fn compress_ranges(ranges: &mut Vec<Range>) {
    assert!(!ranges.is_empty());
    let mut compact = Vec::with_capacity(ranges.len());
    compact.push(ranges[0]);
    for r in &ranges[1..] {
        let last = compact.last_mut().unwrap();
        if Range::can_join(last, r) {
            last.end = last.end.max(r.end);
        } else {
            compact.push(*r);
        }
    }
    *ranges = compact;
}

pub fn part_one(input: &str) -> Option<u64> {
    let ranges = get_fresh_ranges(input);
    let ingredients = get_ingredients(input);
    Some(ingredients.iter().filter(|i| is_fresh(i, &ranges)).count() as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut ranges: Vec<Range> = get_fresh_ranges(input);
    sort_ranges(&mut ranges);
    compress_ranges(&mut ranges);
    let valid_id_count: u64 = ranges.iter().map(|r| r.get_count()).sum();
    Some(valid_id_count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }
}
