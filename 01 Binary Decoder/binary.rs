use std::io::{self, Read, Write};

//fn decode(bits, width){

    //}

    fn write(bytes){
        let mut stdout = io::stdout();
        stdout.write_all(bytes.as_bytes()).expect("Failed to write output");
        stdout.flush().expect("Failed to flush output");
        Ok(());
    }


fn main() {
    let mut rawText = String::new();
    io::stdin() // Using standard in I/O
        .read_to_string(&mut rawText) // Append input to rawText (NEED &mut to change contents of string)
        .expect("Failed to read input"); // If it fails, return



    let len = rawText.len();

    // Determine if input is 7 bit or 8 bit
    let seven_bit = len % 7 == 0;
    let eight_bit = len % 8 == 0;


    // Decodes with 7 or 8 based on above boolean
    // Setup in 2 if statements so if both are true, both run
    if seven_bit {
        let processed = decode(&rawText, 7);
        write(&processed);
    }

    if eight_bit {
        let processed = decode(&rawText, 8);
        write(&processed);
    }

}