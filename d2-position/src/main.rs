fn main() {
    println!("Hello, world!");
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

    println!("VER: {}, HOR: {}", vertical, horizontal);
    return vertical * horizontal;
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn position() {
        let input = vec!["forward 5", "down 5", "forward 8", "up 3", "down 8", "forward 2"];
        assert_eq!(calculate_position(&input), 150);
    }

    // #[test]
    // fn given_windows() {
    //     let input = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
    //     assert_eq!(count_windows(&input), 5);
    // }
}
