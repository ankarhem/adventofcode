use itertools::Itertools;
use lib::parsers::number;
use winnow::ascii::{newline, space1};
use winnow::combinator::{separated, separated_pair};
use winnow::{PResult, Parser};

fn parse_input(input: &mut &str) -> PResult<Vec<Equation>> {
    let equation_parser =
        separated_pair(
            number::<u64>,
            ": ",
            separated(1.., number::<u64>, space1),
        ).map(|(target, values)| Equation(target, values));

    separated(1.., equation_parser, newline).parse_next(input)
}

#[derive(Debug)]
enum Operation {
    Add,
    Multiply,
}

const OPERATIONS: [Operation; 2] = [Operation::Add, Operation::Multiply];

impl Operation {
    fn execute(&self, a: u64, b: u64) -> u64 {
        match self {
            Operation::Add => a + b,
            Operation::Multiply => a * b,
        }
    }
}


struct Equation(u64, Vec<u64>);

impl Equation {
    fn is_valid(&self) -> bool {
        let operation_combinations = (0..self.1.len() - 1)
            .map(|_| OPERATIONS.iter())
            .multi_cartesian_product();

        operation_combinations
            .into_iter()
            .any(|ops| {
                let result = ops
                    .iter().zip(&self.1[1..])
                    .fold(self.1[0], |acc, (op, &value)| op.execute(acc, value));

                result == self.0
            })
    }
}

fn part_one(mut input: &str) -> u64 {
    let input = parse_input(&mut input).unwrap();

    input
        .iter()
        .filter(|eq| eq.is_valid())
        .map(|eq| eq.0)
        .sum()
}

fn part_two(_input: &str) -> u64 {
    todo!()
}

fn main() {
    let input = include_str!("input");
    let result1 = part_one(input);
    println!("Day 7, part 1: {}", result1);

    // let result2 = part_two(input);
    // println!("Day 7, part 2: {}", result2);
}

#[cfg(test)]
mod test {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(190, vec![10, 19])]
    #[case(3267, vec![81, 40, 27])]
    #[case(292, vec![11, 6, 16, 20])]
    fn valid_equation_should_pass_validation(#[case] target: u64, #[case] numbers: Vec<u64>) {
        let equation = Equation(target, numbers);
        assert!(equation.is_valid(), "Equation should be valid");
    }

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