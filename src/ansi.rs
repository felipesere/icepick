use fake_tty::FakeIO;
use tty::IO;

struct Ansi<'a> {
    io: Box<(IO + 'a)>,
}

impl <'a> Ansi<'a> {
    pub fn escape(&mut self, message: &str) {
        let out = format!("\x1b[{}", message);
        self.io.write(out.as_slice());
    }

    pub fn clear(&mut self) {
        self.escape("2J");
    }

    fn hide_cursor(&mut self) {
        self.escape("?251");
    }

    fn show_cursor(&mut self) {
        self.escape("?25h");
    }

    fn set_position(&mut self, line: isize, column: isize) {
        let message = format!("{};{}H", line+1, column+1);
        self.escape(message.as_slice());
    }

    fn invert(&mut self) {
        self.escape("7m");
    }

    fn reset(&mut self) {
        self.escape("0m");
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

#[test]
fn it_hides_the_cursos() {
    let mut ansi = Ansi { io: Box::new(FakeIO::new()) };

    ansi.hide_cursor();
    let inner_box = ansi.io;
    assert_eq!(inner_box.last(), "\x1b[?251");
}

#[test]
fn it_shows_the_cursor() {
    let mut ansi = Ansi { io: Box::new(FakeIO::new()) };

    ansi.show_cursor();
    let inner_box = ansi.io;
    assert_eq!(inner_box.last(), "\x1b[?25h");
}

#[test]
fn it_sets_the_position() {
    let mut ansi = Ansi { io: Box::new(FakeIO::new()) };

    ansi.set_position(8,12);
    let inner_box = ansi.io;
    assert_eq!(inner_box.last(), "\x1b[9;13H");
}

#[test]
fn it_inverts() {
    let mut ansi = Ansi { io: Box::new(FakeIO::new()) };

    ansi.invert();
    let inner_box = ansi.io;
    assert_eq!(inner_box.last(), "\x1b[7m");
}

#[test]
fn it_resets() {
    let mut ansi = Ansi { io: Box::new(FakeIO::new()) };

    ansi.reset();
    let inner_box = ansi.io;
    assert_eq!(inner_box.last(), "\x1b[0m");
}

