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

// Returns true, if all are true
#[inline]
pub fn all (list: &[bool]) -> bool {
    for condition in list {
        if not (*condition) {
            return false
        }
    }
    true
}

// Returns true, if none are true
#[inline]
pub fn none (list: &[bool]) -> bool {
    for condition in list {
        if *condition {
            return false
        }
    }
    true
}

// Returns true, if any are true
#[inline]
pub fn any (list: &[bool]) -> bool {
    for condition in list {
        if *condition {
            return true
        }
    }
    false
}
