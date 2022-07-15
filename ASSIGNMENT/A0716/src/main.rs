pub mod objects;
pub mod io_helper;

use objects::user::User;
use rand::Rng;

fn main() {
    let valid_user = [
        User::new("sircoon", 5000),
        User::new("test", 76),
        User::new("admin", 9999),
    ];

    let username = io_helper::get_str_input("Enter your username.");

    let mut user = User::new("", 0);

    for el in valid_user {
        //if username.eq(&el.username) {
        if username == el.username() {
            user = el;
        }
    }

    if user.username() == "" {
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

        let (num, check) = io_helper::get_usize_input("Enter number: ");
        if !check {
            println!("It's not number..");
            continue;
        }

        match num {
            1 => {
                println!("User name: {}, Balance: {}", user.username(), user.balance())
            }
            2 => {
                let (num, check) = io_helper::get_usize_input("How much?");
                if !check {
                    println!("It's not number..");
                    continue;
                }

                println!("Your Balnce: {}", user.deposit(num));
            }
            3 => {
                let (num, check) = io_helper::get_usize_input("How much?");
                if !check {
                    println!("It's not number..");
                    continue;
                }

                println!("Your Balnce: {}", user.withdraw(num));
            }
            _ => break ,
        }
        io_helper::pause();
    }
}

