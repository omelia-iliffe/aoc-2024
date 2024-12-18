use std::ops::{Not, Range};

advent_of_code::solution!(6);

enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    fn from_byte(input: u8) -> Self {
        use Direction::*;
        match input {
            b'^' => Up,
            b'>' => Right,
            b'v' => Down,
            b'<' => Left,
            _ => panic!("not a valid dir"),
        }
    }

    fn move_in(
        &self,
        c: (usize, usize),
        x_range: &Range<i32>,
        y_range: &Range<i32>,
    ) -> Option<(usize, usize)> {
        use Direction::*;
        let (x, y) = (c.0 as i32, c.1 as i32);
        let (x, y) = match self {
            Up => (x, y - 1),
            Right => (x + 1, y),
            Down => (x, y + 1),
            Left => (x - 1, y),
        };
        (x_range.contains(&x) && y_range.contains(&y)).then_some((x as usize, y as usize))
    }

    fn rotate(self) -> Self {
        use Direction::*;
        match self {
            Up => Right,
            Right => Down,
            Down => Left,
            Left => Up,
        }
    }
}

fn index_2_cood(index: usize, line_len: usize) -> (usize, usize) {
    (index % line_len, index / line_len)
}
fn cood_2_index(c: (usize, usize), line_len: usize) -> usize {
    c.1 * line_len + c.0
}
pub fn part_one(input: &str) -> Option<u32> {
    let line_count = input.split_whitespace().count();
    let line_len = input.find("\n").unwrap() + 1;
    let x_range = 0..line_len as i32;
    let y_range = 0..line_count as i32;
    let gaurd_index = input.find("^").unwrap();
    let mut gaurd_cood = index_2_cood(gaurd_index, line_len);
    let mut dir = Direction::from_byte(input.as_bytes()[gaurd_index]);
    let mut visited = vec![gaurd_cood];
    while let Some(c) = dir.move_in(gaurd_cood, &x_range, &y_range) {
        match input.as_bytes()[cood_2_index(c, line_len)] {
            b'#' => dir = dir.rotate(),
            _ => {
                if visited.contains(&c).not() {
                    visited.push(c);
                }
                gaurd_cood = c
            }
        }
    }

    Some(visited.len() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(41));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
