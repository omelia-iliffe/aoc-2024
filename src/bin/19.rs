use memoize::memoize;
use nom::bytes::complete::tag;
use nom::character::complete::{alpha1, line_ending};
use nom::multi::separated_list1;
use nom::sequence::{pair, preceded, terminated};
use nom::{IResult, Parser};

advent_of_code::solution!(19);

fn parse(input: &str) -> IResult<&str, (Vec<String>, Vec<String>)> {
    pair(
        terminated(
            separated_list1(tag(", "), alpha1.map(String::from)),
            line_ending,
        ),
        preceded(
            line_ending,
            separated_list1(line_ending, alpha1.map(String::from)),
        ),
    )(input)
}

#[memoize]
fn recursive_check(pattern: String, towels: Vec<String>) -> bool {
    towels
        .iter()
        .map(|ref towel| {
            pattern
                .strip_prefix(towel.as_str())
                .is_some_and(|r_pattern| {
                    r_pattern.is_empty() || recursive_check(r_pattern.to_string(), towels.clone())
                })
        })
        .any(|v| v)
}

#[memoize]
fn recursive_find(pattern: String, towels: Vec<String>) -> usize {
    towels
        .iter()
        .filter_map(|towel| {
            pattern
                .strip_prefix(towel.as_str())
                .map(|r_pattern| {
                    if r_pattern.is_empty() {
                        Some(1)
                    } else {
                        Some(recursive_find(r_pattern.to_string(), towels.clone()))
                    }
                })
                .flatten()
        })
        .sum()
}

pub fn part_one(input: &str) -> Option<u32> {
    let (_, (towels, patterns)) = parse(input).unwrap();
    let n = patterns
        .iter()
        .filter(|pattern| recursive_check(pattern.to_string(), towels.clone()))
        .count();
    Some(n as u32)
}

pub fn part_two(input: &str) -> Option<u64> {
    let (_, (towels, patterns)) = parse(input).unwrap();
    let n: usize = patterns
        .iter()
        .map(|pattern| recursive_find(pattern.to_string(), towels.clone()))
        .sum();
    Some(n as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(16));
    }
}
