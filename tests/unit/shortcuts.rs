use super::*;
#[test]
fn recognizes_shift_digits_and_terminal_symbols() {
    for (i, (digit, symbol)) in [('1', '!'), ('2', '@'), ('3', '#'), ('4', '$')]
        .into_iter()
        .enumerate()
    {
        assert_eq!(
            zone_shortcut(KeyCode::Char(digit), KeyModifiers::SHIFT),
            Some(i + 1)
        );
        assert_eq!(
            zone_shortcut(KeyCode::Char(symbol), KeyModifiers::NONE),
            Some(i + 1)
        );
        assert_eq!(
            zone_shortcut(KeyCode::Char(digit), KeyModifiers::NONE),
            None
        );
        assert_eq!(
            zone_shortcut(KeyCode::Char(symbol), KeyModifiers::ALT),
            None
        );
    }
}
