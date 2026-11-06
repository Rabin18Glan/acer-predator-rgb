use super::*;
use ratatui::layout::Rect;
#[test]
fn layout_has_letters_modifiers_and_numpad_within_bounds() {
    let area = Rect::new(2, 3, 76, 6);
    let keys = keys(area);
    for label in ["Esc", "Q", "A", "Z", "Space", "Enter", "Num", "Ent"] {
        assert!(keys.iter().any(|k| k.label == label));
    }
    assert!(
        keys.iter()
            .all(|k| k.area.right() <= area.right() && k.area.bottom() <= area.bottom())
    );
    assert_eq!(keys.iter().find(|k| k.label == "Num").unwrap().zone, 3);
}
