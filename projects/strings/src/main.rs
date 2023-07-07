fn main() {
    println!("Hello, world!");

    let str = create_str_from_new();
    print_str(&str);

    let str = create_str_with_initial_values();
    print_str(&str);

    let mut str = create_str_from_string_literal();
    print_str(&str);

    str.push_str(" world"); // can push str
    str.push('!'); // can only push char

    print_str(&str);

    // use + operator to append
    let str_part_1 = String::from("hello");
    let str_part_2 = String::from("world");

    // 需要使用&进行引用的原因是因为 + operator本质是调用add function，add fn的定义要求传入&str
    // add fn: fn add(self, s: &str) -> String {
    // 因为str_part_1的ownership被add拿走，str_part_1在+号后不再存在，而str_part_2为引用，后续依旧可用。
    let str = str_part_1 + " " + &str_part_2 + "!";

    // print_str(&str_part_1); // 报错 ownership
    // print_str(&str_part_2); // OK

    print_str(&str);

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let str = s1 + "-" + &s2 + "-" + &s3;

    print_str(&str);

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let str = format!("{s1}-{s2}-{s3}");
    print_str(&str);

    // string不支持index访问。
    // 原因是String的本质实现是存放着asciicode的vector: Vec<u8>，
    // 若允许indexing，则会返回相对应的number，为了保证意外的情况，Rust完全禁止了String indexing
    // let s1 = String::from("hello");
    // let h = s1[0]; // Compile Error

    let str = "Здравствуйте";

    let s = &str[0..4];
    // Зд本身占用4个bytes，0-4会得到Зд，而不是Здра
    print_str(&s);
}

fn create_str_from_new() -> String {
    let mut s = String::new();
    s.push('h');
    s.push('e');
    s.push('l');
    s.push('l');
    s.push('o');
    s
}

fn create_str_with_initial_values() -> String {
    let s = String::from("hello");

    s
}

fn create_str_from_string_literal() -> String {
    let s = "hello";

    s.to_string()
}

fn print_str(str: &str) {
    println!("The str created is {}", str);
}
