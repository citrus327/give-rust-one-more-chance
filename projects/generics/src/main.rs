fn main() {
    println!("Hello, world!");

    let arr1 = vec![32, 22, 11];
    let largest = find_largest(&arr1);

    println!("the largest is, {}", largest);

    struct_with_generic();
}

struct Point<T> {
    x: T,
    y: T,
}

fn struct_with_generic() {
    let p1 = Point { x: 2, y: 3 };
    let p2 = Point { x: '2', y: '3' };
}

fn find_largest(list: &[i32]) -> &i32 {
    let mut largest: &i32 = &list[0];

    for number in list {
        if number > largest {
            largest = number
        }
    }

    return largest;
}

// fn find_largest_with_generic<T>(list: &[T]) -> &T {
//     let mut largest = &list[0];

//     for number in list {
//         if number > largest {
//             largest = number
//         }
//     }

//     return largest;
// }
