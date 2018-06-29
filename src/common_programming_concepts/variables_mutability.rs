/// **General**
///
/// _Constants_  - are always immutable and must have type declaration.
/// Declared via const.
/// They can be declared in any scope
///
/// # Example
/// ```rust
/// const MAX_POINTS: u32 = 100_000;
/// ```
///
/// _Shadowing_ - Is different than calling a variable mutable.
/// Can also change type.
/// # Example
///
/// ```rust
/// let spaces = "   ";
/// let spaces = spaces.len();
/// ```
///
/// Will not compile
///
/// ```rust
/// let mut spaces = "   ";
/// let spaces = spaces.len();
/// ```
pub fn run() {
    println!("Empty function.");
}
