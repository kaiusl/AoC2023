use std::str::FromStr;

const DAY: u8 = 3;
pub const INPUT: &str = include_str!("../inputs/day3.txt");

pub fn run() {
    solve(INPUT);
}

fn solve(input: &str) {
    let answer_part1 = solve_part1(input);
    println!("day{DAY}::part1 answer: {}", answer_part1);

    let answer_part2 = solve_part2(input);
    println!("day{DAY}::part2 answer: {}", answer_part2);
}

fn is_symbol(c: char) -> bool {
    !c.is_ascii_digit() && c != '.'
}

fn solve_part1(input: &str) -> u64 {
    let mut lines = input.lines();

    let first_line = lines.next().unwrap();
    let dummy_line = ".".repeat(first_line.len());

    let mut line_buf = [&dummy_line, first_line, lines.next().unwrap_or(&dummy_line)];

    let mut end = false;
    let mut sum = 0;
    loop {
        //print!("{line_num}: ");
        let mut is_important = false;
        let mut number_start = None;

        let iter = line_buf[0]
            .chars()
            .zip(line_buf[1].chars())
            .zip(line_buf[2].chars())
            .enumerate();

        for (pos, ((c0, c1), c2)) in iter {
            match (c0, c1, c2) {
                (above, middle, below) if middle.is_ascii_digit() && number_start.is_none() => {
                    // Number started
                    number_start = Some(pos);
                    if !is_important && (is_symbol(above) || is_symbol(below)) {
                        is_important = true;
                    }
                }
                (above, middle, below) if middle.is_ascii_digit() && number_start.is_some() => {
                    // Number continuing
                    if !is_important && (is_symbol(above) || is_symbol(below)) {
                        is_important = true;
                    }
                }
                (above, middle, below) if !middle.is_ascii_digit() && number_start.is_some() => {
                    // Number ended
                    let was_symbol = is_symbol(above) || is_symbol(below) || is_symbol(middle);
                    if !is_important && was_symbol {
                        is_important = true;
                    }

                    if is_important {
                        let start = number_start.unwrap();
                        let end = pos;
                        let num = &line_buf[1][start..end];
                        //print!("{num}, ");

                        let num = u64::from_str(num).unwrap();
                        sum += num;
                    }

                    if !was_symbol {
                        is_important = false;
                    }

                    number_start = None;
                }
                (c0, c1, c2) if is_symbol(c0) || is_symbol(c1) || is_symbol(c2) => {
                    is_important = true;
                }
                _ => {
                    // ignore
                    is_important = false;
                }
            }
        }

        if is_important && number_start.is_some() {
            let start = number_start.unwrap();
            let num = &line_buf[1][start..];
            //print!("{num}, ");

            let num = u64::from_str(num).unwrap();
            sum += num;
        }

        if let Some(line) = lines.next() {
            line_buf[0] = line;
            line_buf.rotate_left(1);
        } else if !end {
            line_buf[0] = &dummy_line;
            line_buf.rotate_left(1);
            end = true;
        } else {
            break;
        }
    }

    sum
}

#[derive(Debug, Clone, Copy)]
enum Position {
    NW,
    N,
    NE,
    E,
    SE,
    S,
    SW,
    W,
    Unknown,
}

#[derive(Debug, Clone)]
struct GearNumberPositions {
    buf: [Position; 2],
    count: usize,
}

impl GearNumberPositions {
    fn new() -> Self {
        Self {
            buf: [Position::Unknown; 2],
            count: 0,
        }
    }

    fn push(&mut self, pos: Position) -> Result<(), ()> {
        if self.count == 2 {
            return Err(());
        }

        self.buf[self.count] = pos;
        self.count += 1;
        Ok(())
    }
}

fn solve_part2(input: &str) -> u64 {
    let mut lines = input.lines();

    let first_line = lines.next().unwrap();
    let line_len = first_line.len();
    let dummy_line = ".".repeat(first_line.len());

    let mut line_buf = [&dummy_line, first_line, lines.next().unwrap_or(&dummy_line)];

    let mut end = false;
    let mut sum = 0;
    loop {
        let mut chars = line_buf[1].char_indices().peekable();
        let mut num_start: Option<usize> = None;
        while let Some((pos, c)) = chars.next() {
            match c {
                '*' => {
                    let mut positions = GearNumberPositions::new();

                    // west
                    if num_start.is_some() {
                        positions.push(Position::W).unwrap();
                    }

                    // nw, n, ne
                    let is_at_line_end = line_len == pos - 1;
                    let mut above_adjacent = line_buf[0]
                        [pos - 1..pos + 1 + !is_at_line_end as usize]
                        .chars()
                        .map(|c| c.is_ascii_digit());
                    let is_nw_number = above_adjacent.next().unwrap();
                    let is_n_number = above_adjacent.next().unwrap();
                    if is_n_number {
                        // if n is number, then nw and ne are part of the same number, don't check atm
                        positions.push(Position::N).unwrap();
                    } else {
                        if is_nw_number && positions.push(Position::NW).is_err() {
                            // more than two numbers are adjacent to *
                            continue;
                        }
                        //dbg!(is_nw_number);
                        if let Some(ne) = above_adjacent.next() {
                            // dbg!(ne);
                            if ne && positions.push(Position::NE).is_err() {
                                // more than two numbers are adjacent to *
                                continue;
                            }
                        }
                    }

                    // east
                    let is_next_number = chars.peek().map(|(_, c)| c.is_ascii_digit());
                    if let Some(is_next_number) = is_next_number {
                        if is_next_number && positions.push(Position::E).is_err() {
                            // more than two numbers are adjacent to *
                            continue;
                        }
                    }

                    // sw, s, se
                    let mut below_adjacent = line_buf[2]
                        [pos - 1..pos + 1 + !is_at_line_end as usize]
                        .chars()
                        .map(|c| c.is_ascii_digit());
                    let is_sw_number = below_adjacent.next().unwrap();
                    let is_s_number = below_adjacent.next().unwrap();
                    if is_s_number {
                        // if n is number, then nw and ne are part of the same number, don't check atm
                        if positions.push(Position::S).is_err() {
                            continue;
                        }
                    } else {
                        if is_sw_number && positions.push(Position::SW).is_err() {
                            // more than two numbers are adjacent to *
                            continue;
                        }
                        if let Some(se) = below_adjacent.next() {
                            if se && positions.push(Position::SE).is_err() {
                                // more than two numbers are adjacent to *
                                continue;
                            }
                        }
                    }

                    if positions.count == 2 {
                        // exactly two numbers are adjacent to *
                        let mut prod = 1;
                        for num_pos in &positions.buf {
                            match num_pos {
                                Position::NW => {
                                    let end = pos;
                                    let line = &line_buf[0][..end];
                                    let num = parse_num_from_back(line);
                                    prod *= num;
                                }
                                Position::SW => {
                                    let end = pos;
                                    let line = &line_buf[2][..end];
                                    let num = parse_num_from_back(line);
                                    prod *= num;
                                }
                                Position::NE => {
                                    let start = pos + 1;
                                    let line = &line_buf[0][start..];
                                    let num = parse_num_from_front(line);
                                    prod *= num;
                                }
                                Position::SE => {
                                    let start = pos + 1;
                                    let line = &line_buf[2][start..];
                                    let num = parse_num_from_front(line);
                                    prod *= num;
                                }
                                Position::E => {
                                    let mut end = pos + 2;
                                    while let Some((p, d)) = chars.peek() {
                                        if !d.is_ascii_digit() {
                                            end = *p;
                                            break;
                                        }
                                        chars.next();
                                    }

                                    let num = &line_buf[1][pos + 1..end];
                                    prod *= u64::from_str(num).unwrap();
                                }
                                Position::N => {
                                    let line = line_buf[0];
                                    let num = parse_num_from_middle(line, pos);
                                    prod *= num;
                                }
                                Position::S => {
                                    let line = line_buf[2];
                                    let num = parse_num_from_middle(line, pos);
                                    prod *= num;
                                }

                                Position::W => {
                                    let num_start = num_start.unwrap();
                                    let end = pos;
                                    let num = &line_buf[1][num_start..end];
                                    prod *= u64::from_str(num).unwrap();
                                }
                                Position::Unknown => unreachable!(),
                            }
                        }

                        sum += prod;
                    }
                }
                c if c.is_ascii_digit() && num_start.is_none() => {
                    // number start
                    num_start = Some(pos);
                }
                c if c.is_ascii_digit() && num_start.is_some() => {
                    //number continuing
                }
                _ => {
                    num_start = None;
                }
            }
        }

        if let Some(line) = lines.next() {
            line_buf[0] = line;
            line_buf.rotate_left(1);
        } else if !end {
            line_buf[0] = &dummy_line;
            line_buf.rotate_left(1);
            end = true;
        } else {
            break;
        }
    }

    sum
}

fn parse_num_from_front(input: &str) -> u64 {
    let mut chars = input.char_indices();
    let mut end = 1;
    while let Some((p, d)) = chars.next() {
        if d.is_ascii_digit() {
            end = p + 1;
        } else {
            break;
        }
    }

    let num = &input[..end];
    u64::from_str(num).unwrap()
}

fn parse_num_from_back(input: &str) -> u64 {
    let mut chars = input.char_indices();
    let mut start = input.len() - 1;
    while let Some((p, d)) = chars.next_back() {
        if d.is_ascii_digit() {
            start = p;
        } else {
            break;
        }
    }

    let num = &input[start..];
    u64::from_str(num).unwrap()
}

fn parse_num_from_middle(input: &str, mid: usize) -> u64 {
    let mut start = mid;
    let mut end: usize = mid + 1;

    let (front, back) = input.split_at(mid);
    let mut front = front.char_indices();
    while let Some((p, d)) = front.next_back() {
        if d.is_ascii_digit() {
            start = p;
        } else {
            break;
        }
    }

    let mut back = back.char_indices();
    while let Some((p, d)) = back.next() {
        if d.is_ascii_digit() {
            end = mid + p + 1;
        } else {
            break;
        }
    }

    let num = &input[start..end];
    u64::from_str(num).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT1: &str = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

    #[test]
    fn test_part1() {
        let answer = solve_part1(TEST_INPUT1);
        assert_eq!(answer, 4361);
    }

    #[test]
    fn test_part2() {
        let answer = solve_part2(TEST_INPUT1);
        assert_eq!(answer, 467835);
    }

    #[test]
    fn test_part2_2() {
        let input = "
....761.169...............=...524........&......152..........*...975.994.........*....122..........858*...........%.......776...........*...
...........-..180/.850..............$.....524....-...........940.=......*......199........963..............#........836...*.....34...543.448";
        let answer = solve_part2(input.trim());
        assert_eq!(answer, 543 * 448);
    }
}

#[cfg(feature = "divan")]
mod benches {
    use super::*;
    use divan::black_box;

    #[divan::bench]
    fn part1() {
        let answer = solve_part1(black_box(INPUT));
        assert_eq!(answer, 535351);
    }

    #[divan::bench]
    fn part2() {
        let answer = solve_part2(black_box(INPUT));
        assert_eq!(answer, 87287096);
    }
}
