pub fn run() {
    // Print to console
    println!("Hello from the print.rs file! 你好！");

    // Basic Formatting
    println!("{}, say \"{}\"", "Brad", "I hate Chad");

    // Positional Arguments
    println!("{0} is from {1} and {0} likes to {2}", "Clayton", "PA", "Code");

    // Named Arguments
    println!(
        "{name} likes to play {activity}",
        name = "John",
        activity = "Baseball"
    );

    // Placeholder traits
    println!("Binary: {:b} Hex: {:x} Octal: {:o}", 10, 10, 10);

    // Placeholder for debug trait (tuple)
    println!("{:?}", (12, true, "hello"));

    // BASIC MATH
    println!("10 + 10 = {}", 10 + 10);
}