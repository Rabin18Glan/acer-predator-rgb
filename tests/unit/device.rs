use super::*;
struct Mock {
    commands: Vec<String>,
    fail_at: usize,
}
impl Transport for Mock {
    fn command(&mut self, c: &str) -> Result<(), Error> {
        self.commands.push(c.into());
        Ok(())
    }
    fn status(&mut self) -> Result<Status, Error> {
        Ok(Status {
            method: if self.commands.len() == 1 { 20 } else { 6 },
            code: i32::from(self.commands.len() == self.fail_at),
            restored: 0,
        })
    }
}
#[test]
fn stops_on_firmware_rejection_instead_of_claiming_success() {
    let mut mock = Mock {
        commands: vec![],
        fail_at: 2,
    };
    assert!(apply(&Settings::default(), &mut mock).is_err());
    assert_eq!(mock.commands.len(), 2);
}
#[test]
fn sends_all_zones_on_success() {
    let mut mock = Mock {
        commands: vec![],
        fail_at: 99,
    };
    assert!(apply(&Settings::default(), &mut mock).is_ok());
    assert_eq!(mock.commands.len(), 5);
}
#[test]
fn distinguishes_initial_status_from_rejection() {
    let s = Status::parse("last_method=0 last_status=-1 restored_after_sleep=0\n").unwrap();
    assert!(s.describe().contains("no commands"));
    assert!(Status::parse("garbage").is_err());
}
#[test]
fn short_write_is_not_retried() {
    struct Short(usize);
    impl Write for Short {
        fn write(&mut self, _: &[u8]) -> io::Result<usize> {
            self.0 += 1;
            Ok(1)
        }
        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }
    let mut writer = Short(0);
    assert!(write_record(&mut writer, b"effect 0 0 100 0 0 0 0").is_err());
    assert_eq!(writer.0, 1);
}
