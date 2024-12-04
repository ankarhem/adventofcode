use lib::Grid;

fn parse_input(input: &str) -> Grid<char> {
    let input: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    Grid::new(input)
}

fn part_one(input: &str) -> u32 {
    let grid: Grid<char> = parse_input(input);

    let rows: Vec<Vec<&char>> = grid.rows().map(|row| row.collect()).collect();
    let columns: Vec<Vec<&char>> = grid.columns().map(|col| col.collect()).collect();
    let diagonals: Vec<Vec<&char>> = grid.diagonals().map(|diag| diag.collect()).collect();

    let row_count: u32 = rows
        .iter()
        .chain(columns.iter())
        .chain(diagonals.iter())
        .map(|row| {
            let string = row.iter().map(|&x| *x).collect::<String>();
            string.matches("XMAS").count() as u32 + string.matches("SAMX").count() as u32
        })
        .sum();

    row_count
}

fn part_two(input: &str) -> u32 {
    let grid: Grid<char> = parse_input(input);

    let mas_count = grid
        .windows(3, 3)
        .filter(|w| {
            let subgrid = Grid::new(w.clone());

            let diagonals: Vec<Vec<&char>> =
                subgrid.diagonals().map(|diag| diag.collect()).collect();
            let is_mas_x = diagonals.iter().filter(|d| d.len() == 3).all(|d| {
                let string = d.iter().map(|&x| *x).collect::<String>();
                string == "MAS" || string == "SAM"
            });

            is_mas_x
        })
        .count() as u32;

    mas_count
}

fn main() {
    let input = include_str!("input");
    let result1 = part_one(input);
    println!("Day 4, part 1: {}", result1);

    let result2 = part_two(input);
    println!("Day 4, part 2: {}", result2);
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn example_one() {
        let example = include_str!("example");
        let actual = part_one(example);
        assert_eq!(18, actual);
    }

    #[test]
    fn example_two() {
        let example = include_str!("example");
        let actual = part_two(example);
        assert_eq!(9, actual);
    }
}