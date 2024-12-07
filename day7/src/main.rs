use lib::parsers::number;
use winnow::ascii::{newline, space1};
use winnow::combinator::{separated, separated_pair};
use winnow::{PResult, Parser};

fn parse_input(input: &mut &str) -> PResult<Vec<(u64, Vec<u64>)>> {
    let equation_parser = separated_pair(
        number::<u64>,
        ": ",
        separated(1.., number::<u64>, space1),
    );

    separated(1.., equation_parser, newline).parse_next(input)
}

enum Operation {
    Add,
    Multiply,
}

const OPERATIONS: [Operation; 2] = [Operation::Add, Operation::Multiply];
fn is_valid_equation<Eq>(equation: &Eq) -> bool
where
    Eq: std::ops::Deref,
    Eq: std::ops::Deref<Target=(u64, Vec<u64>)>,
{
    let target = equation.0;
    let numbers = &equation.1;

    if numbers.len() == 1 {
        return target == numbers[0] || (
            target % numbers[0] == 0 &&
            target / numbers[0] == 0
        );
    }

    OPERATIONS.iter().any(|op| {
        match op {
            Operation::Add => {
                // prevent subtraction from going negative
                target >= numbers[0] &&
                is_valid_equation(&&(target - numbers[0], numbers[1..].to_vec()))
            }
            Operation::Multiply => {
                // prevent division from going fractional
                target % numbers[0] == 0 &&
                is_valid_equation(&&(target / numbers[0], numbers[1..].to_vec()))
            }
        }
    })
}

fn part_one(mut input: &str) -> u64 {
    let input = parse_input(&mut input).unwrap();

    input.iter().filter(is_valid_equation).map(|(target, _)| *target).sum()
}

fn part_two(_input: &str) -> u64 {
    todo!()
}

fn main() {
    let input = include_str!("input");
    let result1 = part_one(input);
    println!("Day 7, part 1: {}", result1);

    let result2 = part_two(input);
    println!("Day 7, part 2: {}", result2);
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn example_one() {
        let example = include_str!("example");
        let actual = part_one(example);
        assert_eq!(3749, actual);
    }

    // #[test]
    // fn example_two() {
    //     let example = include_str!("example");
    //     let actual = part_two(example);
    //     assert_eq!(31, actual);
    // }
}