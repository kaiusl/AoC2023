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
    let mut matrix = parse(input);
    let mut cursor = MatrixCursor::new(&mut matrix, (0, 0));
    let mut splits_queue = VecDeque::new();

    // Match the first position and get the next step direction
    let next_dir = match cursor.get_current() {
        Token::Empty => {
            cursor.set_current(Token::LightRight);
            Direction::Right
        }
        Token::LeftMirror => {
            cursor.set_current(Token::EnergizedLeftMirror);
            Direction::Down
        }
        Token::VerticalSplitter => {
            cursor.set_current(Token::EnergizedVerticalSplitter);
            Direction::Down
        }
        Token::HorizontalSplitter => {
            cursor.set_current(Token::EnergizedHorizontalSplitter);
            Direction::Right
        }
        _ => {
            // RightMirror shoots the light up straight out of the grid
            return 1;
        }
    };
    splits_queue.push_back((next_dir, cursor.get_current_pos()));

    // Loop through all split light path and mark all energized tiles
    //
    // Light path ends if it exits the grid or maps onto a loop
    // (that is starts moving in the same direction with some earlier path)
    while let Some((dir, pos)) = splits_queue.pop_front() {
        let mut prev_direction = dir;
        cursor.set_pos(pos);
        cursor.step(dir);

        loop {
            let Some((next_dir, split)) = get_next_step(&mut cursor, prev_direction) else {
                break;
            };

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
        .filter(|t| {
            matches!(
                t,
                Token::LightUp
                    | Token::LightDown
                    | Token::LightLeft
                    | Token::LightRight
                    | Token::EnergizedHorizontalSplitter
                    | Token::EnergizedLeftMirror
                    | Token::EnergizedRightMirror
                    | Token::EnergizedVerticalSplitter
            )
        })
        .count() as u64
}

/// Get the next step direction and the next split position
///
/// Marks the current position as energized
fn get_next_step(
    cursor: &mut MatrixCursor<'_, Token>,
    prev_direction: Direction,
) -> Option<(Direction, Option<(Direction, Pos)>)> {
    let mut next_dir = prev_direction;
    let mut split = None;

    match (prev_direction, cursor.get_current()) {
        (Direction::Up, Token::Empty) => {
            cursor.set_current(Token::LightUp);
        }
        (Direction::Up, Token::LeftMirror | Token::EnergizedLeftMirror) => {
            cursor.set_current(Token::EnergizedLeftMirror);
            next_dir = Direction::Left;
        }
        (Direction::Up, Token::RightMirror | Token::EnergizedRightMirror) => {
            cursor.set_current(Token::EnergizedRightMirror);
            next_dir = Direction::Right;
        }
        (Direction::Up, Token::LightUp) => return None,
        (Direction::Down, Token::Empty) => {
            cursor.set_current(Token::LightDown);
        }
        (Direction::Down, Token::LeftMirror | Token::EnergizedLeftMirror) => {
            cursor.set_current(Token::EnergizedLeftMirror);
            next_dir = Direction::Right;
        }
        (Direction::Down, Token::RightMirror | Token::EnergizedRightMirror) => {
            cursor.set_current(Token::EnergizedRightMirror);
            next_dir = Direction::Left;
        }
        (Direction::Down, Token::LightDown) => return None,
        (
            Direction::Up | Direction::Down,
            Token::HorizontalSplitter | Token::EnergizedHorizontalSplitter,
        ) => {
            cursor.set_current(Token::EnergizedHorizontalSplitter);
            split = Some((Direction::Right, cursor.get_current_pos()));
            next_dir = Direction::Left;
        }

        (Direction::Left, Token::Empty) => {
            cursor.set_current(Token::LightLeft);
        }
        (Direction::Left, Token::LeftMirror | Token::EnergizedLeftMirror) => {
            cursor.set_current(Token::EnergizedLeftMirror);
            next_dir = Direction::Up;
        }
        (Direction::Left, Token::RightMirror | Token::EnergizedRightMirror) => {
            cursor.set_current(Token::EnergizedRightMirror);
            next_dir = Direction::Down
        }
        (Direction::Left, Token::LightLeft) => return None,

        (Direction::Right, Token::Empty) => {
            cursor.set_current(Token::LightRight);
        }
        (Direction::Right, Token::LeftMirror | Token::EnergizedLeftMirror) => {
            cursor.set_current(Token::EnergizedLeftMirror);
            next_dir = Direction::Down;
        }
        (Direction::Right, Token::RightMirror | Token::EnergizedRightMirror) => {
            cursor.set_current(Token::EnergizedRightMirror);
            next_dir = Direction::Up;
        }
        (Direction::Right, Token::LightRight) => return None,

        (
            Direction::Right | Direction::Left,
            Token::VerticalSplitter | Token::EnergizedVerticalSplitter,
        ) => {
            cursor.set_current(Token::EnergizedVerticalSplitter);
            split = Some((Direction::Up, cursor.get_current_pos()));
            next_dir = Direction::Down;
        }
        (_, tok) => match tok {
            Token::VerticalSplitter => {
                cursor.set_current(Token::EnergizedVerticalSplitter);
            }
            Token::HorizontalSplitter => {
                cursor.set_current(Token::EnergizedHorizontalSplitter);
            }
            Token::LeftMirror => {
                cursor.set_current(Token::EnergizedLeftMirror);
            }
            Token::RightMirror => {
                cursor.set_current(Token::EnergizedRightMirror);
            }
            _ => {}
        },
    }

    Some((next_dir, split))
}

fn solve_part2(input: &str) -> u64 {
    todo!()
}

#[derive(Debug, Clone, Copy)]
enum Token {
    Empty,
    VerticalSplitter,
    HorizontalSplitter,
    LeftMirror,
    RightMirror,

    LightUp,
    LightDown,
    LightLeft,
    LightRight,
    EnergizedVerticalSplitter,
    EnergizedHorizontalSplitter,
    EnergizedLeftMirror,
    EnergizedRightMirror,
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Token::Empty => write!(f, "."),
            Token::VerticalSplitter | Token::EnergizedVerticalSplitter => write!(f, "|"),
            Token::HorizontalSplitter | Token::EnergizedHorizontalSplitter => write!(f, "-"),
            Token::LeftMirror | Token::EnergizedLeftMirror => write!(f, "\\"),
            Token::RightMirror | Token::EnergizedRightMirror => write!(f, "/"),
            Token::LightUp => write!(f, "^"),
            Token::LightDown => write!(f, "v"),
            Token::LightLeft => write!(f, "<"),
            Token::LightRight => write!(f, ">"),
        }
    }
}

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
        match direction {
            Direction::Up => self.step_up(),
            Direction::Down => self.step_down(),
            Direction::Left => self.step_left(),
            Direction::Right => self.step_right(),
        }
    }
}

fn parse(input: &str) -> Matrix<Token> {
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

            data.push(token);
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
        assert_eq!(answer, todo!());
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
