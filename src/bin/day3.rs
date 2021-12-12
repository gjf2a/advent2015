use std::io;
use advent_code_lib::{all_lines, DirType, generic_main, ManhattanDir, Position};
use itertools::Itertools;

fn main() -> io::Result<()> {
    generic_main("day3", &[], &[], |args| {
        let line = all_lines(args[1].as_str())?.next().unwrap();
        println!("Part 1: {}", position_count(&positions_from(line.as_str())));
        println!("Part 2: {}", position_count(&alternating_positions_from(line.as_str())));
        Ok(())
    })
}

fn directions_from(directions: &str) -> impl Iterator<Item=ManhattanDir> + '_ {
    directions.chars().map(|c| match c {
        '<' => ManhattanDir::W,
        '>' => ManhattanDir::E,
        '^' => ManhattanDir::N,
        'v' => ManhattanDir::S,
        bad => panic!("Unrecognized direction: {}", bad)
    })
}

fn positions_from(directions: &str) -> Vec<Position> {
    let p = Position::new();
    let mut result = vec![p];
    for d in directions_from(directions) {
        result.push(*result.last().unwrap() + d.position_offset());
    }
    result
}

fn position_count(positions: &Vec<Position>) -> usize {
    positions.iter().unique().count()
}

fn alternating_positions_from(directions: &str) -> Vec<Position> {
    let p = Position::new();
    let mut counters = vec![p, p];
    let mut result = vec![p];
    for (i, d) in directions_from(directions).enumerate() {
        let s = i % 2;
        counters[s] += d.position_offset();
        result.push(counters[s]);
    }
    result
}

#[cfg(test)]
mod tests {
    use crate::alternating_positions_from;

    #[test]
    fn test2() {
        for dirs in ["^v", "^>v<", "^v^v^v^v^v"] {
            println!("{}", dirs);
            println!("{:?}", alternating_positions_from(dirs));
        }
    }
}
