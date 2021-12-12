use std::io;
use advent_code_lib::{all_lines, generic_main};

fn main() -> io::Result<()> {
    generic_main("day2", &["(1|2)"], &[], |args| {
        let sides = sides(args[1].as_str())?;
        let part = args[2].as_str();
        let result = match part {
            "1" => sides.map(|(l, w, h)| vec![l*w, w*h, h*l])
                .map(|sides| 2 * sides.iter().sum::<usize>() + sides.iter().min().unwrap())
                .sum::<usize>(),
            "2" => sides.map(|(l, w, h)| {
                let mut all = vec![l, w, h];
                all.sort();
                2 * (all[0] + all[1]) + (l*w*h)})
                .sum::<usize>(),
            bad => panic!("Unrecognized option: {}", bad)
        };
        println!("Part {} score: {}", part, result);
        Ok(())
    })
}

fn sides(filename: &str) -> io::Result<impl Iterator<Item=(usize,usize,usize)>> {
    Ok(all_lines(filename)?
        .map(|line| line.split('x')
            .map(|d| d.parse::<usize>().unwrap())
            .collect::<Vec<usize>>())
        .map(|nums| (nums[0], nums[1], nums[2])))
}