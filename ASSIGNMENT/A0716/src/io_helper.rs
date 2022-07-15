use std::io;
use std::io::prelude::*;

pub fn get_str_input(req: &str) -> String {
    println!("{}", req);
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    String::from(input.trim())
}

pub fn get_usize_input(req: &str) -> (usize, bool) {
    let num = get_str_input(req);

    let mut check = false;
    let num: usize = match num.parse(){
        Ok(num) => {
            check = true;
            num
        },
        Err(_) => {
            check = false;
            0
        }
    };

    (num, check)
}

pub fn pause() {
    let mut stdin = io::stdin();
    let mut stdout = io::stdout();

    // We want the cursor to stay at the end of the line, so we print without a newline and flush manually.
    write!(stdout, "Press any key to continue...").unwrap();
    stdout.flush().unwrap();

    // Read a single byte and discard
    let _ = stdin.read(&mut [0u8]).unwrap();
}