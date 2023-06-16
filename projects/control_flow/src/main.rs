fn main() {
    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    // 可以直接赋值
    let number = if true { 5 } else { 6 };
    println!("The number is {}", number);

    // infinite loop
    // loop {
    //     println!("again");
    // }

    // 使用loop + break直接使用循环对一个变量赋值
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };
    println!("Result is {result}");

    // while loop
    let mut count: i32 = 5;

    while count != 1 {
        println!("in while loop");
        count -= 1;
    }

    // for loop an array
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {element}");
    }

    for number in (1..4).rev() {
        println!("{number}!");
    }
}
