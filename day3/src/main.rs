use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use regex::Regex;

fn main() -> io::Result<()> {
    let path = Path::new("03.txt");
    let file = File::open(&path)?;
    let reader = io::BufReader::new(file);

    let mut numbers = Vec::new();
    let mut numbers_not = Vec::new();
    let re = Regex::new(r"\d+").unwrap();
    let mut lines: Vec<String> = Vec::new();

    for line in reader.lines() {
        lines.push(line?);
    }

    for (line_number, line) in lines.iter().enumerate() {
        for cap in re.captures_iter(line) {
            for mat in cap.iter() {
                let start = mat.unwrap().start();
                let end = mat.unwrap().end();
                let number: u32 = mat.unwrap().as_str().parse().unwrap();

                let mut is_empty_around = true;
                for y in (line_number as i32 - 1)..=(line_number as i32 + 1) {
                    for x in (start as i32 - 1)..=(end as i32) {
                        if (y == line_number as i32) && (x >= start as i32) && (x < end as i32-1) {
                            continue;
                        }                        
                        if y >= 0 && x >= 0 && y < lines.len() as i32 && x < line.len() as i32 {
                            let ch = lines[y as usize].chars().nth(x as usize).unwrap();
                            if ch != '.' && !ch.is_digit(10) {
                                is_empty_around = false;
                            }
                        }
                    }
                }

                if !is_empty_around {
                    numbers.push((number, line_number + 1, start + 1, end));
                } else {
                    numbers_not.push((number, line_number + 1, start + 1, end));
                }
            }
        }
    }

    let sum = numbers.iter().map(|(num, _, _, _)| num).sum::<u32>();
    println!("part1: {:?}", sum);

    //print all numbers_not
    for (num, line_number, start, end) in numbers_not {
        println!("{}: {} {} {}", num, line_number, start, end);
    }

    //for debuging
    write_numbers_as_x(numbers, lines)?;
    Ok(())
}

use std::fs::OpenOptions;
use std::io::Write;

fn write_numbers_as_x(numbers: Vec<(u32, usize, usize, usize)>, lines: Vec<String>) -> std::io::Result<()> {
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .open("03x.txt")?;

    for (line_number, line) in lines.iter().enumerate() {
        let mut new_line_chars = line.chars().collect::<Vec<_>>();
        for &(number, num_line_number, start, end) in &numbers {
            if num_line_number - 1 == line_number {
                for i in start..end {
                    new_line_chars[i] = 'X';
                }
            }
        }
        let new_line = new_line_chars.into_iter().collect::<String>();
        writeln!(file, "{}", new_line)?;
    }

    Ok(())
}