Rust crate for aliasing bool operations

``` rust
fn not (expr: bool) -> bool {
    !expr
}

fn either (left: bool, right: bool) -> bool {
    left || right
}

fn both (left: bool, right: bool) -> bool {
    left && right
}
```

It is recomened to fork the repo instead of using this one.
( or any other way of copy / paste )

### Usage

In Cargo.toml:
```toml
[dependencies]
not_either_both = { git = "https://github.com/askilins/not_either_both_rs" }
```

In main.rs:
```rust
use not_either_both::*;

fn main () {
    if not (false) {println!("Not")};
    if either (false, true) {println!("Either")}
    if both (true, true) {println!("Both")}
}
```