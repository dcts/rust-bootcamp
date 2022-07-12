# Structs
```rust
struct Point {
  x: usize,
  y: usize,
}

struct PointTupleStruct(usize, usize);
let point_struct: Point = Point(0,5);

// tuple deconstruction
let point_tuple: (usize, usize) = (0,5);
let (x, y) = point_tuple;

// Tuple Struct Deconstructions
let PointTupleStruct (x2, y2) = point_struct;
```

# ENUMS
```rust
enum Direction {
  Up,
  Down,
}

let dir: Direction = Direction::Up;

// think ovf it as switch statement for enums
match dir {
  Direction::Up => {
    println!("direction is UP");
  }, 
  Direction::Down => {
    println!("direction is DOWN");
  },
}

// think ovf it as if statement for enums
if let Direction::Up = dir {
  println!("enum is UP");
} 
```
