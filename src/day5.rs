use crate::{io::read_lines, PuzzleRunner};

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Seat {
    row: u16,
    col: u16,
}

impl Seat {
    fn id(&self) -> u16 {
        self.row * 8 + self.col
    }
}

impl From<&str> for Seat {
    fn from(s: &str) -> Self {
        // all entries have 7 for row and 3 for column
        assert!(s.len() == 10);

        // first 7 characters are the row number, last 3 are the column.
        let (row, col) = s.split_at(7);

        // row and column are binary numbers, row uses B for 1 and F for 0 and column uses R
        // for 1 and L for 0.
        let row = row.replace("B", "1").replace("F", "0");
        let col = col.replace("R", "1").replace("L", "0");

        let row = u16::from_str_radix(&row, 2).unwrap();
        let col = u16::from_str_radix(&col, 2).unwrap();

        Self { row, col }
    }
}

pub struct Puzzle;

impl PuzzleRunner for Puzzle {
    const DAY: usize = 5;
    type First = u16;
    type Second = u16;
    type Input = Vec<Seat>;

    fn parse_input(&self, filename: &str) -> crate::result::Result<Self::Input> {
        Ok(read_lines(filename)?
            .into_iter()
            .map(|c| Seat::from(c.as_str()))
            .collect())
    }

    fn part_one(&self, entries: &Self::Input) -> Self::First {
        entries.into_iter().map(Seat::id).max().unwrap_or_default()
    }

    fn part_two(&self, entries: &Self::Input) -> Self::Second {
        let ids: Vec<u16> = entries.iter().map(|c| c.id()).collect();
        let max_id = entries.into_iter().map(Seat::id).max().unwrap_or_default();
        let min_id = entries.into_iter().map(Seat::id).min().unwrap_or_default();
        for id in min_id..max_id {
            if ids.iter().find(|&i| i == &id).is_none() {
                return id;
            }
        }
        0
    }
}

#[cfg(test)]
mod test {
    use super::{Puzzle, Seat};
    use crate::PuzzleRunner;

    #[test]
    fn test_parse_seat() {
        assert_eq!(Seat { row: 44, col: 5 }, Seat::from("FBFBBFFRLR"));
        assert_eq!(Seat { row: 70, col: 7 }, Seat::from("BFFFBBFRRR"));
        assert_eq!(Seat { row: 14, col: 7 }, Seat::from("FFFBBBFRRR"));
        assert_eq!(Seat { row: 102, col: 4 }, Seat::from("BBFFBBFRLL"));
    }

    #[test]
    fn test_seat_id() {
        assert_eq!(357, Seat { row: 44, col: 5 }.id());
        assert_eq!(567, Seat { row: 70, col: 7 }.id());
        assert_eq!(119, Seat { row: 14, col: 7 }.id());
        assert_eq!(820, Seat { row: 102, col: 4 }.id());
    }

    #[test]
    fn test_part_one() {
        let seats = vec![
            Seat { row: 44, col: 5 },
            Seat { row: 70, col: 7 },
            Seat { row: 14, col: 7 },
            Seat { row: 102, col: 4 },
        ];

        let puzzle = Puzzle;
        assert_eq!(820, puzzle.part_one(&seats));
    }
}
