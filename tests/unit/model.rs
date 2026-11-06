use super::*;
#[test]
fn static_sequence_matches_working_python_client() {
    let s = Settings {
        zones: [
            Rgb(255, 0, 0),
            Rgb(0, 255, 0),
            Rgb(0, 0, 255),
            Rgb(255, 255, 255),
        ],
        ..Settings::default()
    };
    assert_eq!(
        s.commands().unwrap(),
        [
            "effect 0 0 100 0 0 0 0",
            "zone 1 255 0 0",
            "zone 2 0 255 0",
            "zone 3 0 0 255",
            "zone 4 255 255 255"
        ]
    );
}
#[test]
fn all_animation_ids_and_directions_match_python() {
    for (i, effect) in Effect::ALL.into_iter().enumerate().skip(1) {
        for direction in [Direction::LeftToRight, Direction::RightToLeft] {
            let s = Settings {
                effect,
                direction,
                color: Rgb(1, 2, 3),
                ..Settings::default()
            };
            assert_eq!(
                s.commands().unwrap(),
                [format!("effect {i} 5 100 {} 1 2 3", direction.id())]
            );
        }
    }
}
#[test]
fn invalid_profiles_cannot_produce_commands() {
    assert!(
        Settings {
            brightness: 101,
            ..Settings::default()
        }
        .commands()
        .is_err()
    );
    assert!(
        Settings {
            speed: 10,
            ..Settings::default()
        }
        .commands()
        .is_err()
    );
    assert!(Rgb::try_from("#éffaa".to_owned()).is_err());
}
#[test]
fn imports_python_settings_without_importing_autostart() {
    let s: Settings = serde_json::from_str(r##"{"effect":"wave","zones":["#0050ff","#0050ff","#0050ff","#0050ff"],"color":"#0050ff","brightness":100,"speed":5,"direction":"left-to-right","restore_at_login":true}"##).unwrap();
    assert_eq!(s.effect, Effect::Wave);
    assert_eq!(
        serde_json::from_str::<Settings>(&serde_json::to_string(&s).unwrap()).unwrap(),
        s
    );
}
