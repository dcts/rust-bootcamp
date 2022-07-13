fn main() {
    // EXAMPLE1
    let mut vec: Vec<i32> = Vec::new();
  
    let mut closure_fun = || { // WHY DOES CLOSURE NEED TO BE MUTABLE?!
        vec.push(1);
    };
    closure_fun();
  
    println!("{:?}", vec);

    // EXAMPLE2
    let one = || 1;
    println!("{}", one());
}
