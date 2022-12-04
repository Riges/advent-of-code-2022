use anyhow::anyhow;
use std::{
    fs::File,
    io::{BufRead, BufReader},
    str::FromStr,
};

#[derive(Debug, PartialEq)]
enum Player1 {
    Rock,
    Paper,
    Scissors,
}

impl Player1 {
    fn score(&self) -> i32 {
        match self {
            Player1::Rock => 1,
            Player1::Paper => 2,
            Player1::Scissors => 3,
        }
    }
}

impl FromStr for Player1 {
    type Err = anyhow::Error;

    fn from_str(letter: &str) -> Result<Player1, Self::Err> {
        match letter {
            "A" => Ok(Player1::Rock),
            "B" => Ok(Player1::Paper),
            "C" => Ok(Player1::Scissors),
            _ => Err(anyhow!("Invalide Player 1 value: {}", letter)),
        }
    }
}

#[derive(Debug, PartialEq)]
enum Player2 {
    Rock,
    Paper,
    Scissors,
}

impl Player2 {
    fn score(&self) -> i32 {
        match self {
            Player2::Rock => 1,
            Player2::Paper => 2,
            Player2::Scissors => 3,
        }
    }
}

impl FromStr for Player2 {
    type Err = anyhow::Error;

    fn from_str(letter: &str) -> Result<Player2, Self::Err> {
        match letter {
            "Y" => Ok(Player2::Paper),
            "X" => Ok(Player2::Rock),
            "Z" => Ok(Player2::Scissors),
            _ => Err(anyhow!("Invalide Player 2 value: {}", letter)),
        }
    }
}

#[derive(Debug, PartialEq)]
enum Winner {
    Player1,
    Player2,
    Draw,
}

#[derive(Debug)]
struct Strategy {
    player1: Player1,
    player2: Player2,
}

impl Strategy {
    fn winner(&self) -> Winner {
        match (&self.player1, &self.player2) {
            (Player1::Rock, Player2::Rock)
            | (Player1::Paper, Player2::Paper)
            | (Player1::Scissors, Player2::Scissors) => Winner::Draw,
            (Player1::Rock, Player2::Scissors)
            | (Player1::Paper, Player2::Rock)
            | (Player1::Scissors, Player2::Paper) => Winner::Player1,
            (_, _) => Winner::Player2,
        }
    }

    fn score(&self) -> i32 {
        let round_score = match self.winner() {
            Winner::Player2 => 6,
            Winner::Draw => 3,
            _ => 0,
        };

        self.player2.score() + round_score
    }
}

fn load_strategies_file(path: &str) -> anyhow::Result<Vec<Strategy>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    reader
        .lines()
        .map(|line| {
            let l = line?;
            let mut split = l.split_whitespace();
            let player1 = split.next().ok_or_else(|| anyhow!("error player 1"))?.parse()?;
            let player2 = split.next().ok_or_else(|| anyhow!("error player 2"))?.parse()?;

            Ok(Strategy { player1, player2 })
        })
        .collect()
}

pub fn day02() -> anyhow::Result<()> {
    let strategies = load_strategies_file("data/day02.txt")?;

    println!("What would your total score be if everything goes exactly according to your strategy guide? {:?}", strategies.iter().map(|s| s.score()).sum::<i32>());

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn player1_should_win() {
        assert_eq!(
            Strategy {
                player1: Player1::Rock,
                player2: Player2::Scissors
            }
            .winner(),
            Winner::Player1
        );
        assert_eq!(
            Strategy {
                player1: Player1::Paper,
                player2: Player2::Rock
            }
            .winner(),
            Winner::Player1
        );
        assert_eq!(
            Strategy {
                player1: Player1::Scissors,
                player2: Player2::Paper
            }
            .winner(),
            Winner::Player1
        );
    }

    #[test]
    fn player2_should_win() {
        assert_eq!(
            Strategy {
                player1: Player1::Scissors,
                player2: Player2::Rock
            }
            .winner(),
            Winner::Player2
        );
        assert_eq!(
            Strategy {
                player1: Player1::Rock,
                player2: Player2::Paper
            }
            .winner(),
            Winner::Player2
        );
        assert_eq!(
            Strategy {
                player1: Player1::Paper,
                player2: Player2::Scissors
            }
            .winner(),
            Winner::Player2
        );
    }

    #[test]
    fn round_should_be_a_draw() {
        assert_eq!(
            Strategy {
                player1: Player1::Rock,
                player2: Player2::Rock
            }
            .winner(),
            Winner::Draw
        );
        assert_eq!(
            Strategy {
                player1: Player1::Paper,
                player2: Player2::Paper
            }
            .winner(),
            Winner::Draw
        );
        assert_eq!(
            Strategy {
                player1: Player1::Scissors,
                player2: Player2::Scissors
            }
            .winner(),
            Winner::Draw
        );
    }

    #[test]
    fn samples() {
        assert_eq!(
            Strategy {
                player1: Player1::Rock,
                player2: Player2::Paper
            }
            .score(),
            8
        );
        assert_eq!(
            Strategy {
                player1: Player1::Paper,
                player2: Player2::Rock
            }
            .score(),
            1
        );
        assert_eq!(
            Strategy {
                player1: Player1::Scissors,
                player2: Player2::Scissors
            }
            .score(),
            6
        );
    }

    #[test]
    fn player1_should_be_parsed() {
        let result: Player1 = "A".parse().unwrap();
        assert_eq!(result, Player1::Rock);
        let result: Player1 = "B".parse().unwrap();
        assert_eq!(result, Player1::Paper);
        let result: Player1 = "C".parse().unwrap();
        assert_eq!(result, Player1::Scissors);
    }

    #[test]
    fn player2_should_be_parsed() {
        let result: Player2 = "Y".parse().unwrap();
        assert_eq!(result, Player2::Paper);
        let result: Player2 = "X".parse().unwrap();
        assert_eq!(result, Player2::Rock);
        let result: Player2 = "Z".parse().unwrap();
        assert_eq!(result, Player2::Scissors);
    }
}
