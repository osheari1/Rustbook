#![allow(unused_variables)]
#![allow(dead_code)]
pub mod common_programming_concepts;
pub mod enums_patternmatching;
pub mod guessing_game;
pub mod ownership;
pub mod structs;
pub mod common_collections;

extern crate rand;

#[cfg(test)]
mod tests {
    use super::structs::defining_instantiating;
    #[test]
    fn it_works() {
        defining_instantiating::run();
    }
}
