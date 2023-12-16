use std::fs;
use regex::Regex;


fn main() {
    let input = fs::read_to_string("02.txt").unwrap();
    let (result1, result2) = getResult(&input);
    println!("Sum: {}, part2: {}", result1, result2);
    
}

fn getResult(input: &String) -> (u32, u32) {
    let mut games: Vec<u32> = Vec::new();
    let mut powers: Vec<u32> = Vec::new();
    for (line_number, input_line) in input.lines().enumerate() {
        let (isOk, power) = lineIsOk(&input_line);
        if isOk {
            games.push((line_number as u32)+1);
        }
        powers.push(power);
    }
    let sum: u32 = games.iter().sum();
    let power: u32 = powers.iter().sum();
    (sum, power)    
}

fn lineIsOk(line: &str) -> (bool, u32) {
    let re = Regex::new(r"(\d+) (red|green|blue)").unwrap();
    let mut max_red = 0;
    let mut max_green = 0;
    let mut max_blue = 0;

    for cap in re.captures_iter(line) {
        let number: u32 = cap[1].parse().unwrap();
        let color = &cap[2];
        match color {
            "red" => {
                if number > max_red {
                    max_red = number;
                }
            }
            "green" => {
                if number > max_green {
                    max_green = number;
                }
            }
            "blue" => {
                if number > max_blue {
                    max_blue = number;
                }
            }
            _ => {}
        }
    }

    (max_red <= 12 && max_green <= 13 && max_blue <= 14,
    max_red * max_green * max_blue)
}