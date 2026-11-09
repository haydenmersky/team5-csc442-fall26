use std::fs;
use std::io::{self, Read, Write};

fn main() -> io::Result<()> {
    let key = fs::read("k3y")?;

    let stdin = io::stdin();
    let mut input = stdin.lock();

    let stdout = io::stdout();
    let mut output = stdout.lock();

    let mut buffer = vec![0u8; 4096];
    let mut position = 0usize;

    loop {
        let bytes_read = input.read(&mut buffer)?;
        if bytes_read == 0 {
            break;
        }

        for i in 0..bytes_read {
            buffer[i] ^= key[(position + i) % key.len()]; // Make sure the key wraps if need be
        }
        output.write_all(&buffer[..bytes_read])?;
        position += bytes_read;
    }

    output.flush()?;
    Ok(())

}