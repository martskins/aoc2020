use crate::{io::read_lines, PuzzleRunner};
use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref HEIGHT_RE: Regex = Regex::new(r"(\d+)(\w+)").unwrap();
    static ref HAIR_COLOR_RE: Regex = Regex::new(r#"#[0-9a-f]{6}"#).unwrap();
    static ref PASSPORT_ID_RE: Regex = Regex::new(r#"^\d{9}$"#).unwrap();
}

enum Height {
    Centimeters(u16),
    Inches(u16),
}

impl Height {
    fn parse(input: &str) -> Option<Height> {
        let captures = HEIGHT_RE.captures(input);
        if captures.is_none() {
            return None;
        }

        let captures = captures.unwrap();
        let cap1 = captures.get(1).map(|m| m.as_str());
        let cap2 = captures.get(2).map(|m| m.as_str());
        match (cap1, cap2) {
            (Some(val), Some("cm")) => val.parse().ok().map(|v| Height::Centimeters(v)),
            (Some(val), Some("in")) => val.parse().ok().map(|v| Height::Inches(v)),
            _ => None,
        }
    }

    fn valid(input: &str) -> bool {
        let hgt = Height::parse(input);
        if hgt.is_none() {
            return false;
        }
        let hgt = hgt.unwrap();
        match hgt {
            Height::Centimeters(val) if val >= 150 && val <= 193 => true,
            Height::Inches(val) if val >= 59 && val <= 76 => true,
            _ => false,
        }
    }
}

enum EyeColor {
    Amb,
    Blu,
    Brn,
    Gry,
    Grn,
    Hzl,
    Oth,
}

impl EyeColor {
    fn parse(input: &str) -> Option<Self> {
        match input {
            "amb" => Some(EyeColor::Amb),
            "blu" => Some(EyeColor::Blu),
            "brn" => Some(EyeColor::Brn),
            "gry" => Some(EyeColor::Gry),
            "grn" => Some(EyeColor::Grn),
            "hzl" => Some(EyeColor::Hzl),
            "oth" => Some(EyeColor::Oth),
            _ => None,
        }
    }

    fn valid(input: &str) -> bool {
        let ec = EyeColor::parse(input);
        if ec.is_none() {
            return false;
        }

        return true;
    }
}

#[derive(Debug, Default)]
pub struct Passport {
    byr: Option<String>,
    iyr: Option<String>,
    eyr: Option<String>,
    hgt: Option<String>,
    hcl: Option<String>,
    ecl: Option<String>,
    pid: Option<String>,
    cid: Option<String>,
}

impl Passport {
    fn parse(input: String) -> Passport {
        let mut passport = Passport::default();
        let parts = input.split_whitespace();
        for part in parts {
            let part: Vec<&str> = part.split(":").collect();
            let key = part.get(0).unwrap();
            let val = part.get(1).unwrap();
            match *key {
                "byr" => passport.byr = Some(val.to_string()),
                "iyr" => passport.iyr = Some(val.to_string()),
                "eyr" => passport.eyr = Some(val.to_string()),
                "hgt" => passport.hgt = Some(val.to_string()),
                "hcl" => passport.hcl = Some(val.to_string()),
                "ecl" => passport.ecl = Some(val.to_string()),
                "pid" => passport.pid = Some(val.to_string()),
                "cid" => passport.cid = Some(val.to_string()),
                _ => {}
            }
        }
        passport
    }

    fn valid_byr(&self) -> bool {
        if self.byr.is_none() {
            return false;
        }
        match self.byr.as_ref().unwrap().parse::<u16>().ok() {
            Some(byr) if byr >= 1920 && byr <= 2002 => true,
            _ => false,
        }
    }

    fn valid_iyr(&self) -> bool {
        if self.iyr.is_none() {
            return false;
        }
        match self.iyr.as_ref().unwrap().parse::<u16>().ok() {
            Some(iyr) if iyr >= 2010 && iyr <= 2020 => true,
            _ => false,
        }
    }

    fn valid_eyr(&self) -> bool {
        if self.eyr.is_none() {
            return false;
        }
        match self.eyr.as_ref().unwrap().parse::<u16>().ok() {
            Some(eyr) if eyr >= 2020 && eyr <= 2030 => true,
            _ => false,
        }
    }

    fn valid_ecl(&self) -> bool {
        if self.ecl.is_none() {
            return false;
        }

        EyeColor::valid(self.ecl.as_ref().unwrap())
    }

    fn valid_hgt(&self) -> bool {
        if self.hgt.is_none() {
            return false;
        }

        Height::valid(self.hgt.as_ref().unwrap())
    }

    fn valid_hcl(&self) -> bool {
        match &self.hcl {
            Some(hcl) if HAIR_COLOR_RE.is_match(&hcl) => true,
            _ => false,
        }
    }

    fn valid_pid(&self) -> bool {
        match &self.pid {
            Some(pid) if PASSPORT_ID_RE.is_match(pid) => true,
            _ => false,
        }
    }
}

pub struct Puzzle;

impl PuzzleRunner for Puzzle {
    const DAY: usize = 4;
    type First = usize;
    type Second = usize;
    type Input = Vec<Passport>;

    fn parse_input(&self, filename: &str) -> crate::result::Result<Self::Input> {
        let mut data = vec![];

        let lines = read_lines(&filename)?.into_iter();
        let mut parts = vec![];
        for line in lines {
            if line.is_empty() {
                data.push(parts.join(" "));
                parts = vec![];
                continue;
            }

            parts.push(line);
        }

        data.push(parts.join(" "));
        Ok(data.into_iter().map(Passport::parse).collect())
    }

    fn part_one(&self, entries: &Self::Input) -> Self::First {
        entries
            .into_iter()
            .filter(|p| {
                p.byr.is_some()
                    && p.iyr.is_some()
                    && p.eyr.is_some()
                    && p.hgt.is_some()
                    && p.hcl.is_some()
                    && p.ecl.is_some()
                    && p.pid.is_some()
            })
            .count()
    }

    fn part_two(&self, entries: &Self::Input) -> Self::Second {
        entries
            .into_iter()
            .filter(|p| {
                p.valid_byr()
                    && p.valid_ecl()
                    && p.valid_eyr()
                    && p.valid_hcl()
                    && p.valid_hgt()
                    && p.valid_iyr()
                    && p.valid_pid()
            })
            .count()
    }
}

#[cfg(test)]
mod test {
    use super::{EyeColor, Height, Puzzle, HAIR_COLOR_RE, PASSPORT_ID_RE};
    use crate::PuzzleRunner;

    #[test]
    fn test_part_one() {
        let puzzle = Puzzle;
        let entries = puzzle.test_input().unwrap();
        assert_eq!(2, puzzle.part_one(&entries));
    }

    #[test]
    fn test_part_two() {
        let puzzle = Puzzle;
        let entries = puzzle.parse_input("./data/day4_part_two_test.txt").unwrap();
        assert_eq!(4, puzzle.part_two(&entries));
    }

    #[test]
    fn test_eye_color() {
        assert!(EyeColor::valid("brn"));
        assert!(!EyeColor::valid("moo"));
    }

    #[test]
    fn test_height_parse() {
        assert!(Height::valid("60in"));
        assert!(Height::valid("190cm"));
        assert!(!Height::valid("190in"));
        assert!(!Height::valid("190"));
    }

    #[test]
    fn test_hair_color() {
        assert!(HAIR_COLOR_RE.is_match("#123abc"));
        assert!(!HAIR_COLOR_RE.is_match("#123abz"));
        assert!(!HAIR_COLOR_RE.is_match("123abc"));
    }

    #[test]
    fn test_validate_pid() {
        assert!(PASSPORT_ID_RE.is_match("123456789"));
        assert!(!PASSPORT_ID_RE.is_match("0123456789"));
    }
}
