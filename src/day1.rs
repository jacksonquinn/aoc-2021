use crate::shared::read_lines;

const INPUT_FILE: &str = "./inputs/day1.txt";

pub fn parse_input() -> Vec<u16> {
    read_lines(INPUT_FILE)
        .expect("Unable to read input file")
        .filter_map(|line| line.parse::<u16>().ok())
        .collect()
}

pub fn day1part1() {
    let depths = parse_input();
    let increases = depths
        .windows(2)
        .filter(|x| x[0] < x[1])
        .count();

    println!("Day1 Part1 Solution: {}", increases);
}

pub fn day1part2() {
    let depths = parse_input();
    let increases = depths
        .windows(4)
        .filter(|x| (x[0] + x[1] + x[2]) < (x[1] + x[2] + x[3]))
        .count();

    println!("Day1 Part2 Solution: {}", increases);
}