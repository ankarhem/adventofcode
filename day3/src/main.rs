use winnow::combinator::{alt, delimited, separated_pair};
use winnow::error::InputError;
use winnow::token::any;
use winnow::{PResult, Parser};
use lib::parsers::number;

#[derive(Debug, PartialEq)]
struct Multiplication {
    x: u32,
    y: u32,
}

fn numbers_in_parens(input: &mut &str) -> PResult<(u32, u32)> {
    delimited('(', separated_pair(number, ',', number), ')').parse_next(input)
}

fn multiplication(input: &mut &str) -> PResult<Multiplication> {
    let (_, (x, y)) = ("mul", numbers_in_parens).parse_next(input)?;
    Ok(Multiplication { x, y })
}

fn part_one(mut input: &str) -> u32 {
    let mut multiplications: Vec<Multiplication> = vec![];

    while !input.is_empty() {
        if let Ok(mul) = multiplication.parse_next(&mut input) {
            multiplications.push(mul);
        } else {
            any::<_, InputError<_>>.parse_next(&mut input).unwrap();
        }
    }

    multiplications.iter().map(|mul| mul.x * mul.y).sum()
}

enum Instruction {
    Do,
    Dont,
}

fn instruction(input: &mut &str) -> PResult<Instruction> {
    let do_ = "do()".map(|_| Instruction::Do);
    let dont = "don't()".map(|_| Instruction::Dont);

    alt((do_, dont)).parse_next(input)
}

fn part_two(mut input: &str) -> u32 {
    let mut multiplications: Vec<Multiplication> = vec![];
    let mut current_instruction = Instruction::Do;

    while !input.is_empty() {
        if let Ok(inst) = instruction.parse_next(&mut input) {
            current_instruction = inst;
        }

        match (&current_instruction, multiplication.parse_next(&mut input)) {
            (Instruction::Do, Ok(mul)) => multiplications.push(mul),
            _ => {
                any::<_, InputError<_>>.parse_next(&mut input).unwrap();
            }
        }
    }

    multiplications.iter().map(|mul| mul.x * mul.y).sum()
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
    fn test_multiplication() {
        let mut input = "mul(123,251)";
        let result = multiplication.parse_next(&mut input).unwrap();
        assert_eq!(result, Multiplication { x: 123, y: 251 });
    }

    #[test]
    fn example_one() {
        let example = include_str!("example_one");
        let actual = part_one(example);
        assert_eq!(161, actual);
    }

    #[test]
    fn example_two() {
        let example = include_str!("example_two");
        let actual = part_two(example);
        assert_eq!(48, actual);
    }
}