// Returns true, if not false 
#[inline(always)]
pub fn not (expr: bool) -> bool {
    !expr
}

// Returns true, if either left or right is true
#[inline(always)]
pub fn either (left: bool, right: bool) -> bool {
    left || right
}

// Returns true, if both left and right are true
#[inline(always)]
pub fn both (left: bool, right: bool) -> bool {
    left && right
}
