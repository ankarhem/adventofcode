fn parse_input(input: &str) -> (Vec<u32>, Vec<u32>) {
    // transpose lines into two vectors
    input
        .lines()
        .map(|line| {
            let pair = line
                .split_whitespace()
                .map(|num| num.parse::<u32>().unwrap())
                .collect::<Vec<u32>>();
            (pair[0], pair[1])
        })
        .unzip()
}

fn part_one(input: &str) -> u32 {
    let (mut col1, mut col2) = parse_input(input);

    // sort the columns
    col1.sort();
    col2.sort();

    // zip the pairs up and calculate the diff
    let total_distance = col1
        .iter()
        .zip(col2.iter())
        .map(|(a, b)| a.abs_diff(*b))
        .sum();
    total_distance
}

fn part_two(input: &str) -> u32 {
    let (col1, col2) = parse_input(input);

    // hashmap to store the counts
    let counts = col2
        .iter()
        .fold(std::collections::HashMap::new(), |mut acc, &num| {
            *acc.entry(num).or_insert(0) += 1;
            acc
        });

    col1.iter()
        .map(|num| num * counts.get(num).unwrap_or(&0))
        .sum()
}

fn main() {
    let input = include_str!("input");
    let result1 = part_one(input);
    println!("Day 1, part 1: {}", result1);

    let result2 = part_two(input);
    println!("Day 1, part 2: {}", result2);
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn example_one() {
        let example = include_str!("example");
        let actual = part_one(example);
        assert_eq!(11, actual);
    }

    #[test]
    fn example_two() {
        let example = include_str!("example");
        let actual = part_two(example);
        assert_eq!(31, actual);
    }
}
