use fake_tty::FakeIO;
use tty::IO;

struct Ansi<'a> {
    io: Box<(IO + 'a)>,
}

impl <'a> Ansi<'a> {
    pub fn escape(&mut self, message: &str) {
        let mut esc = "\x1b[".to_string();
        esc.push_str(message);
        self.io.write(esc.as_slice());
    }

    pub fn clear(&mut self) {
        self.escape("2J");
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

#[test]
fn it_clears_the_screen() {
    let mut ansi = Ansi { io: Box::new(FakeIO::new()) };
    ansi.clear();
    let inner_box = ansi.io;
    assert_eq!(inner_box.last(), "\x1b[2J");
}
