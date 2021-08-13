pub fn run() {
    println!("=== FUNCTIONS ===");
    // EXCERCISE 1.A reverse string
    println!("=== (A) REVERSE STRING ===");
    println!("...reverse_original_string");
    let mut s: String = String::from("Hello World");
    println!("     s (before call): {}", s);
    reverse_original_string(&mut s);
    println!("     s (after call) : {}", s);

    println!("...reverse_string_drop_old_value");
    let s2: String = String::from("Hello World");
    println!("     s2 (before call)   : {}", s2);
    let s2_new: String = reverse_string_drop_old_value(s2);
    println!("     s2 (after call)    : 🚧 will not compile, because value was moved (=dropped)");
    println!("     s2_new (after call): {}", s2_new);

    println!("...reverse_string_by_cloning_str_slice");
    let s3: String = "Hello World".to_string();
    println!("     s3 (before call)   : {}", s3);
    let s3_str_slice = &s3[..];
    let s3_new: String = reverse_string_by_cloning_str_slice(s3_str_slice);
    println!("     s3 (after call)    : {}", s3);
    println!("     s3_new (after call): {}", s3_new);

    // EXCERCISE 1.B reverse string
    println!("\n=== (B) MAX ===");
    let (x1, x2) = (-122387,124);
    println!("     max of {} and {} is {}", x1, x2, max(x1,x2));
    let (x1, x2) = (-125,-2);
    println!("     max of {} and {} is {}", x1, x2, max(x1,x2));
    let (x1, x2) = (142,2);
    println!("     max of {} and {} is {}", x1, x2, max(x1,x2));

    // EXCERCISE 1.C sort an array / vector
    println!("\n=== (C) SORT ARRAY (=> better to use vector) ===");
    println!("...sort_vector_drop_original (return new vector, drop old vector)");
    let vec_1: Vec<i32> = vec![1,42,514,2,3,156,28];
    println!("     original vector              : {:?}", vec_1);
    let vec_1_sorted: Vec<i32> = sort_vector_drop_original(vec_1);
    println!("     original vector (after sort) : 🚧 will not compile, because value was moved (=dropped)");
    println!("     sorted_vec                   : {:?}", vec_1_sorted);

    println!("...sort_vector (leave original => pass reference)");
    let vec_2: Vec<i32> = vec![1,42,514,2,3,156,28];
    println!("     original vector              : {:?}", vec_2);
    let vec_2_sorted: Vec<i32> = sort_vector(& vec_2);
    println!("     original vector (after sort) : {:?}", vec_2);
    println!("     sorted_vec                   : {:?}", vec_2_sorted);

    println!("...sort_vector_in_place (do not return anything)");
    let mut vec_3: Vec<i32> = vec![1,42,514,2,3,156,28];
    println!("     original vector              : {:?}", vec_3);
    sort_vector_in_place(&mut vec_3);
    println!("     original vector (after sort) : {:?}", vec_3);

    // EXCERCISE 1.D remove character from string (in place)
    println!("\n=== (D) REMOVE CHAR OF STRING ===");
    let mut my_string = "Hello World".to_string();
    println!("my_string (before): {}", my_string);
    remove_char_in_place(&mut my_string, 'H');
    println!("my_string (after): {}", my_string);

    // EXCERCISE 1.E sum all numbers of an array
    println!("\n=== (E) SUM ALL NUMERS OF ARRAY ===");
    let vec_1: Vec<i32> = vec![1,2,3,4,5];
    println!("arr: {:?}. SUM = {}", vec_1, sum(&vec_1)); // => 15
    let vec_2: Vec<i32> = vec![829,-12,758,2];
    println!("arr: {:?}. SUM = {}", vec_2, sum(&vec_2)); // => 1577

    // EXCERCISE 1.F chunk an array into an array of arrays
    println!("\n=== (F) CHUNK ARRAY INTO ARRAY CHUNKS ===");
    let vec: Vec<usize> = vec![1,2,3,4,5,6,7,8,9,10];
    println!("vec: {:?} => chunks: {:?}", vec, chunk_vector(&vec, 3));

    // EXCERCISE 1.G createDict out of array
    println!("\n=== (G) CREATE DICT (count occurencies) ===");

    // EXCERCISE 1.H multiply each element by x
    println!("\n=== (H) MULTIPLY BY X ===");
    let target_vec: Vec<i32> = multiply_by_x(&vec![1,2,3,4,5], 10);
    println!("{:?}", target_vec);

    // EXCERCISE 1.I FLATTEN ARRAY
    println!("\n=== (I) FLATTEN ARRAY OF ARRAYS ===");
    let vec_1 = vec![1,2,3];
    let vec_2 = vec![4,5,6];
    let target_vec: Vec<i32> = flat(&vec_1, &vec_2);
    println!("{:?}, {:?}, flat() => {:?}", vec_1, vec_2, target_vec)
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
 * JS   => https://stackoverflow.com/questions/20740513/does-reassigning-a-javascript-variable-empty-its-previous-value-from-memory-righ
 *      => no control over memory assignment
 *      => garbage collector
 * RUST => Memory is direclty freed.
 *      => Developer can reassign memory with shadowwing, which is not possible in JS
 */



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
    let mut vector_sorted: Vec<i32> = my_vec.clone();
    vector_sorted.sort();
    vector_sorted
}
/*
 * SORT VECTOR
 * => returns new vector
 * => does not drop original value, because we input a reference
 *    (original vector is borrowed and NOT destroyed at the end)
 */
fn sort_vector(my_vec: &Vec<i32>) -> Vec<i32> {
    let mut vector_sorted: Vec<i32> = my_vec.clone();
    vector_sorted.sort();
    vector_sorted
}
 /*
  * SORT VECTOR IN PLACE
  * maniuplates original vector
  * does not return anything
  */
fn sort_vector_in_place(my_vec: &mut Vec<i32>) {
    my_vec.sort();
}

/*
 * REMOVE CHARACTER FROM STRING
 */
fn remove_char_in_place(string: &mut String, char: char) {
    let mut new_string = String::from("");
    for c in string.chars() {
        if c != char {
            new_string.push(c);
        }
    };
    *string = new_string;
}
/*
 * SUM OF ARRAY NUMBERS
 */
fn sum(vector: &Vec<i32>) -> i32 {
    let mut sum: i32 = 0;
    for v in vector.iter() {
        sum += v;
    }
    sum
}

/*
 * CHUNK ARRAY (into batches of size n)
 */
fn chunk_vector(vec: &Vec<usize>, size: usize) -> Vec<Vec<usize>> {
    let mut chunks: Vec<Vec<usize>> = Vec::new();
    let mut indx: usize = 0;
    while indx < vec.len() {
        let mut chunk: Vec<usize> = Vec::new();
        while chunk.len() < size && indx < vec.len() {
            chunk.push(vec[indx]);
            indx += 1;
        }
        chunks.push(chunk.clone());
        println!("{}", indx);
    }
    chunks
}

/*
 * MULTIPLY ARRAY ELEMENTS BY X
 */
fn multiply_by_x(numbers: &Vec<i32>, x: i32) -> Vec<i32> {
    numbers.into_iter().map(|num| num * x).collect()
}

/*
 * FLATTEN ARRAY OF ARRAYS
 */
fn flat(vec_1: &Vec<i32>, vec_2: &Vec<i32>) -> Vec<i32> {
    let mut vec_flat = Vec::new();
    vec_flat.append(&mut vec_1.clone());
    vec_flat.append(&mut vec_2.clone());
    vec_flat
}
