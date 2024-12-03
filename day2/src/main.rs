struct Report {
    levels: Vec<u32>
}

impl Report {
    fn new(levels: Vec<u32>) -> Report {
        Report {
            levels
        }
    }

    fn is_valid (&self) -> bool {
        let asc = self.levels.is_sorted();
        let desc = self.levels.iter().rev().is_sorted();

        if !asc && !desc {
            return false;
        }

        self.levels
            .windows(2)
            .all(|pair| pair[0].abs_diff(pair[1]) <= 3 && pair[0].abs_diff(pair[1]) >= 1)
    }

    fn is_valid_with_damper(&self) -> bool {
        if self.is_valid() {
            return true;
        }

        (0..self.levels.len())
            .map(|i| {
                let mut modified_levels = self.levels.clone();
                modified_levels.remove(i);
                Report::new(modified_levels)
            })
            .any(|modified_report| modified_report.is_valid())
    }
}

fn parse_input(input: &str) -> Vec<Report> {
    input
        .lines()
        .map(|line| {
            let levels = line.split_whitespace()
                .map(|num| num.parse::<u32>().unwrap())
                .collect::<Vec<u32>>();
            Report::new(levels)
        })
        .collect()
}

fn part_one(input: &str) -> u32 {
    let reports = parse_input(input);

    let safe_reports = reports
        .iter()
        .filter(|report| report.is_valid());

    safe_reports.count() as u32
}


fn part_two(input: &str) -> u32 {
    let reports = parse_input(input);

    let safe_reports = reports
        .iter()
        .filter(|report| report.is_valid_with_damper());

    safe_reports.count() as u32
}

fn main() {
    let input = include_str!("input");
    let result1 = part_one(input);
    println!("Day 2, part 1: {}", result1);

    let result2 = part_two(input);
    println!("Day 2, part 2: {}", result2);
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn example_one() {
        let example = include_str!("example");
        let actual = part_one(example);
        assert_eq!(2, actual);
    }

    #[test]
    fn example_two() {
        let example = include_str!("example");
        let actual = part_two(example);
        assert_eq!(4, actual);
    }
}