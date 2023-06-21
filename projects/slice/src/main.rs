fn main() {
    let mut s = String::from("Hello World");

    let i = first_word(&s);

    // borrow issue
    // s.clear();

    println!("{}, {}", &s, i);

    // println!("{i}")

    // let first = &s[0..i];

    // println!("first is {first}");
}


// fn first_word (s: &String) -> usize{
//     let bytes = s.as_bytes();

//     for (i, &item) in bytes.iter().enumerate() {
//         println!("The index {i} is {}", &item);
//         if item == b' ' {
//             return i;
//         }
//     }
//     return s.len();
// }


fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}