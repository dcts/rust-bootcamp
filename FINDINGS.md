# Interesting Stuff I learned along the way

### Printing enums or structs to the console with the debug trait
If you want to print an Enum or a Struct to the console with the following code, you need to add the line `#[derive(Debug)]` above the declaration of the enum or struct:
```rust
#[derive(Debug)]
enum State {
  Dead,
  Alive
}

#[derive(Debug)]
struct Person {
  first_name: &str,
}

fn main() {
  let my_state: State = State::Dead;
  println!("{:?}", my_state);  // this is only possible because State derives the Debug Trait (whatever that means lol)

  let frank: Person = Person {
    first_name: "frankie",
  }
  println!("{:?}", frank); // this is only possible because Person derives the Debug Trait (whatever that means lol)
}
```

### Cargo usefull tipp
- download and install `cargo-edit` to be able to add packages on the go with a new command: `cargo add <package-name>`.

### Splitting Strings, you have to collect them first
```rust
let mut split = "some string 123 ffd".split("123");
let vec: Vec<&str> = split.collect();
```
