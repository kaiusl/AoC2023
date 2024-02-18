use std::borrow::Cow;
use std::collections::HashMap;

const DAY: u8 = 14;
pub const INPUT: &str = include_str!("../inputs/day14.txt");

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
    // Iterate the rows in reverse and count the rolling rocks in each column.
    // If we find a square rock in a column, we know where all of the rolling rocks
    // below it will stop. Hence we can calculate the weight of those rocks.
    // We continue until we reach the end of the input.
    // Then finally we'll need to add the final rolling rocks too that stopped at the top edge.

    let cols = input.lines().next().unwrap().chars().count();

    let mut rolling_rocks = vec![0; cols];
    let mut total_load = 0;
    let mut max_weight = 0;
    for (l, weight) in input.lines().rev().zip(1..) {
        // dbg!(weight, l);
        for (i, c) in l.chars().enumerate() {
            match c {
                'O' => rolling_rocks[i] += 1,
                '.' => {}
                '#' => {
                    // for i in 0..rolling_rocks[i] {
                    //     // -1 because the square rock is on the line,
                    //     // rolling rocks start from previous line
                    //     total_load += weight - 1 - i;
                    // }

                    // above has closed form solution
                    let num_rolling_rocks = rolling_rocks[i];
                    if num_rolling_rocks != 0 {
                        total_load +=
                            num_rolling_rocks * (weight - 1) - triag_number(num_rolling_rocks - 1);
                    }
                    rolling_rocks[i] = 0;
                }
                _ => unreachable!(),
            }
        }
        max_weight = weight;
    }

    for num_rolling_rocks in rolling_rocks.into_iter().filter(|&r| r != 0) {
        total_load += num_rolling_rocks * max_weight - triag_number(num_rolling_rocks - 1);
    }

    total_load
}

/// Calculates the triangular number of n.
///
/// That is 1 + 2 + 3 + 4 + ... + n
fn triag_number(n: u64) -> u64 {
    n * (n + 1) / 2
}

fn solve_part2(input: &str) -> u64 {
    const ROTATIONS: usize = 1_000_000_000;
    solve_part2_core(input, ROTATIONS)
}
fn solve_part2_core(input: &str, num_rotations: usize) -> u64 {
    // Note to simplify the comment, we define a rotation as one cycle of north, west, south and east tilts.
    // This is defined as cycle in the AoC problem.
    // A cycle is then defined as some cycle of rotations.
    //
    // General idea is to use a cache to map one grid to the one after rotation (north, west, south, east tilts).
    // Now whenever we hit this cache, we have reached the start of a cycle since the next grid must also be in the cache.
    // Thus we can find the length of this cycle and directly calculate how many times we need to repeat it
    // before we reach the desired number of rotations (`num_rotations`). Note that this doesn't
    // necessarily exactly reach `num_rotations`. Thus finally we continue the rotations until we have exactly
    // reached the desired number of rotations.
    //
    // And finally calculate the load on north support beam.

    let mut cache = HashMap::<Vec<u8>, (usize, Vec<u8>)>::new();
    let num_cols = input.lines().next().unwrap().len() + 1;

    let mut grid = input.as_bytes().to_vec();

    let mut rotation_num = 0;
    let grid_ref = loop {
        if rotation_num >= num_rotations {
            // finished without finding a cycle
            break None;
        }
        rotation_num += 1;

        // Cache hit, find the length of the cycle and get as close as possible to desired number of rotations
        if let Some((cy, next)) = cache.get(&grid) {
            let cycle_len = rotation_num - cy;
            let x = (num_rotations - rotation_num) / cycle_len;
            rotation_num += cycle_len * x;
            break Some(next);
        }

        // No cache hit, calculate the next grid after a rotation
        let prev_grid = grid.clone();
        rotate_grid(&mut grid, num_cols);
        cache.insert(prev_grid, (rotation_num, grid.clone()));
    };

    //dbg!(cache.len());

    // Finish the rest of the rotations
    let grid = if let Some(mut grid_ref) = grid_ref {
        while rotation_num < num_rotations {
            rotation_num += 1;
            grid_ref = &cache[grid_ref].1;
        }
        grid_ref
    } else {
        &grid
    };

    // Calculate the load on north support beam
    let mut total_load = 0;
    for (l, weight) in grid
        .split(|&c| c == b'\n')
        .filter(|l| !l.is_empty())
        .rev()
        .zip(1..)
    {
        total_load += weight * l.iter().filter(|&&c| c == b'O').count();
    }

    total_load as u64
}

/// Perform one rotation on the grid
///
/// `grid` is assumed to be in a row major order
fn rotate_grid(grid: &mut [u8], num_cols: usize) {
    // Go through the grid and rotate it
    //
    // Move from the the tilt direction one row (north, south tilt) or column (west, east tilt) at the time,
    // while keeping track where the next rolling rock will go in each row or column.
    // When a rolling rock was found, move it to the correct place.

    // tilt north
    // first rolling rock will go to the first row
    let mut next_rolling_rock_location = (0..num_cols).collect::<Vec<_>>();

    let mut line_index = 0;
    let mut column_index = 0;
    for char_index in 0..grid.len() {
        let c = &mut grid[char_index];
        //print!("'{}' ", *c as char);
        match c {
            b'.' => {}
            b'#' => {
                next_rolling_rock_location[column_index] = char_index + num_cols;
            }
            b'O' => {
                *c = b'.';
                grid[next_rolling_rock_location[column_index]] = b'O';
                next_rolling_rock_location[column_index] += num_cols;
            }
            b'\n' => {
                line_index += 1;
                column_index = 0;
                continue;
            }
            _ => unreachable!(),
        }
        column_index += 1;
    }

    let num_rows = line_index;

    // tilt west
    // first rolling rock will go to the first column
    let mut next_rolling_rock_location = (0..num_rows).map(|c| c * num_cols).collect::<Vec<_>>();

    // move one column at a time from the left (west)
    for col in 0..num_cols {
        for row in 0..num_rows {
            let char_index = row * num_cols + col;
            let c = &mut grid[char_index];
            // dbg!(&c);
            match c {
                b'.' => {}
                b'#' => {
                    next_rolling_rock_location[row] = char_index + 1;
                }
                b'O' => {
                    *c = b'.';
                    grid[next_rolling_rock_location[row]] = b'O';
                    next_rolling_rock_location[row] += 1;
                }
                b'\n' => {
                    continue;
                }
                _ => unreachable!(),
            }
        }
    }

    // tilt south
    // first rolling rock will go to the last row
    let mut next_rolling_rock_location = (grid.len() - num_cols..grid.len()).collect::<Vec<_>>();

    for row in (0..num_rows).rev() {
        for col in 0..num_cols {
            let char_index = row * num_cols + col;
            let c = &mut grid[char_index];
            match c {
                b'.' => {}
                b'#' => {
                    next_rolling_rock_location[col] = char_index.saturating_sub(num_cols);
                }
                b'O' => {
                    *c = b'.';
                    grid[next_rolling_rock_location[col]] = b'O';
                    next_rolling_rock_location[col] =
                        next_rolling_rock_location[col].saturating_sub(num_cols);
                }
                b'\n' => {
                    continue;
                }
                _ => unreachable!(),
            }
        }
    }

    // tilt east
    // first rolling rock will go to the last column
    let mut next_rolling_rock_location = (0..num_rows)
        .map(|c| c * num_cols + num_cols - 2)
        .collect::<Vec<_>>();
    // move one column at a time from the right (east)
    for col in (0..num_cols).rev() {
        for row in 0..num_rows {
            let char_index = row * num_cols + col;
            let c = &mut grid[char_index];
            // dbg!(&c);
            match c {
                b'.' => {}
                b'#' => {
                    next_rolling_rock_location[row] = char_index.saturating_sub(1);
                }
                b'O' => {
                    *c = b'.';
                    grid[next_rolling_rock_location[row]] = b'O';
                    next_rolling_rock_location[row] =
                        next_rolling_rock_location[row].saturating_sub(1);
                }
                b'\n' => {
                    continue;
                }
                _ => unreachable!(),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT1: &str = indoc::indoc! {"
    O....#....
    O.OO#....#
    .....##...
    OO.#O....O
    .O.....O#.
    O.#..O.#.#
    ..O..#O..O
    .......O..
    #....###..
    #OO..#....
    "};

    #[test]
    fn test_part1() {
        let answer = solve_part1(TEST_INPUT1);
        assert_eq!(answer, 136);
    }

    #[test]
    fn test_part2() {
        let answer = solve_part2(TEST_INPUT1);
        assert_eq!(answer, 64);
    }
}

#[cfg(feature = "divan")]
mod benches {
    use super::*;
    use divan::black_box;

    #[divan::bench]
    fn part1() {
        let answer = solve_part1(black_box(INPUT));
        assert_eq!(answer, 108144);
    }

    #[divan::bench]
    fn part2() {
        let answer = solve_part2(black_box(INPUT));
        assert_eq!(answer, 108404);
    }
}
