pub trait TToken {
    fn new(name: &str, symbol: &str) -> Self;

    fn mint(&mut self, to: &str, value: usize) -> bool;
    fn transfer(&mut self, from: &str, to: &str, value: usize) -> bool;
    fn burn(&mut self, from: &str, value: usize) -> bool;
}