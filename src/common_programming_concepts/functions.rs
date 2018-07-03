/// **Function Parameters**
///
/// Must ALWAYS define parameter types.
///
/// Functions contains statements and expressions
///
/// _Statements_ - Perform some action and do not return a value.
///
/// _Expressions_ - Evaluate to a resulting value.
/// Do not end in a semicolon. If it ends in a semicolon it will become a statement.
///
/// Can return implicitly.
///
/// **General**
///
/// Rust uses snake case.
///
///
///
///
pub fn run() {
    println!("");

    // Statement
    let x: i32 = 6;
    // This will not compile. let x = y; is a statement
    // let y = (let x = 6);

    // Expression
    let _y = {
        let _x = 1;
        _x + 1
    };

    let _same = some_function(x);
}

// Return values are specified with ->
fn some_function(x: i32) -> i32 {

    println!("Some function {}", x);
    x // No semicolon
}
