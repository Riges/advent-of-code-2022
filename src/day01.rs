use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn process_elves_calories_file(path: &str) -> anyhow::Result<Vec<Vec<u32>>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let result = reader
        .lines()
        .fold((vec![], vec![]), |acc: (Vec<Vec<u32>>, Vec<u32>), line| {
            let (mut elves, mut current) = acc;
            match line {
                Ok(l) => match l.parse::<u32>() {
                    Ok(i) => {
                        current.push(i);

                        (elves, current)
                    }
                    Err(_) => {
                        if !current.is_empty() {
                            elves.push(current);
                        }

                        (elves, vec![])
                    }
                },
                Err(_) => {
                    println!("no line");
                    todo!()
                }
            }
        });

    Ok(result.0)
}

fn extract_top_3(elves: Vec<Vec<u32>>) -> Vec<u32> {
    let mut leaderboard = elves.iter().fold(vec![], |acc: Vec<u32>, elve| {
        let mut lb = acc;
        lb.push(elve.iter().sum());

        lb
    });
    leaderboard.sort_by(|a, b| b.cmp(a));
    leaderboard.truncate(3);

    leaderboard
}

pub fn day01() -> anyhow::Result<()> {
    let elves = process_elves_calories_file("data/day01.txt")?;
    let elves_top_3 = extract_top_3(elves);

    println!(
        "Day 1 - How many total Calories is that Elf carrying? {:?}",
        elves_top_3.first().unwrap()
    );
    println!(
        "Day 1 - How many Calories are those Elves carrying in total? {:?}",
        elves_top_3
            .iter()
            .sum::<u32>()
    );

    Ok(())
}
