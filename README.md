## About

Rust crate for aliasing boolean operations.  
It is recomened to fork the repo instead of using this one.
( or any other way of copy / paste )

## Usage

### Crate Method:

In Cargo.toml:
```toml
[dependencies]
not_either_both = { git = "https://github.com/askilins/not_either_both_rs" }
```

In main.rs:
```rust
use not_either_both::*;

fn main () {
    if not (false) {println!("Not")}
    if either (false, true) {println!("Either")}
    if both (true, true) {println!("Both")}
    if all (&[true, true, true]) {println!("All")}
    if none (&[false, false, false]) {println!("None")}
    if any (&[false, false, true]) {println!("Any")}
}
```

### Module Method:

Copy contents of this repo's `/src/lib.rs` into your projects `/src/not_either_both/mod.rs`

In main.rs:
```rust
mod not_either_both::*;

fn main () {
    if not (false) {println!("Not")}
    if either (false, true) {println!("Either")}
    if both (true, true) {println!("Both")}
    if all (&[true, true, true]) {println!("All")}
    if none (&[false, false, false]) {println!("None")}
    if any (&[false, false, true]) {println!("Any")}
}
```
