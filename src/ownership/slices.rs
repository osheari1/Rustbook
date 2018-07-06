/// **General**
///
/// _Slices_ : Let you references a subsequence of elements in a collection.
///
/// _String Slice_ : Reference to part of a string.
/// ```rust
/// let s = String::from("hello world");
/// let hello = &s[0..5];
/// let world = &s[6..11];
///
/// // Start at 0
/// &s[..5];
/// // Go to end
/// &s[6..];
///
/// ```
///
/// Internally, the slice stores the start ix and length
///
/// Slices do not have ownership
///
///
///
///

pub fn run() {
    let s = String::from("Hello world");
    let _word = first_word_returning_index(&s);
    // s.clear(); // This empties the String
    // After this, we have the index to word, but the String has changed

    // String slices
    let s = String::from("hello world");
    let _hello = &s[0..5];
    let _world = &s[6..11];

    // First word with slices
    let s = String::from("hello world");
    let _fw = first_word_pass_string(&s);
    let _fw = first_word_pass_str(&s[..]);
    // The compiler will throw an error if you empty the string
    // s.clear();


    // General slice type
    let a = [1, 2, 3, 4, 5];
    let _slice = &a[1..3];



}

fn first_word_returning_index(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    s.len() // If is one word returns last index
}

fn first_word_pass_string(s: &String) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    &s[..] // If is one word returns last index
}

fn first_word_pass_str(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    &s[..] // If is one word returns last index
}


