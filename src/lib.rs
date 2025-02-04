// Returns true, if not false 
pub fn not (expr: bool) -> bool {
    !expr
}

// Returns true, if either left or right is true
pub fn either (left: bool, right: bool) -> bool {
    left || right
}

// Returns true, if both left and right are true
pub fn both (left: bool, right: bool) -> bool {
    left && right
}
