

use std::io::{self, Read, Write};
use std::net::TcpStream;
use std::time::{Duration, Instant};

// Chat server provided for the assignment.
const ADDRESS: &str = "138.47.99.29";
const PORT: &str = "31337";
const ZERO: u64 = 25;
const ONE: u64 = 100;

// Convert each complete group of eight covert bits to a character.
fn bits_to_string(bits: &str) -> String {
    bits.as_bytes()
        .chunks_exact(8)
        .map(|chunk| {
            u8::from_str_radix(std::str::from_utf8(chunk).unwrap_or(""), 2)
                .map(char::from)
                .unwrap_or('?')
        })
        .collect()
}

fn main() -> io::Result<()> {
    // The default server works with ./chatclient. An optional address allows
    // testing with a local server: ./chatclient 127.0.0.1:1337
    let address = std::env::args()
        .nth(1)
        .unwrap_or_else(|| format!("{ADDRESS}:{PORT}"));
    let mut stream = TcpStream::connect(&address)?;

    // Read one byte at a time to preserve timing information. BufReader can
    // receive multiple bytes at once, obscuring the original delays.
    let mut byte = [0u8; 1];
    let mut previous_arrival: Option<Instant> = None;
    let mut overt_end = Vec::with_capacity(3);
    let mut covert_bits = String::new();
    let mut times: Vec<Duration> = Vec::new();
    let threshold = Duration::from_millis((ZERO + ONE) / 2);

    println!("Connected to server. Receiving overt message:");
    loop {
        if stream.read(&mut byte)? == 0 {
            eprintln!("Server closed the connection before overt EOF.");
            break;
        }
        let arrival = Instant::now();

        // Each delay between two received bytes represents one covert bit.
        if let Some(previous) = previous_arrival {
            let elapsed = arrival.duration_since(previous);
            times.push(elapsed);
            covert_bits.push(if elapsed >= threshold { '1' } else { '0' });
        }
        previous_arrival = Some(arrival);

        // Display the overt message as it arrives.
        print!("{}", char::from(byte[0]));
        io::stdout().flush()?;

        // The overt message ends with EOF.
        overt_end.push(byte[0]);
        if overt_end.len() > 3 {
            overt_end.remove(0);
        }
        if overt_end == b"EOF" {
            // The three EOF marker bytes are not part of the covert message
            // in the tested server protocol; discard their timing bits.
            covert_bits.truncate(covert_bits.len().saturating_sub(3));
            println!("\nMessage received. Disconnecting from server.");
            break;
        }
    }

    println!("Covert message:\n{}", bits_to_string(&covert_bits));

    // Optional timing output for troubleshooting: SHOW_TIMINGS=1 ./chatclient
    if std::env::var_os("SHOW_TIMINGS").is_some() {
        for (i, elapsed) in times.iter().enumerate() {
            eprintln!("Delay {:>3}: {:.3} s", i + 1, elapsed.as_secs_f64());
        }
    }
    Ok(())
}
