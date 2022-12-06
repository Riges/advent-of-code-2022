mod day01;
mod day02;
mod day03;
mod day04;
mod day05;

#[allow(dead_code)]
fn previous_days() {
    day01::day01().unwrap();
    day02::day02().unwrap();
    day03::day03().unwrap();
    day04::day04().unwrap();
}

fn main() {
    day05::day05().unwrap();
}
