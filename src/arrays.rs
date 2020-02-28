// Arrays are fixed and same type
// '&' makes a reference to a variable

use std::mem;

pub fn run() {
    let mut nums: [i32; 4] = [1, 2, 3, 4];

    // Re-assign value
    nums[2] = 20;
    
    println!("{:?}", nums);

    // Get specific val
    println!("1st val: {}", nums[0]);

    // Get Array length
    println!("Array Length: {}", nums.len());

    // Arrays are stack allocated
    println!("Array occupies {} bytes", mem::size_of_val(&nums));

    // Slice Array
    let slice: &[i32] = &nums[0..2];
    println!("Slice: {:?}", slice);
}