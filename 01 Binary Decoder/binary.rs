use std::io::{self, Read};

/*

The code that was given in infection.txt creates a fork bomb in the C language. The “for (;;)” line  
makes an infinite loop. The “system” function works as if it were writing its argument to a  
terminal. The “argv[0]” argument takes the first argument that was used when running the  
program, which runs the program itself. When it is all put together, it makes an infinite number 
of copies of the current process, which in turn make copies of the current process. This program  
slows down the computer until it causes it to crash. This can be used by attackers to crash 
systems and thus deny service to users. Methods of defense include: using virtual machines to 
run untrusted code, setting resource allocation limits for active processes, and screening for 
common exploits in code.

*/

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
        println!("{}", &processed);
    }

    if eight_bit{
        // Decode ASCII 8 bit binary string to string
        let processed = decode8(text);
        println!("{}", &processed);
    } 

    if !eight_bit && !seven_bit{
        eprintln!("Unable to parse");
    }

}