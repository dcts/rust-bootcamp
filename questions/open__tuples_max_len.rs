pub fn main() {
    // tuples only 12 max items, never specified
    // https://stackoverflow.com/q/51846320/6272061
    let short = (0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11);
    println!("\n{:?}", short); // Works fine
    // let long = (0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12);
    // println!("{:?}", long); // DOES NOT WORK
}
