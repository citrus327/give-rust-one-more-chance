// https://doc.rust-lang.org/rust-by-example/macros.html
mod macro_set;

fn foo() {}

use_function!(foo);

fn main() {
    foo();

    no_arguments!();

    use_expression!("helo");

    use_block!({
        let a = 1;
        a
    });
}
