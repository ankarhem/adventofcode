use winnow::combinator::{opt, repeat};
use winnow::stream::AsChar;
use winnow::token::take_while;
use winnow::{Located, PResult, Parser};

fn parse_single_digit(input: &mut Located<&str>) -> PResult<u32> {
    take_while(1..2, AsChar::is_dec_digit)
        .parse_next(input)
        .map(|c| c.parse().unwrap())
}

fn disk_block_parser(input: &mut Located<&str>) -> PResult<String> {
    parse_single_digit.with_span().parse_next(input)
        .map(|(d, range)| format!("{}", range.start / 2).repeat(d as usize))
}

fn free_space_parser(input: &mut Located<&str>) -> PResult<String> {
    parse_single_digit.parse_next(input)
        .map(|d| ".".repeat(d as usize))
}

fn parse_input(input: &mut &str) -> PResult<String> {
    let mut located_input = Located::new(*input);
    let results: Vec<(String, Option<String>)> = repeat(1.., (
        disk_block_parser,
        opt(free_space_parser),
    )).parse_next(&mut located_input)?;

    let output = results.iter().fold(String::new(), |acc, (disk_block, free_space)| {
        acc + disk_block + free_space.as_deref().unwrap_or("")
    });

    Ok(output)
}

fn defragment(input: &str) -> String {
    let mut output = String::new();
    let mut file_blocks_to_move = input
        .chars()
        .rev()
        .filter(|&c| c != '.');

    for c in input.chars() {
        let f = file_blocks_to_move.next();
        match (c, f) {
            (_, None) => output.push('.'),
            ('.', Some(f)) => output.push(f),
            _ => output.push(c)
        }
    }

    output
}

fn part_one(mut input: &str) -> u32 {
    let input = parse_input(&mut input).unwrap();

    let mut output = defragment(&input);

    todo!()
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
    fn parser_mini() {
        let mut example = "12345";
        let actual = parse_input(&mut example).unwrap();
        assert_eq!("0..111....22222", actual);
    }

    #[test]
    fn parser_example() {
        let mut example = include_str!("example");
        let actual = parse_input(&mut example).unwrap();
        assert_eq!("00...111...2...333.44.5555.6666.777.888899", actual);
    }

    #[test]
    fn defragment_mini() {
        let input = "0..111....22222";
        let actual = defragment(input);
        assert_eq!("022111222......", actual);
    }


    #[test]
    fn example_one() {
        let example = include_str!("example");
        let actual = part_one(example);
        assert_eq!(1928, actual);
    }

    // #[test]
    // fn example_two() {
    //     let example = include_str!("example");
    //     let actual = part_two(example);
    //     assert_eq!(31, actual);
    // }
}