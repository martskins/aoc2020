use std::collections::HashSet;

use crate::{io::read_lines, PuzzleRunner};

#[derive(Debug, PartialEq)]
pub struct Rule {
    color: String,
    bags: Vec<(usize, String)>,
}

impl From<String> for Rule {
    fn from(input: String) -> Self {
        let parts: Vec<&str> = input.split("bags contain").collect();
        assert!(parts.len() == 2);

        let color = parts[0].trim().to_string();
        if parts[1].trim() == "no other bags." {
            return Rule {
                color,
                bags: vec![],
            };
        }

        let bags = parts[1]
            .split(",")
            .into_iter()
            .map(|i| {
                let parts: Vec<&str> = i.trim().splitn(2, ' ').collect();
                assert!(parts.len() == 2);
                (
                    parts[0].trim().parse().unwrap(),
                    parts[1].split(" bag").collect::<Vec<&str>>()[0].to_string(),
                )
            })
            .collect();

        Rule { color, bags }
    }
}

pub struct Puzzle;

impl PuzzleRunner for Puzzle {
    const DAY: usize = 7;
    type First = usize;
    type Second = usize;
    type Input = Vec<Rule>;

    fn parse_input(&self, filename: &str) -> crate::result::Result<Self::Input> {
        Ok(read_lines(filename)?.into_iter().map(Rule::from).collect())
    }

    fn part_one(&self, entries: &Self::Input) -> Self::First {
        let mut bags_that_can = HashSet::new();
        for entry in entries {
            if entry.bags.iter().any(|f| f.1 == "shiny gold") {
                bags_that_can.insert(entry.color.clone());
            }
        }

        let mut count = 0;
        for entry in entries {
            if can_contain(&bags_that_can, entries, &entry.color) {
                bags_that_can.insert(entry.color.clone());
                count += 1;
            }
        }

        count
    }

    fn part_two(&self, entries: &Self::Input) -> Self::Second {
        let mut bag_count = 0;
        add_bags(entries, &mut bag_count, "shiny gold");
        bag_count
    }
}

fn add_bags(entries: &[Rule], bag_count: &mut usize, color: &str) {
    match entries.iter().find(|f| f.color == color) {
        Some(rule) => {
            for bag in &rule.bags {
                *bag_count += bag.0;
                for _ in 0..bag.0 {
                    add_bags(entries, bag_count, &bag.1);
                }
            }
        }
        None => {}
    }
}

fn can_contain(bags_that_can: &HashSet<String>, entries: &[Rule], bag_color: &str) -> bool {
    if bags_that_can.contains(bag_color) {
        return true;
    }

    let rule = entries.iter().find(|f| f.color == bag_color);
    if rule.is_none() {
        return false;
    }

    for bag in &rule.unwrap().bags {
        if can_contain(bags_that_can, entries, &bag.1) {
            return true;
        }
    }

    return false;
}

#[cfg(test)]
mod test {
    use crate::PuzzleRunner;

    use super::{Puzzle, Rule};

    #[test]
    fn test_parse_rule() {
        let rule: String = "light red bags contain 1 bright white bag, 2 muted yellow bags.".into();
        assert_eq!(
            Rule {
                color: "light red".into(),
                bags: vec![(1, "bright white".into()), (2, "muted yellow".into())]
            },
            Rule::from(rule)
        );
    }

    #[test]
    fn test_part_one() {
        let puzzle = Puzzle;
        let entries = puzzle.test_input().unwrap();
        assert_eq!(4, puzzle.part_one(&entries));
    }

    #[test]
    fn test_part_two() {
        let puzzle = Puzzle;
        let entries = puzzle.test_input().unwrap();
        assert_eq!(32, puzzle.part_two(&entries));
    }
}
