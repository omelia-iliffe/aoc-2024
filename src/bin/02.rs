use itertools::Itertools;

advent_of_code::solution!(2);

pub fn evaluate(levels: &[u32]) -> bool {
    let (safe, dir): (Vec<bool>, Vec<bool>) = levels
        .iter()
        .tuple_windows()
        .map(|(x, y)| check_safe(x, y))
        .collect();
    let all_safe = safe.iter().all(|b| *b);
    let all_same_dir = dir.iter().all_equal();
    all_safe && all_same_dir
}

pub fn check_safe(x: &u32, y: &u32) -> (bool, bool) {
    let is_increasing = x < y;
    let difference = x.abs_diff(*y);
    let diff_ok = (1..=3).contains(&difference);
    (diff_ok, is_increasing)
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .filter_map(|line| {
                let levels = line
                    .split_whitespace()
                    .map(|c| c.parse::<u32>().unwrap())
                    .collect_vec();
                evaluate(&levels).then_some(())
            })
            .count() as u32,
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let r = input
        .lines()
        .filter_map(|line| {
            let levels = line
                .split_whitespace()
                .map(|c| c.parse::<u32>().unwrap())
                .collect_vec();
            let all_safe = evaluate(&levels);
            if all_safe {
                return Some(());
            }
            for p in 0..levels.len() {
                //todo: check only problem indexes
                let mut levels = levels.clone();
                levels.remove(p);
                if evaluate(&levels) {
                    return Some(());
                }
            }
            None
        })
        .count();
    Some(r as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }
}
