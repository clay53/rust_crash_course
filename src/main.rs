mod print;
mod vars;
mod types;
mod strings;
mod tuples;
mod arrays;
mod vectors;
mod conditionals;
mod loops;
mod functions;
mod pointer_refs;
mod structs;
mod enums;
mod cli;

fn main() {
    let separator = "-------------------------";

    println!("Print Section:");
    print::run();

    println!("{}", separator);
    println!("Vars Section:");
    vars::run();

    println!("{}", separator);
    println!("Types Section:");
    types::run();
    
    println!("{}", separator);
    println!("Strings Section:");
    strings::run();

    println!("{}", separator);
    println!("Tuples Section:");
    tuples::run();

    println!("{}", separator);
    println!("Arrays Section:");
    arrays::run();
    
    println!("{}", separator);
    println!("Vectors Section:");
    vectors::run();

    println!("{}", separator);
    println!("Conditionals Section:");
    conditionals::run();

    println!("{}", separator);
    println!("Loops Section:");
    loops::run();

    println!("{}", separator);
    println!("Functions Section:");
    functions::run();

    println!("{}", separator);
    println!("Pointers/References Section:");
    pointer_refs::run();

    println!("{}", separator);
    println!("Structs Section:");
    structs::run();

    println!("{}", separator);
    println!("Enums Section:");
    enums::run();

    println!("{}", separator);
    println!("Command Line Interface Section:");
    cli::run();
}