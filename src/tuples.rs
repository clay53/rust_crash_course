// Group max 12 values of different types

pub fn run() {
    let person: (&str, &str, i8) = ("Clayton", "PA", 15);
    println!("{} is from {} and is {}", person.0, person.1, person.2);
}