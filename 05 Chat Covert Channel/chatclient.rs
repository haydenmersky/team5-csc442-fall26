use std::io::{self, BufReader, Write, Read};
use std::net::TcpStream;
use std::time::{Duration, Instant};

// CHANGE ADDRESS AND PORT HERE
const ADDRESS: &str = "localhost";
const PORT: &str = "1337";
const ZERO: u64 = 25;
const ONE: u64 = 100;

// helper function from section 2.4
fn bits_to_string(bits: &str) -> String {
	let mut result = String::new();
	let mut i = 0;

	// getting an extra zero when decoding, so removing first char
	let input = &bits[1..bits.len() - 1];	

	while i + 8 <= input.len()
	{
		// each char is 8 bits, so split string into chunks of 8 bits
		match u8::from_str_radix(&input[i..i + 8], 2) 
		{
			Ok(n) => result.push(n as char),
			Err(_) => result.push('?'),
		}
		i += 8;
	}
	result
}


fn main()
{
	// connecting to server
	let stream = match TcpStream::connect(format!("{ADDRESS}:{PORT}"))
	{
		Ok(s) => s,
		Err(e) =>
		{
			eprintln!("Error connecting: {e}");
			std::process::exit(1);
		}
	};
	println!("Connected to server. Recieving overt message: ");

	//let mut writer = stream.try_clone().expect("Failed to clone stream.");
	let mut reader = BufReader::new(stream);
	let mut byte = [0u8; 1];
	let mut times: Vec<Duration> = Vec::new();
	let mut covert_string = String::new();
	let threshold = Duration::from_millis((ZERO + ONE) / 2);
	let mut covert_bits = String::new();
	let mut overt_string = String::new();
	

	// from section 2.2 of covert timing lab
	loop
	{
		let mut start = Instant::now();
		let n = reader.read(&mut byte);

		match n
		{
			Ok(0) => // print when the server runs out of characters to send
			{
				println!("\nServer closed connection.");
				break;
			}
			Ok(_) => // print each letter from server
			{
				// measure time since last read
				let now = Instant::now();
				let elapsed = now.duration_since(start);
            	times.push(elapsed);

				//store recieved message
				overt_string.push(byte[0] as char);

				print!("{}", byte[0] as char);
				io::stdout().flush().ok();
				
				if elapsed >= threshold
				{
					covert_bits.push('1');
				}
				else
				{
					covert_bits.push('0');
				}

				// if message has EOF, end loop
				if overt_string.ends_with("EOF")
				{
					println!("\nMessage recieved. Disconnecting from server.");
					break;
				}

			}
			Err(e) => // print error if there is one
			{
				eprintln!("\nError reading from server: {e}");
				break;
			}
		}

		


		//let temp_str = &bits_to_string(&covert_bits);
		//println!("{temp_str}");	
	}

	//println!("{covert_bits}");
	covert_string.push_str(&bits_to_string(&covert_bits));

	println!("Covert message:");
	println!("{covert_string}");
	
	//println!("{times:?}");
	
	/*
	// read lines from stdin, send to server
	let stdin = io::stdin();

	for line in stdin.lock().lines()
	{
		// read what the user types in the terminal
		let text = match line
		{
			Ok(t) => t,
			Err(_) => break,
		};

		// write what they typed to the network
		if writeln!(writer, "{text}").is_err()
		{
			break;
		}

		// read what server responds with
		let mut response = String::new();
		match reader.read_line(&mut response)
		{
			Ok(0) => break, // server closed the connection
			Ok(_) => print!("Server replied: {response}"),
			Err(e) =>
			{
				eprintln!("Error reading from server: {e}");
				break;
			}
		}
	}
	*/
}

