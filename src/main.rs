pub mod common_programming_concepts;
pub mod guessing_game;

extern crate rand;


fn main() {
    println!("===== Getting Started =====");
    println!("Hello, world!");

    println!("===== Guesing Game =====");
    guessing_game::run();

    println!("===== Common Programming Concepts =====");
    common_programming_concepts::control_flow::run();
    common_programming_concepts::data_types::run();
    common_programming_concepts::variables_mutability::run();
    common_programming_concepts::functions::run();
}
