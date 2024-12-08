use itertools::Itertools;
use lib::parsers::number;
use winnow::ascii::{newline, space1};
use winnow::combinator::{separated, separated_pair};
use winnow::{PResult, Parser};

fn parse_input(input: &mut &str) -> PResult<Vec<Equation>> {
    let equation_parser =
        separated_pair(number::<u64>, ": ", separated(1.., number::<u64>, space1))
            .map(|(target, values)| Equation(target, values));

    separated(1.., equation_parser, newline).parse_next(input)
}

#[derive(Debug)]
enum Operation {
    Add,
    Multiply,
    Concatenate,
}

const PART_ONE_OPERATIONS: [Operation; 2] = [Operation::Add, Operation::Multiply];
const PART_TWO_OPERATIONS: [Operation; 3] = [Operation::Add, Operation::Multiply, Operation::Concatenate];

impl Operation {
    fn execute(&self, a: u64, b: u64) -> u64 {
        match self {
            Operation::Add => a + b,
            Operation::Multiply => a * b,
            Operation::Concatenate => format!("{}{}", a, b).parse().unwrap()
        }
    }
}

struct Equation(u64, Vec<u64>);

impl Equation {
    fn is_valid_using(&self, ops: &[Operation]) -> bool {
        let operation_combinations = (0..self.1.len() - 1)
            .map(|_| ops.iter())
            .multi_cartesian_product();

        operation_combinations.into_iter().any(|ops| {
            let result = ops
                .iter()
                .zip(&self.1[1..])
                .fold(self.1[0], |acc, (op, &value)| op.execute(acc, value));

            result == self.0
        })
    }
}

fn part_one(mut input: &str) -> u64 {
    let input = parse_input(&mut input).unwrap();

    input.iter().filter(|eq| eq.is_valid_using(&PART_ONE_OPERATIONS)).map(|eq| eq.0).sum()
}

fn part_two(mut input: &str) -> u64 {
    let input = parse_input(&mut input).unwrap();

    input.iter().filter(|eq| eq.is_valid_using(&PART_TWO_OPERATIONS)).map(|eq| eq.0).sum()
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
    use rstest::rstest;

    #[rstest]
    #[case(190, vec![10, 19])]
    #[case(3267, vec![81, 40, 27])]
    #[case(292, vec![11, 6, 16, 20])]
    fn valid_equation_should_pass_validation(#[case] target: u64, #[case] numbers: Vec<u64>) {
        let equation = Equation(target, numbers);
        assert!(equation.is_valid_using(&PART_ONE_OPERATIONS), "Equation should be valid");
        assert!(equation.is_valid_using(&PART_TWO_OPERATIONS), "Equation should be valid");
    }
    
    #[rstest]
    #[case(156, vec![15, 6])]
    #[case(7290, vec![6, 8, 6, 15])]
    #[case(192, vec![17, 8, 14])]
    fn valid_equation_using_concat_should_pass_validation(#[case] target: u64, #[case] numbers: Vec<u64>) {
        let equation = Equation(target, numbers);
        assert!(equation.is_valid_using(&PART_TWO_OPERATIONS), "Equation should be valid");
    }

    #[test]
    fn example_one() {
        let example = include_str!("example");
        let actual = part_one(example);
        assert_eq!(3749, actual);
    }

    #[test]
    fn example_two() {
        let example = include_str!("example");
        let actual = part_two(example);
        assert_eq!(11387, actual);
    }
}