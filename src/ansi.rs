use fake_tty::FakeIO;
use tty::IO;

pub struct Ansi<'a> {
    pub io: Box<(IO + 'a)>,
}

impl <'a> Ansi<'a> {
    pub fn escape(&mut self, message: &str) {
        let out = Ansi::esc(message);
        self.io.write(out.as_slice());
    }

    fn esc(input: &str) -> String {
        format!("\x1b[{}", input)
    }

    pub fn clear(&mut self) {
        self.escape("2J");
    }

    pub fn hide_cursor(&mut self) {
        self.escape("?251");
    }

    pub fn show_cursor(&mut self) {
        self.escape("?25h");
    }

    pub fn set_position(&mut self, line: usize, column: usize) {
        let message = format!("{};{}H", line+1, column+1);
        self.escape(message.as_slice());
    }

    fn invert(&mut self) {
        self.escape("7m");
    }

    fn reset(&mut self) {
        self.escape("0m");
    }

    pub fn inverted(&mut self, line: &str) {
        let compound = format!("{}{}{}", Ansi::esc("7m"), line, Ansi::esc("0m"));
        self.io.write(compound.as_slice());
    }

    pub fn print(&mut self, line: &str) {
        self.io.write(line);
    }

    pub fn blank_line(&mut self, line: usize) {
        self.set_position(line, 0);
        self.escape("2K");
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

fn it_prints_inverted() {
    let mut ansi = Ansi { io: Box::new(FakeIO::new()) };

    ansi.inverted("test");
    let inner_box = ansi.io;
    assert_eq!(inner_box.last(), "\x1b[7mtest\x1b[0m");
}

#[test]
fn it_resets() {
    let mut ansi = Ansi { io: Box::new(FakeIO::new()) };

    ansi.reset();
    let inner_box = ansi.io;
    assert_eq!(inner_box.last(), "\x1b[0m");
}
