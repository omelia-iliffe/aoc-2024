use std::cmp::PartialEq;
use std::collections::HashMap;
use std::ops::{Not, Range};
use itertools::Itertools;

advent_of_code::solution!(6);

#[derive(Debug, Clone, Copy, PartialEq)]
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
            _ => panic!("not a valid dir")
        }
    }

    fn as_byte(&self) -> u8 {
        match self {
            Direction::Up | Direction::Down=> b'|',
            Direction::Right |Direction::Left => b'-',
        }
    }
    
    fn move_in(&self, c: (usize, usize),
               x_range: &Range<i32>,
               y_range: &Range<i32>,
 ) -> Option<(usize, usize)> {
        use Direction::*;
        let (x, y) = (c.0 as i32, c.1 as i32);
        let (x, y) = match self {
            Up => (x, y - 1),
            Right => (x+1, y),
            Down => (x, y+1),
            Left => (x-1, y),
        };
        (x_range.contains(&x) && y_range.contains(&y)).then_some((x as usize, y as usize))
    }

    fn rotate(self) -> Self{
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
    while let Some(c)= dir.move_in(gaurd_cood, &x_range, &y_range) {
        match input.as_bytes()[cood_2_index(c, line_len)] {
            b'#' => {
                dir = dir.rotate()
            }
            _ => {
                if visited.contains(&c).not() {
                    visited.push(c);
                }
                gaurd_cood = c
            }
        }
    };
    Some(visited.len() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let line_count = input.split_whitespace().count();
    let line_len = input.find("\n").unwrap() + 1;
    let x_range = 0..line_len as i32;
    let y_range = 0..line_count as i32;
    let gaurd_index = input.find("^").unwrap();
    let mut gaurd_cood = index_2_cood(gaurd_index, line_len);
    let mut dir = Direction::from_byte(input.as_bytes()[gaurd_index]);
    let mut block_positions = 0;
    let mut output_map = input.as_bytes().to_vec();
    let mut visited_positions = HashMap::new();
    visited_positions.insert(gaurd_cood, vec![dir]);
    while let Some(c)= dir.move_in(gaurd_cood, &x_range, &y_range) {
        match input.as_bytes()[cood_2_index(c, line_len)] {
            b'#' => {
                dir = dir.rotate();
                let char = match output_map[cood_2_index(c, line_len)] {
                    b'0' => b'0',
                    b'^' => b'^',
                    _ => b'+'
                };
                output_map[cood_2_index(gaurd_cood, line_len)] = char;
            }
            _ => {
                'test: {
                    let Some(potential_block) = dir.move_in(c, &x_range, &y_range) else {
                        break 'test
                    };
                    let test_dir = dir.rotate();
                    let mut test_c = c;
                    while let Some(right_c) = test_dir.move_in(test_c, &x_range, &y_range) {
                        let next_c = test_dir.move_in(right_c, &x_range, &y_range);
                        let next_c = next_c.map(|c| input.as_bytes()[cood_2_index(c, line_len)]);
                        if matches!(next_c, Some(b'#')) {
                            if let Some(right_c) = test_dir.rotate().move_in(right_c, &x_range, &y_range) {
                                if visited_positions.get(&right_c).is_some_and(|p| p.contains(&test_dir.rotate())) {
                                    println!("found loop, if block places at {:?}", potential_block );

                                    output_map[cood_2_index(potential_block, line_len)] = b'0';
                                    // let map_string = String::from_utf8_lossy(&output_map);
                                    // map_string.split_whitespace().for_each(|line| {
                                    //     println!("{line}");
                                    // });
                                    block_positions += 1;
                                }
                            }
                        }
                        test_c = right_c;
                    }
                }
                let char = match output_map[cood_2_index(c, line_len)] {
                    b'0' => b'0',
                    b'^' => b'^',
                    b if dir.rotate().as_byte() == b => b'+',
                    _ => dir.as_byte(),
                };
                output_map[cood_2_index(c, line_len)] = char;
                visited_positions.entry(c).or_default().push(dir);
                gaurd_cood = c
            }
        }
    };
    let map_string = String::from_utf8_lossy(&output_map);
    map_string.split_whitespace().for_each(|line| {
        println!("{line}");
    });
    Some(block_positions)
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
        assert_eq!(result, Some(6));
    }
}
