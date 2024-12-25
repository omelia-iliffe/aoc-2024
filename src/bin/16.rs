use glam::IVec2;
use itertools::Itertools;
use pathfinding::prelude::*;
use std::collections::HashSet;

advent_of_code::solution!(16);

struct Map {
    map_size: IVec2,
    walls: Vec<IVec2>,
    start: IVec2,
    end: IVec2,
}

impl Map {
    fn successors(&self, pos: &IVec2, facing: &IVec2) -> Vec<((IVec2, IVec2), usize)> {
        let next_pos = pos + facing;
        if self.walls.contains(&next_pos) {
            vec![
                ((*pos, facing.perp()), 1000),
                ((*pos, -facing.perp()), 1000),
            ]
        } else {
            vec![
                ((next_pos, *facing), 1),
                ((*pos, facing.perp()), 1000),
                ((*pos, -facing.perp()), 1000),
            ]
        }
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
    let (_, cost) = dijkstra(
        &(map.start, IVec2::X),
        |(p, facing)| map.successors(p, facing),
        |(p, _)| p == &map.end,
    )
    .unwrap();

    Some(cost as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let map = parse(input);
    let (paths, _) = astar_bag(
        &(map.start, IVec2::X),
        |(p, facing)| map.successors(p, facing),
        |_| 0,
        |(p, _)| p == &map.end,
    )
    .unwrap();
    let tiles = paths
        .flat_map(|path| path.into_iter().map(|(p, _)| p))
        .collect::<HashSet<_>>();

    Some(tiles.len() as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(7036));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(45));
    }
}
