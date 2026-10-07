use std::process::{Command,Stdio};
use std::io::{Write, Read};


fn getPerms() {

    let mut ftp = Command::new("ftp")
                
        .arg("localhost")
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
    println!("FTP OUTPUT:");
    println!("{}", output);
    
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

fn main(){
    let test = getPerms();
}