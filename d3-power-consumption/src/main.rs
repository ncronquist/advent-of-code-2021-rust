use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let filename = "diagnostic-report.txt";
    let d: Vec<String> = read_file_lines_to_vec(filename);
    let diagnostics: Vec<&str> = d
        .iter()
        .map(|s| &**s)
        .collect();

    let power_consumption = calculate_power(&diagnostics);
    println!("Power Consumption: {}", power_consumption);
}

fn read_file_lines_to_vec(filename: &str) -> Vec<String> {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let r: Vec<String> = reader.lines().map(|line| line.unwrap()).collect();
    return r;
}

// ["00100","11110","10110"]
// ["2", "1", "3", "2", "0"] // Reduce and Sum indices
// ["1", "0", "1", "1", "0"] // Map - value > length/2 ? 1 : 0 to create gamma array
// "10110" => 22 // Join indices and convert binary to decimal
// ["0", "1", "0", "0", "1"] // Map gamma array to opposite; 1 -> 0 and 0 -> 1 to create epsilon array
// "01001" => 9 // Join indices and convert binary to decimal
// 22 * 9 = 182 // Multiply two numbers
fn calculate_power(diagnostics: &Vec<&str>) -> i32 {
    let sd = std::iter::repeat(0).take(diagnostics[0].len()).collect::<Vec<_>>();
    let r = diagnostics.iter().fold(sd, |mut acc, d| {
        for (i, c)  in d.chars().enumerate() {
            acc[i] += c.to_digit(10).unwrap();
        };

        acc
    });

    let rate_vecs: Vec<(&str, &str)> = r
        .iter()
        .map(|&x| if x > u32::try_from(diagnostics.len()).unwrap() / 2 { ("1", "0") } else { ("0", "1") })
        .collect();

    let (gamma_rate_bin, epsilon_rate_bin): (String, String) = rate_vecs
        .iter()
        .fold((String::new(), String::new()), |mut acc, (g, e)| {
            acc.0.push_str(g);
            acc.1.push_str(e);
            acc
        });

    let gamma_rate = i32::from_str_radix(&gamma_rate_bin, 2).unwrap();
    let epsilon_rate =  i32::from_str_radix(&epsilon_rate_bin, 2).unwrap();

    return gamma_rate * epsilon_rate;
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn position() {
        let input: Vec<&str> = vec![
            "00100",
            "11110",
            "10110",
            "10111",
            "10101",
            "01111",
            "00111",
            "11100",
            "10000",
            "11001",
            "00010",
            "01010",
        ];
        assert_eq!(calculate_power(&input), 198);
    }
}
