# Dictionaries
### Simple relational key-value storage structure in Rust.

---

To use this library in your code, add the following to your `Cargo.toml`:
```toml
[dependencies]
dictionaries = { git = "https://github.com/Raforawesome/dictionaries-rs" }
```

#### Usage:
```rust
use dictionaries::{Dictionary, RemoveError};

fn main() {
    let dict: Dictionary = Dictionary::new();
    
    // Set function requires owned Strings
    dict.set(key: String, value: String);
    
    // Get returns an option containing a reference to the value
    // Also takes string slices/String references as an arg, not owned strings
    let result: Option<&String> = dict.get(key: &str);
    
    // Similar to Dictionary.get(), but gets a key from a value
    let result: Option<&String> = dict.get_key(value: &str);
    
    // Removes a key-value pair when given a key
    let result: Result<(), RemoveError> = dict.remove(key: &str);
    
    
    // This library also exposes the key and value vectors
    // So you can write custom logic
    let keys: Vec<String> = dict.keys;
    let values: Vec<String> = dict.values;
    
    // Dictionaries can also be iterated through
    for (k, v) in dict.iter() {
        println!("{}, {}", k, v);
    }
}
```