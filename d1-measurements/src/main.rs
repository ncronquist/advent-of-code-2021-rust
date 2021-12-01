use std::fs::File;
use std::io::{BufRead, BufReader};

fn part1(filename: &str) -> i32 {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let mut previous: i32 = -1;
    let mut num_increases = 0;

    for (_index, line) in reader.lines().enumerate() {
        let line = line.unwrap(); // Ignore errors.
        let cur: i32 = line.parse().unwrap();

        if cur > previous && previous != -1 {
            num_increases += 1
        }

        previous = cur;
    }

    return num_increases;
}

fn part2(filename: &str) -> i32 {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let mut window = Vec::new();
    let mut window_sum: i32 = 0;
    let mut num_increases = 0;

    for (_index, line) in reader.lines().enumerate() {
      let line = line.unwrap();
      let cur: i32 = line.parse().unwrap();

      if window.len() < 3 {
        window_sum += cur;
        window.push(cur);
      } else {
        let previous_window_sum = window_sum;
        // remove is an O(n) operation where n is always 3
        // not sure on time implication vs using pointers or
        // referencing the last 3 indexes
        window_sum -= window.remove(0);
        window.push(cur);
        window_sum += cur;
        if window_sum > previous_window_sum {
          num_increases += 1;
        }
      }
    }

    return num_increases;
}

fn main() {
    let filename= "measurements.txt";

    let num_increases = part1(filename);
    println!("Number of Increases: {}", num_increases);

    let sum_increases = part2(filename);
    println!("Number of Sum Increases: {}", sum_increases);
}
