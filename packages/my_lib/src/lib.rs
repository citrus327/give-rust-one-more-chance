mod front_of_house;

use crate::front_of_house::hosting;

use crate::hosting as my_hosting;
// pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
    my_hosting::add_to_waitlist();
}
