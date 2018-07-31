/// # General
///
/// * Allows a less verbose way to patternmatch only one variant
///
/// # Examples
/// ```rust
/// let some_u8_value = Some(0u8);
/// if let Some(3) = some_u8_value {
///     println!("three");
/// }
/// ```
///

pub fn run() {
    let some_u8_value = Some(0u8);
    if let Some(3) = some_u8_value {
        println!("three");
    }
}
