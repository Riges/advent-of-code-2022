use std::fs;

pub fn day01() -> anyhow::Result<()> {
  let contents = fs::read_to_string("data/day01.txt")
  .expect("Should have been able to read the file");

  let result = contents.lines().fold((0,0), |acc, line| {
      let (max, current) = acc;
      match line.parse::<i32>()  {
          Ok(i) => {
              let actual = current+i;
              if actual > max {
                  (actual, actual)
              } else {
                  (max, actual)
              }
          },
          Err(_) => (max, 0)
      }
  });

  let (max, _) = result;

  println!("Day 1 - How many total Calories is that Elf carrying? {:?}", max);

  Ok(())
}
