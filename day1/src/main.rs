use std::fs;
use std::collections::HashMap;

fn getResult(input: &String) -> u32 {
    let mut sums: Vec<u32> = Vec::new();
    for ( input_line)  in input.lines() {
        let numbers: String = input_line
        .chars()
        .filter(|&c| c.is_digit(10))
        .collect();
        let first_digit = numbers.chars().next().unwrap_or_default().to_digit(10).unwrap_or(0);
        let last_digit = numbers.chars().last().unwrap_or_default().to_digit(10).unwrap_or(0);

        sums.push(first_digit*10 + last_digit);
    }
    let sum: u32 = sums.iter().sum();
    sum    
}

fn modify_input(input: &str) -> String {
    let word_to_digit: HashMap<&str, &str> = [
        ("eight", "eight8eight"),
        ("seven", "seven7seven"),
        ("six", "six6six"),
        ("five", "five5five"),
        ("four", "four4four"),
        ("three", "three3three"),
        ("two", "two2two"),
        ("one", "one1one"),
        ("nine", "nine9nine"),
    ]
    .iter()
    .cloned()
    .collect();

    let mut modified_output = String::new();

    for line in input.lines() {
        let mut modified_line = line.to_string();

        // Replace all occurrences
        for (word, digit) in &word_to_digit {
            modified_line = modified_line.replace(word, digit);
        }

        modified_output.push_str(&modified_line);
        modified_output.push('\n');
    }
    
    modified_output
}

fn main() {
    let input = fs::read_to_string("01.txt").unwrap();
    println!("part 1 result: {}",getResult(&input));

    let new_input = modify_input(&input);
    println!("part 2 result: {}",getResult(&new_input));
}
