use aoc2023::*;
use clap::Parser;

#[derive(Parser)]
struct Cli {
    /// Run only the solution for specified day. If not specified the solutions for all days are run.
    #[arg(short, long, value_parser = clap::value_parser!(u8).range(1..26))]
    day: Option<u8>,
}

fn main() {
    let cli = Cli::parse();
    match cli.day {
        Some(1) => day1::run(),
        Some(2) => day2::run(),
        Some(3) => day3::run(),
        None => {
            day1::run();
            day2::run();
            day3::run();
        }
        Some(d) => println!("day {d} not implemented"),
    }
}
