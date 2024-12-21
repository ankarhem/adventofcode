use cached::proc_macro::cached;
use lib::grid::*;
use std::collections::HashSet;

struct TrailMap {
    grid: Grid<char>,
    starting_points: HashSet<(usize, usize)>,
}

fn parse_input(input: &str) -> TrailMap {
    let grid: Grid<char> = input.parse().unwrap();

    let starting_points = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line
                .chars()
                .enumerate()
                .filter_map(|(x, char)| {
                    if char == '0' {
                        Some((x, y))
                    } else {
                        None
                    }
                })
                .collect::<HashSet<(usize, usize)>>()
        })
        .collect();

    TrailMap {
        grid,
        starting_points,
    }
}

const DIRECTIONS: [(i32, i32); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];
const END: char = '9';

#[cached(key = "(usize, usize)", convert = r#"{ current }"#)]
fn traverse(trail_map: &TrailMap, current: (usize, usize)) -> Option<u32> {
    let value = trail_map.grid.get(current)?;

    if (*value == END) {
        return Some(1);
    }

    let parsed = value.to_digit(10)?;

    let trails = DIRECTIONS.iter().filter_map(|(dx, dy)| {
        let (x, y) = (current.0 as i32 + dx, current.1 as i32 + dy);
        if x < 0 || y < 0 || x >= trail_map.grid.width() as i32 || y >= trail_map.grid.height() as i32 {
            return None;
        }

        let next = (x as usize, y as usize);
        let next_value = trail_map.grid.get(next)?;
        let next_parsed = next_value.to_digit(10)?;

        if next_parsed == parsed + 1 {
            return traverse(trail_map, next);
        }

        None
    }).sum();

    Some(trails)
}

fn part_one(input: &str) -> u32 {
    let trail_map = parse_input(input);

    trail_map.starting_points.iter().filter_map(|starting_point| {
        traverse(&trail_map, *starting_point)
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
    fn example_mini() {
        let example = r#"...0...
...1...
...2...
6543456
7.....7
8.....8
9.....9"#;

        let score = part_one(example);
        assert_eq!(2, score);
    }

    #[test]
    fn example_one() {
        let example = include_str!("example");
        let actual = part_one(example);
        assert_eq!(36, actual);
    }

    // #[test]
    // fn example_two() {
    //     let example = include_str!("example");
    //     let actual = part_two(example);
    //     assert_eq!(31, actual);
    // }
}