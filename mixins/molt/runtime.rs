// Shared standalone/injected runtime for the Molt native bundle.
// The first milestone only proves that the native code is called from Horizon.

pub fn block_break_message(pos: [i32; 3]) -> String {
    format!(
        "[molt] Native hook reached at {}, {}, {}",
        pos[0], pos[1], pos[2]
    )
}

pub fn before_break(pos: [i32; 3]) {
    eprintln!("{}", block_break_message(pos));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn message_preserves_signed_block_coordinates() {
        assert_eq!(
            block_break_message([-3, 64, 12]),
            "[molt] Native hook reached at -3, 64, 12"
        );
    }
}
