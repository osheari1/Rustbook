/// # General
/// * vec macro infers type: vec![1, 2, 3];
///
///
///
pub fn run() {
    let mut v: Vec<i32> = Vec::new();
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);

    let _v = vec![1, 2, 3];

    // Reading elements of vectors
    let v = vec![1, 2, 3, 4, 5];
    let third: &i32 = &v[2];
    let third: Option<&i32> = v.get(2);

    // This will panick
    // let does_not_exist = &v[100];
    let does_not_exist: Option<&i32> = v.get(100);

    // This wont work due to ownership
    let v = vec![1, 2, 3];
    let first = &v[0];
    // v.push(6);  // Cant have mutable and immutable references in same scope

    // For loops
    for i in &v {
        println!("{}", i);
    }
    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
    }

    // Using enum to store variables of different types
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
}
