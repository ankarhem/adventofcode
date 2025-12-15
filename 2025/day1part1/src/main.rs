use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
enum Rotation {
    Left(i16),
    Right(i16),
}

fn parse(input: &str) -> Vec<Rotation> {
    let rotations: Vec<Rotation> = input
        .lines()
        .filter_map(|s| {
            let (dir, num) = s.trim().split_at_checked(1)?;
            match dir {
                "L" => Some(Rotation::Left(num.parse().ok()?)),
                "R" => Some(Rotation::Right(num.parse().ok()?)),
                _ => None,
            }
        })
        .collect();
    rotations
}

const MAX_ROTATIONS: i16 = 100;
fn run(input: &str) -> u16 {
    let rotations = parse(input);

    let mut count_0s: u16 = 0;
    let mut current_state: i16 = 50;

    for rotation in rotations {
        let next_state: i16 = match rotation {
            Rotation::Left(n) => (current_state - n) % MAX_ROTATIONS,
            Rotation::Right(n) => (current_state + n) % MAX_ROTATIONS,
        };

        if next_state == 0 {
            count_0s += 1;
        }
        current_state = next_state;
    }

    count_0s
}

fn main() {
    let input = include_str!("../input.txt");

    let count_0s = run(input);
    println!("Number of times at position 0: {}", count_0s);
}

#[cfg(test)]
mod test {
    use insta::assert_yaml_snapshot;

    use super::*;

    const INPUT: &str = r#"
            L68
            L30
            R48
            L5
            R60
            L55
            L1
            L99
            R14
            L82
        "#;

    #[test]
    fn parses_example() {
        let rotations = parse(INPUT);
        assert_yaml_snapshot!(rotations);
    }

    #[test]
    fn handles_example() {
        let result = run(INPUT);
        assert_eq!(result, 3);
    }
}
