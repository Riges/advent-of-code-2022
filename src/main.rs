mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day10;

#[allow(dead_code)]
fn previous_days() {
    day01::day01().unwrap();
    day02::day02().unwrap();
    day03::day03().unwrap();
    day04::day04().unwrap();
    day05::day05().unwrap();
    day06::day06().unwrap();
    day07::day07().unwrap();
    day08::day08().unwrap();
}

fn main() {
    day10::day10().unwrap();
}
