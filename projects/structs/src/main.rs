struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username: username,
        email: email,
        sign_in_count: 1,
    }
}

struct Color (u32, u32, u32);
struct Point (i32, i32, i32);



fn main() {
    let mut user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    // user1需要整体标注为mut
    user1.email = String::from("anotheremail@example.com");

    let user2 = build_user(
        String::from("citrus327@outlook.com"),
        String::from("hophop")
    );

    // 取user1的username，剩下的采用user2，而非使用user2进行全量覆盖
    let merged_user = User {username: user1.username, sign_in_count: user2.sign_in_count, ..user2 };

    println!("user email is {}", user1.email);

    // user1.username已经被merged_user借用，无法再次使用。
    // println!("user email is {}", user1.username); 

    // 而sign_in_count的类型属于u64, 属于scalar type, 可以再次使用（实现了Copy Trait)
    println!("user2 sign_in_count is {}", user2.sign_in_count); 
    println!("merged user name is {}", merged_user.username); // someusername123
    println!("merged user email is {}", merged_user.email); // citrus327@outlook.com


    let white = Color(255, 255, 255);
    let origin = Point(0, 0, 0);

    println!("tuple struct black is {}", white.0) // 255
}
