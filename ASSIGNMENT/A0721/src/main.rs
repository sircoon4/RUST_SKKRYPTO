pub mod objects;
pub mod io_helper;

use objects::user::User;
use rand::Rng;

fn main() {
    let mut valid_users: Vec<User> = Vec::new();
    valid_users.push(User::new("sircoon", rand::thread_rng().gen_range(1..=10000)));
    valid_users.push(User::new("test", 76));
    valid_users.push(User::new("admin", 9999999));

    let username = io_helper::get_str_input("Enter your username.");

    let mut is_user_valid = false;
    // let mut user: User = User::new("", 0);
    // 아래 반복문에서 user = el 이렇게 할 방법은 없을지....
    let mut i = 0;
    for el in &valid_users {
        if username == el.username() {
            is_user_valid = true;
            break;
        }
        i += 1;
    }

    if !is_user_valid {
        println!("No User Information Matched!");
        return
    }
    
    println!("User Info: {:?}", &valid_users[i]);
    io_helper::pause();

    let mut nft_id = 0;

    loop {
        println!("\n\n\n");
        println!("What do you want to do?");
        println!("1. Get a info");
        println!("2. Deposit");
        println!("3. Withdraw");
        println!("4. Create User");
        println!("5. Change User");
        println!("6. Mint NFT");
        println!("Other number for quit");

        let (num, check) = io_helper::get_usize_input("Enter number: ");
        if !check {
            println!("It's not number..");
            continue;
        }

        match num {
            1 => {
                println!("User name: {}, Balance: {}", &valid_users[i].username(), &valid_users[i].balance())
            }
            2 => {
                let (num, check) = io_helper::get_usize_input("How much?");
                if !check {
                    println!("It's not number..");
                    continue;
                }

                println!("Your Balnce: {}", &valid_users[i].deposit(num));
            }
            3 => {
                let (num, check) = io_helper::get_usize_input("How much?");
                if !check {
                    println!("It's not number..");
                    continue;
                }

                println!("Your Balnce: {}", &valid_users[i].withdraw(num));
            }
            4 => {
                let username = io_helper::get_str_input("Enter new username.");
                valid_users.push(User::new(&username[..], 0));

                println!("\nValid Users");
                for el in &valid_users {
                    println!("{:?}", el);
                }
            }
            5 => {
                let username = io_helper::get_str_input("Enter your username.");
                
                i = 0;
                is_user_valid = false;
                for el in &valid_users {
                    if username == el.username() {
                        is_user_valid = true;
                        break;
                    }
                    i += 1;
                }

                if !is_user_valid {
                    println!("No User Information Matched!");
                    return
                }
                
                println!("User Info: {:?}", &valid_users[i]);
            }
            6 => {
                let username = io_helper::get_str_input("Enter nft uri.");
                &valid_users[i].mint(nft_id, &username[..]);

                nft_id += 1;
            }
            _ => break ,
        }
        io_helper::pause();
    }
}

