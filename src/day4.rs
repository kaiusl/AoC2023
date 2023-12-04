const DAY: u8 = 4;
pub const INPUT: &str = include_str!("../inputs/day4.txt");

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
    let mut winning_numbers = Vec::new();

    let mut sum = 0;
    for line in input.lines() {
        let (_, numbers_str) = line.split_once(": ").unwrap();
        let (winning_numbers_str, numbers_str) = numbers_str.split_once(" | ").unwrap();

        winning_numbers.extend(
            winning_numbers_str
                .split(' ')
                .filter(|s| !s.is_empty())
                .map(|n| n.parse::<u32>().unwrap()),
        );

        let mut won_numbers = numbers_str.split(' ').filter(|n| {
            if n.is_empty() {
                false
            } else {
                let num = n.parse::<u32>().unwrap();
                winning_numbers.contains(&num)
            }
        });

        let prod = won_numbers.next().is_some() as u64;
        sum += won_numbers.fold(prod, |p, _| p * 2);

        winning_numbers.clear();
    }

    sum
}

fn solve_part2(input: &str) -> u64 {
    let mut winning_numbers = Vec::new();
    let mut card_multipliers = Vec::new();
    let mut total_number_of_cards = 0;

    for (card_nr, line) in input.lines().enumerate() {
        let current_card_multiplier = match card_multipliers.get_mut(card_nr) {
            Some(m) => {
                *m += 1;
                *m
            }
            None => {
                card_multipliers.push(1);
                1
            }
        };

        // The number of current card's won't change anymore, add to total
        total_number_of_cards += current_card_multiplier;

        let (_, numbers_str) = line.split_once(": ").unwrap();
        let (winning_numbers_str, numbers_str) = numbers_str.split_once(" | ").unwrap();

        winning_numbers.extend(
            winning_numbers_str
                .split(' ')
                .filter(|s| !s.is_empty())
                .map(|n| n.parse::<u32>().unwrap()),
        );

        let won_numbers = numbers_str.split(' ').filter(|n| {
            if n.is_empty() {
                false
            } else {
                let num = n.parse::<u32>().unwrap();
                winning_numbers.contains(&num)
            }
        });

        for (_, won_card_nr) in won_numbers.zip(card_nr + 1..) {
            match card_multipliers.get_mut(won_card_nr) {
                Some(m) => *m += current_card_multiplier,
                None => card_multipliers.push(current_card_multiplier),
            };
        }

        winning_numbers.clear();
    }

    total_number_of_cards
}
#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT1: &str = indoc::indoc! {"
    Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
    Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
    Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
    Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
    Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
    Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
    "};

    #[test]
    fn test_part1() {
        let answer = solve_part1(TEST_INPUT1);
        assert_eq!(answer, 13);
    }

    #[test]
    fn test_part2() {
        let answer = solve_part2(TEST_INPUT1);
        assert_eq!(answer, 30);
    }
}

#[cfg(feature = "divan")]
mod benches {
    use super::*;
    use divan::black_box;

    #[divan::bench]
    fn part1() {
        let answer = solve_part1(black_box(INPUT));
        assert_eq!(answer, 21485);
    }

    #[divan::bench]
    fn part2() {
        let answer = solve_part2(black_box(INPUT));
        assert_eq!(answer, 11024379);
    }
}
