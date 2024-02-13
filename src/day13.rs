use core::panic;

const DAY: u8 = 13;
pub const INPUT: &str = include_str!("../inputs/day13.txt");

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
    solve_core(input, solve_one_grid)
}

fn solve_part2(input: &str) -> u64 {
    solve_core(input, solve_one_grid_with_smudge)
}

fn solve_core(input: &str, solve_one_grid: fn(&[&str]) -> u64) -> u64 {
    let mut lines = Vec::new();
    let mut sum = 0;
    for line in input.lines() {
        if line.is_empty() {
            sum += solve_one_grid(&lines);
            lines.clear();
        } else {
            lines.push(line);
        }
    }

    sum + solve_one_grid(&lines)
}

fn solve_one_grid(lines: &[&str]) -> u64 {
    // Look for horizontal reflection first if not found, look for vertical

    let horizontal_reflection = find_reflection(lines);
    match horizontal_reflection {
        Some(reflection) => (reflection * 100) as u64,
        None => {
            let num_cols = lines[0].chars().count();
            let mut cols = Vec::with_capacity(num_cols);
            for c in 0..num_cols {
                let mut s = String::with_capacity(lines.len());
                for l in lines.iter() {
                    s.push(l.chars().nth(c).unwrap());
                }
                cols.push(s);
            }

            //println!("{:?}", cols);
            match find_reflection(&cols) {
                Some(reflection) => reflection as u64,
                None => {
                    println!("{}", lines.join("\n"));
                    panic!();
                }
            }
        }
    }
}

fn solve_one_grid_with_smudge(lines: &[&str]) -> u64 {
    // Look for horizontal reflection first if not found, look for vertical

    let horizontal_reflection = find_reflection_with_smudge(lines);
    match horizontal_reflection {
        Some(reflection) => (reflection * 100) as u64,
        None => {
            let num_cols = lines[0].chars().count();
            let mut cols = Vec::with_capacity(num_cols);
            for c in 0..num_cols {
                let mut s = String::with_capacity(lines.len());
                for l in lines.iter() {
                    s.push(l.chars().nth(c).unwrap());
                }
                cols.push(s);
            }

            //println!("{:?}", cols);
            match find_reflection_with_smudge(&cols) {
                Some(reflection) => reflection as u64,
                None => {
                    println!("{}", lines.join("\n"));
                    panic!();
                }
            }
        }
    }
}

fn find_reflection<T: PartialEq>(items: &[T]) -> Option<usize> {
    // 1. Find items next to each other that are equal, this is possible reflection point
    // 2. Iterate outward from that point and check if every next item outward is equal too
    // 3. We stop if we reach either end and return the found reflection point
    //    Or if we find a not equal pair, then go to the next possible reflection point and repeat from 2

    'outer: for (possible_reflection_point, ls) in items.windows(2).enumerate() {
        if ls[0] != ls[1] {
            continue;
        }

        // we can at most take possible_reflection_point steps back, since we came from that direction
        for step in 1..possible_reflection_point + 1 {
            let backward = possible_reflection_point - step;
            let forward = possible_reflection_point + step + 1;

            match (items.get(backward), items.get(forward)) {
                (Some(a), Some(b)) if a == b => {}
                (Some(_), Some(_)) => continue 'outer, // lines not equal, go to next possible reflection point
                (None, _) | (_, None) => break,        // reached end, we found the solution
            }
        }
        return Some(possible_reflection_point + 1);
    }

    None
}

fn find_reflection_with_smudge<T: AsRef<str>>(items: &[T]) -> Option<usize> {
    // 1. Find items next to each other that are equal or only differ by one char (smudge),
    //    this is possible reflection point
    // 2. Iterate outward from that point and check if every next item outward is equal or differs by one too.
    //    Note that throughout this walk we can only use one smudge.
    // 3. We stop if we reach either end and return the found reflection point.
    //    An additional condition is that we must have used the smudge, otherwise we found the solution for part1,
    //    and must keep searching.
    //    Or if we find a not equal pair, then go to the next possible reflection point and repeat from 2

    'outer: for (possible_reflection_point, ls) in items.windows(2).enumerate() {
        let ls0 = ls[0].as_ref();
        let ls1 = ls[1].as_ref();
        let diffs_by_one = differs_by_one(ls0.chars(), ls1.chars());

        if !(ls0 == ls1 || diffs_by_one) {
            continue;
        }

        let mut is_smudge_fixed = diffs_by_one;
        // we can at most take possible_reflection_point steps back, since we came from that direction
        for step in 1..possible_reflection_point + 1 {
            let backward = possible_reflection_point - step;
            let forward = possible_reflection_point + step + 1;

            match (
                items.get(backward).map(AsRef::as_ref),
                items.get(forward).map(AsRef::as_ref),
            ) {
                (Some(a), Some(b)) if a == b => {}
                (Some(a), Some(b)) if !is_smudge_fixed && differs_by_one(a.chars(), b.chars()) => {
                    is_smudge_fixed = true;
                }
                (Some(_), Some(_)) => continue 'outer, // lines not equal, go to next possible reflection point
                (None, _) | (_, None) => break,        // reached end, we maybe found the solution
            }
        }
        if is_smudge_fixed {
            return Some(possible_reflection_point + 1);
        }
    }

    None
}

fn differs_by_one<T: PartialEq>(l1: impl Iterator<Item = T>, l2: impl Iterator<Item = T>) -> bool {
    l1.zip(l2).filter(|(a, b)| a != b).count() == 1
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT1: &str = indoc::indoc! {"
    #.##..##.
    ..#.##.#.
    ##......#
    ##......#
    ..#.##.#.
    ..##..##.
    #.#.##.#.
    
    #...##..#
    #....#..#
    ..##..###
    #####.##.
    #####.##.
    ..##..###
    #....#..#
    "};

    const TEST_INPUT2: &str = indoc::indoc! {"
    ###.####.
    #..#....#
    #..######
    .#..####.
    ###......
    ##.#....#
    ##.#....#
    ###......
    .##.####.
    #..######
    #..#....#
    ###.####.
    #.#######
    "};

    #[test]
    fn test_part1() {
        let answer = solve_part1(TEST_INPUT1);
        assert_eq!(answer, 405);
    }

    #[test]
    fn test_part1_2() {
        let answer = solve_part1(TEST_INPUT2);
        assert_eq!(answer, 6);
    }

    #[test]
    fn test_part2() {
        let answer = solve_part2(TEST_INPUT1);
        assert_eq!(answer, 400);
    }
}

#[cfg(feature = "divan")]
mod benches {
    use super::*;
    use divan::black_box;

    #[divan::bench]
    fn part1() {
        let answer = solve_part1(black_box(INPUT));
        assert_eq!(answer, 31956);
    }

    #[divan::bench]
    fn part2() {
        let answer = solve_part2(black_box(INPUT));
        assert_eq!(answer, 37617);
    }
}
