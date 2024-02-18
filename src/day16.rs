use std::collections::VecDeque;
use std::path::Display;

const DAY: u8 = 16;
pub const INPUT: &str = include_str!("../inputs/day16.txt");

pub fn run() {
    solve(INPUT);
}

fn solve(input: &str) {
    let answer_part1 = solve_part1(input);
    println!("day{DAY}::part1 answer: {}", answer_part1);

    let answer_part2 = solve_part2(input);
    println!("day{DAY}::part2 answer: {}", answer_part2);
}

type Pos = (usize, usize);

fn solve_part1(input: &str) -> u64 {
    let matrix = parse(input);
    solve_core(matrix, Direction::Right, (0, 0))
}

fn solve_part2(input: &str) -> u64 {
    let matrix = parse(input);

    assert_eq!(matrix.rows, matrix.cols);

    // go through all the starting positions
    let mut max = 0;
    for i in 0..matrix.rows {
        // start from first column
        let result = solve_core(matrix.clone(), Direction::Right, (i, 0));
        max = max.max(result);

        // start from last column
        let result = solve_core(matrix.clone(), Direction::Left, (i, matrix.rows - 1));
        max = max.max(result);

        // start from first row
        let result = solve_core(matrix.clone(), Direction::Down, (0, i));
        max = max.max(result);

        // start from last row
        let result = solve_core(matrix.clone(), Direction::Up, (matrix.rows - 1, i));
        max = max.max(result);

        //dbg!(i, max);
    }

    max
}

// Assumes that start_dir and start_pos combo is valid
fn solve_core(mut matrix: Matrix<(Token, Direction)>, start_dir: Direction, start_pos: Pos) -> u64 {
    // General idea here is to loop through the light path
    // and for every visited tile mark in which direction we moved from it.
    // `matrix` contain the token type and the seen directions.
    //
    // Light path ends if it exits the grid or maps onto a loop.
    // A loop starts if we find a tile from which we already moved in that same direction.

    let mut cursor = MatrixCursor::new(&mut matrix, start_pos);
    // Store the splits in the light path that we cannot follow right away and come back to them
    let mut splits_queue = VecDeque::new();

    // Match the first position and get the next step direction
    let (next_dir, split) = get_next_step(&mut cursor, start_dir).unwrap();
    cursor.get_current_mut().1.insert(next_dir);
    splits_queue.push_back((next_dir, cursor.get_current_pos()));
    if let Some(split) = split {
        splits_queue.push_back(split);
    }

    // Loop through all split light path and mark all energized tiles
    //
    // Light path ends if it exits the grid or maps onto a loop
    // (that is starts moving in the same direction with some earlier path)

    while let Some((dir, pos)) = splits_queue.pop_front() {
        let mut prev_direction = dir;
        cursor.set_pos(pos);
        cursor.step(dir);

        loop {
            //dbg!(prev_direction);
            let Some((next_dir, split)) = get_next_step(&mut cursor, prev_direction) else {
                break;
            };

            cursor.get_current_mut().1.insert(next_dir);

            if let Some(split) = split {
                splits_queue.push_back(split);
            }

            if !cursor.step(next_dir) {
                break;
            }

            prev_direction = next_dir;
        }
    }

    matrix
        .data
        .iter()
        .filter(|(_, dir)| !dir.is_empty())
        .count() as u64
}

/// Get the next step direction and the next split position
fn get_next_step(
    cursor: &mut MatrixCursor<'_, (Token, Direction)>,
    prev_direction: Direction,
) -> Option<(Direction, Option<(Direction, Pos)>)> {
    let mut next_dir = prev_direction;
    let mut split = None;
    let (current_token, _) = cursor.get_current_mut();

    match current_token {
        Token::LeftMirror if prev_direction.contains(Direction::Up) => {
            next_dir = Direction::Left;
        }
        Token::RightMirror if prev_direction.contains(Direction::Up) => {
            next_dir = Direction::Right;
        }

        Token::LeftMirror if prev_direction.contains(Direction::Down) => {
            next_dir = Direction::Right;
        }
        Token::RightMirror if prev_direction.contains(Direction::Down) => {
            next_dir = Direction::Left;
        }
        Token::HorizontalSplitter
            if prev_direction.contains(Direction::Down)
                || prev_direction.contains(Direction::Up) =>
        {
            split = Some((Direction::Right, cursor.get_current_pos()));
            next_dir = Direction::Left;
        }

        Token::LeftMirror if prev_direction.contains(Direction::Left) => {
            next_dir = Direction::Up;
        }
        Token::RightMirror if prev_direction.contains(Direction::Left) => {
            next_dir = Direction::Down
        }
        Token::LeftMirror if prev_direction.contains(Direction::Right) => {
            next_dir = Direction::Down;
        }
        Token::RightMirror if prev_direction.contains(Direction::Right) => {
            next_dir = Direction::Up;
        }

        Token::VerticalSplitter
            if prev_direction.contains(Direction::Right)
                || prev_direction.contains(Direction::Left) =>
        {
            split = Some((Direction::Up, cursor.get_current_pos()));
            next_dir = Direction::Down;
        }
        _ => {
            // keep moving in the same direction
        }
    }

    let (_, seen_direction) = cursor.get_current_mut();
    if seen_direction.contains(next_dir) {
        // We have reached a loop, since we have already moved from this tile in the same direction
        return None;
    }

    Some((next_dir, split))
}

#[derive(Debug, Clone, Copy)]
enum Token {
    Empty,
    VerticalSplitter,
    HorizontalSplitter,
    LeftMirror,
    RightMirror,
}

use bitflags::bitflags;

bitflags! {
    #[derive(Debug, Clone, Copy)]
    pub struct Direction: u8 {
        const Up = 1 << 0;
        const Down = 1 << 1;
        const Left = 1 << 2;
        const Right = 1 << 3;
    }
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Token::Empty => write!(f, "."),
            Token::VerticalSplitter => write!(f, "|"),
            Token::HorizontalSplitter => write!(f, "-"),
            Token::LeftMirror => write!(f, "\\"),
            Token::RightMirror => write!(f, "/"),
        }
    }
}

#[derive(Debug, Clone)]
struct Matrix<T> {
    data: Vec<T>,
    rows: usize,
    cols: usize,
}

impl<T> std::fmt::Display for Matrix<T>
where
    T: std::fmt::Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in 0..self.rows {
            for col in 0..self.cols {
                write!(f, "{}", self.data[row * self.cols + col])?;
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

struct MatrixCursor<'a, T> {
    matrix: &'a mut Matrix<T>,
    row: usize,
    col: usize,
}

impl<'a, T> MatrixCursor<'a, T> {
    fn new(matrix: &'a mut Matrix<T>, pos: Pos) -> Self {
        Self {
            matrix,
            row: pos.0,
            col: pos.1,
        }
    }

    fn get_current(&self) -> &T {
        &self.matrix.data[self.row * self.matrix.cols + self.col]
    }

    fn get_current_mut(&mut self) -> &mut T {
        &mut self.matrix.data[self.row * self.matrix.cols + self.col]
    }

    fn set_current(&mut self, value: T) {
        *self.get_current_mut() = value
    }

    fn get_current_pos(&self) -> Pos {
        (self.row, self.col)
    }

    fn set_pos(&mut self, pos: Pos) -> bool {
        if pos.0 < self.matrix.rows && pos.1 < self.matrix.cols {
            self.row = pos.0;
            self.col = pos.1;
            return true;
        }

        false
    }

    fn step_up(&mut self) -> bool {
        if self.row > 0 {
            self.row -= 1;
            return true;
        }

        false
    }

    fn step_down(&mut self) -> bool {
        if self.row < self.matrix.rows - 1 {
            self.row += 1;
            return true;
        }

        false
    }

    fn step_left(&mut self) -> bool {
        if self.col > 0 {
            self.col -= 1;
            return true;
        }

        false
    }

    fn step_right(&mut self) -> bool {
        if self.col < self.matrix.cols - 1 {
            self.col += 1;
            return true;
        }

        false
    }

    fn step(&mut self, direction: Direction) -> bool {
        if direction.contains(Direction::Up) {
            self.step_up()
        } else if direction.contains(Direction::Down) {
            self.step_down()
        } else if direction.contains(Direction::Left) {
            self.step_left()
        } else if direction.contains(Direction::Right) {
            self.step_right()
        } else {
            false
        }
    }
}

fn parse(input: &str) -> Matrix<(Token, Direction)> {
    let num_cols = input.lines().next().unwrap().len();

    let mut data = Vec::new();
    let mut num_rows = 0;
    for l in input.lines() {
        num_rows += 1;

        for c in l.chars() {
            let token = match c {
                '|' => Token::VerticalSplitter,
                '-' => Token::HorizontalSplitter,
                '/' => Token::RightMirror,
                '\\' => Token::LeftMirror,
                '.' => Token::Empty,
                _ => unreachable!(),
            };

            data.push((token, Direction::empty()));
        }
    }

    Matrix {
        data,
        rows: num_rows,
        cols: num_cols,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT1: &str = indoc::indoc! {r"
    .|...\....
    |.-.\.....
    .....|-...
    ........|.
    ..........
    .........\
    ..../.\\..
    .-.-/..|..
    .|....-|.\
    ..//.|....
    "};

    #[test]
    fn test_part1() {
        let answer = solve_part1(TEST_INPUT1);
        assert_eq!(answer, 46);
    }

    #[test]
    fn test_part2() {
        let answer = solve_part2(TEST_INPUT1);
        assert_eq!(answer, 51);
    }
}

#[cfg(feature = "divan")]
mod benches {
    use super::*;
    use divan::black_box;

    #[divan::bench]
    fn part1() {
        let answer = solve_part1(black_box(INPUT));
        assert_eq!(answer, 7939);
    }

    #[divan::bench]
    fn part2() {
        let answer = solve_part2(black_box(INPUT));
        assert_eq!(answer, todo!());
    }
}
