use chrono::Utc;

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
    // Use the current time then remove the seconds elapsed since current minute
    let timestamp = Utc::now().timestamp();
    let timeset = timestamp - timestamp.rem_euclid(60);

    // First MD5: hash the current time bucket
    let first_hash = md5::compute(timeset.to_string());

    // second MD5:
    let second_hash = md5::compute(format!("{:x}", first_hash));

    // Convert final hash to hexadecimal
    let final_hash = format!("{:x}", second_hash);

    // Extract the four-character TimeLock code
    let code = get_code(&final_hash);
    
    // Output only the code
    println!("{}", code);

    Ok(())
}
