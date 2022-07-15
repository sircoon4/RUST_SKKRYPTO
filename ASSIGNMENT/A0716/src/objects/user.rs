use std::cmp::Ordering;

#[derive(Debug)]
pub struct User {
    username: String,
    balance: usize,
}

impl User {
    pub fn new(username: &str, balance: usize) -> User {
        User {
            username: String::from(username),
            balance: balance
        }
    }
}

impl User {
    pub fn username(&self) -> &str {
        &self.username[..]
    }

    pub fn balance(&self) -> usize {
        self.balance
    }

    pub fn deposit(&mut self, amount: usize) -> usize {
        self.balance += amount;
        self.balance
    }

    pub fn withdraw(&mut self, amount: usize) -> usize {
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