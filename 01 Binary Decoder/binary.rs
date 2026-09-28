use std::io::{self, Read};

// Decode 8 bit binary string to ASCII
fn decode8(bits: &str) -> String {
    bits
        // Convert the string to bytes
        .as_bytes()
        // Split the bytes into chunks of 8 bits
        .chunks(8)
        // Map each chunk to a character
        .map(|chunk| {
            // Convert the chunk of bytes to a string
            let binary = std::str::from_utf8(chunk).unwrap();
            // Convert the binary string to a u8 integer
            let byte = u8::from_str_radix(binary, 2).unwrap();
            // Convert the u8 integer to a character
            byte as char
        })
        // Collect the characters into a string
        .collect()
}
// Decode 7 bit binary string to ASCII
fn decode7(bits: &str) -> String {
    bits
        // Convert the string to bytes
        .as_bytes()
        // Split the bytes into chunks of 7 bits
        .chunks(7)
        // Map each chunk to a character
        .map(|chunk| {
            // Convert the chunk of bytes to a string
            let binary = std::str::from_utf8(chunk).unwrap();
            // Convert the binary string to a u8 integer
            let byte = u8::from_str_radix(binary, 2).unwrap();
            // Convert the u8 integer to a character
            byte as char
        })
        // Collect the characters into a string
        .collect()
}

// Write to standard output
fn write(bytes: &str){
    // Print the bytes to standard output
    print!("{}", bytes);
    print!("\n");
}


fn main() {
    let mut raw_text = String::new();
    io::stdin() // Using standard in I/O
        .read_to_string(&mut raw_text) // Append input to raw_text (NEED &mut to change contents of string)
        .expect("Failed to read input"); // If it fails, return


    let text = raw_text.trim();// Remove \n from input
    let len = text.len();

    // Determine if input is 7 bit or 8 bit
    let seven_bit = len % 7 == 0;
    let eight_bit = len % 8 == 0;


    // Decodes with 7 or 8 based on above boolean
    // Setup in 2 if statements so if both are true, both run
    if seven_bit{
        // Decode ASCII 7 bit binary string to string
        let processed = decode7(text);
        write(&processed);
    }

    if eight_bit{
        // Decode ASCII 8 bit binary string to string
        let processed = decode8(text);
        write(&processed);
    } 

}