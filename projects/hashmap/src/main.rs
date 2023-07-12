use std::collections::HashMap;

fn main() {
    let scores = create_hash_map();

    let score = scores.get("Blue").copied().unwrap_or(0);

    println!("score is {}", score);

    iterate_hash_map(&scores);

    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);

    // map.insert(&field_name, &field_value);

    // ownership已经被转移到了map，后续无法再获取field_name和field_value
    // println!("field_name {}", field_name);
    // println!("field_value {}", field_value);

    update_hash_map();
}

fn create_hash_map() -> HashMap<String, i32> {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    scores
}

fn iterate_hash_map(map: &HashMap<String, i32>) {
    for (key, value) in map {
        println!("{key}: {value}");
    }
}

fn update_hash_map() {
    let mut scores = HashMap::new();

    // overwriting
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);

    // overwriting by checking if already has value
    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);

    println!("{:?}", scores);
}
