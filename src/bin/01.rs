use itertools::Itertools;
advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let mut list_one: Vec<u32> = Vec::new();
    let mut list_two: Vec<u32> = Vec::new();
    input.split_whitespace().tuples().for_each(|(x, y)| {
        list_one.push(x.parse().unwrap());
        list_two.push(y.parse().unwrap())
    });
    list_two.sort();
    list_one.sort();
    Some(
        list_one
            .into_iter()
            .zip(list_two)
            .map(|(x, y)| x.abs_diff(y))
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut list_one: Vec<u32> = Vec::new();
    let mut list_two: Vec<u32> = Vec::new();
    input.split_whitespace().tuples().for_each(|(x, y)| {
        list_one.push(x.parse().unwrap());
        list_two.push(y.parse().unwrap())
    });

    let counts = list_two.iter().counts();

    Some(
        list_one
            .into_iter()
            .map(|x| {
                let count = counts.get(&x).unwrap_or(&0);
                x * *count as u32
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
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }
}
