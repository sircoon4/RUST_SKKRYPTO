use std::collections::HashMap;
use super::t_token::TToken;

#[derive(Debug)]
pub struct NFT {
    name: String,
    symbol: String,
    token_map: HashMap<usize, String>
}

impl TToken for NFT {
    fn new(name: &str, symbol: &str) -> Self {
        Self {
            name: String::from(name),
            symbol: String::from(symbol),
            token_map: HashMap::new()
        }
    }

    fn mint(&mut self, to: &str, value: usize) -> bool {
        self.token_map.entry(value).or_insert(String::from(to));
        true
    }

    fn transfer(&mut self, from: &str, to: &str, value: usize) -> bool {
        self.token_map.insert(value, String::from(to));

        true
    }

    fn burn(&mut self, from: &str, value: usize) -> bool {
        self.token_map.insert(value, String::from("0x0"));

        return true
    }
}