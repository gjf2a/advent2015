use std::io;
use advent_code_lib::{all_lines, generic_main};

fn main() -> io::Result<()> {
    generic_main("day1", &[], &[], |args| {
        let line = all_lines(args[1].as_str())?.next().unwrap();
        let floors = flooring(line.as_str());
        println!("Part 1: {}", floors.last().unwrap());
        println!("Part 2: {}", floors.iter().enumerate()
            .find(|(_, f)| **f < 0)
            .map(|(i, _)| i)
            .unwrap());
        Ok(())
    })
}

fn flooring(line: &str) -> Vec<isize> {
    let mut floors = vec![0];
    for c in line.chars() {
        floors.push(floors.last().unwrap() +
        match c {
            '(' => 1,
            ')' => -1,
            _ => 0
        });
    }
    floors
}
