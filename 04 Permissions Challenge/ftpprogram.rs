use std::process::{Command,Stdio};
use std::io::{Write};

// Constants
const HOST: &str = "138.47.98.222";
const PORT: u16 = 21;
const USER: &str = "polaris";
const PASS: &str = "northstar";
const DIR: &str = "/files/7";
const METHOD: u8 = 7; // 7 or 10

fn get_perms() -> Vec<String> {

    let mut ftp = Command::new("ftp")
        .arg("-n")
        .arg(HOST)
        .arg(PORT.to_string())
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .spawn()
        .unwrap();
    {
    let stdin = ftp.stdin.as_mut().unwrap();
    writeln!(stdin, "user {} {}", USER, PASS).unwrap();
    //writeln!(stdin, "passive").unwrap();
    writeln!(stdin, "cd {}", DIR).unwrap();
    writeln!(stdin, "ls").unwrap();
    writeln!(stdin, "quit").unwrap(); 
    }

    drop(ftp.stdin.take()); // Close stdin to signal EOF to the ftp process

    let output = ftp.wait_with_output().unwrap();
    let text = String::from_utf8_lossy(&output.stdout);

    let mut perms = Vec::new();
    for line in text.lines() {
        let p: Vec<char> = line.chars().take(10).collect();
        if p.len() == 10 
            && "-dlcbps".contains(p[0]) // Check if the first character is a valid file type
            && p[1..].iter().all(|&c| "rwxsStT-".contains(c)) // Check if the remaining characters are valid permission characters
            {
                perms.push(p.into_iter().collect()); // Collect the characters into a String and push to perms
            }
    }
    perms
}

fn decode(perms: &[String]) -> String {
    // collect the bits from every file, in order
    let mut bits: Vec<u8> = Vec::new();

    for perm in perms {
        let chars: Vec<char> = perm.chars().collect();

        let used: &[char] = if METHOD == 7 {
            if chars[..3].iter().any(|&c| c != '-') {
                continue; // filter out files with any of the first three bits set if METHOD is 7
            }
            &chars[3..] // use permissions after first three if first 3 are empty
        } else {
            &chars[..] // use all ten bits if METHOD is 10
        };

        // '-' = 0, anything else = 1
        bits.extend(used.iter().map(|&c| (c != '-') as u8));
    }

    // cut the bits into 7-bit ASCII characters
    let mut decoded = String::new();
    for chunk in bits.chunks(7) {
        if chunk.len() < 7 {
            break; // leftover padding bits at the end
        }
        let value = chunk.iter().fold(0u32, |acc, &b| (acc << 1) | b as u32);
        decoded.push(value as u8 as char);
    }
    decoded
}

    
fn main(){
    let test = get_perms();
    println!("{}", decode(&test));
}