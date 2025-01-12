#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]

mod front_of_house;

pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_watchlist();
}
