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

