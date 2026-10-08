use std::process::{Command,Stdio};
use std::io::{Write, Read};

// Constants
const HOST: &str = "localhost";
const PORT: u16 = 21;
const DIR: &str = "/";
const METHOD: u8 = 7;

fn getPerms() {

    let mut ftp = Command::new("ftp")
                
        .arg(HOST)
        .arg(PORT.to_string())
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();
  
    let stdin = ftp.stdin.as_mut().unwrap();    
    writeln!(stdin, "anonymous").unwrap();
    writeln!(stdin, "").unwrap();
    writeln!(stdin, "passive").unwrap();
    writeln!(stdin, "ls").unwrap();
    writeln!(stdin, "quit").unwrap();  
    

    let _ = stdin;    

    let mut output = String::new();

    ftp.stdout
        .as_mut()
        .unwrap()
        .read_to_string(&mut output)
        .unwrap();    
    
    let mut unsigned_int: u8 = 1;
    for line in output.lines() {
        if line.len() >= 10 && unsigned_int<1{
            println!("{}", &line[0..10]);
        }
        else {
            unsigned_int -= 1;
        }
    }
}

fn decode(perms: &[String]) -> String {
    let mut decoded = String::new();

    for perm in perms {
        let chars: Vec<char> = perm.chars().collect();

        let bits: &[char] = if METHOD == 7 {
            if chars[..3].iter().any(|&c| c != '-') {
                continue; // If METHOD is 7, skip this permission string if the first three characters are not all -
            }
            &chars[3..] // If METHOD is 7, still use permissions after first three if first 3 are empty
        } else {
            &chars[..] // If METHOD isn't 7, use all characters of permission string
        };

        let value = bits
            .iter()
            .fold(0u32, |acc, &c| acc << 1 | (c != '-') as u32);

        if let Some(ch) = char::from_u32(value) {
            decoded.push(ch);
        }

    }
    decoded
}

    
fn main(){
    let test = getPerms();
    println!("{}", decode(&test));
}