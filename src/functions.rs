// Functions - reuse blocks of code

pub fn run() {
    greeting("Hola", "Jane");
    
    // Bind function values to variables
    let get_sum = add(5, 5);
    println!("Sum: {}", get_sum);

    // Closure - Basically just functions that can have local variables as well as PIPED variables
    let n3 = 10;
    let add_nums = |n1: i32, n2: i32| n1 + n2 + n3; // This has nothing to do with the add function
    println!("C Sum: {}", add_nums(3, 3));
}

fn greeting(greet: &str, name: &str) {
    println!("{} {}, nice to mee you!", greet, name);
}

fn add(n1: i32, n2: i32) -> i32 {
    return n1 + n2;
}