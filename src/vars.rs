// Vars hold primitives & references
// Vars are default immutable
// Rust is block-scoped

pub fn run() {
    let name = "Clayton";
    let mut age = 15;

    age = age+1;

    println!("My name is {} and I will be {}", name, age);

    // Define constant
    const ID: i32 = 1;
    println!("ID: {}", ID);

    // Multiple variables at once
    let ( my_name, my_age) = ("Clayton", 15);
    println!("{} is {}", my_name, my_age);
}