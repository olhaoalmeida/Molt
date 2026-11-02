// Smoke hook for the initial native bundle. It leaves Horizon's method unchanged.
fn before_break(pos: [i32; 3]) {
    crate::game::molt::before_break(pos);
}
