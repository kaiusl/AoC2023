const DAY: u8 = 15;
pub const INPUT: &str = include_str!("../inputs/day15.txt");

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
    input.split(',').map(|s| hash(s) as u64).sum()
}

fn solve_part2(input: &str) -> u64 {
    let mut map = HashMap::new();

    for op in input.split(',') {
        if op.ends_with('-') {
            let label = &op[0..op.len() - 1];
            map.remove(label);
        } else {
            let (label, lens) = op.split_once('=').unwrap();
            let lens = lens.parse::<Lens>().unwrap();
            map.insert(label, lens);
        }
    }

    let mut focusing_power = 0;
    for (box_number, boks) in (1..).zip(map.data) {
        if boks.is_empty() {
            continue;
        }

        for (lens_number, (_, focal_length)) in (1..).zip(boks) {
            focusing_power += box_number * lens_number * focal_length as u64;
        }
    }

    focusing_power
}

fn hash(input: &str) -> u8 {
    input.as_bytes().iter().fold(0, |acc, &ascii| {
        let mut tmp = acc as u16;
        tmp += ascii as u16;
        tmp *= 17;
        (tmp % 256) as u8
    })
}

type Lens = u8;
type Label = str;

#[derive(Debug)]
struct HashMap<'a> {
    data: [Vec<(&'a Label, Lens)>; 256],
}

impl<'a> HashMap<'a> {
    fn new() -> Self {
        Self {
            data: std::array::from_fn(|_| Vec::new()),
        }
    }

    fn insert(&mut self, label: &'a Label, lens: Lens) {
        let hash = hash(label) as usize; // hash is already between 0 and 255
        let bucket = &mut self.data[hash];
        if let Some((_, _lens)) = bucket.iter_mut().find(|(_label, _)| *_label == label) {
            *_lens = lens;
        } else {
            bucket.push((label, lens));
        }
    }

    fn remove(&mut self, label: &'a Label) {
        let hash = hash(label) as usize;
        let bucket = &mut self.data[hash];
        bucket.retain(|(_label, _)| *_label != label);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT1: &str = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";

    #[test]
    fn test_hash() {
        assert_eq!(hash("HASH"), 52);
    }

    #[test]
    fn test_part1() {
        let answer = solve_part1(TEST_INPUT1);
        assert_eq!(answer, 1320);
    }

    #[test]
    fn test_part2() {
        let answer = solve_part2(TEST_INPUT1);
        assert_eq!(answer, 145);
    }
}

#[cfg(feature = "divan")]
mod benches {
    use super::*;
    use divan::black_box;

    #[divan::bench]
    fn part1() {
        let answer = solve_part1(black_box(INPUT));
        assert_eq!(answer, 513214);
    }

    #[divan::bench]
    fn part2() {
        let answer = solve_part2(black_box(INPUT));
        assert_eq!(answer, 258826);
    }
}
