/// # General
///
/// * All possible cases are caught by the compiler
///
/// * Use _ in patterns to match any value.
///
///
///
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

pub fn run() {
    let f = Some(5);
    let s = plus_one(Some(6));
    let none = plus_one(None);
}

fn value_in_cents(coin: Coin) -> u32 {
    match coin {
        Coin::Penny => {
            println!("This is a penny");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}
