extern crate rand;

// Bring types into scope
use std::io;
use std::cmp::Ordering;
use rand::Rng;
// use guessing_game::rand::Rng; // Could also use self::rand::Rng;


/// **let** -  statements are used to create variables
///
/// **expect** - Unwraps a results, yielding the content of an Ok. Panics if the value is an Err.
///
/// **variables** are immutable by default.
/// Adding mut in front of name turns it into a mutable variable.
///
/// **String::new()** - Indicates new() is an associated function with the type String.
/// Associated functions are implemented on a type (static function)
///
/// **&** - Indicates a reference to a variable.
/// References are immutable by default. Need to add &mut guess to make mutable.
///
/// **Ordering** - Enum with variants {Less, Greater, Equal}
///
/// **trim()** - Eliminates white space and newlines. read_line(&var) will assign the
/// value given by the use AND \n.
///
/// **loop** - Creates an infinite loop.
///
/// **===== General =====**
///
/// When calling methods with '.', push to new line.
///
/// Run cargo doc --open to build documentation of all dependancies and open in browser.
///
/// guess.cmp(&secret_number) cannot infer that guess should be converted to a
/// numberical type for comparison.
pub fn run() {

    println!("Guess the number");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        // Should be doing error handling here
        io::stdin().read_line(&mut guess).expect(
            "Failed to read line",
        );

        // Use pattern matching to properly handle non numerical input
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too large!"),
            Ordering::Equal => {
                println!("Correct!");
                break;
            }
        }
    }








}
