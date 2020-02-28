// Structs are custom data types/classes
// Structs/Types use CamelCase

// Traditional Struct
struct TraditionalColor { // Uppercase convention
    red: u8,
    green: u8,
    blue: u8
}

struct Person {
    first_name: String,
    last_name: String
}

impl Person {
    // Construct person
    fn new(first: &str, last: &str) -> Person {
        Person {
            first_name: first.to_string(),
            last_name: last.to_string()
        }
    }

    // Get full name
    fn full_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }

    fn set_last_name(&mut self, last: &str) {
        self.last_name = last.to_string();
    }

    fn to_tuple(self) -> (String, String) {
        (self.first_name, self.last_name)
    }
}

// Tuple Struct
struct TupleColor(u8, u8, u8);

pub fn run() {
    // Traditional Color
    let mut c = TraditionalColor {
        red: 255,
        green: 0,
        blue: 0
    };

    c.red = 200;

    println!("Color: {} {} {}", c.red, c.green, c.blue);

    // Tuple Color
    let mut c = TupleColor(255, 0, 0);

    c.0 = 200;

    println!("Color: {} {} {}", c.0, c.1, c.2);

    // Struct functions
    let mut p = Person::new("Marry", "Doe");
    println!("Person {}", p.full_name());
    p.set_last_name("Williams");
    println!("Person {}", p.full_name());
    println!("Person {:?}", p.to_tuple()); // Note that to_tuple moves p so p can no longer be referenced :(
}