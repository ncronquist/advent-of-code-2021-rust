use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let filename = "movements.txt";
    let m: Vec<String> = read_file_lines_to_vec(filename);
    let movements: Vec<&str> = m
        .iter()
        .map(|s| &**s)
        .collect();

    let position = calculate_position(&movements);
    println!("Position: {}", position);

    let better_position = calculate_better_position(&movements);
    println!("Better Position: {}", better_position);
}

// TODO: https://stackoverflow.com/questions/33216514/how-do-i-convert-a-vecstring-to-vecstr
fn read_file_lines_to_vec(filename: &str) -> Vec<String> {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let r: Vec<String> = reader.lines().map(|line| line.unwrap()).collect();
    return r;
}

fn calculate_position(movements: &Vec<&str>) -> i32 {
    let (vertical, horizontal) = movements
        .iter()
        .map(|movement| {
            let mov_parts: Vec<&str> = movement.split(" ").collect();
            let direction = mov_parts[0];
            let distance = mov_parts[1].parse::<i32>().unwrap();
            (direction, distance)
        })
        .fold((0, 0), |pos, (direction, distance)| {
            let (vertical, horizontal) = pos;

            let new_pos = match direction {
                "down" => (vertical + distance, horizontal),
                "up" => (vertical - distance, horizontal),
                "forward" => (vertical, horizontal + distance),
                _ => panic!("Unknown direction: {}", direction),
            };

            new_pos
        });

    return vertical * horizontal;
}

fn calculate_better_position(movements: &Vec<&str>) -> i32 {
    let (_aim, vertical, horizontal) = movements
        .iter()
        .map(|movement| {
            let mov_parts: Vec<&str> = movement.split(" ").collect();
            let direction = mov_parts[0];
            let distance = mov_parts[1].parse::<i32>().unwrap();
            (direction, distance)
        })
        .fold((0, 0, 0), |pos, (direction, distance)| {
            let (aim, vertical, horizontal) = pos;

            let new_pos = match direction {
                "down" => (aim + distance, vertical, horizontal),
                "up" => (aim - distance, vertical, horizontal),
                "forward" => (aim, vertical + (aim * distance), horizontal + distance),
                _ => panic!("Unknown direction: {}", direction),
            };

            new_pos
        });

    return vertical * horizontal;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn position() {
        let input: Vec<&str> = vec![
            "forward 5",
            "down 5",
            "forward 8",
            "up 3",
            "down 8",
            "forward 2",
        ];
        assert_eq!(calculate_position(&input), 150);
    }

    #[test]
    fn better_position() {
        let input: Vec<&str> = vec![
            "forward 5",
            "down 5",
            "forward 8",
            "up 3",
            "down 8",
            "forward 2",
        ];
        assert_eq!(calculate_better_position(&input), 900);
    }
}
