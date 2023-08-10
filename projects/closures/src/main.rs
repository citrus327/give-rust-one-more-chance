#[derive(Debug, Copy, PartialEq, Clone)]
enum ShirtColor {
    Red,
    Blue,
}

struct Inventory {
    shirts: Vec<ShirtColor>,
}

impl Inventory {
    fn giveaway(&mut self, user_preference: Option<ShirtColor>) -> ShirtColor {
        let target_shirt = user_preference.unwrap_or_else(|| self.most_stocked());

        if let Some(index) = self
            .shirts
            .iter()
            .position(|&target| target == target_shirt)
        {
            self.shirts.remove(index);
        }

        target_shirt
    }

    fn most_stocked(&self) -> ShirtColor {
        let mut num_red = 0;
        let mut num_blue = 0;

        for color in &self.shirts {
            match color {
                ShirtColor::Blue => num_blue += 1,
                ShirtColor::Red => num_red += 1,
            }
        }
        let result = if num_red >= num_blue {
            ShirtColor::Red
        } else {
            ShirtColor::Blue
        };

        result
    }
}

fn main() {
    let mut store = Inventory {
        shirts: vec![
            ShirtColor::Red,
            ShirtColor::Red,
            ShirtColor::Blue,
            ShirtColor::Blue,
        ],
    };

    let user_pref1 = Some(ShirtColor::Red);
    let giveaway1 = store.giveaway(user_pref1);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref1, giveaway1
    );

    let user_pref2 = None;
    let giveaway2 = store.giveaway(user_pref2);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref2, giveaway2
    );
}
