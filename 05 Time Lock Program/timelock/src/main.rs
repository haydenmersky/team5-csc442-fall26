use chrono::{Local, LocalResult, NaiveDate, TimeZone};
use std::io::{self, Read};

fn get_code(input: &str) -> String
{
	let mut output_code: String = String::new();
	
	// getting the letters for the start of the code
	for char in input.to_string().chars()
	{
		if output_code.len() == 2
		{
			break;
		}
		else if char.is_alphabetic()
		{
			output_code.push(char);
		}
	}

	// getting the numbers for the end of the code
	for char in input.to_string().chars().rev()
	{
		if output_code.len() == 4
		{
			break;
		}
		else if char.is_numeric()
		{
			output_code.push(char);
		}
	}

	// return code
	output_code
}

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
    
    // Construct the epoch date and time
    let date = NaiveDate::from_ymd_opt(year, month, day)
        .ok_or("Invalid date")?;

    let epoch_naive = date.and_hms_opt(hour, minute, second)
        .ok_or("Invalid time")?;

    // Interpret epoch using the system's local timezone
    let epoch = match Local.from_local_datetime(&epoch_naive) {
        LocalResult::Single(dt) => dt,

        // If the time occurs twice, choose the earlier occurrence
        LocalResult::Ambiguous(first, _) => first,

        // If the time does not exist because of DST, reject it
        LocalResult::None => {
            return Err("Epoch falls in a nonexistent local time".into());
        }
    };

    // Get current system time
    let now = Local::now();

    // Convert both times to Unix timestamps
    let elapsed = now.timestamp() - epoch.timestamp();

    if elapsed < 0 {
        return Err("Epoch cannot be in the future".into());
    }

    // First MD5: hash the elapsed seconds
    let first_hash = md5::compute(elapsed.to_string().as_bytes());

    Ok(())

    // second MD5:
    let second_hash = md5::compute(first_hash.as_bytes());

    println("{}", second_hash);
}
