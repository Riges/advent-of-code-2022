use anyhow::anyhow;
use std::{
    fs::File,
    io::{BufRead, BufReader},
    str::FromStr,
};

struct SectionAssigment {
    start: u32,
    end: u32,
}

impl FromStr for SectionAssigment {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split('-').collect::<Vec<&str>>();
        if parts.len() != 2 {
            return Err(anyhow!("Invalid SectionAssigment: {}", s));
        }
        let start = parts[0].parse::<u32>()?;
        let end = parts[1].parse::<u32>()?;

        Ok(SectionAssigment { start, end })
    }
}

struct SectionAssigmentPair {
    first: SectionAssigment,
    second: SectionAssigment,
}

impl SectionAssigmentPair {
    fn fully_contained(&self) -> bool {
        (self.first.start >= self.second.start && self.first.end <= self.second.end)
            || (self.second.start >= self.first.start && self.second.end <= self.first.end)
    }

    fn have_overlap(&self) -> bool {
        (self.first.start >= self.second.start && self.first.start <= self.second.end)
            || (self.second.start >= self.first.start && self.second.start <= self.first.end)
            || (self.first.end >= self.second.start && self.first.end <= self.second.end)
            || (self.second.end >= self.first.start && self.second.end <= self.first.end)
    }
}

// Parse from string with format "SectionAssigment,SectionAssigment"
impl FromStr for SectionAssigmentPair {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split(',').collect::<Vec<&str>>();
        if parts.len() != 2 {
            return Err(anyhow!("Invalid SectionAssigmentPair: {}", s));
        }
        let first = parts[0].parse::<SectionAssigment>()?;
        let second = parts[1].parse::<SectionAssigment>()?;

        Ok(SectionAssigmentPair { first, second })
    }
}

fn load_from_file(path: &str) -> anyhow::Result<Vec<SectionAssigmentPair>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    reader
        .lines()
        .map(|l| l?.parse::<SectionAssigmentPair>())
        .collect()
}

pub fn day04() -> anyhow::Result<()> {
    let section_assigment_pairs = load_from_file("data/day04.txt")?;

    let fully_count = section_assigment_pairs
        .iter()
        .filter(|s| s.fully_contained())
        .count();
    println!("Day 04 - Part 1: {}", fully_count);

    let overlapping_count = section_assigment_pairs
        .iter()
        .filter(|s| s.have_overlap())
        .count();
    println!("Day 04 - Part 2: {}", overlapping_count);

    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn should_be_fully_contained() {
        assert!(SectionAssigmentPair {
            first: SectionAssigment { start: 2, end: 8 },
            second: SectionAssigment { start: 3, end: 7 }
        }
        .fully_contained());
        assert!(SectionAssigmentPair {
            first: SectionAssigment { start: 6, end: 6 },
            second: SectionAssigment { start: 4, end: 6 }
        }
        .fully_contained());
    }

    #[test]
    fn should_overlapping() {
        assert_eq!(
            SectionAssigmentPair {
                first: SectionAssigment { start: 2, end: 4 },
                second: SectionAssigment { start: 6, end: 8 }
            }
            .have_overlap(),
            false
        );
        assert_eq!(
            SectionAssigmentPair {
                first: SectionAssigment { start: 2, end: 3 },
                second: SectionAssigment { start: 4, end: 5 }
            }
            .have_overlap(),
            false
        );
        assert!(SectionAssigmentPair {
            first: SectionAssigment { start: 5, end: 7 },
            second: SectionAssigment { start: 7, end: 9 }
        }
        .have_overlap());
        assert!(SectionAssigmentPair {
            first: SectionAssigment { start: 2, end: 8 },
            second: SectionAssigment { start: 3, end: 7 }
        }
        .have_overlap());
        assert!(SectionAssigmentPair {
            first: SectionAssigment { start: 6, end: 6 },
            second: SectionAssigment { start: 4, end: 6 }
        }
        .have_overlap());
        assert!(SectionAssigmentPair {
            first: SectionAssigment { start: 2, end: 6 },
            second: SectionAssigment { start: 4, end: 8 }
        }
        .have_overlap());
    }
}
