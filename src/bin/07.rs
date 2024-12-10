use itertools::Itertools;

advent_of_code::solution!(7);

#[derive(Debug, Clone, Copy)]
enum Operation {
    Add,
    Mult,
    Concat,
}

impl Operation {
    fn compute(&self, a: u64, b: u64) -> u64 {
        match self {
            Operation::Add => a + b,
            Operation::Mult => a * b,
            Operation::Concat => a * 10u64.pow(b.ilog10() + 1) + b,
        }
    }
}

fn recursive_op(
    operations: &[Operation],
    goal: u64,
    cur_total: u64,
    numbers: &[u64],
) -> Result<Vec<(u64, Option<Operation>)>, ()> {
    if let Some(num) = numbers.first() {
        for op in operations {
            let cur_total = op.compute(cur_total, *num);
            let r = recursive_op(operations, goal, cur_total, &numbers[1..]);
            if let Ok(mut r) = r {
                r.push((*num, Some(*op)));
                return Ok(r);
            }
        }
    } else if goal == cur_total {
        return Ok(Vec::new());
    }
    Err(())
}

fn solve(operations: &[Operation], input: &str) -> Option<u64> {
    let lines = input.lines().collect_vec();
    let lines = lines
        .iter()
        .flat_map(|line| {
            line.split_once(":").map(|(first, rest)| {
                let first: u64 = first.parse().unwrap();
                let rest: Vec<u64> = rest
                    .split_whitespace()
                    .map(|n| n.parse().unwrap())
                    .collect();
                (first, rest)
            })
        })
        .flat_map(|(solution, numbers)| {
            let r = recursive_op(
                operations,
                solution,
                *numbers.first().unwrap(),
                &numbers[1..],
            );
            //dbg!(solution, &r);
            r.map(|_| solution)
        })
        .sum();

    Some(lines)
}

pub fn part_one(input: &str) -> Option<u64> {
    let operations = vec![Operation::Mult, Operation::Add];
    solve(&operations, input)
}

pub fn part_two(input: &str) -> Option<u64> {
    let operations = vec![Operation::Mult, Operation::Add, Operation::Concat];
    solve(&operations, input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_7_a() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3749));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11387));
    }
}
