const DAY: u8 = 10;
pub const INPUT: &str = include_str!("../inputs/day10.txt");

pub fn run() {
    solve(INPUT);
}

fn solve(input: &str) {
    let answer_part1 = solve_part1::<140>(input);
    println!("day{DAY}::part1 answer: {}", answer_part1);

    let answer_part2 = solve_part2::<140>(input);
    println!("day{DAY}::part2 answer: {}", answer_part2);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
    Unknown,
}

fn solve_part1<const SIZE: usize>(input: &str) -> u64 {
    let (data, start) = parse::<SIZE>(input);
    let mut cursors = [data.cursor(start), data.cursor(start)];

    // Two pipes must be connected to the start, find them
    let mut next_step_dirs = find_all_start_dirs(&cursors[0]);

    let mut steps = 1;
    loop {
        for i in [0, 1] {
            cursors[i].step(next_step_dirs[i]);
            let current_pipe = cursors[i].current();
            next_step_dirs[i] =
                next_step_dir(next_step_dirs[i], *current_pipe).unwrap_or(Dir::Unknown);
        }

        if matches!(next_step_dirs[0], Dir::Unknown)
            || cursors[0].current_pos() == cursors[1].current_pos()
        {
            break;
        }

        steps += 1;
    }

    steps as u64
}

fn solve_part2<const SIZE: usize>(input: &str) -> u64 {
    // 1. Find the path
    // 2. Move up to down and count horizontal pipes
    //    Note that ┌---┘ and └---┐ are form a horizontal pipe as well.
    //    We can detect them by one of the corners from each.
    //    Not 100% sure about it but seems to work with current corners.
    //    UL and DL give 1 larger result for some reason...
    // 3. Inside part has odd parity
    // see here https://www.reddit.com/r/adventofcode/comments/18evyu9/comment/kcqgo61/
    // I didn't realize that corners can form a horizontal pipe as well and we need to count them
    let (data, start) = parse::<SIZE>(input);
    let mut cursor = data.cursor(start);

    let mut next_dir = find_any_start_dir(&mut cursor);
    let mut path = [[Pipe::None; SIZE]; SIZE];
    loop {
        cursor.step(next_dir);
        let prev_dir = next_dir;
        let current_pipe = cursor.current();

        let (x, y) = cursor.current_pos();
        path[x][y] = if matches!(
            current_pipe,
            Pipe::Horizontal | Pipe::URCorner | Pipe::DRCorner
        ) {
            Pipe::Horizontal
        } else {
            Pipe::Start
        };
        let Some(next) = next_step_dir(prev_dir, *current_pipe) else {
            break;
        };
        next_dir = next;
    }

    let mut count_inside = 0;
    for c in path {
        let mut inside = false;
        for &it in &c {
            if it.is_horizontal() {
                inside = !inside;
            } else if inside && it.is_none() {
                count_inside += 1;
            }
        }
    }

    count_inside
}

fn find_all_start_dirs<const SIZE: usize>(cursor: &MatrixCursor<'_, Pipe, SIZE>) -> [Dir; 2] {
    let mut dirs = [Dir::Unknown; 2];

    if let Some(Pipe::Vertical | Pipe::DLCorner | Pipe::DRCorner) = cursor.peek_up() {
        dirs[0] = Dir::Up;
    } else {
        // cannot be part of the loop
    }

    if let Some(Pipe::Horizontal | Pipe::ULCorner | Pipe::DLCorner) = cursor.peek_right() {
        match dirs[0] {
            Dir::Unknown => dirs[0] = Dir::Right,
            _ => dirs[1] = Dir::Right,
        }
    } else {
        // cannot be part of the loop
    }

    if let Some(Pipe::Vertical | Pipe::URCorner | Pipe::ULCorner) = cursor.peek_down() {
        match dirs[0] {
            Dir::Unknown => dirs[0] = Dir::Down,
            _ => dirs[1] = Dir::Down,
        }
    } else {
        // cannot be part of the loop
    }

    if let Some(Pipe::Horizontal | Pipe::URCorner | Pipe::DRCorner) = cursor.peek_left() {
        match dirs[0] {
            Dir::Unknown => dirs[0] = Dir::Left,
            _ => dirs[1] = Dir::Left,
        }
    } else {
        // cannot be part of the loop
    }
    dirs
}

fn find_any_start_dir<const N: usize>(cursor: &mut MatrixCursor<'_, Pipe, N>) -> Dir {
    if let Some(Pipe::Vertical | Pipe::DLCorner | Pipe::DRCorner) = cursor.peek_up() {
        return Dir::Up;
    }

    if let Some(Pipe::Horizontal | Pipe::ULCorner | Pipe::DLCorner) = cursor.peek_right() {
        return Dir::Right;
    }

    if let Some(Pipe::Vertical | Pipe::URCorner | Pipe::ULCorner) = cursor.peek_down() {
        return Dir::Down;
    }

    if let Some(Pipe::Horizontal | Pipe::URCorner | Pipe::DRCorner) = cursor.peek_left() {
        return Dir::Left;
    }

    unreachable!()
}

fn next_step_dir(came_from: Dir, current_pipe: Pipe) -> Option<Dir> {
    Some(match (came_from, current_pipe) {
        (Dir::Up, Pipe::Vertical) => Dir::Up,
        (Dir::Up, Pipe::DLCorner) => Dir::Left,
        (Dir::Up, Pipe::DRCorner) => Dir::Right,

        (Dir::Down, Pipe::Vertical) => Dir::Down,
        (Dir::Down, Pipe::ULCorner) => Dir::Left,
        (Dir::Down, Pipe::URCorner) => Dir::Right,

        (Dir::Left, Pipe::Horizontal) => Dir::Left,
        (Dir::Left, Pipe::URCorner) => Dir::Up,
        (Dir::Left, Pipe::DRCorner) => Dir::Down,

        (Dir::Right, Pipe::Horizontal) => Dir::Right,
        (Dir::Right, Pipe::ULCorner) => Dir::Up,
        (Dir::Right, Pipe::DLCorner) => Dir::Down,
        (_, Pipe::Start) => return None, // Both cursors must loop at the same time
        s => unreachable!("{:?}", s),
    })
}

fn parse<const N: usize>(input: &str) -> (Matrix<Pipe, N>, (usize, usize)) {
    let mut data = [[Pipe::None; N]; N];

    let mut start = (0, 0);
    for (y, l) in input.lines().enumerate() {
        for (x, c) in l.chars().enumerate() {
            data[x][y] = Pipe::from_char(c);
            if c == 'S' {
                start = (x, y);
            }
        }
    }

    (Matrix { data }, start)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Pipe {
    Vertical,
    Horizontal,
    ULCorner,
    URCorner,
    DLCorner,
    DRCorner,
    None,
    Start,
}

impl Pipe {
    fn from_char(c: char) -> Self {
        match c {
            '|' => Self::Vertical,
            '-' => Self::Horizontal,
            'L' => Self::URCorner,
            'J' => Self::ULCorner,
            '7' => Self::DLCorner,
            'F' => Self::DRCorner,
            '.' => Self::None,
            'S' => Self::Start,
            _ => unreachable!(),
        }
    }

    /// Returns `true` if the pipe is [`Horizontal`].
    ///
    /// [`Horizontal`]: Pipe::Horizontal
    #[must_use]
    fn is_horizontal(&self) -> bool {
        matches!(self, Self::Horizontal)
    }

    /// Returns `true` if the pipe is [`None`].
    ///
    /// [`None`]: Pipe::None
    #[must_use]
    fn is_none(&self) -> bool {
        matches!(self, Self::None)
    }
}

#[derive(Debug, Clone)]
struct Matrix<T, const N: usize> {
    data: [[T; N]; N],
}

impl<T, const N: usize> Matrix<T, N> {
    fn cursor(&self, pos: (usize, usize)) -> MatrixCursor<T, N> {
        MatrixCursor::new(&self.data, pos)
    }
}

#[derive(Debug, Clone)]
struct MatrixCursor<'a, T, const N: usize> {
    data: &'a [[T; N]; N],
    posx: usize,
    posy: usize,
}

impl<'a, T, const N: usize> MatrixCursor<'a, T, N> {
    fn new(data: &'a [[T; N]; N], pos: (usize, usize)) -> Self {
        Self {
            data,
            posx: pos.0,
            posy: pos.1,
        }
    }

    fn current(&self) -> &T {
        &self.data[self.posx][self.posy]
    }

    fn current_pos(&self) -> (usize, usize) {
        (self.posx, self.posy)
    }

    fn peek_up(&self) -> Option<&T> {
        if self.posy > 0 {
            Some(&self.data[self.posx][self.posy - 1])
        } else {
            None
        }
    }

    fn peek_down(&self) -> Option<&T> {
        if self.posy < N - 1 {
            Some(&self.data[self.posx][self.posy + 1])
        } else {
            None
        }
    }

    fn peek_left(&self) -> Option<&T> {
        if self.posx > 0 {
            Some(&self.data[self.posx - 1][self.posy])
        } else {
            None
        }
    }

    fn peek_right(&self) -> Option<&T> {
        if self.posx < N - 1 {
            Some(&self.data[self.posx + 1][self.posy])
        } else {
            None
        }
    }

    fn step(&mut self, dir: Dir) -> bool {
        match dir {
            Dir::Up => self.move_up(),
            Dir::Down => self.move_down(),
            Dir::Left => self.move_left(),
            Dir::Right => self.move_right(),
            Dir::Unknown => unreachable!(),
        }
    }

    fn move_up(&mut self) -> bool {
        if self.posy > 0 {
            self.posy -= 1;
            return true;
        }

        false
    }

    fn move_down(&mut self) -> bool {
        if self.posy < N - 1 {
            self.posy += 1;
            return true;
        }

        false
    }

    fn move_left(&mut self) -> bool {
        if self.posx > 0 {
            self.posx -= 1;
            return true;
        }

        false
    }

    fn move_right(&mut self) -> bool {
        if self.posx < N - 1 {
            self.posx += 1;
            return true;
        }

        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT1: &str = indoc::indoc! {"
    ..F7.
    .FJ|.
    SJ.L7
    |F--J
    LJ...
    "};

    #[test]
    fn test_parse() {
        let cursor = parse::<5>(TEST_INPUT1);
        println!("{:?}", cursor);

        println!("{:?}", cursor.0.cursor((2, 2)).peek_right());
    }

    #[test]
    fn test_part1() {
        let answer = solve_part1::<5>(TEST_INPUT1);
        assert_eq!(answer, 8);
    }

    #[test]
    fn test_part2() {
        let answer = solve_part2::<5>(TEST_INPUT1);
        assert_eq!(answer, 1);
    }

    const TEST_INPUT2: &str = indoc::indoc! {"
    ...........
    .S-------7.
    .|F-----7|.
    .||.....||.
    .||.....||.
    .|L-7.F-J|.
    .|..|.|..|.
    .L--J.L--J.
    ...........
    "};

    #[test]
    fn test_part2_2() {
        let answer = solve_part2::<11>(TEST_INPUT2);
        assert_eq!(answer, 4);
    }
    const TEST_INPUT3: &str = indoc::indoc! {"
    FF7FSF7F7F7F7F7F---7
    L|LJ||||||||||||F--J
    FL-7LJLJ||||||LJL-77
    F--JF--7||LJLJ7F7FJ-
    L---JF-JLJ.||-FJLJJ7
    |F|F-JF---7F7-L7L|7|
    |FFJF7L7F-JF7|JL---7
    7-L-JL7||F7|L7F-7F7|
    L.L7LFJ|||||FJL7||LJ
    L7JLJL-JLJLJL--JLJ.L
    "};

    #[test]
    fn test_part2_3() {
        let answer = solve_part2::<25>(TEST_INPUT3);
        assert_eq!(answer, 10);
    }
}

#[cfg(feature = "divan")]
mod benches {
    use super::*;
    use divan::black_box;

    #[divan::bench]
    fn part1() {
        let answer = solve_part1::<140>(black_box(INPUT));
        assert_eq!(answer, 6907);
    }

    #[divan::bench]
    fn part2() {
        let answer = solve_part2::<140>(black_box(INPUT));
        assert_eq!(answer, 541);
    }
}
