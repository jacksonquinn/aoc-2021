mod day1;
mod day2;
mod shared;

#[macro_use]
extern crate lazy_static;

fn main() {
    let problem = std::env::args().nth(1).expect("no problem name given");
    match problem.as_str() {
        "day1part1" => day1::day1part1(),
        "day1part2" => day1::day1part2(),
        "day2part1" => day2::day2part1(),
        "day2part2" => day2::day2part2(),
        _ => panic!("problem name {} not found", problem),
    }
}
