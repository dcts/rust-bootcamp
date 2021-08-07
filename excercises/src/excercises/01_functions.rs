pub fn run() {
    println!("=== FUNCTIONS ===");
    // EXCERCISE 1.A reverse string
    println!("=== REVERSE STRING ===");
    println!("(1) reverse_original_string");
    let mut s: String = String::from("Hello World");
    println!("     s (before call): {}", s);
    reverse_original_string(&mut s);
    println!("     s (after call) : {}", s);

    println!("\n(2) reverse_string_drop_old_value");
    let s2: String = String::from("Hello World");
    println!("     s2 (before call)   : {}", s2);
    let s2_new: String = reverse_string_drop_old_value(s2);
    println!("     s2 (after call)    : 🚧 will not compile, because value was moved (=dropped)");
    println!("     s2_new (after call): {}", s2_new);

    println!("\n(3) reverse_string_by_cloning_str_slice");
    let s3: String = "Hello World".to_string();
    println!("     s3 (before call)   : {}", s3);
    let s3_str_slice = &s3[..];
    let s3_new: String = reverse_string_by_cloning_str_slice(s3_str_slice);
    println!("     s3 (after call)    : {}", s3);
    println!("     s3_new (after call): {}", s3_new);

    // // EXCERCISE 1.B reverse string
    println!("\n=== MAX ===");
    let (x1, x2) = (-122387,124);
    println!("     max of {} and {} is {}", x1, x2, max(x1,x2));
    let (x1, x2) = (-125,-2);
    println!("     max of {} and {} is {}", x1, x2, max(x1,x2));
    let (x1, x2) = (142,2);
    println!("     max of {} and {} is {}", x1, x2, max(x1,x2));

    // // EXCERCISE 1.C sort an array
    println!("\n=== SORT ARRAY (VECTOR) ===");
    let mut my_arr: [i32; 10] = [12,34,2,4,129,1,34,1,9,62];
    println!("(1) sorting with .sort() function");
    println!("     original array               : {:?}", my_arr);
    my_arr.sort();
    println!("     original array (after sorting: {:?}", my_arr);

    println!("(2) sort_vector_drop_original (return new vector, drop old vector)");
    let vec_1: Vec<i32> = vec![1,42,514,2,3,156,28];
    println!("     original vector              : {:?}", vec_1);
    let vec_1_sorted: Vec<i32> = sort_vector_drop_original(vec_1);
    println!("     original vector (after sort) : 🚧 will not compile, because value was moved (=dropped)");
    println!("     sorted_vec                   : {:?}", vec_1_sorted);

    println!("(3) sort_vector (leave original => pass mutable reference instead of original value)");
    let mut vec_2: Vec<i32> = vec![1,42,514,2,3,156,28];
    println!("     original vector              : {:?}", vec_2);
    let vec_2_sorted: Vec<i32> = sort_vector(&mut vec_2);
    println!("     original vector (after sort) : {:?}", vec_2);
    println!("     sorted_vec                   : {:?}", vec_2_sorted);

    println!("(4) sort_vector_in_place (do not return anything)");
    let mut vec_3: Vec<i32> = vec![1,42,514,2,3,156,28];
    println!("     original vector              : {:?}", vec_3);
    sort_vector_in_place(&mut vec_3);
    println!("     original vector (after sort) : {:?}", vec_3);

}

/*
 * MUTATES ORIGINAL STRING, does not return anything
 * => hence we need to pass in a mutable reference
 * => reverse_original_string has borrowed the ownership of the string variable
 * => it does not return anything, and all memory that was used for the
 *    reversing is freed after the function reaches it's end (remember,
 *    rust has no garbage collection)
 * => uses the dereferencing operator `*`
 *    (Dereferencing is used to access the data of the reference and reassign it!)
 *    (the &mut only means that the reference is mutable, so the data can change)
 */
fn reverse_original_string(s: &mut String) {
    *s = s.chars().rev().collect::<String>();
}

/*
 * TAKES OWNERSHIP, RETURNS NEW VALUE
 * => be aware, the original string is dropped after this functions ends
 */
fn reverse_string_drop_old_value(s: String) -> String {
    s.chars().rev().collect::<String>()
}
/*
 * TAKES AN IMMUTABLE STRING SLICE
 * => clones it, reverses it and returns a new string
 */
fn reverse_string_by_cloning_str_slice(s: &str) -> String {
    s.clone().chars().rev().collect::<String>()
}

/*
 * COMPARISON Rust with Javascript
 */
// JS   => https://stackoverflow.com/questions/20740513/does-reassigning-a-javascript-variable-empty-its-previous-value-from-memory-righ
//      => no control over memory assignment
//      => garbage collector
// RUST => Memory is direclty freed.
//      => Developer can reassign memory with shadowwing, which is not possible in JS

/*
 * MAX OF 2 NUMBERS
 * => return new value
 */
fn max(x: i32, y: i32) -> i32 {
    if x > y { x } else { y }
}


/*
 * SORT A NUMERIC VECTOR
 * => here it does not make sense to use an array,
 *    because then you would need to write a function
 *    for each array length. But we normally want to input
 *    arbitrarily big arrays.
 * => thats why I chose to use a vector instead
 * => returns new Vector, DROPS original value
 */
fn sort_vector_drop_original(my_vec: Vec<i32>) -> Vec<i32> {
    let mut sorted_vector: Vec<i32> = vec![];
    for val in my_vec.iter() {
        if sorted_vector.len() == 0 {
            sorted_vector.push(*val);
        } else {
            prepend_to_vec(&mut sorted_vector, *val);
        }
    }
    sorted_vector
}
/*
 * SORT VECTOR
 * => returns new vector
 * => does not drop original value, because we input a mutable reference
 *    (original vector is borrowed and NOT destroyed at the end)
 */
fn sort_vector(my_vec: &mut Vec<i32>) -> Vec<i32> {
    let mut sorted_vector: Vec<i32> = vec![];
    for val in my_vec.iter() {
        if sorted_vector.len() == 0 {
            sorted_vector.push(*val);
        } else {
            if (sorted_vecor[sorted_vector.len()])
            prepend_to_vec(&mut sorted_vector, *val);
        }
    }
    sorted_vector
}

// HELPER FUNCTION TO PUSH TO BEGINNING OF A VECTOR
fn prepend_to_vec(my_vec: &mut Vec<i32>, value: i32) {
    let mut start_vec: Vec<i32> = vec![value];
    start_vec.append(&mut my_vec);
}
/*
 * SORT VECTOR (IN PLACE => manipulate original array)
 */
fn sort_vector_in_place(my_vec: &mut Vec<i32>) {
    let mut sorted_vector: Vec<i32> = vec![];
    for val in my_vec.iter() {
        if sorted_vector.len() == 0 {
            sorted_vector.push(*val);
        } else {
            prepend_to_vec(&mut sorted_vector, *val);
        }
    }
    *my_vec = sorted_vector;
}


