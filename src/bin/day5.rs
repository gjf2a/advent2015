use std::collections::HashSet;
use std::io;
use advent_code_lib::{all_lines, generic_main};

fn main() -> io::Result<()> {
    generic_main("day5", &["(1|2)"], &[], |args| {
        let part = args[2].as_str();
        let score = if part == "1" {
            all_lines(args[1].as_str())?
                .filter(|line| is_nice(line.as_str()))
                .count()
        } else {
            all_lines(args[1].as_str())?
                .filter(|line| is_nicer(line.as_str()))
                .count()
        };
        println!("Part {}: {}", part, score);
        Ok(())
    })
}

const BAD_SUBSTRINGS: [&'static str; 4] = ["ab", "cd", "pq", "xy"];
const VOWELS: &'static str = "aeiou";

fn is_nice(line: &str) -> bool {
    !BAD_SUBSTRINGS.iter().any(|s| line.contains(s)) &&
        vowel_count(line) >= 3 &&
        doubled_letters(line).len() >= 1
}

fn vowel_count(line: &str) -> usize {
    line.chars().filter(|c| VOWELS.chars().find(|v| v == c).is_some()).count()
}

fn doubled_letters(line: &str) -> HashSet<char> {
    let mut doubled = HashSet::new();
    let mut chars = line.chars();
    let mut current = chars.next().unwrap();
    loop {
        match chars.next() {
            Some(next) => {
                if current == next {
                    doubled.insert(current);
                }
                current = next;
            }
            None => return doubled,
        }
    }
}

fn is_nicer(line: &str) -> bool {
    nicer_double(line) && nicer_between(line)
}

fn nicer_double(line: &str) -> bool {
    for i in (0..line.len()).step_by(2) {
        let pair = &line[i..i+2];
        if line[i+2..].contains(pair) {
            return true;
        }
    }
    false
}

fn nicer_between(line: &str) -> bool {
    let mut chars = line.chars();
    let mut one = chars.next().unwrap();
    let mut two = chars.next().unwrap();
    loop {
        match chars.next() {
            Some(three) => {
                if one == three {
                    return true;
                }
                one = two;
                two = three;
            }
            None => return false
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::is_nicer;

    #[test]
    fn test2() {
        for (test, nice) in [("qjhvhtzxzqqjkmpb", true), ("xxyxx", true), ("uurcxstgmygtbstg", false), ("ieodomkazucvgmuy", false)] {
            let outcome = is_nicer(test);
            println!("{}: expected {}, actually {}", test, nice, outcome);
            assert_eq!(outcome, nice);
        }
    }
}
