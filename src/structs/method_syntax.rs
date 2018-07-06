/// **General**
///
/// _impl_ : Define the implementation block of a struct via impl Struct {...}
/// These can also be separated out into multiple blocks
///
/// Methods can borrow self immutably, mutably, and take ownership.
///
/// Rust has automatic references and dereferencing. There are only a few places
/// where this is the case. Calling methods is one of them.
///
/// _Associated Functions_ : In _impl_ blocks can define functions that don't
/// take self as a parameter. These are often used as constructors that return
/// a new instance of the struct.
///
/// 
///


#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}


impl Rectangle {
    fn square(len: u32) -> Rectangle {
        // Can also use Rectangle(len, len)
        Rectangle{ width: len, height: len }
    }
}

pub fn run() {
    let rect1 = Rectangle { width: 30, height 50};
    println!(
        "The area of the rectangle is {} pixels.",
        rect1.area()
    );

    let _rect2 = Rectangle::square(3);



}

