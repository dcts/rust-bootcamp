// move_semantics2.rs
// Make me compile without changing line 13 or moving line 10!
// Execute `rustlings hint move_semantics2` for hints :)

// SOLUTIONS:
// (a) => clone vec0 so its not dropped after fill_vec (vec0.clone())
// (b) => make fill_vec borrow its argument instead of taking ownership
// (c) => make fill_vec mutably borrow its argument (BEST SOLUTION I THINK)
fn main() {
    let mut vec0 = Vec::new();
    fill_vec(&mut vec0);

    // Do not change the following line!
    println!("{} has length {} content `{:?}`", "vec0", vec0.len(), vec0);

    vec0.push(88);

    println!("{} has length {} content `{:?}`", "vec1", vec0.len(), vec0);
}

fn fill_vec(vec: &mut Vec<i32>) {
    vec.push(22);
    vec.push(44);
    vec.push(66);
}
