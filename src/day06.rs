use anyhow::anyhow;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn load_data_stream(path: &str) -> anyhow::Result<String> {
    Ok(BufReader::new(File::open(path)?)
        .lines()
        .next()
        .ok_or_else(|| anyhow!("No data found"))??)
}

fn subroutine(data_stream: &str, marker_size: usize) -> usize {
    let mut marker: Vec<char> = vec![];
    let mut marker_index = 0;
    for (index, letter) in data_stream.chars().enumerate() {
        if let Some(position) = marker.iter().position(|&c| c == letter) {
            for _ in 0..=position {
                marker.remove(0);
            }
        }

        marker.push(letter);
        marker_index = index;

        if marker.len() == marker_size {
            break;
        }
    }

    marker_index + 1
}

pub fn day06() -> anyhow::Result<()> {
    let data_stream = load_data_stream("data/day06.txt")?;

    println!(
        "Day 06 start-of-packet marker: {}",
        subroutine(&data_stream, 4)
    );
    println!("Day 06 messages: {}", subroutine(&data_stream, 14));

    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn should_return_subroutine_of_marker() {
        assert_eq!(subroutine("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 4), 7);
        assert_eq!(subroutine("bvwbjplbgvbhsrlpgdmjqwftvncz", 4), 5);
        assert_eq!(subroutine("nppdvjthqldpwncqszvftbrmjlhg", 4), 6);
        assert_eq!(subroutine("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 4), 10);
        assert_eq!(subroutine("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 4), 11);
    }

    #[test]
    fn should_return_subroutine_of_message() {
        assert_eq!(subroutine("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 14), 19);
        assert_eq!(subroutine("bvwbjplbgvbhsrlpgdmjqwftvncz", 14), 23);
        assert_eq!(subroutine("nppdvjthqldpwncqszvftbrmjlhg", 14), 23);
        assert_eq!(subroutine("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 14), 29);
        assert_eq!(subroutine("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 14), 26);
    }
}
