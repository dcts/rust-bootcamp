// Reference pointers - point to a ressource in memory

pub fn run() {
    println!("=== POINTER REFERENCES ===");

    // primitive array
    let arr1 = [1,2,3];
    let arr2 = arr1;
    println!("{:?}", (arr1, arr2));

    // with non-primitives, if you assign another variable to a piece of data, the first variable will no longer hold that value. You'll need to use a reference (&) to point to the ressource
    let vec1 = vec![1,2,3];
    let vec2 = &vec1;
    println!("{:?}", (&vec1, vec2));
}
