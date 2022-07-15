mod front_of_house;

mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    fn fix_incorrect_order() {
        cook_order();
        super::deliver_order();
    }

    fn cook_order() {}

    pub enum Appetizer {
        Soup,
        Salad,
    }
}

fn deliver_order() {}

use crate::front_of_house::hosting;

use std::collections::HashMap;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();

    front_of_house::hosting::add_to_waitlist();

    let order1 = back_of_house::Appetizer::Salad;

    let mut meal = back_of_house::Breakfast::summer("Rye");

    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    let mut map = HashMap::new();
    map.insert(1, 2);
}