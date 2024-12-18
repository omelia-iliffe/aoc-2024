use itertools::Itertools;
use std::cmp::Ordering;

advent_of_code::solution!(5);

#[derive(Debug)]
struct Rule {
    pub before: u32,
    pub after: u32,
}
impl Rule {
    fn parse(input: &str) -> Rule {
        let (before, after) = input
            .split("|")
            .map(|s| s.trim().parse().unwrap())
            .collect_tuple()
            .unwrap();
        Rule { before, after }
    }
}

#[derive(PartialEq, Eq)]
struct Page {
    number: u32,
    must_be_before: Vec<u32>,
}

impl PartialEq<u32> for Page {
    fn eq(&self, other: &u32) -> bool {
        self.number == *other
    }
}

impl PartialOrd for Page {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Page {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.must_be_before.iter().any(|r| *r == other.number) {
            Ordering::Less
        } else {
            Ordering::Greater
        }
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let (rules, manuals) = input
        .split("\n\n")
        .map(|s| s.split_whitespace().collect_vec())
        .collect_tuple()
        .unwrap();
    let rules = rules.into_iter().map(Rule::parse).collect_vec();
    let manuals: Vec<Vec<u32>> = manuals
        .iter()
        .map(|s| s.split(",").map(|s| s.parse().unwrap()).collect_vec())
        .collect_vec();
    Some(
        manuals
            .into_iter()
            .filter_map(|page_list| {
                // println!("page list: {page_list:?}");
                let relevant_rules = rules
                    .iter()
                    .filter(|r| page_list.contains(&r.before) && page_list.contains(&r.after))
                    .collect_vec();
                let page_correct = page_list
                    .iter()
                    .enumerate()
                    .flat_map(|(index, page)| {
                        // println!("checking page {page} ({index}");
                        relevant_rules
                            .iter()
                            .filter(|r| r.before == *page)
                            .map(|r| {
                                let other_page_index =
                                    page_list.iter().position(|p| p == &r.after).unwrap();
                                // println!("{index}, {other_page_index} {}", index <= other_page_index);
                                index <= other_page_index
                            })
                            .collect_vec()
                    })
                    .all(|b| b);
                if page_correct {
                    Some(page_list[page_list.len() / 2])
                } else {
                    None
                }
            })
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let (rules, manuals) = input
        .split("\n\n")
        .map(|s| s.split_whitespace().collect_vec())
        .collect_tuple()
        .unwrap();
    let rules = rules.into_iter().map(Rule::parse).collect_vec();
    let manuals: Vec<Vec<u32>> = manuals
        .iter()
        .map(|s| s.split(",").map(|s| s.parse().unwrap()).collect_vec())
        .collect_vec();
    Some(
        manuals
            .into_iter()
            .filter_map(|page_list| {
                let mut page_list_sorted = page_list
                    .iter()
                    .map(|page| Page {
                        number: *page,
                        must_be_before: rules
                            .iter()
                            .filter_map(|r| (&r.before == page).then_some(r.after))
                            .collect(),
                    })
                    .collect_vec();
                page_list_sorted.sort();
                let changes = page_list_sorted != page_list;
                changes.then_some(page_list_sorted[page_list_sorted.len() / 2].number)
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
        assert_eq!(result, Some(143));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(123));
    }
}
