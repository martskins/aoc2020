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

    fn from_str(input: &str) -> Self {
        let mut row: u16 = 0;
        let mut col: u16 = 0;
        for (idx, ch) in input.chars().enumerate() {
            match (idx, ch) {
                (7..=9, 'R') => col += (2 as u16).pow(2 - (idx - 7) as u32),
                (0..=6, 'B') => row += (2 as u16).pow((6 - idx) as u32),
                _ => {}
            }
        }

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
            .map(|c| Seat::from_str(&c))
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
        assert_eq!(Seat { row: 44, col: 5 }, Seat::from_str("FBFBBFFRLR"));
        assert_eq!(Seat { row: 70, col: 7 }, Seat::from_str("BFFFBBFRRR"));
        assert_eq!(Seat { row: 14, col: 7 }, Seat::from_str("FFFBBBFRRR"));
        assert_eq!(Seat { row: 102, col: 4 }, Seat::from_str("BBFFBBFRLL"));
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
