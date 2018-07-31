///
/// **General**
///
///
/// _println!()_ : Be default uses formatting 'Display'.
/// Structs don't have a default implimentation of Display.
///
/// _#[derive(Debug)] : Annotation that applys the default implimentation
/// of the debug trait. Used in println!({:?}, obj) and {:#?}
///
/// _derive_ : Derive annotaitons are listed in Appendix C
///
///
///
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

pub fn run() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    println!(
        "The area of hte rectangle is {} square pixels",
        area(&rect1)
    );

    println!("rect1 fields {:?}", rect1);
}

// Want to borrow the rectangle instead of taking ownership.
// This allows rect1 to be reused by main.
fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
