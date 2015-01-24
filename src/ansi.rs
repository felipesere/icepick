use fake_tty::FakeIO;
use tty::IO;

struct Ansi<'a> {
    io: Box<(IO + 'a)>,
}

impl <'a> Ansi<'a> {
    pub fn escape(&mut self, message: &str) {
        self.io.write("\x1b[something");
    }
}

#[cfg(test)]

#[test]
fn it_escapes_a_str() {
    let mut ansi = Ansi { io: Box::new(FakeIO::new()) };
    ansi.escape("something");
    let inner_box = ansi.io;
    assert_eq!(inner_box.last(), "\x1b[something");
}
