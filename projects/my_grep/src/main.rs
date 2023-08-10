use std::{env, process};

use my_grep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Reading config wrong, errr is {}", err);
        process::exit(1)
    });

    println!(
        "Searching for {}, in file: {}",
        config.query, config.file_path
    );

    my_grep::run(&config).unwrap_or_else(|err| {
        eprintln!("Application error: {err}");
        process::exit(1);
    })
}
