pub fn not (expr: bool) -> bool {
    !expr
}

pub fn either (left: bool, right: bool) -> bool {
    left || right
}

pub fn both (left: bool, right: bool) -> bool {
    left && right
}
