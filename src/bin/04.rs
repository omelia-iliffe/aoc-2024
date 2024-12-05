use itertools::Itertools;
use std::ops::Range;

advent_of_code::solution!(4);

fn grid_coordinates(
    c: (usize, usize),
    x_range: &Range<i32>,
    y_range: &Range<i32>,
    exclude_c: bool,
) -> Vec<(usize, usize)> {
    let mut r = Vec::new();
    for dy in -1..=1 {
        for dx in -1..=1 {
            let (x, y) = (c.0 as i32 + dx, c.1 as i32 + dy);
            if x_range.contains(&x)
                && y_range.contains(&y)
                && (!exclude_c || (x as usize, y as usize) != c)
            {
                r.push((x as usize, y as usize));
            }
        }
    }
    r
}
fn corners(
    c: (usize, usize),
    x_range: &Range<i32>,
    y_range: &Range<i32>,
) -> Vec<Option<(usize, usize)>> {
    let mut r = Vec::new();
    for dy in (-1..=1).step_by(2) {
        for dx in (-1..=1).step_by(2) {
            let (x, y) = (c.0 as i32 + dx, c.1 as i32 + dy);
            r.push(
                (x_range.contains(&x) && y_range.contains(&y)).then_some((x as usize, y as usize)),
            );
        }
    }
    r
}
#[allow(unused)]
fn show(bs: &[u8]) -> String {
    String::from_utf8_lossy(bs).into_owned()
}

fn get_u8(line_len: usize, input: &[u8], c: &(usize, usize)) -> u8 {
    let index = (c.1 * (line_len)) + c.0;
    *input.get(index).unwrap_or(&0)
}

fn calc_a_s_cood(
    x_c: &(usize, usize),
    m_c: &(usize, usize),
    x_range: &Range<i32>,
    y_range: &Range<i32>,
) -> Option<((usize, usize), (usize, usize))> {
    let x_c = (x_c.0 as i32, x_c.1 as i32);
    let c = (m_c.0 as i32, m_c.1 as i32);
    let (dx, dy) = (x_c.0 - c.0, x_c.1 - c.1);
    let func = |d| {
        let (x, y) = (c.0 - dx * d, c.1 - dy * d);
        (x_range.contains(&x) && y_range.contains(&y)).then_some((x as usize, y as usize))
    };
    let a = func(1);
    let s = func(2);
    a.zip(s)
}

pub fn part_one(input: &str) -> Option<u32> {
    let line_count = input.split_whitespace().count();
    let line_len = input.find("\n").unwrap() + 1;
    let x_range = 0..line_len as i32;
    let y_range = 0..line_count as i32;
    Some(
        input
            .as_bytes()
            .iter()
            .positions(|b| b == &b'X')
            .map(|p| (p % line_len, p / line_len))
            .map(|x_c| {
                let s = grid_coordinates(x_c, &x_range, &y_range, true);
                let surrounding_m = s
                    .iter()
                    .filter_map(|c| {
                        let char = get_u8(line_len, input.as_bytes(), c);
                        if char == b'M' {
                            Some((x_c, c))
                        } else {
                            None
                        }
                    })
                    .collect_vec();
                let found = surrounding_m
                    .iter()
                    .map(|(x_c, m_c)| {
                        calc_a_s_cood(x_c, m_c, &x_range, &y_range).is_some_and(|(a, s)| {
                            let maybe_a = get_u8(line_len, input.as_bytes(), &a);
                            let maybe_s = get_u8(line_len, input.as_bytes(), &s);
                            maybe_a == b'A' && maybe_s == b'S'
                        })
                    })
                    .filter(|b| *b)
                    .count();
                found as u32
            })
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let line_count = input.split_whitespace().count();
    let line_len = input.find("\n").unwrap() + 1;
    let x_range = 0..line_len as i32;
    let y_range = 0..line_count as i32;
    Some(
        input
            .as_bytes()
            .iter()
            .positions(|b| b == &b'A')
            .map(|p| (p % line_len, p / line_len))
            .filter_map(|x_c| {
                let s = corners(x_c, &x_range, &y_range);
                let (tl, tr, bl, br) = s
                    .iter()
                    .map(|c| c.map(|c| get_u8(line_len, input.as_bytes(), &c)))
                    .collect_tuple()
                    .unwrap();
                let one = matches!(
                    (tl, br),
                    (Some(b'M'), Some(b'S')) | (Some(b'S'), Some(b'M'))
                );
                let two = matches!(
                    (bl, tr),
                    (Some(b'M'), Some(b'S')) | (Some(b'S'), Some(b'M'))
                );
                (one && two).then_some(())
            })
            .count() as u32,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9));
    }
}
