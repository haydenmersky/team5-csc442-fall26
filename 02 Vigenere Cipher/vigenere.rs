use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::io::{self, Write, Read};


// Encrypt plaintext using the Vigenere cipher.
fn encode(text: &str, key: &str) -> String { 
    // Keep only letters from the key and convert them to uppercase.
    let key: Vec<u8> = key
        .bytes()
        .filter(|b| b.is_ascii_alphabetic())
        .map(|b| b.to_ascii_uppercase())
        .collect();

    
    let mut result = String::new();
    // Tracks which key letter is being used.
    let mut key_index = 0;

    for ch in text.chars() {
        if ch.is_ascii_alphabetic() {
            // Use A/a as the starting point so uppercase and lowercase
            // letters can be handled separately.
            let base = if ch.is_ascii_uppercase() { b'A' } else { b'a' };
            // Convert the plaintext letter to a value from 0 to 25.
            let plaintext = ch as u8 - base;
             // Convert the current key letter to a value from 0 to 25.
            let key_value = key[key_index % key.len()] - b'A';

            let encrypted = (plaintext + key_value) % 26 + base;

            result.push(encrypted as char);
            // Move to the next key letter.
            key_index += 1;
        } else {
            result.push(ch);
        }
    }

    result
}

// Decrypt ciphertext using the Vigenere cipher.
fn decode(text: &str, key: &str) -> String {
    // Keep only letters from the key and convert them to uppercase.
    let key: Vec<u8> = key
        .bytes()
        .filter(|b| b.is_ascii_alphabetic())
        .map(|b| b.to_ascii_uppercase())
        .collect();

    let mut result = String::new();
    // Tracks which key letter is being used.
    let mut key_index = 0;

    for ch in text.chars() {
        if ch.is_ascii_alphabetic() {
            let base = if ch.is_ascii_uppercase() { b'A' } else { b'a' };
        // Convert the ciphertext letter to a value from 0 to 25.
            let ciphertext = ch as u8 - base;
            // Convert the current key letter to a value from 0 to 25.
            let key_value = key[key_index % key.len()] - b'A';

            let decrypted = (ciphertext + 26 - key_value) % 26 + base;

            result.push(decrypted as char);
            key_index += 1;
        } else {
            result.push(ch);
        }
    }

    result
}

// reads from a file 
/*fn read_file(filename: &str){
    let file = File::open(filename).expect("Could not open log file");
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line.expect("Could not read line");
        println!("{}", line);
    }

} */

//writes the results to the terminal
fn write(mode: &str, key: &str){
    let stdin = io::stdin();
        let mut input = String::new();
        loop {
            input.clear();

            stdin.read_line(&mut input).unwrap();

            if mode == "-e"{
                print!("{}", encode(&input, &key));

            } else if mode == "-d"{
                 print!("{}", decode(&input, &key));
            } else{
                print!("Cannot Print");
            }
        
        }

}



// prints the usage instructions
fn print_usage(){
    println!("Usage:");
    println!("For usage in the terminal:\n./vigenere (-e or -d) (key)\nWhere -e is for encoding, and -d is for decoding.");
    println!("For usage with a file:\n./vigenere (-e or -d) (key) (< or >) (filename)\nWhere < is for reading from a file and > is for writing to a file.");
    std::process::exit(1);
}


fn main(){
    let args: Vec<String> = env::args().collect();
    let mut coding_mode = "";
    let mut key = "";


    // finds if the input has the right number of arguments
    // and if it has the right parameters
    // or else it prints the instructions on how to use the program
    /*let mut buffer = String::new();
    let mut isBuffer = io::stdin().read_to_string(&mut buffer).is_ok();
    if isBuffer {
        println!("Received from stdin:\n{}", buffer);
    }
    print!("{:?}", buffer); */
        
            if (&args[1] == "-e") || &args[1] == "-d" {
                coding_mode = &args[1];
                key = &args[2];
                /*if(isBuffer){
                    write(coding_mode, key);
                } */
                write(coding_mode, key);
            }
            else{
                print_usage();
            }
    

       

    // testing prints
    // note: after testing a few times, it looks like ">" automatically writes what is printed
    // to the terminal to the given file. so you probably could just print the decoded/encoded text here
    println!("Coding mode: {}", coding_mode);
    println!("Key: {}", key);
}
