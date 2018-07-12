/// *General*
///
/// Define a struct with the 'Struct' keyword
///
/// ```rust
/// struct User {
/// _username: String,
/// email: String,
/// _sign_in_count: u64,
/// _active: bool}
/// ```
///
/// Can change fields by calling instance.field = ...., if the entire
/// object is mutable.
///
/// _field init shorthand_ : When function parameters and struct field names
/// are the same, they do not have to be explicitly passed to the struct
///
/// #### Example
/// ```rust
///
/// struct User {
///     _username: String,
///     email: String,
///     _sign_in_count: u64,
///     _active: bool,
/// }
/// fn build_user(email: String, _username: String) -> User {
///     User { email, _username, _active: true, _sign_in_count: 1}
/// }
/// ```
/// _.._ : Struct update syntax
///
/// _Tuple Structs_ : Structs without name fields
/// ```rust
/// struct Color(i32, i32, i32);
/// let black = Color(0, 0, 0);
/// ```
///
/// _Unit structs_ : Structs without any fields
///
///
///
struct User {
    _username: String,
    email: String,
    _sign_in_count: u64,
    _active: bool,
}
struct Color(i32, i32, i32);

pub fn run() {
    let _user1 = User {
        email: String::from("derp@gmail.com"),
        _username: String::from("somename1"),
        _active: true,
        _sign_in_count: 1,
    };

    let mut _user2 = User {
        email: String::from("derp@gmail.com"),
        _username: String::from("somename1"),
        _active: true,
        _sign_in_count: 1,
    };

    // wont compile
    // user1.email = String::from("derp2@gmail.com");

    _user2.email = String::from("derp2@gmail.com");

    // Init syntax
    let _user3 = build_user(String::from("hi@gmail.com"), String::from("four"));

    // Update syntax
    let _user4 = User {
        email: String::from("example@gmail.com"),
        _username: String::from("example2"),
        .._user3
    };

    let _black = Color(0, 0, 0);
}

fn build_user(email: String, _username: String) -> User {
    User {
        email,
        _username,
        _active: true,
        _sign_in_count: 1,
    }
}
