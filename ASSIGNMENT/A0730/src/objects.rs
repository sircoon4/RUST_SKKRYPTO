pub mod nft;
pub mod ft;
pub mod t_token;

use t_token::TToken;

pub fn mint<T: TToken>(token: &mut T, to: &str, value: usize) -> bool {
    token.mint(to, value)
}

pub fn transfer<T: TToken>(token: &mut T, from: &str, to: &str, value: usize) -> bool {
    token.transfer(from, to, value)
}

pub fn burn<T: TToken>(token: &mut T, from: &str, value: usize) -> bool {
    token.burn(from, value)
}