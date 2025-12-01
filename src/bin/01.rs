advent_of_code::solution!(1);

#[derive(Debug)]
enum Rotation {
    Left(u16),
    Right(u16),
}

impl Rotation {
    fn from(input: &str) -> Option<Rotation> {
        match input.chars().next() {
            Some('L') => input[1..].parse::<u16>().ok().map(Rotation::Left),
            Some('R') => input[1..].parse::<u16>().ok().map(Rotation::Right),
            _ => None,
        }
    }
}

#[derive(Debug)]
struct Lock {
    // invariant that position will always be between 0-99
    position: u8,
    // number of times that the lock came to rest at 0 after applying a rotation
    zero_resting_count: u64,
    // number of times that the lock passed 0 while applying a rotation
    zero_pass_count: u64,
}

impl Lock {
    fn new() -> Lock {
        Lock {
            position: 50,
            zero_resting_count: 0,
            zero_pass_count: 0,
        }
    }

    fn apply_rotation(&mut self, rotation: &Rotation) {
        let unbounded_position: i32 = match rotation {
            Rotation::Left(n) => self.position as i32 - *n as i32,
            Rotation::Right(n) => self.position as i32 + *n as i32,
        };
        match unbounded_position {
            v if v >= 100 => {
                self.zero_pass_count += (unbounded_position / 100) as u64;
                self.position = (unbounded_position % 100) as u8;
                if self.position == 0 {
                    self.zero_resting_count += 1;
                }
            }
            v if v < 0 => {
                if self.position != 0 {
                    self.zero_pass_count += 1;
                }
                self.zero_pass_count += (unbounded_position.abs() / 100) as u64;
                self.position = unbounded_position.rem_euclid(100) as u8;
                if self.position == 0 {
                    self.zero_resting_count += 1;
                }
            }
            0 => {
                self.position = 0;
                self.zero_resting_count += 1;
                self.zero_pass_count += 1;
            }
            _ => {
                self.position = unbounded_position as u8;
            }
        };
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut lock = Lock::new();
    for line in input.lines() {
        let rotation = Rotation::from(line).expect("Each line can be converted to a rotation");
        lock.apply_rotation(&rotation);
    }
    Some(lock.zero_resting_count)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut lock = Lock::new();
    for line in input.lines() {
        let rotation = Rotation::from(line).expect("Each line can be converted to a rotation");
        lock.apply_rotation(&rotation);
    }
    Some(lock.zero_pass_count)
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
        assert_eq!(result, Some(6));
    }
}
