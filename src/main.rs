extern crate rustbook;

use rustbook::{
    common_programming_concepts,
    enums_patternmatching,
    ownership,
    structs,
    common_collections};

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
    enums_patternmatching::if_let::run();

    println!("===== Collections =====" );
    common_collections::vectors::run();
    common_collections::strings::run();



}
