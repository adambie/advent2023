use std::fs;
use regex::Regex;


fn main() {
    let input = fs::read_to_string("02.txt").unwrap();
    println!("part 1 result: {}",getResult(&input));
    
}

fn getResult(input: &String) -> u32 {
    let mut games: Vec<u32> = Vec::new();
    for (line_number, input_line) in input.lines().enumerate() {
        if lineIsOk(&input_line) {
            games.push((line_number as u32)+1);
        }
    }
    let sum: u32 = games.iter().sum();
    sum    
}

fn lineIsOk(line: &str) -> bool {
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

    max_red <= 12 && max_green <= 13 && max_blue <= 14
}