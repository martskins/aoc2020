mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod io;
mod result;

use crate::result::Result;

trait PuzzleRunner {
    const DAY: usize;
    /// output for the first part of the puzzle
    type First: std::fmt::Display;
    /// output for the second part of the puzzle
    type Second: std::fmt::Display;
    /// input for both parts of the puzzle
    type Input;

    fn parse_input(&self, filename: &str) -> Result<Self::Input>;
    fn part_one(&self, entries: &Self::Input) -> Self::First;
    fn part_two(&self, entries: &Self::Input) -> Self::Second;

    #[cfg(test)]
    fn test_input(&self) -> Result<Self::Input> {
        let filename = format!("./data/day{}_test.txt", Self::DAY);
        let entries = self.parse_input(&filename)?;
        Ok(entries)
    }

    fn input(&self) -> Result<Self::Input> {
        let filename = format!("./data/day{}.txt", Self::DAY);
        let entries = self.parse_input(&filename)?;
        Ok(entries)
    }

    fn run(&self) -> Result<()> {
        let entries = self.input()?;
        println!("\nday {} solution", Self::DAY);
        println!("    part one: {}", self.part_one(&entries));
        println!("    part two: {}", self.part_two(&entries));
        Ok(())
    }
}

fn main() -> Result<()> {
    day1::Puzzle.run()?;
    day2::Puzzle.run()?;
    day3::Puzzle.run()?;
    day4::Puzzle.run()?;
    day5::Puzzle.run()?;
    day6::Puzzle.run()?;
    day7::Puzzle.run()?;
    day8::Puzzle.run()?;
    day9::Puzzle::new(25).run()?;

    Ok(())
}
