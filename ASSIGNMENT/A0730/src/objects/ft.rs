use std::collections::HashMap;
use super::t_token::TToken;

#[derive(Debug)]
pub struct FT {
    name: String,
    symbol: String,
    token_map: HashMap<String, usize>
}

impl TToken for FT {
    fn new(name: &str, symbol: &str) -> Self{
        Self {
            name: String::from(name),
            symbol: String::from(symbol),
            token_map: HashMap::new()
        }
    }

    fn mint(&mut self, to: &str, value: usize) -> bool {
        let amount = self.token_map.entry(String::from(to)).or_insert(value);
        if *amount != value {
            *amount += value
        }
        true
    }

    fn transfer(&mut self, from: &str, to: &str, value: usize) -> bool {
        let amount = self.token_map.get(&String::from(from));
        match amount {
            Some(amount) => {
                self.token_map.insert(String::from(from), amount - value);
                self.mint(to, value);
            }
            _ => {}
        }

        return true
    }

    fn burn(&mut self, from: &str, value: usize) -> bool {
        let amount = self.token_map.get(&String::from(from));
        match amount {
            Some(amount) => {
                self.token_map.insert(String::from(from), amount - value);
            }
            _ => {}
        }

        true
    }
}