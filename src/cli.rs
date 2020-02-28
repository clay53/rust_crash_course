use std::env;

pub fn run() {
    let args: Vec<String> = env::args().collect();
    let command = args[1].clone();
    let name = "User";
    let status = "0%";

    println!("Args: {:?}", args);
    println!("Command: {}", command);

    if command == "hello" {
        println!("你好{}， 今天你怎么样？", name); // Hello {name}, how are you today?
    } else if command == "status" {
        println!("Status is {}", status);
    } else {
        println!("Unknown command");
    }
}