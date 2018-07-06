#![allow(dead_code)]
#![allow(unused_variables)]

pub mod common_programming_concepts;
pub mod guessing_game;
pub mod ownership;
pub mod structs;
pub mod enums_patternmatching;

extern crate rand;


fn main() {
    println!("===== Getting Started =====");
    println!("Hello, world!");

    println!("===== Guesing Game =====");
    // guessing_game::run();

    println!("===== Common Programming Concepts =====");
    common_programming_concepts::control_flow::run();
    common_programming_concepts::data_types::run();
    common_programming_concepts::variables_mutability::run();
    common_programming_concepts::functions::run();


    println!("===== Ownership =====");
    ownership::what_is_ownership::run();
    ownership::references_borrowing::run();
    ownership::slices::run();

    println!("===== Structs =====");
    structs::defining_instantiating::run();
    structs::example_prog::run();
    structs::method_syntax::run();

    println!("===== Enums and Pattern matching =====");
    enums_patternmatching::define_enum::run();
    enums_patternmatching::pattern_match::run();





}
