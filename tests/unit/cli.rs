use super::Command;

fn parse(args: &[&str]) -> Result<Command, String> {
    Command::parse(&args.iter().map(|arg| (*arg).to_owned()).collect::<Vec<_>>())
}

#[test]
fn no_arguments_selects_interactive_mode() {
    assert_eq!(parse(&[]).unwrap(), Command::Interactive);
}

#[test]
fn supported_flags_map_to_commands() {
    for (flag, command) in [
        ("--help", Command::Help),
        ("-h", Command::Help),
        ("--status", Command::Status),
        ("--dry-run", Command::DryRun),
        ("--apply", Command::Apply),
    ] {
        assert_eq!(parse(&[flag]).unwrap(), command);
    }
}

#[test]
fn rejects_unknown_flags_and_multiple_commands() {
    for args in [
        vec!["--unknown"],
        vec!["--status", "--apply"],
        vec!["--help", "extra"],
    ] {
        assert!(parse(&args).is_err());
    }
}
