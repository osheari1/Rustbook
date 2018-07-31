use std::collections::HashMap;

pub fn run() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    // Can convert a vector of tuples to hashmap
    let teams = vec![String::from("Blue"), String::from("yellow")];
    let initial_scores = vec![10, 50];

    let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();

    // Types that implement the Copy() Trait are copied into the hash map
    // For other values, like String, they are moved
    let field_name = String::from("Favourite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // At this point field_name and field_value are not valid
    // Can pass references to hash map to not take ownership

    // Accessing
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    let team_name = String::from("Blue");
    let score = scores.get(&team_name);

    // Iterate over k, v pairs
    for (k, v) in &scores {
        println!("{}: {}", k, v);
    }

    // Overwriting
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 50);

    // Insert if does not exist
    // .entry().or_insert() returns an Entry, a mutable reference to the
    // value at the key.
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);

    // Updating
    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
}
