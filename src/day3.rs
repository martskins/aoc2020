use crate::io::read_lines;
use crate::result::Result;

enum Cell {
    Tree,
    OpenSquare,
}

pub struct Grid {
    rows: Vec<Vec<Cell>>,
}

impl Grid {
    fn get(&self, row: usize, col: usize) -> Option<&Cell> {
        if row >= self.rows.len() {
            return None;
        }

        let line = &self.rows[row];
        if col >= line.len() {
            return Some(&self.rows[row][col % line.len()]);
        }

        Some(&self.rows[row][col])
    }
}

pub struct Puzzle;

impl crate::PuzzleRunner for Puzzle {
    const DAY: usize = 3;
    type First = usize;
    type Second = usize;
    type Input = Grid;

    fn parse_input(&self, filename: &str) -> Result<Self::Input> {
        let rows = read_lines(filename)?
            .into_iter()
            .map(|l| {
                let l: Vec<Cell> = l
                    .split("")
                    .filter_map(|c| match c {
                        "." => Some(Cell::OpenSquare),
                        "#" => Some(Cell::Tree),
                        _ => None,
                    })
                    .collect();
                l
            })
            .collect();
        Ok(Grid { rows })
    }

    fn part_one(&self, grid: &Self::Input) -> Self::First {
        let mut row = 1;
        let mut col = 3;
        let mut trees = 0;
        while let Some(cell) = grid.get(row, col) {
            if matches!(cell, Cell::Tree) {
                trees += 1;
            }

            row += 1;
            col += 3;
        }

        trees
    }

    fn part_two(&self, grid: &Self::Input) -> Self::Second {
        let traversals = vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
        let mut results = vec![];
        for traversal in traversals.into_iter() {
            let mut row = traversal.1;
            let mut col = traversal.0;
            let mut trees = 0;
            while let Some(cell) = grid.get(row, col) {
                if matches!(cell, Cell::Tree) {
                    trees += 1;
                }

                row += traversal.1;
                col += traversal.0;
            }

            results.push(trees);
        }

        results.into_iter().fold(1, |acc, i| acc * i)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::PuzzleRunner;

    #[test]
    fn test_part_one() {
        let puzzle = Puzzle;
        let grid = puzzle.test_input().unwrap();
        assert_eq!(7, puzzle.part_one(&grid));
    }

    #[test]
    fn test_part_two() {
        let puzzle = Puzzle;
        let grid = puzzle.test_input().unwrap();
        assert_eq!(336, puzzle.part_two(&grid));
    }
}
