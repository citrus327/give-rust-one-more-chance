use crate::garden::vegetables::Asparagus;
use crate::houses::front_of_house::hosting;

pub mod garden;
pub mod houses;

fn main() {
    let plant = Asparagus {};
    println!("I'm growing {:?}!", plant);

    test_house();
}

fn test_house() {
    hosting::add_to_waitlist();
}
