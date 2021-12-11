use crate::shared::read_lines;
use crate::shared::AOCParseError;
use std::str::FromStr;

use regex::Regex;

const INPUT_FILE: &str = "./inputs/day2.txt";

lazy_static! {
    static ref COMMAND_RE: Regex =
        Regex::new(r"^(?P<direction>forward|up|down) (?P<value>\d{1,2})$").unwrap();
}

enum SubCommand {
    Forward(u8),
    Down(u8),
    Up(u8),
}

impl FromStr for SubCommand {
    type Err = AOCParseError;
    fn from_str(s: &str) -> Result<Self, AOCParseError> {
        let cap = COMMAND_RE.captures(s);
        if let Some(c) = cap {
            let direction = &c["direction"];
            if let Ok(value) = c["value"].parse::<u8>() {
                return match direction {
                    "forward" => Ok(SubCommand::Forward(value)),
                    "down" => Ok(SubCommand::Down(value)),
                    "up" => Ok(SubCommand::Up(value)),
                    _ => Err(AOCParseError),
                };
            }
        }

        Err(AOCParseError)
    }
}

fn parse_input() -> Vec<SubCommand> {
    read_lines(INPUT_FILE)
        .expect("Unable to read input file")
        .filter_map(|s| SubCommand::from_str(&s).ok())
        .collect()
}

pub fn day2part1() {
    let commands = parse_input();
    let mut x: i32 = 0;
    let mut y: i32 = 0;
    commands.iter().for_each(|cmd| match cmd {
        SubCommand::Forward(value) => {
            x += *value as i32;
        }
        SubCommand::Down(value) => y += *value as i32,
        SubCommand::Up(value) => y -= *value as i32,
    });

    println!(
        "Day1 Part2 Solution: position={}, depth{}, answer={}",
        x,
        y,
        x * y
    );
}

pub fn day2part2() {
    let commands = parse_input();
    let mut x: i32 = 0;
    let mut y: i32 = 0;
    let mut aim: i32 = 0;
    commands.iter().for_each(|cmd| match cmd {
        SubCommand::Forward(value) => {
            x += *value as i32;
            y += aim * *value as i32
        }
        SubCommand::Down(value) => aim += *value as i32,
        SubCommand::Up(value) => aim -= *value as i32,
    });

    println!(
        "Day1 Part2 Solution: position={}, depth={}, aim={}, answer={}",
        x,
        y,
        aim,
        x * y
    );
}
