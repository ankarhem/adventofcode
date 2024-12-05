use itertools::Itertools;
use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};
use std::ops::Not;

fn parse_input(input: &str) -> (Vec<(u32, u32)>, Vec<Vec<u32>>) {
    let mut parts = input.split("\n\n");

    let pairs: Vec<(u32, u32)> = parts
        .next()
        .unwrap()
        .lines()
        .map(|nums| {
            nums.split('|')
                .map(|n| n.parse::<u32>().unwrap())
                .collect_tuple()
                .unwrap()
        })
        .collect();
    let updates: Vec<Vec<u32>> = parts
        .next()
        .unwrap()
        .lines()
        .map(|nums| nums.split(',').map(|n| n.parse::<u32>().unwrap()).collect())
        .collect();
    (pairs, updates)
}

fn build_table(pairs: &Vec<(u32, u32)>) -> HashMap<&u32, HashSet<&u32>> {
    let table: HashMap<&u32, HashSet<&u32>> = HashMap::new();
    pairs.iter().fold(table, |mut acc, (x, y)| {
        acc.entry(x).or_insert(HashSet::new()).insert(y);
        acc
    })
}
fn part_one(input: &str) -> u32 {
    let (pairs, updates) = parse_input(input);

    let table = build_table(&pairs);

    let sorted = updates.iter().filter(|update| {
        update.windows(2).all(|w| {
            let (a, b) = (&w[0], &w[1]);
            table.get(a).map_or(false, |set| set.contains(b))
        })
    });

    sorted
        .map(|u| u.get((u.len() - 1) / 2).unwrap())
        .sum::<u32>()
}

fn part_two(input: &str) -> u32 {
    let (pairs, updates) = parse_input(input);

    let table = build_table(&pairs);

    let sorted = updates
        .iter()
        // filter out all that is already sorted
        .filter(|update| {
            update
                .windows(2)
                .all(|w| {
                    let (a, b) = (&w[0], &w[1]);
                    table.get(a).map_or(false, |set| set.contains(b))
                })
                .not()
        })
        // sort the remaining updates
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