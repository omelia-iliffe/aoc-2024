use glam::IVec2;
use itertools::Itertools;
use pathfinding::num_traits::CheckedSub;
use pathfinding::prelude::{bfs, dijkstra};
use std::collections::{HashMap, VecDeque};
use std::ops::{AddAssign, Not};

advent_of_code::solution!(20);

struct Map {
    map_size: IVec2,
    walls: Vec<IVec2>,
    start: IVec2,
    end: IVec2,
}

impl Map {
    fn successors(&self, pos: &IVec2) -> Vec<IVec2> {
        [IVec2::X, -IVec2::X, IVec2::Y, -IVec2::Y]
            .iter()
            .flat_map(|m| {
                let next = pos + m;
                self.walls.contains(&next).not().then_some(next)
            })
            .collect()
    }

    fn jumpable(&self, pos: &IVec2) -> Vec<IVec2> {
        [IVec2::X, -IVec2::X, IVec2::Y, -IVec2::Y]
            .iter()
            .flat_map(|m| {
                let next = pos + m;
                let next_jump = next + m;
                (self.walls.contains(&next) && self.walls.contains(&next_jump).not())
                    .then_some(next_jump)
            })
            .collect()
    }
    #[allow(unused)]
    fn print_path(&self, path: &[IVec2]) {
        for y in 0..self.map_size.y {
            for x in 0..self.map_size.x {
                if self.walls.contains(&IVec2::new(x, y)) {
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
}

fn parse(input: &str) -> Map {
    let walls = input
        .split_whitespace()
        .enumerate()
        .flat_map(|(y, line)| {
            line.char_indices()
                .filter(|(_, c)| c == &'#')
                .map(move |(x, _)| IVec2::new(x as i32, y as i32))
        })
        .collect_vec();
    let start = input
        .split_whitespace()
        .enumerate()
        .flat_map(move |(y, line)| {
            line.char_indices()
                .find(|(_, c)| c == &'S')
                .map(|(x, _)| IVec2::new(x as i32, y as i32))
        })
        .exactly_one()
        .unwrap();
    let end = input
        .split_whitespace()
        .enumerate()
        .flat_map(move |(y, line)| {
            line.char_indices()
                .find(|(_, c)| c == &'E')
                .map(|(x, _)| IVec2::new(x as i32, y as i32))
        })
        .exactly_one()
        .unwrap();
    let height = input.split_whitespace().count();
    let width = input.split_whitespace().next().unwrap().len();
    Map {
        map_size: IVec2::new(width as i32, height as i32),
        walls,
        start,
        end,
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let map = parse(input);
    let path = bfs(&map.start, |(p)| map.successors(p), |p| p == &map.end).unwrap();
    let mut paths: HashMap<usize, usize> = HashMap::new();
    for (i, p) in path.iter().enumerate() {
        map.jumpable(p)
            .iter()
            .flat_map(|j| {
                path.iter()
                    .position(|p| p == j)
                    // subtracting 2 for the cheat length
                    .and_then(|less| less.checked_sub(i)?.checked_sub(2))
            })
            .for_each(|len| {
                paths.entry(len).or_default().add_assign(1);
            });
    }

    for (len, count) in paths.iter().sorted_by_key(|(len, _)| *len) {
        if len == &1 {
            println!("There is one cheat that saves {} picoseconds.", count);
        } else {
            println!("There are {} cheats that save {} picoseconds.", count, len);
        }
    }

    let over_100 = paths
        .iter()
        .filter(|(len, count)| len >= &&100)
        .map(|(_, count)| *count as u32)
        .sum();
    Some(over_100)
}

pub fn part_two(input: &str) -> Option<u32> {
    let map = parse(input);
    let path = bfs(&map.start, |(p)| map.successors(p), |p| p == &map.end).unwrap();
    let mut paths: HashMap<usize, usize> = HashMap::new();
    let base_cost = path.len();

    // borrowed from https://github.com/ChristopherBiscardi/advent-of-code/blob/main/2024/rust/day-20/src/part2.rs :)
    let result = path
        .iter()
        .enumerate()
        .tuple_combinations()
        .filter_map(|((start_cost, start_pos), (end_cost, end_pos))| {
            let distance: usize = (start_pos - end_pos).abs().element_sum() as usize;
            if distance > 20 {
                return None;
            };
            let cheat_cost = start_cost + distance + base_cost - end_cost;
            Some(base_cost - cheat_cost)
        })
        .filter(|savings| savings >= &100)
        .count();

    Some(result as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(0));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
