use std::io;
use std::io::prelude::*;
use std::cmp::Ordering;
use rand::Rng;

#[derive(Debug)]
struct User {
    username: String,
    balance: usize,
}

impl User {
    fn username(&self) -> &str {
        &self.username[..] // & 빼면 안됨... 그 이유...?
    }

    fn balance(&self) -> usize {
        self.balance
    }

    fn deposit(&mut self, amount: usize) -> usize {
        self.balance += amount;
        self.balance
    }

    fn withdraw(&mut self, amount: usize) -> usize {
        match self.balance.cmp(&amount) {
            Ordering::Less => {
                println!("Not Enough Balance!");
            }
            other => {
                self.balance -= amount;
            }
        }
        self.balance
    }
}

fn main() {
    let mut valid_user = [
        User {
            username: String::from("sircoon"),
            balance: 5000
        },
        User {
            username: String::from("tester"),
            balance: 76
        },
        User {
            username: String::from("admin"),
            balance: 99999
        },
    ];

    println!("Enter your username.");
    let mut username = String::new();
    io::stdin()
        .read_line(&mut username)
        .expect("Failed to read line");

    let username = username.trim();

    let mut user = User {
        username: String::from(""),
        balance: 0
    };

    for el in valid_user {
        //if username.eq(&el.username) {
        if username == el.username {
            user = el;
        }
    }

    if user.username == "" {
        println!("No User Information Matched!");
        return
    }
    
    println!("User Info: {:?}", user);

    'main_loop: loop {
        println!("\n\n\n");
        println!("What do you want to do?");
        println!("1. Get a info");
        println!("2. Deposit");
        println!("3. Withdraw");
        println!("Other number for quit");
        println!("Enter number: ");

        let mut num = String::new();

        io::stdin()
            .read_line(&mut num)
            .expect("Failed to read line");

        let num: u32 = match num.trim().parse(){
            Ok(num) => num,
            Err(_) => {
                println!("It's not number!");
                continue
            }
        };

        match num {
            1 => {
                println!("User name: {}, Balance: {}", user.username(), user.balance())
            }
            2 => {
                println!("How much?");
                let mut num = String::new();

                io::stdin()
                    .read_line(&mut num)
                    .expect("Failed to read line");

                let num: usize = match num.trim().parse(){
                    Ok(num) => num,
                    Err(_) => {
                        println!("It's not number!");
                        continue
                    }
                };

                println!("Your Balnce: {}", user.deposit(num));
            }
            3 => {
                println!("How much?");
                let mut num = String::new();

                io::stdin()
                    .read_line(&mut num)
                    .expect("Failed to read line");

                let num: usize = match num.trim().parse(){
                    Ok(num) => num,
                    Err(_) => {
                        println!("It's not number!");
                        continue
                    }
                };

                println!("Your Balnce: {}", user.withdraw(num));
            }
            _ => break ,
        }
        pause();
    }
}

fn pause() {
    let mut stdin = io::stdin();
    let mut stdout = io::stdout();

    // We want the cursor to stay at the end of the line, so we print without a newline and flush manually.
    write!(stdout, "Press any key to continue...").unwrap();
    stdout.flush().unwrap();

    // Read a single byte and discard
    let _ = stdin.read(&mut [0u8]).unwrap();
}