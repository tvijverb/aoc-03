// Rust
use std::fs::File;
use std::io::{BufReader, BufRead};
use std::str::FromStr;
use {
    once_cell::sync::Lazy,
    regex::Regex,
};

#[derive(Debug)]
pub struct EnginePart {
    pub line_number: i32,
    pub part_number: u32,
    pub char_loc: (i32, i32),
}

#[derive(Debug)]
pub struct SpecialChar {
    pub line_number: i32,
    pub char_loc: i32,
    pub char: String,
}

fn parse_line(line: &str, line_number: &i32) -> (Vec<EnginePart>, Vec<SpecialChar>) {
    let mut parts = Vec::new();
    let mut special_chars = Vec::new();

    static PARTNO_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"\d+").unwrap());
    // get character positions for each part number in the line
    let part_number_matches = PARTNO_REGEX.find_iter(line);
    for part_number_match in part_number_matches {
        println!("part_number: {:?}", part_number_match);
        let part_number = part_number_match.as_str();
        let part_number = part_number.parse::<u32>().unwrap();
        let part = EnginePart {
            line_number: *line_number,
            part_number: part_number,
            char_loc: (part_number_match.start() as i32 - 1, part_number_match.end() as i32),
        };

        parts.push(part);
    }
    println!("parts: {:?}", parts);

    static SPECIALCHAR_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"\D").unwrap());
    // get character positions for each special character in the line
    let special_char_matches = SPECIALCHAR_REGEX.find_iter(line);
    for special_char_match in special_char_matches {
        if special_char_match.as_str() == "." {
            continue;
        }
        println!("special_char: {:?}", special_char_match);
        let special_char = SpecialChar {
            line_number: *line_number,
            char_loc: special_char_match.start() as i32,
            char: special_char_match.as_str().to_string()
        };

        special_chars.push(special_char);
    }

    (parts, special_chars)
}

fn main() -> std::io::Result<()> {
    let file = File::open("input1.txt")?;
    let reader = BufReader::new(file);

    let mut sum_part_numbers = 0;
    let mut sum_gear_ratios = 0;

    let mut engine_parts = Vec::new();
    let mut special_chars = Vec::new();

    let mut line_number = 0;
    for line in reader.lines() {
        let line = line?;
        let (parts, special_chars_found) = parse_line(&line, &line_number);
        for part in parts {
            engine_parts.push(part);
        }
        for special_char in special_chars_found {
            special_chars.push(special_char);
        }
        line_number += 1;
    }

    // for each part, check if there is a special character in the same line, the line above or the line below and within the part's character location
    for part in &engine_parts {
        let mut special_char_found = false;
        for special_char in &special_chars {
            if special_char.char_loc >= part.char_loc.0 && special_char.char_loc <= part.char_loc.1 && (special_char.line_number == part.line_number || special_char.line_number == part.line_number - 1 || special_char.line_number == part.line_number + 1){
                special_char_found = true;
                println!("part: {:?}, special_char: {:?}", part, special_char);
                sum_part_numbers += part.part_number;
                break;
            }
        }
    }

    // for each * special character check if there are exactly two part numbers in the same line, the line above or the line below and within the part's character location
    for special_char in &special_chars {
        let mut part_numbers_found = 0;
        let mut part_1_number = 0;
        let mut part_2_number = 0;
        if special_char.char != "*" {
            continue;
        }
        for part in &engine_parts {
            if special_char.char_loc >= part.char_loc.0 && special_char.char_loc <= part.char_loc.1 && (part.line_number == special_char.line_number || part.line_number == special_char.line_number - 1 || part.line_number == special_char.line_number + 1){
                part_numbers_found += 1;
                if part_numbers_found == 1 {
                    part_1_number = part.part_number;
                } else if part_numbers_found == 2 {
                    part_2_number = part.part_number;
                }
            }
        }
        if part_numbers_found == 2 {
            println!("special_char: {:?}", special_char);
            sum_gear_ratios += part_1_number * part_2_number;
        }
    }

    println!("sum_part_numbers: {:?}", sum_part_numbers);
    println!("sum_gear_ratios: {:?}", sum_gear_ratios);

    Ok(())
}