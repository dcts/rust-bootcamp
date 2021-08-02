use std::mem;

// Arrays => List with fixed length where elements are the same data types
// arrays are stack allocated

// ARRAY STRANGENESS:
// https://www.reddit.com/r/rust/comments/330tu1/array_slice_strangeness/

pub fn run() {
    println!("=== ARRAYS ===");

    // define array
    let fibonacci: [u8; 9] = [0,1,1,2,3,5,8,13,21];
    println!("fibonacci numbers: {:?}", fibonacci);

    // get single value
    println!("third fibonacci number: {}", fibonacci[2]);

    // iterate
    let mut index: u8 = 0;
    for val in fibonacci {
        println!("index {}: {}", index, val);
        index += 1;
    }

    // re-assign value
    let mut numbers: [i32; 5] = [1,2,3,4,5];
    println!("original array  : {:?}", numbers);
    numbers[4] = 10;
    println!("reassigned array: {:?}", numbers);

    // get array length
    println!("Array length: {}", numbers.len());

    // arrays are stack allocated
    println!("Array occupies {} bytes", mem::size_of_val(&numbers));

    // slice array
    let slice: &[i32] = &numbers[0..2];
    println!("Slice: {:?}", slice);
}
