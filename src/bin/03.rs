use regex::Regex;

advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u32> {
    let regex = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    Some(
        regex
            .captures_iter(input)
            .map(|c| c.extract())
            .map(|(_, [x, y])| {
                let x: u32 = x.parse().unwrap();
                let y: u32 = y.parse().unwrap();
                x * y
            })
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let regex =
        Regex::new(r"(?<mul>mul\((?<x>\d+),(?<y>\d+)\))|(?<do>do\(\))|(?<dont>don't\(\))").unwrap();
    let mut mul_enable = true;
    Some(
        regex
            .captures_iter(input)
            .flat_map(|c| {
                if c.name("mul").is_some() {
                    if mul_enable {
                        let x: u32 = c["x"].parse().unwrap();
                        let y: u32 = c["y"].parse().unwrap();
                        Some(x * y)
                    } else {
                        None
                    }
                } else if c.name("do").is_some() {
                    mul_enable = true;
                    None
                } else if c.name("dont").is_some() {
                    mul_enable = false;
                    None
                } else {
                    panic!()
                }
            })
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(48));
    }
}
