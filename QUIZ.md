# Quiz Ideas for Session 1
- sort the following variables to whethere they are stack or heap allocated:
  - **STACK**
    - array
    - integer
    - &str (String literal)
    - f64, f32
    - i128, u128, i32, i64
  - **HEAP ALLOCATION**
    - tuple
    - String::from("hello world")


# Excercise Ideas
- how does one represent an array of arrays in Rust? Or use another data type?
- create a function that has 3 inputs: (1) firstName (2) lastName (3) age and returns
- function that inputs the date in string format "2021-05-12" and returns the UNIX timestamp as a number.
- implement 3x+1 (Collatz conjecture). Inputs a seed: => returns a sequence

# Topics to discuss (ideas)
- Shadowing (https://doc.rust-lang.org/book/ch03-01-variables-and-mutability.html#shadowing)
- Heap & Stack Allocation (in General and specifically in Rust)

# Quiz Mock Questions
### 1) What happens when you run this code?
- (A) prints 2 times "hello" to console
- (B) does not compile
- (C) prints "" and "hello"
```rust
fn main() {
    let s1 = "hello".to_string();
    let s2 = some_func(s1);
    println!("s1 = {}", s1);
    println!("s2 = {}", s2);
}
fn some_func(a_string: String) -> String {
    a_string
}
```

### 2) Name the two string types
- (A) String Slice and String
- (B) String Reference and mutable String Reference
- (C) mutable String and immutable String

### 3) How do you convert a variable `my_string` of type String to a string slice?
- (A) let str_slice = my_string.to_slice
- (B) let str_slice = &my_string[..];
- (C) let str_slice: &str = my_string;

### 4) Where is the "hello world" data stored?
```rust
let str: String = "hello world".to_string();
```
- (A) in the Stack Memory
- (B) in the Heap Memory
- (C) in the binary

### 5) where is the variable `str_literal` stored?
```rust
let str_literal = "hello world";
```
- (A) in the Stack Memory
- (B) in the Heap Memory
- (C) in the binary

### 6) where is the variable str stored?
```rust
let str: String = "hello world".to_string();
```
- (A) in the Stack Memory
- (B) in the Heap Memory
- (C) in the binary

