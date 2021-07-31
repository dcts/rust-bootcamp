pub fn run() {
    let x = 12;  // integer
    let y = 2.0; // Float
    let name = "Thomas"; // character type
    let boolean = true; // boolean
    let arr = [1,2,3,4];
    let my_tuple = (1,2,"myString",4);
    println!("Hello, {}. X is {}, y is {}, boolean is {}", name, x, y, boolean);
    println!("ArrayValues: {},{},{},{}", arr[0], arr[1], arr[2], arr[3]);
    println!("tupleValues: {},{},{},{}", my_tuple.0, my_tuple.1, my_tuple.2, my_tuple.3);
}
