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
