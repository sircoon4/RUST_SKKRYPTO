pub mod objects;

use objects::nft::NFT;
use objects::ft::FT;
use objects::t_token::TToken;

fn main() {
    let address1 = "0xF39E4961C046BA913f835c08Bf25De348184F3a8";
    let address2 = "0x22c31403fc94d750DA528E3468d95b657d399B4B";

    let mut ft = FT::new("SircoonFT", "SFT");
    let mut nft = NFT::new("SircoonNFT", "SNFT");

    println!("{:#?}", ft);
    println!("{:#?}", nft);
    println!("-----------------------------------------------");

    objects::mint(&mut ft, address1, 500);
    objects::mint(&mut nft, address1, 500);

    println!("{:#?}", ft);
    println!("{:#?}", nft);
    println!("-----------------------------------------------");

    objects::transfer(&mut ft, address1, address2, 500);
    objects::transfer(&mut nft, address1, address2, 500);

    println!("{:#?}", ft);
    println!("{:#?}", nft);
    println!("-----------------------------------------------");

    objects::burn(&mut ft, address2, 500);
    objects::burn(&mut nft, address2, 500);

    println!("{:#?}", ft);
    println!("{:#?}", nft);
    println!("-----------------------------------------------");
}
