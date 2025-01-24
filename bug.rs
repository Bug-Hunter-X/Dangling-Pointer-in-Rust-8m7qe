fn main() {
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    vec.push(3);

    let ptr = vec.as_ptr();

    // Drop the vector, invalidating the pointer
    drop(vec);

    // Using the invalid pointer leads to undefined behavior
    println!("Value at pointer: {}", unsafe { *ptr });
}