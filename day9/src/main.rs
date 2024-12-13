use winnow::combinator::{opt, repeat};
use winnow::stream::AsChar;
use winnow::token::take_while;
use winnow::{Located, PResult, Parser};
mod disk_block;
use disk_block::*;

fn parse_single_digit(input: &mut Located<&str>) -> PResult<u32> {
    take_while(1..2, AsChar::is_dec_digit)
        .parse_next(input)
        .map(|c| c.parse().unwrap())
}

fn disk_block_parser(input: &mut Located<&str>) -> PResult<DiskBlock> {
    parse_single_digit.with_span().parse_next(input)
        .map(|(d, range)| DiskBlock::File { id: range.start / 2, size: d })
}

fn free_space_parser(input: &mut Located<&str>) -> PResult<DiskBlock> {
    parse_single_digit.parse_next(input)
        .map(|d| DiskBlock::Free(d))
}

fn parse_input(input: &mut &str) -> PResult<DiskMap> {
    let mut located_input = Located::new(*input);
    let output: DiskMap = repeat(1.., (
        disk_block_parser,
        opt(free_space_parser),
    )).parse_next(&mut located_input)?;

    Ok(output)
}

fn part_one(mut input: &str) -> u128 {
    let input = parse_input(&mut input).unwrap();
    
    dbg!(input.defragmented().collect::<DiskMap>().to_string());

    let output: u128 = input.defragmented()
        .flat_map(|block| {
            match block {
                DiskBlock::Free(_) => vec![block],
                DiskBlock::File {id, size} => {
                    (0..size)
                        .map(|_| DiskBlock::File {id, size: 1})
                        .collect()
                }
            }
        })
        .enumerate()
        .map(|(i, block)| {
            match block {
                DiskBlock::Free(_) => 0,
                DiskBlock::File {id, size} => id as u128 * i as u128
            }
        })
        .sum();


    output
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
        let actual = parse_input(&mut example).unwrap().to_string();
        assert_eq!("0..111....22222", actual);
    }

    #[test]
    fn parser_example() {
        let mut example = include_str!("example");
        let actual = parse_input(&mut example).unwrap().to_string();
        assert_eq!("00...111...2...333.44.5555.6666.777.888899", actual);
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