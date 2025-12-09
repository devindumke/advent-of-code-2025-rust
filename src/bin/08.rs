use itertools::Itertools;
use ordered_float::NotNan;
use std::str::FromStr;

advent_of_code::solution!(8);

#[derive(Eq, PartialEq, Copy, Clone)]
struct Point {
    x: u32,
    y: u32,
    z: u32,
}

impl FromStr for Point {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.trim().split(',');
        let parse = |p: Option<&str>| p.ok_or(())?.parse().map_err(|_| ());
        let x = parse(parts.next())?;
        let y = parse(parts.next())?;
        let z = parse(parts.next())?;
        Ok(Self { x, y, z })
    }
}

#[derive(Copy, Clone)]
struct Edge {
    from: Point,
    to: Point,
    distance: NotNan<f64>,
}

impl Edge {
    fn from(from: &Point, to: &Point) -> Self {
        let dx = from.x as f64 - to.x as f64;
        let dy = from.y as f64 - to.y as f64;
        let dz = from.z as f64 - to.z as f64;
        let distance =
            unsafe { NotNan::new_unchecked((dx.powi(2) + dy.powi(2) + dz.powi(2)).sqrt()) };
        Self {
            from: *from,
            to: *to,
            distance,
        }
    }
}

struct Circuit {
    points: Vec<Point>,
}

impl Circuit {
    fn from(point: Point) -> Self {
        Self {
            points: vec![point],
        }
    }

    fn has_point(&self, point: &Point) -> bool {
        self.points.iter().any(|p| p == point)
    }

    fn combine(left: Circuit, right: Circuit) -> Self {
        let mut points = Vec::with_capacity(left.points.len() + right.points.len());
        points.extend(left.points.iter().cloned());
        points.extend(right.points.iter().cloned());
        Self { points }
    }
}

fn remove_and_return(circuits: &mut Vec<Circuit>, point: &Point) -> Circuit {
    assert_eq!(circuits.iter().filter(|c| c.has_point(point)).count(), 1);
    let pos = circuits
        .iter()
        .position(|c| c.has_point(point))
        .expect("The point to be present in a circuit");
    circuits.remove(pos)
}

#[cfg(test)]
const CONNECTIONS: usize = 10;

#[cfg(not(test))]
const CONNECTIONS: usize = 1000;

pub fn part_one(input: &str) -> Option<u64> {
    let points: Vec<Point> = input
        .trim()
        .lines()
        .filter_map(|l| l.parse().ok())
        .collect();
    let mut edges: Vec<Edge> = points
        .iter()
        .combinations(2)
        .map(|c| Edge::from(c[0], c[1]))
        .collect();
    edges.sort_unstable_by_key(|e| e.distance);
    let mut circuits: Vec<Circuit> = points.iter().map(|p| Circuit::from(*p)).collect();
    for edge in edges.iter().take(CONNECTIONS) {
        let from_circuit = remove_and_return(&mut circuits, &edge.from);
        if from_circuit.has_point(&edge.to) {
            // Do nothing, edge is already in circuit
            circuits.push(from_circuit);
            continue;
        } else {
            let to_circuit = remove_and_return(&mut circuits, &edge.to);
            let combined_circuit = Circuit::combine(from_circuit, to_circuit);
            circuits.push(combined_circuit);
        }
    }
    let mut circuit_size: Vec<_> = circuits.iter().map(|c| c.points.len() as u64).collect();
    circuit_size.sort_unstable();
    let answer = circuit_size.iter().rev().take(3).product();
    Some(answer)
}

pub fn part_two(input: &str) -> Option<u64> {
    let points: Vec<Point> = input
        .trim()
        .lines()
        .filter_map(|l| l.parse().ok())
        .collect();
    let mut edges: Vec<Edge> = points
        .iter()
        .combinations(2)
        .map(|c| Edge::from(c[0], c[1]))
        .collect();
    edges.sort_unstable_by_key(|e| e.distance);
    let mut circuits: Vec<Circuit> = points.iter().map(|p| Circuit::from(*p)).collect();
    for edge in edges.iter() {
        let from_circuit = remove_and_return(&mut circuits, &edge.from);
        if from_circuit.has_point(&edge.to) {
            // Do nothing, edge is already in circuit
            circuits.push(from_circuit);
            continue;
        } else {
            let to_circuit = remove_and_return(&mut circuits, &edge.to);
            let combined_circuit = Circuit::combine(from_circuit, to_circuit);
            circuits.push(combined_circuit);
            if circuits.len() == 1 {
                return Some(edge.from.x as u64 * edge.to.x as u64);
            }
        }
    }
    panic!("The circuit should not be empty");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(40));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(25272));
    }
}
