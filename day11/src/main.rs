fn parse_input(input: &str) -> Vec<u128> {
    input
        .split_whitespace()
        .map(|s| s.parse::<u128>().unwrap())
        .collect()
}

fn process_stone(stone: &u128) -> Vec<u128> {
    if *stone == 0 {
        return vec![1];
    }

    let digits = stone.to_string();
    let n_digits = digits.len();

    if n_digits % 2 == 0 {
        let half = n_digits / 2;
        let (left, right) = digits.split_at(half);
        let left = left.parse::<u128>().unwrap();
        let right = right.parse::<u128>().unwrap();
        return vec![left, right];
    }

    vec![*stone * 2024]
}

fn process_stones(stones: Vec<u128>, blinks: usize) -> Vec<u128> {
    (0..blinks).fold(stones, |acc, _| {
        acc.iter()
            .flat_map(|s| process_stone(s))
            .collect()
    })
}

fn part_one(input: &str) -> u32 {
    let stones = parse_input(input);

    let stones = process_stones(stones, 25);

    stones.len() as u32
}

fn part_two(input: &str) -> u32 {
    todo!()
}

fn main() {
    let input = include_str!("input");
    let result1 = part_one(input);
    println!("Day 11, part 1: {}", result1);

    // let result2 = part_two(input);
    // println!("Day 11, part 2: {}", result2);
}

#[cfg(test)]
mod test {
    use super::*;
    use itertools::Itertools;

    #[test]
    fn example_one_1_blink() {
        let example = parse_input("125 17");
        let actual = process_stones(example, 1).iter().join(" ");
        assert_eq!("253000 1 7", actual);
    }
    #[test]
    fn example_one_6_blinks() {
        let example = parse_input("125 17");
        let actual = process_stones(example, 6).iter().join(" ");
        assert_eq!("2097446912 14168 4048 2 0 2 4 40 48 2024 40 48 80 96 2 8 6 7 6 0 3 2", actual);
    }
    
    #[test]
    fn part_one_example() {
        let actual = part_one("125 17");
        assert_eq!(55312, actual);
    }
}