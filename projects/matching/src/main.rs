#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        // Coin::Nickel => 5,
        // Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("the state is from {:?}", state);
            return 25;
        }
        _ => {
            println!("other");
            return 2;
        }
    }
}

fn main() {
    // println!(
    //     "value is {}",
    //     value_in_cents(Coin::Quarter(UsState::Alaska))
    // );

    // println!("value is {}", value_in_cents(Coin::Penny));

    let config_max = Some(3);
    match config_max {
        Some(max) => println!("The maximum is configured to be {}", max),
        _ => (),
    }

    let config_max = Some(3u8);
    if let Some(max) = config_max {
        println!("The maximum is configured to be {}", max);
    }

    let coin = Coin::Quarter(UsState::Alabama);
    let mut count = 0;
    // match coin {
    //     Coin::Quarter(state) => println!("State quarter from {:?}!", state),
    //     _ => count += 1,
    // }

    if let Coin::Quarter(state) = coin {
        println!("State quarter from {:?}!", state)
    } else {
        count += 1;
    }
    println!("The count is {}", count)
}

// fn plus_one(x: Option<i32>) -> Option<i32> {
//     match x {
//         None => None,
//         Some(i) => Some(i + 1),
//     }
// }

// fn main() {
//     let five = Some(5);
//     let six = plus_one(five);
//     let none = plus_one(None);
// }
