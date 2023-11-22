# Hashell

Implementation of hashing function made by [Mark_Rus-Scratch-Lab](https://scratch.mit.edu/users/Mark_Rus-Scratch-Lab/) in Rust.
Some computations of this function were omitted for sake of performance, which means it can fail in certain untested scenarios or simply produce a different hash.

## Example

```rust
use hashell::hash_string;

fn main() {
    assert_eq!(hash_string("some value", 16), "5149710603511119".to_owned());
}
```
