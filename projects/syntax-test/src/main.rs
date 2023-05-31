fn str_comparision() {
    // // String creation
    // let string_literal: &str = "Hello"; // Immutable reference to a string slice
    // let string_owned: String = String::from("Hello"); // Owned, mutable string

    // // Concatenation
    // let concatenated: String = string_owned + " world!"; // Combine two strings

    // println!("Owned string: {}", string_owned);
    // println!("String literal: {}", string_literal);
    // println!("Concatenated string: {}", concatenated);

    // String creation
    let string_literal: &str = "Hello"; // Immutable reference to a string slice
    let string_owned: String = String::from("Hello"); // Owned, mutable string
    println!("Owned string: {}", string_owned);

    // Concatenation
    let concatenated: String = string_owned + " world!"; // Combine two strings

    println!("String literal: {}", string_literal);
    println!("Concatenated string: {}", concatenated);
}

fn main() {
    // a mutatable variable
    let mut apples = 5;
    apples = 6;
    println!("{}", apples);

    // an immutable variable
    let pears = 5;
    println!("{}", pears);

    str_comparision()
}
