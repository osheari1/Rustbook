/// **General**
///
/// _IpAddr_ : Has built in Ip Addr funcitonality
///
/// Can define methods on structs with _impl_
///
/// If declaring an Option type as None, must specify <T>
///

enum IpAddrKind {
    V4,
    V6
}

enum IpAddr {
    V4(String),
    V6(String)
}

enum IpAddr2 {
    V4(u8, u8, u8, u8),
    V6(String)
}

enum Message {
    Quit,
    Move { x: i32, y: i32}, // Anonymous struct
    Write(String),
    ChangeColor(i32, i32, i32)
}

impl Message {
    fn call(&self) {}
}



pub fn run() {
    let _four = IpAddr::V4;
    let _six = IpAddr::V6;

    let _home = IpAddr::V4(String::from("127.0.0.1"));
    let _loopback = IpAddr::V6(String::from("::1"));

    let m = Message::Write(String::from("Something"));
    m.call();
}


fn options() {
    let some_number = Some(5);
    let soem_string = Some("a string");

    let absent_number: Option<i32> = None;
}




