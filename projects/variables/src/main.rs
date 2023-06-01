fn main() {
    // mutable variables
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    // constant variables, and the type must be anotated
    const A_CONSTANT_VALUE: i32 = 6;
    const YET_ANTHOER_CONSTANT_VALUE: i32 = A_CONSTANT_VALUE * 12;

    println!("The constant value is {}", A_CONSTANT_VALUE);
    println!(
        "The caculated constant value is {}",
        YET_ANTHOER_CONSTANT_VALUE
    );

    let x = 5;
    println!("The shadowed x value is {x}");

    tuple_test()
}

fn tuple_test() {
    // tuple解构的2种方式
    let tup = (500, 6.4, 1);

    let (x, y, z) = tup;

    println!("The value of x is: {x}");
    println!("The value of y is: {y}");
    println!("The value of z is: {z}");

    let five_hundred = tup.0;

    let six_point_four = tup.1;

    let one = tup.2;

    println!("The value of five_hundred is: {five_hundred}");
    println!("The value of six_point_four is: {six_point_four}");
    println!("The value of one is: {one}");
}
