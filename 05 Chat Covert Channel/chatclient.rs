use std::io::{self, BufReader, Write, Read};
use std::net::TcpStream;

fn main()
{
	let stream = match TcpStream::connect("localhost:1337")
	{
		Ok(s) => s,
		Err(e) =>
		{
			eprintln!("Error connecting: {e}");
			std::process::exit(1);
		}
	};

	println!("Connected to server. Recieving message: ");
	//let mut writer = stream.try_clone().expect("Failed to clone stream.");
	let mut reader = BufReader::new(stream);
	let mut byte = [0u8, 1];

	// from section 2.2 of covert timing lab
	loop
	{
		match reader.read(&mut byte)
		{
			Ok(0) => // print when the server runs out of characters to send
			{
				println!("\nServer closed connection.");
				break;
			}
			Ok(_) => // print each letter from server
			{
				print!("{}", byte[0] as char);
				io::stdout().flush().ok();
			}
			Err(e) => // print error if there is one
			{
				eprintln!("\nError reading from server: {e}");
				break;
			}
		}
	}

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
