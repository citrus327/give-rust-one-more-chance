use std::{fs::File, io::ErrorKind};

fn main() {
    // a manual panic! call
    // panic!("plz panic!");

    // let vec = [1, 2, 3];

    // // this will break the program
    // vec[99];

    try_open_file();
    // try_open_file_with_specific_errors_with_fancy_looking();

    // let greeting_file = File::open("hello.txt").expect("yes!");

    // println!("success!, {:#?}", greeting_file)

    let a = read_username_from_file().expect("yes!");

    println!("success!, {}", a)
}

fn try_open_file() -> File {
    let greeting_file_result = File::open("hello.txt");

    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };

    return greeting_file;
    // println!("success!")
}

fn try_open_file_with_specific_errors() {
    let greeting_file_result = File::open("hello.txt");

    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            // 如果是NotFound的Kind，直接生成文件。
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error);
            }
        },
    };

    println!("success!, {:#?}", greeting_file)
}

fn try_open_file_with_specific_errors_with_fancy_looking() {
    let greeting_file = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });

    println!("success!, {:#?}", greeting_file)
}

use std::io::{self, Read};

fn read_username_from_file() -> Result<String, io::Error> {
    let username_file_result = File::open("hello.txt");

    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => {
            return Err(e);
        }
    };

    println!("123");

    let mut username = String::new();

    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}

fn read_username_from_file_simplified() -> Result<String, io::Error> {
    let mut username = String::from("");
    File::open("hello.txt")?.read_to_string(&mut username)?;
    Result::Ok(username)
}
