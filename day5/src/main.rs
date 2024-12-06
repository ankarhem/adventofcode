use itertools::Itertools;
use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};

fn parse_input(input: &str) -> (HashMap<u32, HashSet<u32>>, Vec<Vec<u32>>) {
    let mut parts = input.split("\n\n");

    let table: HashMap<u32, HashSet<u32>> = HashMap::new();
    let table = parts
        .next()
        .unwrap()
        .lines()
        .map(|nums| {
            nums.split('|')
                .map(|n| n.parse::<u32>().unwrap())
                .collect_tuple()
                .unwrap()
        })
        .fold(table, |mut acc, (x, y)| {
            acc.entry(x).or_default().insert(y);
            acc
        });
    let updates: Vec<Vec<u32>> = parts
        .next()
        .unwrap()
        .lines()
        .map(|nums| nums.split(',').map(|n| n.parse::<u32>().unwrap()).collect())
        .collect();
    (table, updates)
}

fn is_sorted_by_rule_table(table: &HashMap<u32, HashSet<u32>>, update: &[u32]) -> bool {
    update.windows(2).all(|w| {
        let (a, b) = (&w[0], &w[1]);
        table.get(a).map_or(false, |set| set.contains(b))
    })
}

fn part_one(input: &str) -> u32 {
    let (table, updates) = parse_input(input);

    let sorted = updates
        .iter()
        .filter(|u| is_sorted_by_rule_table(&table, u));

    sorted
        .map(|u| u.get((u.len() - 1) / 2).unwrap())
        .sum::<u32>()
}

fn part_two(input: &str) -> u32 {
    let (table, updates) = parse_input(input);

    let sorted = updates
        .iter()
        .filter(|u| !is_sorted_by_rule_table(&table, u))
        .map(|update| {
            update
                .iter()
                .sorted_by(|&a, &b| {
                    table.get(a).map_or(Ordering::Equal, |set| {
                        if set.contains(b) {
                            Ordering::Less
                        } else {
                            Ordering::Greater
                        }
                    })
                })
                .collect::<Vec<&u32>>()
        });

    sorted
        .map(|u| *u.get((u.len() - 1) / 2).unwrap())
        .sum::<u32>()
}

fn main() {
    let input = include_str!("input");
    let result1 = part_one(input);
    println!("Day 5, part 1: {}", result1);

    let result2 = part_two(input);
    println!("Day 5, part 2: {}", result2);
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn example_one() {
        let example = include_str!("example");
        let actual = part_one(example);
        assert_eq!(143, actual);
    }

    #[test]
    fn example_two() {
        let example = include_str!("example");
        let actual = part_two(example);
        assert_eq!(123, actual);
    }
}