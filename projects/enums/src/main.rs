
// enum IpKind {
//     V4(String),
//     V6(String)
// }

//  struct IpAddr {
//     kind: IpKind,
//     address: String,
// }

enum IpKind {
    V4(u8, u8, u8, u8),
    V6(String),
}

struct Test {
    kind: IpKind,
    str: String
}

fn main() {

    let four = IpKind::V4(0, 0, 0, 0);
    let six = IpKind::V6(String::from("thisisv6"));
    let a = Option::Some(1);
    let b = 5;

    // cannot add `{integer}` to `Option<{integer}>
    // println!("the sum is {}", a + b);
}
