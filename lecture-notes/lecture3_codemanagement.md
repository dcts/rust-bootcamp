# Code management

### Entry Points

### Modules

### Use
```rust
use my_mod; // scopes the modue for easier 
```

### Get definitions from "upper" paths
```rust
```

### Prelude convention
- prelude is a module that exposes / aliases / uses all the important functions to the current namespüace where its imported! => RUST CONVENTION, not requirement
- pub use => put all functions into the global namespace

# Generic 3 Levels of Abstraction
1. define the generic struct
2. initializing the struct => the thing becomes an instance with fixed types
3. using the instance

# Generic Contract Analogy
1. 
2. 
3. 

# Generic Market Analogy
1. vendor has box full of fruits, but they are far away. 
2. 
3. 

# Traits
- traits for getters / setters?
- trait bound => `<T: Summary>` 
  => variable can be anything that implements Summary Trait
  => 