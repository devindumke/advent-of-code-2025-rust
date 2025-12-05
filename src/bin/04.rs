use itertools::Itertools;

advent_of_code::solution!(4);

#[derive(Copy, Clone, PartialEq, Debug)]
enum Location {
    Occupied,
    Empty,
}

impl Location {
    fn from_char(c: char) -> Location {
        match c {
            '@' => Location::Occupied,
            '.' => Location::Empty,
            _ => panic!(),
        }
    }
}

fn line_to_location(line: &str) -> Vec<Location> {
    line.trim().chars().map(Location::from_char).collect()
}
fn read_input(input: &str) -> Vec<Vec<Location>> {
    input.lines().map(line_to_location).collect()
}

fn get_adjacent_locations(all_locations: &[Vec<Location>], x: usize, y: usize) -> Vec<Location> {
    let x_max = (x + 1).min(all_locations.len() - 1);
    let x_min = x.saturating_sub(1);
    let y_min = y.saturating_sub(1);
    let y_max = (y + 1).min(all_locations[0].len() - 1);

    (x_min..=x_max)
        .cartesian_product(y_min..=y_max)
        .map(|(i, j)| all_locations[i][j])
        .collect()
}

fn location_is_removable(adjacent_locations: &[Location]) -> bool {
    adjacent_locations
        .iter()
        .filter(|l| **l == Location::Occupied)
        .count()
        < 5
}

fn get_removable_locations(locations: &[Vec<Location>]) -> Vec<(usize, usize)> {
    let mut ret: Vec<(usize, usize)> = Vec::new();
    for i in 0..locations.len() {
        for j in 0..locations[i].len() {
            if locations[i][j] == Location::Occupied {
                let adjacent_locations = get_adjacent_locations(locations, i, j);
                if location_is_removable(&adjacent_locations) {
                    ret.push((i, j));
                }
            };
        }
    }
    ret
}

fn remove_location(locations: &mut [Vec<Location>], removable_location: &(usize, usize)) {
    let (x, y) = removable_location;
    assert_eq!(locations[*x][*y], Location::Occupied);
    locations[*x][*y] = Location::Empty;
}
fn remove_locations(locations: &mut [Vec<Location>], removable_locations: &[(usize, usize)]) {
    for removable_location in removable_locations {
        remove_location(locations, removable_location);
    }
}
pub fn part_one(input: &str) -> Option<u64> {
    let locations = read_input(input);
    let removable_locations = get_removable_locations(&locations).len();
    Some(removable_locations as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut locations = read_input(input);
    let mut removed_location_count = 0;
    loop {
        let removable_locations = get_removable_locations(&locations);
        if removable_locations.is_empty() {
            return Some(removed_location_count);
        }
        removed_location_count += removable_locations.len() as u64;
        remove_locations(&mut locations, &removable_locations);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(43));
    }
}
