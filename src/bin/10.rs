use glam::IVec2;
use itertools::Itertools;
use nom::Parser;
use pathfinding::prelude::strongly_connected_components_from;

advent_of_code::solution!(10);

pub fn part_one(input: &str) -> Option<u32> {
    let width = input.split_whitespace().next().unwrap().len();
    let height = input.split_whitespace().count();
    let from_index = |i: usize| -> IVec2 {
        let x = i % width;
        let y = i / width;
        IVec2::new(x as i32, y as i32)
    };
    let from_vec2 = |c: &IVec2| -> usize { c.y as usize * width + c.x as usize };
    let input: Vec<char> = input.split_whitespace().flat_map(|s| s.chars()).collect();
    let map_size = IVec2::new(width as i32, height as i32);

    let ans: usize = input
        .iter()
        .enumerate()
        .filter_map(|(i, c)| (c == &'0').then_some(from_index(i)))
        .map(|zero| {
            let nodes = strongly_connected_components_from(&zero, |c| {
                let num = input[from_vec2(c)].to_string().parse::<u32>().unwrap();
                let next_num = num + 1;
                [IVec2::X, IVec2::NEG_X, IVec2::Y, IVec2::NEG_Y]
                    .into_iter()
                    .filter_map(|d| {
                        let new = c + d;
                        (new.cmpge(IVec2::ZERO).all() && new.cmplt(map_size).all()).then_some(new)
                    })
                    .filter(|d| input[from_vec2(&d)].to_string() == next_num.to_string())
                    .collect_vec()
            });
            nodes
                .into_iter()
                .flatten()
                .filter(|n| input[from_vec2(n)] == '9')
                .count()
        })
        .inspect(|score| println!("score: {}", score))
        .sum();
    Some(ans as u32)
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
        assert_eq!(result, Some(36));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
