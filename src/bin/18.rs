use glam::IVec2;
use itertools::Itertools;
use nom::bytes::complete::tag;
use nom::character::complete;
use nom::character::complete::line_ending;
use nom::multi::separated_list1;
use nom::sequence::separated_pair;
use nom::IResult;
use pathfinding::prelude::*;
use std::ops::Not;

advent_of_code::solution!(18);

const MAP_SIZE: IVec2 = if cfg!(test) {
    IVec2::new(7, 7)
} else {
    IVec2::new(71, 71)
};

fn parse_ivec2(input: &str) -> IResult<&str, IVec2> {
    let (input, (x, y)) = separated_pair(complete::i32, tag(","), complete::i32)(input)?;
    Ok((input, IVec2::new(x, y)))
}

const NUM_BYTES: usize = if cfg!(test) { 12 } else { 1024 };

struct Map {
    bytes: Vec<IVec2>,
    fallen_bytes: usize,
}

impl Map {
    fn fallen_bytes(&self) -> &[IVec2] {
        &self.bytes[..self.fallen_bytes]
    }
    #[allow(unused)]
    fn print_map(&self) {
        self.print_path(&Vec::new())
    }

    fn print_path(&self, path: &[IVec2]) {
        for y in 0..MAP_SIZE.y {
            for x in 0..MAP_SIZE.x {
                if self.fallen_bytes().contains(&IVec2::new(x, y)) {
                    print!("#");
                } else if path.contains(&IVec2::new(x, y)) {
                    print!("0");
                } else {
                    print!(".");
                }
            }
            println!();
        }
    }

    fn successors(&self, pos: &IVec2) -> Vec<IVec2> {
        // get the cells adj to pos
        [(-1, 0), (1, 0), (0, -1), (0, 1)]
            .into_iter()
            .map(|(x, y)| pos + &IVec2::new(x, y))
            // check it's not a fallen byte '#' and not out of bounds
            .filter(|in_grid| {
                self.fallen_bytes().contains(&in_grid).not()
                    && in_grid.cmpge(IVec2::ZERO).all()
                    && in_grid.cmplt(MAP_SIZE).all()
            })
            .collect_vec()
    }
}

fn parse(input: &str) -> Map {
    let (_, bytes) = separated_list1(line_ending, parse_ivec2)(input).unwrap();
    Map {
        bytes,
        fallen_bytes: NUM_BYTES,
    }
}
pub fn part_one(input: &str) -> Option<u32> {
    let map = parse(input);

    let path = bfs(
        &IVec2::ZERO,
        |p| map.successors(p),
        |p| p == &IVec2::new(MAP_SIZE.x - 1, MAP_SIZE.y - 1),
    )
    .expect("no path found");

    map.print_path(&path);
    Some(path.len() as u32 - 1)
}

pub fn part_two(input: &str) -> Option<String> {
    let mut map = parse(input);
    let mut first_known_success = map.fallen_bytes;
    let mut last_known_failure = map.bytes.len();
    let mut last_path = Vec::new();

    let last_byte = loop {
        if first_known_success + 1 == last_known_failure {
            break map.bytes[last_known_failure - 1];
        }
        map.fallen_bytes = first_known_success + (last_known_failure - first_known_success) / 2;
        match bfs(
            &IVec2::ZERO,
            |p| map.successors(p),
            |p| p == &IVec2::new(MAP_SIZE.x - 1, MAP_SIZE.y - 1),
        ) {
            None => last_known_failure = map.fallen_bytes,
            Some(p) => {
                first_known_success = map.fallen_bytes;
                last_path = p;
            }
        }
    };

    map.print_path(&last_path);
    Some(format!("{},{}", last_byte.x, last_byte.y))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(22));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY)).unwrap();
        assert_eq!(result, "6,1");
    }
}
