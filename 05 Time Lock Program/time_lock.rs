use std::time::{SystemTime, UNIX_EPOCH};

fn main(){
    let now = SystemTime::now();
    println!("Time now {}",now)
    let mut epoch = UNIX_EPOCH;
    let mut stanin = String::new();
    io::stdin() // Using standard in I/O
        .read_to_string(&mut stanin) // Append input to stanin (NEED &mut to change contents of string)
        .expect("Failed to read input"); // If it fails, return
    

    let since_epoch = now
        .duration_since(epoch)
        .expect("Time went backwards");
    println!("epoch {}",epoch)

    let seconds = since_epoch.as_secs();
}