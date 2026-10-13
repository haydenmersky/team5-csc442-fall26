use std::io::{self, BufRead, BufReader, Write};
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

	println!("Connected to server. Type a message and then press Enter: ");
	let mut writer = stream.try_clone().expect("Failed to clone stream.");
	let mut reader = BufReader::new(stream);

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
}
