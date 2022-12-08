use anyhow::anyhow;
use std::{
    fs::File,
    io::{BufRead, BufReader},
    str::FromStr,
};

#[derive(Debug, PartialEq, Clone)]
struct Rucksacks {
    compartment1: String,
    compartment2: String,
}

impl Rucksacks {
    fn get_duplicate_item(&self) -> Option<char> {
        let compartment2_items: Vec<char> = self.compartment2.chars().collect();
        self.compartment1
            .chars()
            .find(|&c| compartment2_items.contains(&c))
    }

    fn get_all_items(&self) -> String {
        let mut items = self.compartment1.to_owned();
        items.push_str(&self.compartment2.to_owned());

        items
    }
}

impl FromStr for Rucksacks {
    type Err = anyhow::Error;

    fn from_str(rucksacks: &str) -> Result<Rucksacks, Self::Err> {
        let compartments = rucksacks
            .chars()
            .collect::<Vec<char>>()
            .chunks(rucksacks.chars().count() / 2)
            .map(|c| c.iter().collect::<String>())
            .collect::<Vec<String>>();

        if compartments.len() != 2 {
            return Err(anyhow!("Invalide value: {:?}", compartments));
        }

        let compartment1: String = match compartments.first() {
            Some(s) => s.to_string(),
            None => String::new(),
        };
        let compartment2: String = match compartments.last() {
            Some(s) => s.to_string(),
            None => String::new(),
        };

        Ok(Rucksacks {
            compartment1,
            compartment2,
        })
    }
}

fn load_rucksacks(path: &str) -> anyhow::Result<Vec<Rucksacks>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    reader.lines().map(|l| l?.parse::<Rucksacks>()).collect()
}

fn convert_item_to_priority(item: char) -> u32 {
    let ascii_number = item as u32;
    if ascii_number >= 97 {
        ascii_number - 96
    } else {
        ascii_number - 38
    }
}

fn split_loosers_in_team(rucksacks: Vec<Rucksacks>) -> Vec<Vec<Rucksacks>> {
    let mut groups: Vec<Vec<Rucksacks>> = vec![];
    for chunk in rucksacks.chunks(3) {
        groups.push(chunk.to_vec());
    }

    groups
}

fn find_groups_badges(groups: Vec<Vec<Rucksacks>>) -> Vec<Option<char>> {
    groups
        .iter()
        .map(|group| {
            let mut group_iterrator = group.iter().map(|r| r.get_all_items());

            match (
                group_iterrator.next(),
                group_iterrator.next(),
                group_iterrator.next(),
            ) {
                (Some(first), Some(second), Some(third)) => {
                    for c in first.chars() {
                        if second.contains(c) && third.contains(c) {
                            return Some(c);
                        }
                    }
                    None
                }
                _ => None,
            }
        })
        .collect()
}

pub fn day03() -> anyhow::Result<()> {
    let rucksacks = load_rucksacks("data/day03.txt")?;
    let sum_of_priorities: u32 = rucksacks
        .iter()
        .map(|rucksack| match rucksack.get_duplicate_item() {
            Some(c) => convert_item_to_priority(c),
            None => 0,
        })
        .sum();

    println!(
        "What is the sum of the priorities of those item types? {}",
        sum_of_priorities
    );

    let badges_priorities: u32 = find_groups_badges(split_loosers_in_team(rucksacks))
        .iter()
        .map(|badge| match badge {
            Some(c) => convert_item_to_priority(c.to_owned()),
            None => 0,
        })
        .sum();
    println!(
        "What is the sum of the priorities of those item types? {}",
        badges_priorities
    );

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rucksacks_should_be_parse() {
        assert_eq!(
            "vJrwpWtwJgWrhcsFMMfFFhFp".parse::<Rucksacks>().unwrap(),
            Rucksacks {
                compartment1: "vJrwpWtwJgWr".to_string(),
                compartment2: "hcsFMMfFFhFp".to_string()
            }
        );
        assert_eq!(
            "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL"
                .parse::<Rucksacks>()
                .unwrap(),
            Rucksacks {
                compartment1: "jqHRNqRjqzjGDLGL".to_string(),
                compartment2: "rsFMfFZSrLrFZsSL".to_string()
            }
        );
        assert_eq!(
            "PmmdzqPrVvPwwTWBwg".parse::<Rucksacks>().unwrap(),
            Rucksacks {
                compartment1: "PmmdzqPrV".to_string(),
                compartment2: "vPwwTWBwg".to_string()
            }
        );
        assert_eq!(
            "wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn"
                .parse::<Rucksacks>()
                .unwrap(),
            Rucksacks {
                compartment1: "wMqvLMZHhHMvwLH".to_string(),
                compartment2: "jbvcjnnSBnvTQFn".to_string()
            }
        );
        assert_eq!(
            "ttgJtRGJQctTZtZT".parse::<Rucksacks>().unwrap(),
            Rucksacks {
                compartment1: "ttgJtRGJ".to_string(),
                compartment2: "QctTZtZT".to_string()
            }
        );
        assert_eq!(
            "CrZsJsPPZsGzwwsLwLmpwMDw".parse::<Rucksacks>().unwrap(),
            Rucksacks {
                compartment1: "CrZsJsPPZsGz".to_string(),
                compartment2: "wwsLwLmpwMDw".to_string()
            }
        );
    }

    #[test]
    fn should_find_expected_item() {
        assert_eq!(
            Rucksacks {
                compartment1: "vJrwpWtwJgWr".to_string(),
                compartment2: "hcsFMMfFFhFp".to_string()
            }
            .get_duplicate_item(),
            Some('p')
        );
        assert_eq!(
            Rucksacks {
                compartment1: "jqHRNqRjqzjGDLGL".to_string(),
                compartment2: "rsFMfFZSrLrFZsSL".to_string()
            }
            .get_duplicate_item(),
            Some('L')
        );
        assert_eq!(
            Rucksacks {
                compartment1: "PmmdzqPrV".to_string(),
                compartment2: "vPwwTWBwg".to_string()
            }
            .get_duplicate_item(),
            Some('P')
        );
        assert_eq!(
            Rucksacks {
                compartment1: "wMqvLMZHhHMvwLH".to_string(),
                compartment2: "jbvcjnnSBnvTQFn".to_string()
            }
            .get_duplicate_item(),
            Some('v')
        );
        assert_eq!(
            Rucksacks {
                compartment1: "ttgJtRGJ".to_string(),
                compartment2: "QctTZtZT".to_string()
            }
            .get_duplicate_item(),
            Some('t')
        );
        assert_eq!(
            Rucksacks {
                compartment1: "CrZsJsPPZsGz".to_string(),
                compartment2: "wwsLwLmpwMDw".to_string()
            }
            .get_duplicate_item(),
            Some('s')
        );
    }

    #[test]
    fn rucksacks_should_return_allitem() {
        assert_eq!(
            Rucksacks {
                compartment1: "vJrwpWtwJgWr".to_string(),
                compartment2: "hcsFMMfFFhFp".to_string()
            }
            .get_all_items(),
            "vJrwpWtwJgWrhcsFMMfFFhFp"
        );
    }

    #[test]
    fn convert_item_to_priority_should_return_expected_priority() {
        assert_eq!(convert_item_to_priority('p'), 16);
        assert_eq!(convert_item_to_priority('L'), 38);
        assert_eq!(convert_item_to_priority('P'), 42);
        assert_eq!(convert_item_to_priority('v'), 22);
        assert_eq!(convert_item_to_priority('t'), 20);
        assert_eq!(convert_item_to_priority('s'), 19);
    }

    #[test]
    fn should_find_badge() {
        assert_eq!(
            find_groups_badges(vec![
                vec![
                    "vJrwpWtwJgWrhcsFMMfFFhFp".parse::<Rucksacks>().unwrap(),
                    "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL"
                        .parse::<Rucksacks>()
                        .unwrap(),
                    "PmmdzqPrVvPwwTWBwg".parse::<Rucksacks>().unwrap()
                ],
                vec![
                    "wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn"
                        .parse::<Rucksacks>()
                        .unwrap(),
                    "ttgJtRGJQctTZtZT".parse::<Rucksacks>().unwrap(),
                    "CrZsJsPPZsGzwwsLwLmpwMDw".parse::<Rucksacks>().unwrap()
                ]
            ]),
            vec![Some('r'), Some('Z')]
        );
    }
}
