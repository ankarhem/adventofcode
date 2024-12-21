use lib::diophantine::Diophantine;
use winnow::ascii::{alpha1, digit1, newline, space1};
use winnow::combinator::{alt, preceded, separated, separated_pair};
use winnow::{PResult, Parser};

#[derive(Debug)]
struct Machine {
    diophantine_a: Diophantine,
    diophantine_b: Diophantine,
}

impl Machine {}

fn coord_parser(input: &mut &str) -> PResult<i32> {
    preceded((alpha1, alt(("+", "="))), digit1.try_map(|s: &str| s.parse())).parse_next(input)
}

fn button_parser(input: &mut &str) -> PResult<(i32, i32)> {
    ("Button ", alpha1, ":", space1).parse_next(input)?;

    separated_pair(coord_parser, (",", space1), coord_parser).parse_next(input)
}

fn prize_parser(input: &mut &str) -> PResult<(i32, i32)> {
    "Prize: ".parse_next(input)?;
    separated_pair(coord_parser, (",", space1), coord_parser).parse_next(input)
}

fn machine_parser(input: &mut &str) -> PResult<Machine> {
    let (b1, _, b2, _, p) = (
        button_parser,
        newline,
        button_parser,
        newline,
        prize_parser,
    ).parse_next(input)?;

    Ok(Machine {
        diophantine_a: Diophantine::new(b1.0, b1.1, p.0),
        diophantine_b: Diophantine::new(b2.0, b2.1, p.1),
    })
}

fn parse_input(input: &mut &str) -> PResult<Vec<Machine>> {
    separated(1.., machine_parser, (newline, newline)).parse_next(input)
}

fn part_one(mut input: &str) -> i32 {
    let input = parse_input(&mut input).unwrap();

    todo!()
}

fn part_two(_input: &str) -> i32 {
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
    use rstest::rstest;

    #[rstest]
    #[case("X+94", 94)]
    #[case("Y+34", 34)]
    #[case("X=8400", 8400)]
    #[case("Y=5400", 5400)]
    fn coord_parser_test(#[case] mut input: &str, #[case] expected: i32) {
        let actual = coord_parser(&mut input).unwrap();
        assert_eq!(expected, actual);
    }

    #[rstest]
    #[case("Button A: X+94, Y+34", (94, 34))]
    #[case("Button B: X+22, Y+67", (22, 67))]
    #[case("Button A: X+26, Y+66", (26, 66))]
    #[case("Button B: X+67, Y+21", (67, 21))]
    fn button_parser_test(#[case] mut input: &str, #[case] expected: (i32, i32)) {
        let actual = button_parser(&mut input).unwrap();
        assert_eq!(expected, actual);
    }

    #[rstest]
    #[case("Prize: X=8400, Y=5400", (8400, 5400))]
    #[case("Prize: X=12748, Y=12176", (12748, 12176))]
    fn price_parser_test(#[case] mut input: &str, #[case] expected: (i32, i32)) {
        let actual = prize_parser(&mut input).unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn can_parse_example() {
        let mut example = include_str!("example");
        let actual = parse_input(&mut example).unwrap();
        insta::assert_debug_snapshot!(actual);
    }

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