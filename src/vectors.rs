// Basically resizable array
// '*' allows the mutation of the referenced variable of a reference (https://doc.rust-lang.org/1.8.0/book/references-and-borrowing.html)

use std::mem;

pub fn run() {
    let mut nums: Vec<i32> = vec![1, 2, 3, 4];

    // Re-assign value
    nums[2] = 20;

    // Add to vector
    nums.push(5);
    nums.push(6);

    // Remove (pop) last value
    nums.pop();
    
    println!("{:?}", nums);

    // Get specific val
    println!("1st val: {}", nums[0]);

    // Get Vector length
    println!("Vector Length: {}", nums.len());

    // Vector are stack allocated
    println!("Vector occupies {} bytes", mem::size_of_val(&nums));

    // Slice Vector
    let slice: &[i32] = &nums[1..3];
    println!("Slice: {:?}", slice);

    // Loop through vector
    for num in nums.iter() {
        println!("Num: {}", num);
    }

    // Loop through vector & mutate
    for num in nums.iter_mut() {
        *num *= 2; // '*' required because
    }

    println!("Nums vector: {:?}", nums);
}