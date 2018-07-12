/// **Ownership Rules**
/// * Each value in Rust has a variable that's called _owner_.
/// * There can only be one owner at a time.
/// * When the owner groes out of scope, the value will be dropped.
///
///
/// **String**
/// * len - How much memory in bytes the String is currently using
/// * capacity - Total memory allocated by OS to String
///
///
/// **General**
/// * Two types of strings: string literals "...", and String
///
/// * When variables on the heap go out of scop, _drop()_ is called to free memory.
///
/// * When pointers are copied from one variable to another, the first variable is _invalidated_.
/// This is considered a _move_.
/// This only applies to variables on the stack.
///
/// * Rust has a Copy trait that can by placed on anything stored on the stack. Anything
/// with the Drop trait cannot implement the Copy trait.
///
/// * Returning values can also transfer ownership
///
///
pub fn run() {
    println!("");

    {
        let _s = "hello";
        println!("s is in scope");
        println!("End of block");
    }
    println!("s is out of scope");

    // String objects can be mutated.
    {
        let mut _s = String::from("hello");

        // append a literal to a Strin
        println!("Append a literal to a String");
        _s.push_str(", world!");
        println!("{}", _s);
    } // Once s goes out of scope, memory is freed.

    // Ways variables and data interact
    let x = 5;
    let _y = x; // x is copied to create y

    // s1 copies to the reference to s2
    // Printing s1 will fail as it has been invalidated.
    let s1 = String::from("hello");
    let _s2 = s1;

    // The function clone can deepcopy the data.
    let s1 = String::from("hello");
    let _s2 = s1.clone();
    println!("Can still use s1 {}", s1);

    // Ownership and functions
    println!("Ownership and functions");
    let s = String::from("hello"); // s is in scope
    take_ownership(s); // s's value moves into the function and is no longer valid here
                       // println!(s); Throws compile error

    let x = 5; // x is in scope
    makes_copy(x); // x moves into function, but i32 is Copy, so x can still be used.
    println!("x can still be used {}", x);

    println!("Return values and scope.");
    let _s1 = give_ownership(); // Ownership transfered to s1
    let s2 = String::from("Hello"); // s2 in scope
    let _s3 = take_and_give_back(s2); // s2 is moved into take_and_give_back, then
                                      // returned by function.
                                      // s1 is dropped, s2 was moved so already dead, s3 is dropped after block end
}

fn take_and_give_back(string: String) -> String {
    string // string is returned
}

fn give_ownership() -> String {
    String::from("Hello")
}

fn take_ownership(string: String) {
    // s comes into scope
    println!("{}", string);
} // string goes out of scope and drop is called

fn makes_copy(integer: i32) {
    // integer in scope
    println!("{}", integer);
} // integer goes out of scope but nothing special happens, there is no drop method.
