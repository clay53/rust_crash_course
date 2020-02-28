// Primitive str
// String = Growable

pub fn run() {
    let mut hello = String::from("Hello"); // Mutable

    // Get length (works for both types)
    println!("Length: {}", hello.len());

    // Adding
    hello.push('!'); // char
    hello.push_str(" 大家好！"); // string

    // Check capacity
    println!("Capacity: {}", hello.capacity());

    // Is Empty?
    println!("Is Empty: {}", hello.is_empty());

    // Contains string
    println!("Contains '大家' {}", hello.contains("大家"));

    // Replace
    println!("Replace: {}", hello.replace("大家", "你们"));

    // Loop through by whitespace
    for word in hello.split_whitespace() {
        println!("{}", word);
    }

    // Create string w/ capacity
    let mut s = String::with_capacity(10);
    s.push('a');
    s.push('b');
    println!("{}", s);

    // Assertion testing - If not true DIE DIE DIE DIE DIE DIE DIE DIE DIE DIE DIE
    assert_eq!(2, s.len());
    assert_eq!(10, s.capacity());

    println!("{}", hello);
}