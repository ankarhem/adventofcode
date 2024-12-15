fn parse_input(input: &str) -> (Vec<u32>, Vec<u32>) {
todo!()
}

fn part_one(input: &str) -> u32 {
    todo!()
}

fn part_two(input: &str) -> u32 {
    todo!()
}

fn main() {
    let input = include_str!("input");
    let result1 = part_one(input);
    println!("Day 13, part 1: {}", result1);

    // let result2 = part_two(input);
    // println!("Day 13, part 2: {}", result2);
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

    // #[test]
    // fn example_two() {
    //     let example = include_str!("example");
    //     let actual = part_two(example);
    //     assert_eq!(31, actual);
    // }
}