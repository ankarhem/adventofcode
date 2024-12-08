use itertools::Itertools;
use std::collections::HashMap;
use std::ops::Deref;

fn parse_input(input: &str) -> HashMap<char, Vec<(isize, isize)>> {
    let mut antennas = HashMap::new();
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c.is_alphanumeric() {
                antennas.entry(c).or_insert_with(Vec::new).push((x as isize, y as isize));
            }
        }
    }
    antennas
}

fn part_one(input: &str) -> u32 {
    let height = input.lines().count() as isize;
    let width = input.lines().next().unwrap().len() as isize;

    let antennas = parse_input(input);

    antennas.iter().map(|(_, coords)| {
        coords.iter()
            .combinations(2)
            .flat_map(|pair| {
                let (x1, y1) = pair[0];
                let (x2, y2) = pair[1];
                
                let distance = (x1.abs_diff(*x2) as isize, y1.abs_diff(*y2) as isize);

                vec![
                    (x1 - distance.0, y1 - distance.1),
                    (x1 + distance.0, y1 + distance.1),
                ]
            })
            .filter(|&(x, y)| x >= 0 && x < width && y >= 0 && y < height)
            .unique()
            .count() as u32
    }).sum()
}

fn part_two(input: &str) -> u32 {
    todo!()
}

fn main() {
    let input = include_str!("input");
    let result1 = part_one(input);
    println!("Day 1, part 1: {}", result1);

    // let result2 = part_two(input);
    // println!("Day 1, part 2: {}", result2);
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn example_one() {
        let example = include_str!("example");
        let actual = part_one(example);
        assert_eq!(14, actual);
    }

    // #[test]
    // fn example_two() {
    //     let example = include_str!("example");
    //     let actual = part_two(example);
    //     assert_eq!(31, actual);
    // }
}