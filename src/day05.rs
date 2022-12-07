use anyhow::anyhow;
use std::{
    fs::File,
    io::{BufRead, BufReader},
    str::FromStr, hash::Hash, collections::HashMap
  };

use itertools::Itertools;

#[derive(Debug, PartialEq, Clone, Hash, Eq)]
struct Crate {
  name: char
}

impl FromStr for Crate {
  type Err = anyhow::Error;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    if !(s.len() == 3 || s.len()  == 4) {
      return Err(anyhow!("Invalid crate format: {}", s));
    }

    let mut chars = s.chars();
    match (chars.next(), chars.next(), chars.next(), chars.next(), chars.next()) {
      (Some('['), Some(c), Some(']'), None, None)
      | (Some('['), Some(c), Some(']'), Some(' '), None) => Ok(Crate { name: c }),
      _ => Err(anyhow!("Invalid crate : {}", s)),
    }
  }
}

fn parse_crate_row(row: &str) -> Vec<Option<Crate>>{
  row.chars().collect::<Vec<char>>().chunks(4).map(|c| {
    match c.len()
    {
      3 => c.iter().collect::<String>().parse::<Crate>().ok(),
      4 =>  c.iter().collect::<String>().parse::<Crate>().ok(),
      _ => None
    }
  })
  .collect::<Vec<Option<Crate>>>()
}

#[derive(Debug, Clone)]
struct Stack  {
  crates: Vec<Crate>
}

impl Stack {
  fn new() -> Stack {
    Stack { crates: vec![] }
  }

  fn add(&mut self, crate_: &Crate) {
    self.crates.push(crate_.to_owned());
  }

  fn remove(&mut self) -> Option<Crate> {
    self.crates.pop()
  }
}

enum CraneType {
  CrateMover9000,
  CrateMover9001
}

#[derive(Debug, Clone)]
struct Cargo {
  stacks: HashMap<usize, Stack>
}

impl Cargo {
  fn apply_serie(&mut self, serie: &Serie, crane_type: CraneType) -> anyhow::Result<()> {
    let origin = self.stacks.get_mut(&serie.origin).ok_or_else(|| anyhow!("Invalid serie: origin stack not found"))?;

    let mut to_add = (0..serie.quantity).fold(vec![], | acc: Vec<Crate>, _ | {
      let mut acc = acc;
      if let Some(crate_) = origin.remove() {
        acc.push(crate_);
      }

      acc
    });

    drop(origin);

    let destination = self.stacks.get_mut(&serie.destination).ok_or_else(|| anyhow!("Invalid serie: destination stack not found"))?;

    match crane_type {
      CraneType::CrateMover9000 => {
        for crate_ in to_add {
          destination.add(&crate_);
        }
      },
      CraneType::CrateMover9001 => {
        loop  {
          let crate_ = to_add.pop();
          match crate_ {
            Some(crate_) => destination.add(&crate_),
            None => break
          }
        }
      }
    }

    Ok(())
  }

  fn get_crate_in_top(&self) -> String{
    self.stacks.keys().sorted().fold(String::new(), | acc, key | {
      let mut acc = acc;
      if let Some(crate_) = self.stacks.get(key).unwrap().crates.last() {
        acc.push(crate_.name);
      }

      acc
    })
  }
}

#[derive(Debug, Clone)]
struct Serie {
  quantity: u32,
  origin: usize,
  destination: usize
}

// parse from string with format "move quantity from origin to destination"
impl FromStr for Serie {
  type Err = anyhow::Error;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let parts = s.split(' ').collect::<Vec<&str>>();
    if parts.len() != 6 {
      return Err(anyhow!("Invalid serie format: {}", s));
    }
    let quantity = parts[1].parse::<u32>()?;
    let origin = parts[3].parse::<usize>()?;
    let destination = parts[5].parse::<usize>()?;

    Ok(Serie { quantity, origin, destination })
  }
}

fn load_cargo(lines: &[String]) -> anyhow::Result<Cargo> {
  let mut stacks = HashMap::new();
  let (stacks_line, supplies_lines) = lines.split_last().ok_or_else(|| anyhow!("Invalid cargo format"))?;
  stacks_line.chars().for_each(|c| {
    if let Ok(c) = c.to_string().parse::<usize>() {
      stacks.insert(c, Stack::new());
    }
  });

  let mut supplies_lines = supplies_lines.to_vec();
  supplies_lines.reverse();
  supplies_lines.iter().for_each(|line| {
    let crates = parse_crate_row(line);
    crates.iter().enumerate().for_each(|(i, c)| {
      if let Some(c) = c {
        let key = i+1;
        if let Ok(stack) = stacks.get_mut(&key).ok_or_else(|| anyhow!("Invalid cargo format")) {
          stack.add(c);
        }
      }
    });
  });

  Ok(Cargo { stacks })
}

fn load_from_file(path: &str) -> anyhow::Result<(Cargo, Vec<Serie>)>
{
  let file = File::open(path)?;
  let reader = BufReader::new(file);
  let mut cargo_lines = vec![];
  let mut series: Vec<Serie> = vec![];

  for line in reader.lines() {
    let line = line?;
    if line.is_empty()
    {
      continue;
    }

    let serie = line.parse::<Serie>();
    match serie {
      Ok(serie) => series.push(serie),
      Err(_) => cargo_lines.push(line)
    }
  }

  let cargo = load_cargo(&cargo_lines)?;

  Ok((cargo, series))
}

pub fn day05() -> anyhow::Result<()> {
  let (mut cargo, series) = load_from_file("data/day05.txt")?;

  let mut cargo_part2 = cargo.clone();
  let series_part2 = series.clone();

  for serie in series {
    cargo.apply_serie(&serie, CraneType::CrateMover9000)?;
  }

  println!("Day05 part1: {}", cargo.get_crate_in_top());

  for serie in series_part2 {
    cargo_part2.apply_serie(&serie, CraneType::CrateMover9001)?;
  }
  println!("Day05 part1: {}", cargo_part2.get_crate_in_top());

  Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn should_parse_crate() {
      assert_eq!("[A]".parse::<Crate>().unwrap().name, 'A');
      assert_eq!("[A] ".parse::<Crate>().unwrap(), Crate{ name: 'A' });
    }

    #[test]
    fn should_parse_crate_row_with_empty_and_crate() {
      let resut = parse_crate_row("    [D]");
      assert_eq!(resut.len(), 2);
      assert_eq!(resut[0], None);
      assert_eq!(resut[1], Some(Crate{ name: 'D' }));
    }

    #[test]
    fn should_parse_crate_row_with_two_crates() {
      let resut = parse_crate_row("[N] [C]");
      assert_eq!(resut.len(), 2);
      assert_eq!(resut[0], Some(Crate{ name: 'N' }));
      assert_eq!(resut[1], Some(Crate{ name: 'C' }));
    }

    #[test]
    fn should_parse_crate_row_with_three_crates() {
      let resut = parse_crate_row("[Z] [M] [P]");
      assert_eq!(resut.len(), 3);
      assert_eq!(resut[0], Some(Crate{ name: 'Z' }));
      assert_eq!(resut[1], Some(Crate{ name: 'M' }));
      assert_eq!(resut[2], Some(Crate{ name: 'P' }));
    }

    #[test]
    fn should_parse_crate_row_with_one_crate() {
      let resut = parse_crate_row("[L]");
      assert_eq!(resut.len(), 1);
      assert_eq!(resut[0], Some(Crate{ name: 'L' }));
    }

    #[test]
    fn should_parse_series() {
      let serie = "move 1 from 2 to 1".parse::<Serie>().unwrap();
      assert_eq!(serie.quantity, 1);
      assert_eq!(serie.origin, 2);
      assert_eq!(serie.destination, 1);
    }


    #[test]
    fn should_get_crate_in_top() {
      let mut stacks = HashMap::new();
      stacks.insert(1, Stack { crates: vec![Crate{ name: 'C'}] });
      stacks.insert(2, Stack { crates: vec![Crate{ name: 'M'}] });
      stacks.insert(3, Stack { crates: vec![Crate{ name: 'P'}, Crate{ name: 'D'}, Crate{ name: 'N'}, Crate{ name: 'Z'}] });
      let cargo = Cargo {
        stacks: stacks
      };

      assert_eq!(cargo.get_crate_in_top(), "CMZ");
    }
}
