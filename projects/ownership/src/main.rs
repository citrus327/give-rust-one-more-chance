// fn main() {
//     // let mut s = String::from("hello");

//     // s.push_str(", world!"); // push_str() appends a literal to a String

//     // println!("{}", s); // This will print `hello, world!`

    
//     let s1 = String::from("hello");
//     // s1已经move到了s2, s1在后续scope中不再存在
//     // let s2 = s1;
//     // 可以使用clone来进行shallowcopy
//     let s2 = s1.clone();

//     println!("{}, world!", s1);
// }


// fn main() {
//     let s = String::from("hello"); 

//     takes_ownership(s);

//     let x = 5;

//     makes_copy(x);

//     println!("{}", x)

// } // Here, x goes out of scope, then s. But because s's value was moved, nothing
//   // special happens.

// fn takes_ownership(some_string: String) { // some_string comes into scope
//     println!("{}", some_string);
// } // Here, some_string goes out of scope and `drop` is called. The backing
//   // memory is freed.

// fn makes_copy(some_integer: i32) { // some_integer comes into scope
//     println!("{}", some_integer);
// } // Here, some_integer goes out of scope. Nothing special happens.


// fn main() {
//     let s1 = String::from("hello");

//     let len = calculate_length(&s1);

//     println!("The length of '{}' is {}. The Ref is {}", s1, len, &s1);
// }

// fn calculate_length(s: &String) -> usize {
//     s.len()
// }

// fn main() {
//     let mut s = String::from("hello");

//     change(&mut s);

//     println!("The changed s is {s}");


//     {
//         let r1 = &mut s;
//         r1.push_str(" inner")
//     } // r1 goes out of scope here, so we can make a new reference with no problems.

//     let r2 = &mut s;

//     r2.push_str(" outter");

//     println!("after two &mut, s is ${s}")
// }

// fn change(some_string: &mut String) {
//     some_string.push_str(", world");
// }


fn main() {
    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    // let r3 = &mut s; // BIG PROBLEM

    // println!("{}, {}, and {}", r1, r2, r3);
    println!("{}, {}", r1, r2);

    let r3 = &mut s;
    r3.push_str(" world");

    println!("{}, {}, {}", r1, r2, r3);
    // println!("{}", r3);
}