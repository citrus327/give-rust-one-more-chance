fn main() {
    fn longest<'a, 'b>(x: &'a str, y: &'b str) -> &'b str {
        if x.len() > y.len() {
            y
        } else {
            y
        }
    }

    let str1 = "1234";
    let str2 = "123";

    let longest = longest(&str1, &str2);

    println!("longest is, {}", longest)
}
