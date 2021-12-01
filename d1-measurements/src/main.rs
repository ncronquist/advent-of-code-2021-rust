use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let filename = "measurements.txt";
    // Open the file in read-only mode (ignoring errors).
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let mut previous: i32 = -1;
    let mut num_increases = 0;

    // Read the file line by line using the lines() iterator from std::io::BufRead.
    for (_index, line) in reader.lines().enumerate() {
        let line = line.unwrap(); // Ignore errors.
        let cur: i32 = line.parse().unwrap();

        if cur > previous && previous != -1 {
            num_increases += 1
        }

        previous = cur;
    }

    println!("Number of Increases: {}", num_increases); // Should print 1316
}
