use std::env;
use std::fs;
use std::io::{self, Write};

// The six-byte marker tells us where the hidden data ends.
const SENTINEL: [u8; 6] = [0x00, 0xFF, 0x00, 0x00, 0xFF, 0x00];

#[derive(Clone, Copy)]
enum Action {
    Store,
    Retrieve,
}

#[derive(Clone, Copy)]
enum Mode {
    Bit,  // Store one secret bit in each wrapper byte's least significant bit.
    Byte, // Replace one whole wrapper byte with one secret byte.
}

struct Options {
    action: Action,
    mode: Mode,
    offset: usize,
    interval: usize,
    wrapper: String,
    hidden: Option<String>,
}

// Read command-line flags such as -s -b -o 1024 -w picture.bmp -h secret.txt.
fn parse_args() -> Result<Options, String> {
    let args: Vec<String> = env::args().skip(1).collect();
    let mut action = None;
    let mut mode = None;
    let mut offset = 0usize; // Default offset from the assignment.
    let mut interval = 1usize; // Default interval from the assignment.
    let mut wrapper = None;
    let mut hidden = None;
    let mut i = 0;

    while i < args.len() {
        match args[i].as_str() {
            "-s" => {
                if action.is_some() {
                    return Err("Choose exactly one of -s (store) and -r (retrieve).".into());
                }
                action = Some(Action::Store);
            }
            "-r" => {
                if action.is_some() {
                    return Err("Choose exactly one of -s (store) and -r (retrieve).".into());
                }
                action = Some(Action::Retrieve);
            }
            "-b" => {
                if mode.is_some() {
                    return Err("Choose exactly one of -b (bit) and -B (byte).".into());
                }
                mode = Some(Mode::Bit);
            }
            "-B" => {
                if mode.is_some() {
                    return Err("Choose exactly one of -b (bit) and -B (byte).".into());
                }
                mode = Some(Mode::Byte);
            }
            "-o" | "-i" | "-w" | "-h" => {
                let flag = args[i].as_str();
                i += 1;
                if i >= args.len() {
                    return Err(format!("Missing value after {flag}."));
                }
                let value = &args[i];
                match flag {
                    "-o" => {
                        offset = value.parse::<usize>()
                            .map_err(|_| "Offset must be a nonnegative integer.".to_string())?;
                    }
                    "-i" => {
                        interval = value.parse::<usize>()
                            .map_err(|_| "Interval must be a positive integer.".to_string())?;
                    }
                    "-w" => wrapper = Some(value.clone()),
                    "-h" => hidden = Some(value.clone()),
                    _ => unreachable!(),
                }
            }
            other => return Err(format!("Unrecognized argument: {other}")),
        }
        i += 1;
    }

    let action = action.ok_or("Missing -s or -r.".to_string())?;
    let mode = mode.ok_or("Missing -b or -B.".to_string())?;
    let wrapper = wrapper.ok_or("Missing wrapper file (-w).".to_string())?;

    if interval == 0 {
        return Err("Interval (-i) must be greater than zero.".into());
    }
    if matches!(action, Action::Store) && hidden.is_none() {
        return Err("Store mode requires a hidden file (-h).".into());
    }

    Ok(Options { action, mode, offset, interval, wrapper, hidden })
}

// Before storing, check that every hidden byte AND the sentinel will fit.
// Byte mode needs one wrapper position per byte; bit mode needs eight.
fn check_capacity(
    wrapper_len: usize, hidden_len: usize, offset: usize, interval: usize, mode: Mode,
) -> Result<(), String> {
    let total_bytes = hidden_len.checked_add(SENTINEL.len())
        .ok_or("Hidden data is too large.")?;
    let positions = match mode {
        Mode::Byte => total_bytes,
        Mode::Bit => total_bytes.checked_mul(8).ok_or("Hidden data is too large.")?,
    };
    // The final write occurs at offset + (positions - 1) * interval.
    let last_position = (positions - 1).checked_mul(interval)
        .and_then(|distance| offset.checked_add(distance))
        .ok_or("Offset or interval is too large.")?;
    if last_position >= wrapper_len {
        return Err("Wrapper is too small for the hidden data, offset, interval, and sentinel.".into());
    }
    Ok(())
}

// Insert the hidden data into a copy of the original wrapper.
fn store(
    wrapper: &[u8], hidden: &[u8], mode: Mode, offset: usize, interval: usize,
) -> Result<Vec<u8>, String> {
    check_capacity(wrapper.len(), hidden.len(), offset, interval, mode)?;
    let mut output = wrapper.to_vec();

    // Append the sentinel after the secret bytes, so retrieval knows when to stop.
    for (byte_index, secret_byte) in hidden.iter().copied()
        .chain(SENTINEL.iter().copied()).enumerate()
    {
        match mode {
            Mode::Byte => {
                // Replace the entire byte at each selected wrapper position.
                let position = offset + byte_index * interval;
                output[position] = secret_byte;
            }
            Mode::Bit => {
                // Take the secret byte's bits left-to-right (MSB first).
                for bit_index in 0..8 {
                    let bit = (secret_byte >> (7 - bit_index)) & 1;
                    let position = offset + (byte_index * 8 + bit_index) * interval;
                    // Clear wrapper's last bit (AND 11111110), then set our bit (OR).
                    output[position] = (output[position] & 0xFE) | bit;
                }
            }
        }
    }
    Ok(output)
}

// Extract hidden bytes until we encounter the six-byte sentinel.
fn retrieve(
    wrapper: &[u8], mode: Mode, mut position: usize, interval: usize,
) -> Result<Vec<u8>, String> {
    let mut hidden = Vec::new();

    loop {
        let secret_byte = match mode {
            Mode::Byte => {
                // Read one complete byte from the next wrapper position.
                match wrapper.get(position) {
                    Some(&value) => {
                        position = position.saturating_add(interval);
                        value
                    }
                    None => break,
                }
            }
            Mode::Bit => {
                // Rebuild a secret byte from the LSB of eight wrapper bytes.
                let mut value = 0u8;
                for _ in 0..8 {
                    let wrapper_byte = *wrapper.get(position)
                        .ok_or("Wrapper ended in the middle of a hidden byte.")?;
                    value = (value << 1) | (wrapper_byte & 1);
                    position = position.saturating_add(interval);
                }
                value
            }
        };

        hidden.push(secret_byte);
        if hidden.ends_with(&SENTINEL) {
            // Remove the marker: it is not part of the original secret file.
            hidden.truncate(hidden.len() - SENTINEL.len());
            return Ok(hidden);
        }
    }

    Err("End marker (sentinel) not found. Check mode, offset, and interval.".into())
}

fn run() -> Result<(), String> {
    let options = parse_args()?;
    let wrapper = fs::read(&options.wrapper)
        .map_err(|error| format!("Cannot read wrapper '{}': {error}", options.wrapper))?;

    let output = match options.action {
        Action::Store => {
            let path = options.hidden.as_deref()
                .ok_or("Store mode requires a hidden file (-h).")?;
            let hidden = fs::read(path)
                .map_err(|error| format!("Cannot read hidden file '{path}': {error}"))?;
            store(&wrapper, &hidden, options.mode, options.offset, options.interval)?
        }
        Action::Retrieve => {
            retrieve(&wrapper, options.mode, options.offset, options.interval)?
        }
    };

    // Write raw binary bytes to stdout, NOT println! (which changes file data).
    // In the shell, use > filename to save these bytes to a file.
    io::stdout().lock().write_all(&output)
        .map_err(|error| format!("Cannot write output: {error}"))?;
    Ok(())
}

fn main() {
    if let Err(message) = run() {
        // Errors go to stderr so they never contaminate redirected binary output.
        eprintln!("Error: {message}");
        eprintln!("Usage: ./steg -(s|r) -(b|B) [-o offset] [-i interval] -w wrapper [-h hidden]");
        std::process::exit(1);
    }
}
