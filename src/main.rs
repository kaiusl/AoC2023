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

    macro_rules! match_day {
        ($($num:literal =>$day:ident),*) => {
            match cli.day {
                $(Some($num) => $day::run(),)*
                None => {
                    $(
                        $day::run();
                    )*
                }
                Some(d) => println!("day {d} not implemented"),
            }
        }
    }
    match_day!(1 => day1, 2 => day2, 3 => day3, 4 => day4, 5 => day5, 6 => day6);
}
