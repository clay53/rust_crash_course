// Referece Pointers - Point to another resource

pub fn run() {
    // Primitive Array
    let arr1 = [1,2,3];
    let arr2 = arr1;

    println!("Values: {:?}", (arr1, arr2));

    // If another variable is assigned to a previously assigned non-primitive, the old variable will lose its value. So, a '&' must be used to point to the old variable instead to borrow the reference.

    // Vector (non-primitive)
    let vec1 = vec![1,2,3];
    let vec2 = &vec1;

    println!("Values: {:?}", (&vec1, vec2)); // Must make reference to vec1 because vec2 is still borrowing it
    println!("Values: {:?}", (vec2)); // Last reference of borrowing variable - unborrows vec1
    println!("Values: {:?}", (vec1));
}