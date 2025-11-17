use nom::character::complete;
use nom::character::complete::space1;
use nom::multi::separated_list1;
use nom::IResult;

advent_of_code::solution!(11);

type Stone = u64;

fn blink(stone: Stone) -> Vec<Stone> {
    match stone {
        0 => vec![1],
        num if num.to_string().len() % 2 == 0 => {
            let num = num.to_string();
            let mid = num.len() / 2;
            let (left, right) = num.split_at(mid);
            let left = left.parse().unwrap();
            let right = right.parse().unwrap();
            vec![left, right]
        }
        num => vec![num * 2024],
    }
}

fn parse(input: &str) -> IResult<&str, Vec<Stone>> {
    separated_list1(space1, complete::u64)(input)
}

pub fn part_one(input: &str) -> Option<u32> {
    let (_, mut stones) = parse(input).unwrap();
    for i in 0..75 {
        println!("working on {i}");
        stones = stones.into_iter().flat_map(|stone| blink(stone)).collect();
    }
    Some(stones.len() as u32)
}

pub fn part_two(input: &str) -> Option<Stone> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(55312));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
