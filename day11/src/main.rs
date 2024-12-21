use cached::proc_macro::cached;
fn parse_input(input: &str) -> Vec<u128> {
    input
        .split_whitespace()
        .map(|s| s.parse::<u128>().unwrap())
        .collect()
}

#[cached]
fn process_stone(stone: u128, blinks: usize) -> u64 {
    if blinks == 0 {
        return 1;
    }

    if stone == 0 {
        return process_stone(1, blinks - 1);
    }

    let digits = stone.to_string();
    let n_digits = digits.len();

    if n_digits % 2 == 0 {
        let half = n_digits / 2;
        let (left, right) = digits.split_at(half);
        let left = left.parse::<u128>().unwrap();
        let right = right.parse::<u128>().unwrap();
        return process_stone(left, blinks - 1) + process_stone(right, blinks - 1);
    }

    process_stone(stone * 2024, blinks - 1)
}

fn process_stones(stones: Vec<u128>, blinks: usize) -> u64 {
    stones
        .iter()
        .map(|&stone| process_stone(stone, blinks))
        .sum()
}

fn part_one(input: &str) -> u64 {
    let stones = parse_input(input);

    process_stones(stones, 25)
}

fn part_two(input: &str) -> u64 {
    let stones = parse_input(input);

    process_stones(stones, 75)
}

fn main() {
    let input = include_str!("input");
    let result1 = part_one(input);
    println!("Day 11, part 1: {}", result1);

    let result2 = part_two(input);
    println!("Day 11, part 2: {}", result2);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_one_1_blink() {
        let example = parse_input("125 17");
        let actual = process_stones(example, 1);
        assert_eq!(3, actual);
    }
    #[test]
    fn example_one_6_blinks() {
        let example = parse_input("125 17");
        let actual = process_stones(example, 6);
        assert_eq!(22, actual);
    }

    #[test]
    fn part_one_example() {
        let actual = part_one("125 17");
        assert_eq!(55312, actual);
    }
}
