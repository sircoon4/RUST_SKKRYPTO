use std::cmp::Ordering;
use std::collections::HashMap;

#[derive(Debug)]
pub struct User {
    username: String,
    balance: usize,
    nfts: HashMap<usize, String>
}

impl User {
    pub fn new(username: &str, balance: usize) -> User {
        User {
            username: String::from(username),
            balance: balance,
            nfts: HashMap::new()
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

    pub fn mint(&mut self, token_id: usize, token_url: &str) {
        self.nfts.insert(token_id, String::from(token_url));

        for (key, value) in &self.nfts {
            println!("{}: {}", key, value);
        }
    }
}