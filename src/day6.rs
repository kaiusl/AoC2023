use core::num;

const DAY: u8 = 6;
pub const INPUT: &str = include_str!("../inputs/day6.txt");

pub fn run() {
    solve(INPUT);
}

fn solve(input: &str) {
    let answer_part1 = solve_part1(input);
    println!("day{DAY}::part1 answer: {}", answer_part1);

    let answer_part2 = solve_part2(input);
    println!("day{DAY}::part2 answer: {}", answer_part2);
}

fn solve_part1(input: &str) -> u64 {
    let races = parse_input_part1(input);
    races.iter().map(Race::num_ways_to_win).product()
}

fn solve_part2(input: &str) -> u64 {
    let race = parse_input_part2(input);
    race.num_ways_to_win()
}

fn parse_input_part1(input: &str) -> Vec<Race> {
    let (times, dist) = input.split_once('\n').unwrap();
    let times = times
        .trim_start_matches("Time:")
        .split(' ')
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .map(|s| s.trim().parse::<u64>().unwrap());
    let dist = dist
        .trim_start_matches("Distance:")
        .split(' ')
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .map(|s| s.trim().parse::<u64>().unwrap());

    times
        .zip(dist)
        .map(|(time, distance_record)| Race {
            time,
            distance_record,
        })
        .collect()
}

fn parse_input_part2(input: &str) -> Race {
    let (times, dist) = input.split_once('\n').unwrap();
    let times = times
        .trim_start_matches("Time:")
        .trim()
        .replace(' ', "")
        .parse::<u64>()
        .unwrap();
    let dist = dist
        .trim_start_matches("Distance:")
        .trim()
        .replace(' ', "")
        .parse::<u64>()
        .unwrap();

    Race {
        time: times,
        distance_record: dist,
    }
}

struct Race {
    time: u64,
    distance_record: u64,
}

impl Race {
    fn race_distance(&self, hold_button_time: u64) -> u64 {
        if hold_button_time >= self.time {
            return 0;
        }

        hold_button_time * (self.time - hold_button_time)
    }

    /// Calculate the minimum and maximum time that the button was held for distance record
    fn record_hold_times(&self) -> (f64, f64) {
        // distance is quadratic formula in button hold time
        // ht^2 - ht * t + d = 0
        // hence ht = t/2 +- sqrt( t^2/4 - d )

        let sqrt = (self.time * self.time / 4 - self.distance_record) as f64;
        let sqrt = sqrt.sqrt();

        let half_time = self.time as f64 / 2.0;
        (half_time - sqrt, half_time + sqrt)
    }

    /// Number of ways to win
    fn num_ways_to_win(&self) -> u64 {
        let (record_min, record_max) = self.record_hold_times();

        let min_to_win = record_min.ceil() as u64;
        let max_to_win = record_max.floor() as u64;

        // every step between min and max can win
        // however min and max themselves can only win if they don't result in
        // exactly the record distance
        let mut num_ways = max_to_win - min_to_win + 1;

        if self.race_distance(min_to_win) == self.distance_record {
            num_ways -= 1;
        }

        if self.race_distance(max_to_win) == self.distance_record {
            num_ways -= 1;
        }

        num_ways
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT1: &str = indoc::indoc! {"
    Time:      7  15   30
    Distance:  9  40  200
    "};

    #[test]
    fn dbg_print_results() {
        let races = parse_input_part1(TEST_INPUT1);

        for race in races {
            println!(
                "{}, {} => {:?}",
                race.time,
                race.distance_record,
                race.record_hold_times()
            );

            let ts = (0..race.time)
                .map(|t| (t, race.race_distance(t)))
                .collect::<Vec<_>>();
            println!("{:?}", ts);

            println!("{:?}", race.num_ways_to_win());
        }
    }

    #[test]
    fn test_part1() {
        let answer = solve_part1(TEST_INPUT1);
        assert_eq!(answer, 288);
    }

    #[test]
    fn test_part2() {
        let answer = solve_part2(TEST_INPUT1);
        assert_eq!(answer, 71503);
    }
}

#[cfg(feature = "divan")]
mod benches {
    use super::*;
    use divan::black_box;

    #[divan::bench]
    fn part1() {
        let answer = solve_part1(black_box(INPUT));
        assert_eq!(answer, 800280)
    }

    #[divan::bench]
    fn part2() {
        let answer = solve_part2(black_box(INPUT));
        assert_eq!(answer, 45128024);
    }
}
