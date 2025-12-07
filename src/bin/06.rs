use std::str::FromStr;

advent_of_code::solution!(6);

#[derive(Clone, Copy)]
enum Operator {
    Multiply,
    Add,
}

impl FromStr for Operator {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim() {
            "+" => Ok(Operator::Add),
            "*" => Ok(Operator::Multiply),
            _ => Err(()),
        }
    }
}

fn read_part1(input: &str) -> (Vec<Operator>, Vec<Vec<u64>>) {
    let mut numbers = Vec::new();
    for line in input.lines() {
        let numbers_line: Vec<u64> = line
            .trim()
            .split(" ")
            .filter_map(|x| x.parse().ok())
            .collect();
        if numbers_line.is_empty() {
            let operators: Vec<Operator> = line
                .trim()
                .split(" ")
                .filter_map(|x| x.parse().ok())
                .collect();
            return (operators, numbers);
        }
        numbers.push(numbers_line);
    }
    panic!();
}

fn get_nth_vertical_line(idx: usize, lines: &Vec<&str>) -> String {
    lines
        .iter()
        .filter_map(|line| line.chars().nth(idx))
        .collect::<String>()
}

fn get_vertical_lines(input: &str) -> Vec<String> {
    let lines: Vec<&str> = input.lines().collect();
    let longest_line_length = lines.iter().map(|x| x.len()).max().unwrap();
    (0..longest_line_length)
        .rev()
        .map(|idx| get_nth_vertical_line(idx, &lines))
        .collect()
}

fn parse_vertical(input: &str) -> (u64, Option<Operator>) {
    let last_char = input.chars().next_back();
    match last_char.expect("A non-empty input") {
        '+' => (
            input[0..input.len().saturating_sub(1)]
                .trim()
                .parse()
                .unwrap(),
            Some(Operator::Add),
        ),
        '*' => (
            input[0..input.len().saturating_sub(1)]
                .trim()
                .parse()
                .unwrap(),
            Some(Operator::Multiply),
        ),
        _ => (input.trim().parse().unwrap(), None),
    }
}
fn read_part2(lines: &[String]) -> Vec<Problem> {
    let mut problems = Vec::new();
    let mut numbers = Vec::new();
    for line in lines {
        if line.trim().is_empty() {
            continue;
        }
        let (number, operator) = parse_vertical(line);
        numbers.push(number);
        if let Some(operator) = operator {
            problems.push(Problem {
                numbers: std::mem::take(&mut numbers),
                operator,
            });
        }
    }
    problems
}

struct Problem {
    numbers: Vec<u64>,
    operator: Operator,
}

impl Problem {
    fn solve(&self) -> u64 {
        match self.operator {
            Operator::Add => self.numbers.iter().sum(),
            Operator::Multiply => self.numbers.iter().product(),
        }
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let (operators, all_numbers) = read_part1(input);
    assert_eq!(operators.len(), all_numbers[0].len());
    assert!(!all_numbers.is_empty());
    let sum = (0..operators.len())
        .map(|idx| {
            let numbers = all_numbers.iter().map(|row| row[idx]).collect::<Vec<u64>>();
            let operator = operators[idx];
            let problem = Problem { numbers, operator };
            problem.solve()
        })
        .sum();
    Some(sum)
}

pub fn part_two(input: &str) -> Option<u64> {
    let lines = get_vertical_lines(input);
    let problems = read_part2(&lines);
    let sum = problems.iter().map(Problem::solve).sum();
    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4277556));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3263827));
    }
}
