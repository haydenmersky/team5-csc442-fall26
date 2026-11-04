use chrono::{Local, LocalResult, NaiveDate, TimeZone};
use std::io::{self, Read};

fn main() -> Result<(), Box<dyn std::error::Error>> {

    // Read epoch from stdin
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    // Parse YYYY MM DD HH mm SS
    let values: Vec<u32> = input
        .split_whitespace()
        .map(|s| s.parse::<u32>())
        .collect::<Result<Vec<u32>, _>>()?;

    if values.len() != 6 {
        return Err("Expected format: YYYY MM DD HH mm SS".into());
    }

    let year = values[0] as i32;
    let month = values[1];
    let day = values[2];
    let hour = values[3];
    let minute = values[4];
    let second = values[5];
    
}