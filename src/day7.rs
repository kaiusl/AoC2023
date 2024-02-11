use std::collections::{BTreeMap, BTreeSet};

const DAY: u8 = 7;
pub const INPUT: &str = include_str!("../inputs/day7.txt");

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
    let hands = read_hands_part1(input);

    hands
        .iter()
        .rev()
        .zip(1u64..)
        .map(|(hand, rank)| rank * hand.bid)
        .sum()
}

fn solve_part2(input: &str) -> u64 {
    let hands = read_hands_part2(input);

    hands
        .iter()
        .rev()
        .zip(1u64..)
        .map(|(hand, rank)| rank * hand.bid)
        .sum()
}

/// Hand which is ordered by it's strength for part1 rules
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct HandPart1<'a> {
    cards: &'a str,
    kind: HandKind,
    bid: u64,
}

impl<'a> PartialOrd for HandPart1<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<'a> Ord for HandPart1<'a> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.kind.cmp(&other.kind) {
            std::cmp::Ordering::Equal => {}
            ord => return ord,
        }

        // card order  A, K, Q, J, T, 9, 8, 7, 6, 5, 4, 3, or 2
        fn card_to_strength(c: char) -> u8 {
            match c {
                'A' => 14,
                'K' => 13,
                'Q' => 12,
                'J' => 11,
                'T' => 10,
                '9' => 9,
                '8' => 8,
                '7' => 7,
                '6' => 6,
                '5' => 5,
                '4' => 4,
                '3' => 3,
                '2' => 2,
                _ => unreachable!(),
            }
        }
        for (a, b) in self.cards.chars().zip(other.cards.chars()) {
            match card_to_strength(b).cmp(&card_to_strength(a)) {
                std::cmp::Ordering::Equal => {}
                ord => return ord,
            }
        }

        std::cmp::Ordering::Equal
    }
}

/// Hand which is ordered by it's strength for part2 rules
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct HandPart2<'a> {
    cards: &'a str,
    kind: HandKind,
    bid: u64,
}

impl<'a> PartialOrd for HandPart2<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<'a> Ord for HandPart2<'a> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.kind.cmp(&other.kind) {
            std::cmp::Ordering::Equal => {}
            ord => return ord,
        }

        // card order  A, K, Q, T, 9, 8, 7, 6, 5, 4, 3, 2, J
        fn card_to_strength(c: char) -> u8 {
            match c {
                'A' => 14,
                'K' => 13,
                'Q' => 12,
                'T' => 10,
                '9' => 9,
                '8' => 8,
                '7' => 7,
                '6' => 6,
                '5' => 5,
                '4' => 4,
                '3' => 3,
                '2' => 2,
                'J' => 1,
                _ => unreachable!(),
            }
        }
        for (a, b) in self.cards.chars().zip(other.cards.chars()) {
            match card_to_strength(b).cmp(&card_to_strength(a)) {
                std::cmp::Ordering::Equal => {}
                ord => return ord,
            }
        }

        std::cmp::Ordering::Equal
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
enum HandKind {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
}

impl HandKind {
    fn from_hand_part1(hand: &str) -> Self {
        // Since we have max 5 cards, don't bother with map as it's faster to search the array
        let mut counts = [('0', 0); 5];
        for (i, char) in hand.chars().enumerate() {
            if let Some((_, count)) = counts.iter_mut().find(|(card, _)| card == &char) {
                *count += 1;
            } else {
                counts[i] = (char, 1);
            }
        }
        counts.sort_by(|(_, v1), (_, v2)| v2.cmp(v1));

        let mut kind = Self::HighCard;
        for (_, count) in counts {
            match (count, kind) {
                (1, _) => {}
                (2, Self::HighCard) => kind = Self::OnePair,
                (2, Self::OnePair) => kind = Self::TwoPair,
                (2, Self::ThreeOfAKind) => kind = Self::FullHouse,
                (3, Self::HighCard) => kind = Self::ThreeOfAKind,
                (3, Self::OnePair) => kind = Self::FullHouse,
                (4, _) => kind = Self::FourOfAKind,
                (5, _) => kind = Self::FiveOfAKind,
                _ => {}
            }
        }

        kind
    }

    fn from_hand_part2(hand: &str) -> Self {
        // Since we have max 5 cards, don't bother with map as it's faster to search
        let mut counts = [('0', 0); 5];
        let mut jcount = 0;
        for (i, char) in hand.chars().enumerate() {
            if char == 'J' {
                jcount += 1;
                continue;
            }
            if let Some((_, count)) = counts.iter_mut().find(|(card, _)| card == &char) {
                *count += 1;
            } else {
                counts[i] = (char, 1);
            }
        }
        counts.sort_by(|(_, v1), (_, v2)| v2.cmp(v1));

        let mut kind = Self::HighCard;
        let mut counts = counts.into_iter().filter(|(c, _)| c != &'J' && c != &'0');
        match jcount {
            0 => {
                for (_, count) in counts {
                    match (count, kind) {
                        (1, _) => {}
                        (2, Self::HighCard) => kind = Self::OnePair,
                        (2, Self::OnePair) => kind = Self::TwoPair,
                        (2, Self::ThreeOfAKind) => kind = Self::FullHouse,
                        (3, Self::HighCard) => kind = Self::ThreeOfAKind,
                        (3, Self::OnePair) => kind = Self::FullHouse,
                        (4, _) => kind = Self::FourOfAKind,
                        (5, _) => kind = Self::FiveOfAKind,
                        _ => unreachable!("{}:{:?}", count, kind),
                    }
                }
            }
            1 => {
                for (_, count) in counts {
                    match (count, kind) {
                        // must be xyzwJ, xxxyJ or xxyzJ
                        (1, Self::HighCard) => return Self::OnePair,
                        // must be xxyzJ
                        (1, Self::ThreeOfAKind) => {
                            // it's possible but we cannot increase the strength
                        }
                        // must be xxyyJ or xxyzJ,
                        // note that for xxyzJ, we have ThreeOfAKind and TwoPairs as possibilities,
                        // first of those is better, so we choose that
                        (2, Self::HighCard) => kind = Self::ThreeOfAKind,
                        // must be xxyyJ
                        (2, Self::ThreeOfAKind) => return Self::FullHouse,
                        // must be xxxyJ
                        (3, _) => return Self::FourOfAKind,
                        // must by xxxxJ
                        (4, _) => return Self::FiveOfAKind,
                        _ => unreachable!("{}:{:?}", count, kind),
                    }
                }
            }
            2 => {
                if let Some((_, count)) = counts.next() {
                    match (count, kind) {
                        // must be xyzJJ since we iterate from highest counts down
                        (1, _) => return Self::ThreeOfAKind,
                        // must be xxyJJ
                        (2, _) => return Self::FourOfAKind,
                        // must by xxxJJ
                        (3, _) => return Self::FiveOfAKind,
                        _ => unreachable!("{}:{:?}", count, kind),
                    }
                }
            }
            3 => {
                if let Some((_, count)) = counts.next() {
                    match (count, kind) {
                        // must be xyJJJ
                        (1, _) => return Self::FourOfAKind,
                        // must be xxJJJ
                        (2, _) => return Self::FiveOfAKind,
                        _ => unreachable!("{}:{:?}", count, kind),
                    }
                }
            }
            4 => return Self::FiveOfAKind,
            5 => return Self::FiveOfAKind,
            _ => {}
        }

        kind
    }
}

fn read_hands_part1(input: &str) -> BTreeSet<HandPart1<'_>> {
    let mut set = BTreeSet::new();

    for line in input.lines() {
        let (cards, bid) = line.split_once(' ').unwrap();
        set.insert(HandPart1 {
            cards,
            kind: HandKind::from_hand_part1(cards),
            bid: bid.parse().unwrap(),
        });
    }

    set
}

fn read_hands_part2(input: &str) -> BTreeSet<HandPart2<'_>> {
    let mut set = BTreeSet::new();

    for line in input.lines() {
        let (cards, bid) = line.split_once(' ').unwrap();
        set.insert(HandPart2 {
            cards,
            kind: HandKind::from_hand_part2(cards),
            bid: bid.parse().unwrap(),
        });
    }

    set
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT1: &str = indoc::indoc! {"
    32T3K 765
    T55J5 684
    KK677 28
    KTJJT 220
    QQQJA 483
    "};

    #[test]
    fn test_parse() {
        let answer = read_hands_part2(TEST_INPUT1);
        println!("{:#?}", answer);
    }

    #[test]
    fn test_part1() {
        let answer = solve_part1(TEST_INPUT1);
        assert_eq!(answer, 6440);
    }

    #[test]
    fn test_part2() {
        let answer = solve_part2(TEST_INPUT1);
        assert_eq!(answer, 5905);
    }
}

#[cfg(feature = "divan")]
mod benches {
    use super::*;
    use divan::black_box;

    #[divan::bench]
    fn part1() {
        let answer = solve_part1(black_box(INPUT));
        assert_eq!(answer, 250058342);
    }

    #[divan::bench]
    fn part2() {
        let answer = solve_part2(black_box(INPUT));
        assert_eq!(answer, 250506580);
    }
}
