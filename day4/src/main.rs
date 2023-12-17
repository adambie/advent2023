use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashSet;

fn main() -> io::Result<()> {
    let path = Path::new("04.txt");
    let file = File::open(&path)?;
    let reader = io::BufReader::new(file);

    let mut total = 0;

    for line in reader.lines() {
        let line = line?;
        let line = line.chars().skip(9).collect::<String>();
        let parts: Vec<&str> = line.split('|').collect();
        let numbers_before: HashSet<u32> = parts[0].split_whitespace()
            .filter_map(|s| s.parse::<u32>().ok())
            .collect();
        let numbers_after: HashSet<u32> = parts[1].split_whitespace()
            .filter_map(|s| s.parse::<u32>().ok())
            .collect();

        let matching_numbers: HashSet<_> = numbers_before.intersection(&numbers_after).collect();
        if !matching_numbers.is_empty() {
            total += 2u32.pow(matching_numbers.len() as u32 - 1);
        }
    }

    println!("Total: {}", total);

    Ok(())
}