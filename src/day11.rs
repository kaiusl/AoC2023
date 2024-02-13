use std::collections::btree_set::Difference;
use std::collections::{BTreeSet, HashSet};

const DAY: u8 = 11;
pub const INPUT: &str = include_str!("../inputs/day11.txt");

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
    solve_core(input, 2)
}

fn solve_part2(input: &str) -> u64 {
    solve_core(input, 1_000_000)
}

fn solve_core(input: &str, expansion_rate: usize) -> u64 {
    let galaxies = get_galaxy_locations(input, expansion_rate);

    galaxies
        .iter()
        .enumerate()
        .map(|(i, g1)| {
            galaxies[i + 1..]
                .iter()
                .map(|g2| distance(g1, g2) as u64)
                .sum::<u64>()
        })
        .sum()
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct Galaxy {
    x: usize,
    y: usize,
}

fn get_galaxy_locations(input: &str, expansion_multiplier: usize) -> Vec<Galaxy> {
    let grid_size = input.lines().next().unwrap().chars().count();

    let mut galaxies = Vec::new();
    // Keep track of which rows and columns haven't seen a galaxy
    let mut empty_xs = BTreeSet::from_iter(0usize..grid_size);
    let mut empty_ys = BTreeSet::from_iter(0usize..grid_size);

    for (y, l) in input.lines().enumerate() {
        for (x, c) in l.chars().enumerate() {
            if c == '#' {
                empty_ys.remove(&y);
                empty_xs.remove(&x);
                let loc = Galaxy { x, y };
                galaxies.push(loc);
            }
        }
    }

    // Since we need to iterate over the sets many times, we collect them before iterating
    let empty_xs = empty_xs.into_iter().collect::<Vec<_>>();
    let empty_ys = empty_ys.into_iter().collect::<Vec<_>>();

    for galaxy in galaxies.iter_mut() {
        let xshift = empty_xs.iter().filter(|&&x| galaxy.x > x).count();
        let yshift = empty_ys.iter().filter(|&&y| galaxy.y > y).count();

        // if expansion_multiplier == 2, then we have 1 more row, eg shift is 1 less than the rate
        galaxy.x += xshift * (expansion_multiplier - 1);
        galaxy.y += yshift * (expansion_multiplier - 1);
    }

    // Below are couple other variants on how to shift the galaxies
    // They are slightly slower..

    // Iterate other way around
    // for (shift, x) in (1..empty_xs.len() + 1).zip(empty_xs).rev() {
    //     for galaxy in galaxies.iter_mut() {
    //         if galaxy.x > x {
    //             galaxy.x += (expansion_multiplier - 1);
    //         }
    //     }
    // }

    // for (shift, y) in (1..empty_ys.len() + 1).zip(empty_ys).rev() {
    //     for galaxy in galaxies.iter_mut() {
    //         if galaxy.y > y {
    //             galaxy.y += (expansion_multiplier - 1);
    //         }
    //     }
    // }

    // Sort and iterate only once
    // galaxies.sort_by(|g1, g2| g1.x.cmp(&g2.x));

    // let mut shifts = empty_xs.iter();
    // let mut next_shift_x = *shifts.next().unwrap_or(&grid_size);
    // let mut shift = 0;

    // for galaxy in galaxies.iter_mut() {
    //     while galaxy.x > next_shift_x {
    //         next_shift_x = *shifts.next().unwrap_or(&grid_size);
    //         shift += expansion_multiplier - 1;
    //     }

    //     galaxy.x += shift;
    // }

    // galaxies.sort_by(|g1, g2| g1.y.cmp(&g2.y));

    // let mut shifts = empty_ys.iter();
    // let mut next_shift_y = *shifts.next().unwrap_or(&grid_size);
    // let mut shift = 0;

    // for galaxy in galaxies.iter_mut() {
    //     while galaxy.y > next_shift_y {
    //         next_shift_y = *shifts.next().unwrap_or(&grid_size);
    //         shift += expansion_multiplier - 1;
    //     }

    //     galaxy.y += shift;
    // }

    galaxies
}

fn distance(loc1: &Galaxy, loc2: &Galaxy) -> usize {
    // The shortest distance with up left down right movement is same as going straight and turn 90 degrees
    // For example:
    // .5----.......
    // .##..|......6
    // ..##.|.......
    // ...##|.......
    // ....##...7...
    // 8....9.......
    // Pipes and # result in same distance.
    (loc1.x as isize - loc2.x as isize).unsigned_abs()
        + (loc1.y as isize - loc2.y as isize).unsigned_abs()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT1: &str = indoc::indoc! {"
    ...#......
    .......#..
    #.........
    ..........
    ......#...
    .#........
    .........#
    ..........
    .......#..
    #...#.....
    "};

    // ....1........
    // .........2...
    // 3............
    // .............
    // .............
    // ........4....
    // .5...........
    // ............6
    // .............
    // .............
    // .........7...
    // 8....9.......

    #[test]
    fn test_get_locations() {
        let locations = get_galaxy_locations(TEST_INPUT1, 2);
        println!("locations: {:?}", locations);
    }

    #[test]
    fn test_part1() {
        let answer = solve_part1(TEST_INPUT1);
        assert_eq!(answer, 374);
    }

    #[test]
    fn test_part2_example() {
        let answer = solve_core(TEST_INPUT1, 10);
        assert_eq!(answer, 1030);

        let answer = solve_core(TEST_INPUT1, 100);
        assert_eq!(answer, 8410);
    }
}

#[cfg(feature = "divan")]
mod benches {
    use super::*;
    use divan::black_box;

    #[divan::bench]
    fn part1() {
        let answer = solve_part1(black_box(INPUT));
        assert_eq!(answer, 9608724);
    }

    #[divan::bench]
    fn part2() {
        let answer = solve_part2(black_box(INPUT));
        assert_eq!(answer, 904633799472);
    }
}
