use crate::shared::read_lines;

const INPUT_FILE: &str = "./inputs/day3.txt";

fn parse_input() -> Vec<String> {
    read_lines(INPUT_FILE)
        .expect("Unable to read input file")
        .collect()
}

pub fn day3part1() {
    let input = parse_input();
    let bit_str_len = input.first().expect("Day 3 input is empty").len();
    let mut pop_count = vec![0; bit_str_len];

    for line in &input {
        for (idx, bit) in line.chars().enumerate() {
            if bit == '1' {
                pop_count[idx] += 1;
            }
        }
    }

    let common_bits = pop_count
        .iter()
        .map(|count| if *count >= input.len() / 2 { 1 } else { 0 });

    let mut gamma: usize = 0;
    let mut epsilon: usize = 0;
    for (idx, bit) in common_bits.enumerate() {
        gamma |= bit << (bit_str_len - idx - 1);
        epsilon |= (bit ^ 1) << (bit_str_len - idx - 1)
    }

    println!(
        "Day3 Part1 Solution: gamma={}, epislon={}, gamma*epsilon={}",
        gamma,
        epsilon,
        gamma * epsilon
    );
}
