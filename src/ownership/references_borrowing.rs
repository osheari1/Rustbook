/// **General**
/// _&_ : Create references uing &
///
/// _*_ : Dereference operator
///
/// After a reference goes out of scope, the value it refers to will not drop.
///
/// Having references as function parameters is called borrowing.
///
/// Cannot borrow mutably using a reference, must make referencem mutable.
///
/// Mutable references can only be used on mutable variables
///
/// You can only have 1 mutable reference toa particular piece of data in
/// a particular scope
///
/// Cannot borrow reference as mutable if already borrowed as immutable
///
/// _Dangling Pointers_ : Occur when a reference is left to a place in memory
/// that was previously freed.
///
/// Rust prevents dangling pointers at compile time
///
/// _Rules of Reference_ :
/// * At any given time, you can have either one mutable reference or any
/// number of immutable references.
/// * References must always be valid.
///
pub fn run() {
    // To prevent s1 from passing ownership, pass as a reference
    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("lenght of '{}' is {}.", s1, len);

    println!("");

    // Try to mutate s1 by via a reference
    change_immut(&s1);

    // Try to mutate s1 by via a mutable reference
    let mut s2 = String::from("hello");
    change_mut(&mut s2);

    // One reference to mut data only
    {
        let mut s = String::from("mutable hello");
        let _r1 = &mut s; // first mutable borrow
        // let _r2 = &mut s; // cannot borrow here as s has already been borrowed
    }

    // But will work if in different scopes
    let mut s3 = String::from("mutable hello");
    {
        let _r1 = &mut s3; // first mutable borrow
    }
    {
        let _r2 = &mut s3; // second mutable borrow
    }

    // Cannot mix mutability
    {
        let _r1 = &s3; // first immutable borrow
        let _r2 = &s3; // second immutable borrow
        // let _r3 = &mut s3; // Wont compile
    }


    //
    // Dangling references
    //
    // dangle();
    no_dangle();




}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change_immut(_s: &String) {
    // This will not compile
    // s.push_str(" world");
}

fn change_mut(s: &mut String) {
    // this works
    s.push_str(" world");
}


// wont compile
// fn dangle() -> &String {
//     let s = String::from("hello");
//     &s
// } returns a reference to s, but s is dropped
fn no_dangle() -> String {
    String::from("hello")
}
