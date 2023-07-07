pub mod enum_vector;

use crate::enum_vector::create_enum_vector;

fn main() {
    let mut v: Vec<i32> = Vec::new();

    // mutating vector
    v.push(1);
    v.push(2);
    v.push(3);
    v.push(4);

    println!("the vector is {:?}", v);

    reading_vec_by_index(&v, 2);
    reading_vec_by_get(&v, 2);

    let mut v = vec![1, 2, 3, 4, 5];

    // 针对第一个元素进行reference，将会hold整个vector的ownership，所以不允许再push值了。
    let first = &v[0];

    // v.push(6);

    println!("The first element is: {:?}", first);

    v.push(6);

    iterate_vec(&v);

    iterate_vec_and_mutating(&mut v);

    iterate_vec(&v);

    create_enum_vector();
}

fn iterate_vec(v: &Vec<i32>) {
    for i in v {
        println!("the value at index {i} is {}", i);
    }
}

fn iterate_vec_and_mutating(v: &mut Vec<i32>) {
    for i in v {
        *i += 50;
    }
}

fn reading_vec_by_index(v: &Vec<i32>, index: usize) {
    println!(
        "reading index {} from vector {:?}, value is {}",
        index, v, v[index]
    );
}

fn reading_vec_by_get(v: &Vec<i32>, index: usize) {
    // let third3 = v.get(index);

    // let result: i32;
    // match third3 {
    //     Some(v) => result = *v,
    //     None => println!("There is no third element."),
    // }

    // return result;

    let third4 = v.get(index);
    if let Some(third) = third4 {
        println!(
            "reading index {} from vector {:?}, value is {}",
            index, v, third
        );
    } else {
        println!("There is no third element.");
    }
}
