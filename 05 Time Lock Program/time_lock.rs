use std::time::{SystemTime, UNIX_EPOCH};
use std::io::{self, Read};

fn main() {
    let now = SystemTime::now();

    println!("Time now: {:?}", now);

    let mut stanin = String::new();

    io::stdin()
        .read_to_string(&mut stanin)
        .expect("Failed to read input");

    let since_epoch = now
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");

    println!("Seconds since epoch: {}", since_epoch.as_secs());

    let seconds = since_epoch.as_secs();

    println!("Seconds: {}", seconds);
}