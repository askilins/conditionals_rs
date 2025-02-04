// Returns true, if not false 
#[inline]
pub fn not (expr: bool) -> bool {
    !expr
}

// Returns true, if either left or right is true
#[inline]
pub fn either (left: bool, right: bool) -> bool {
    left || right
}

// Returns true, if both left and right are true
#[inline]
pub fn both (left: bool, right: bool) -> bool {
    left && right
}
