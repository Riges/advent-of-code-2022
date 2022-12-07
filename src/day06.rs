use anyhow::anyhow;
use std::{
    fs::File,
    io::{BufRead, BufReader}
  };

fn load_data_stream(path: &str) -> anyhow::Result<String> {
  let file = File::open(path)?;
  let reader = BufReader::new(file);
  let mut data = String::new();
  for line in reader.lines() {
    match line {
      Ok(line) => {
        data.push_str(&line);
        break;
      },
      Err(_) => return Err(anyhow!("Error reading line"))
    }
  }

  Ok(data)
}

fn subroutine(data_stream: &str, marker_size: usize) -> usize {
  let mut marker: Vec<char> = vec![];
  let mut marker_index = 0;
  for (index, letter) in data_stream.chars().enumerate() {
    match marker.iter().position(|&c| c == letter) {
      Some(position) => {
        for _ in 0..=position {
          marker.remove(0);
        }
      },
      None => {}
    }

    marker.push(letter);
    marker_index = index;

    if marker.len() == marker_size
    {
      break;
    }
  }

  marker_index + 1
}

pub fn day06() -> anyhow::Result<()>  {
  let data_stream = load_data_stream("data/day06.txt")?;
  let result = subroutine(&data_stream, 4);

  println!("Day 06: {}", result);

  Ok(())
}

#[cfg(test)]
mod tests {

  use super::*;

  #[test]
  fn should_return_subroutine() {
    assert_eq!(subroutine("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 4), 7);
    assert_eq!(subroutine("bvwbjplbgvbhsrlpgdmjqwftvncz", 4), 5);
    assert_eq!(subroutine("nppdvjthqldpwncqszvftbrmjlhg", 4), 6);
    assert_eq!(subroutine("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 4), 10);
    assert_eq!(subroutine("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw",4), 11);
  }
}
