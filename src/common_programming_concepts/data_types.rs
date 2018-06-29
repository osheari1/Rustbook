/// **Scalar Types** - integers, floating point, boolean, and characters.
///
/// _characters_ - Represent 'Unicode Scalar Values'. Can represent a lot more than
/// just ascii. eg. Japanese, Chinese, emojis ect.
///
/// **Compound Types** - Groups multiple values into 1 type: tuple, array
///
/// _tuples_ - Don't need to have the same type
///
/// _arrays_ - Only one type per array.
/// Arrays are static in size. They are allocated on the stack, not the heap.
///
///
/// **General**
///
/// Rust is statically typed - Must know all types at compile time.
///
pub fn run() {
    let tup: (i64, f64, u8) = (20, 30.4, 2);

    // Destructoring
    let (_x, _y, _z) = tup;
    println!("The value of y is {}", _y);

    // Via index
    let _x1 = tup.0;
    let _y1 = tup.1;
    let _z1 = tup.2;

    // arrays
    let a = [1, 2, 3, 4, 5];

    // Accessing array elements
    let _first = a[0];
    let _second = a[1];

    // Need to be careful of index out of bounds errors.
}

