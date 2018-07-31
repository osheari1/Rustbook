///
///
///
///
pub fn run () {

    let s = String::new();

    let data = "inital contents";

    let s = data.to_string();
    let s = "string".to_string();

    let hello = String::from("السلام عليكم");
    let hello = String::from("Dobrý den");
    let hello = String::from("Hello");
    let hello = String::from("שָׁלוֹם");
    let hello = String::from("नमस्ते");
    let hello = String::from("こんにちは");
    let hello = String::from("안녕하세요");
    let hello = String::from("你好");
    let hello = String::from("Olá");
    let hello = String::from("Здравствуйте");
    let hello = String::from("Hola");

    // Update string
    let mut s = String::from("hello");
    // Takes a string slice, ie it does not take ownership.
    s.push_str("bar");
    // Push a single character
    s.push('l');


    // Concat with + or !format
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // s1 has been moved and cannot be used.
    // println!("{}", s1); wont compile

    // Format works in the same way as println!,
    // but returns a string
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = format!("{}-{}-{}", s1, s2, s3);

    // Do not use slices to access strings
    let hello = "здравствуйте";
    let s = &hello[0..4];


    // Can iterate over strings
    // for c in "здравствуйте".bytes()  {
    for c in "здравствуйте".chars()  {
        println!("{}", c);
    }


}
