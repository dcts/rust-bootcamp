use std::mem;

pub fn run() {
    println!("=== VECTORS ===");

    // define vector
    let fibonacci: Vec<u8> = vec![0,1,1,2,3,5,8,13,21];
    println!("fibonacci numbers: {:?}", fibonacci);

    // get single value
    println!("third fibonacci number: {}", fibonacci[2]);

    // re-assign value
    let mut numbers: Vec<i32> = vec![1,2,3,4,5];
    println!("original vector  : {:?}", numbers);
    numbers[4] = 10;
    println!("reassigned vector: {:?}", numbers);

    // get vector length
    println!("Vector length: {}", numbers.len());

    // vectors are stack allocated
    // https://doc.rust-lang.org/std/vec/struct.Vec.html#guarantees
    /*
        "Most fundamentally, Vec is and always will be a (pointer, capacity, length) 
        triplet. No more, no less. The order of these fields is completely 
        unspecified, and you should use the appropriate methods to modify these. 
        The pointer will never be null, so this type is null-pointer-optimized."
    */
    println!("current vector   : {:?}", numbers);
    println!("Vector occupies  : {} bytes", mem::size_of_val(&numbers));
    numbers.push(6);
    numbers.push(7);
    numbers.push(8);
    numbers.push(9);
    numbers.push(10);
    println!("extended vector  : {:?}", numbers);
    println!("Vector occupies  : {} bytes (after push)", mem::size_of_val(&numbers));

    // pop vectors
    numbers.pop();

    // slice vector
    let slice: &[i32] = &numbers[0..2];
    println!("Slice: {:?}", slice);

    // loop through vector values
    let mut index: u8 = 0;
    for val in numbers.iter() {
        println!("index {}: {}", index, val);
        index += 1;
    }

    // loop and mutate through values
    println!("{:?}", numbers);
    for val in numbers.iter_mut() {
        *val *= -2;
    }
    println!("{:?}", numbers);
}
