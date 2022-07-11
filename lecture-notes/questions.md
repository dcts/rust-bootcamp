# Coding Questions
### HOW TO SET THE TYPE OF THE ITERATOR "num" PROPERLY?
```rust
pub fn factorial(n: u32) -> u32 {
    if n == 0 { 
        return 0; 
    }
    let mut product: u32 = 1;
    for num in 1..=n {
        product = product * num;
    }
    return product;
}
```
### IntoIter VS Iter vs regular

```rust
let num: [u32] = []
for num in array.into_iter() {
    
}
// array.into_iter() VS without using an iterator at all 
```
