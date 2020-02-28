// Conditionals - If and stuff
// I made this one spicier (Must install rand dependency)

pub fn run() {
    let age: u8 = if rand::random() {15} else {21};
    let knows_person_of_age = rand::random();
    let check_id: bool = rand::random();

    println!("You're {}.", age);

    if knows_person_of_age {
        println!("* You walk into your favorite bar. *");
    } else {
        println!("* You walk into a bar you've never been in before. *")
    }
    
    if check_id && knows_person_of_age == false {
        println!("* The bartender checks your ID *");
    }

    if age >= 21 && check_id || knows_person_of_age {
        println!("Bartender: You want a drink?");
    } else if age < 21 && check_id {
        println!("Bartender: Bruh, get out.");
    } else {
        println!("Bartender: Show me that ID boi.");
    }

    // Shorthand If
    let is_of_age = if age >= 21 {true} else {false};
    println!("Is of age? {}", is_of_age);
}