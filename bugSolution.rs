fn main() {
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    vec.push(3);

    // Safe approach: Clone data before dropping the vector
    let cloned_data = vec.clone();
    drop(vec);

    // Access the cloned data
    println!("Value at index 0: {}", cloned_data[0]);
    
    // Or using iterators
    for x in cloned_data.iter(){
        println!("Value: {}", x);
    }
} 