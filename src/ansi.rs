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
        let message = format!("{};{}H", line + 1, column + 1);
        self.escape(message.as_slice());
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
mod tests {
    use fake_tty::FakeIO;
    use super::*;

    fn assert_results_in<F: FnMut(&mut Ansi)> (expected: &str, mut f: F) {
        let mut ansi = Ansi { io: Box::new(FakeIO::new()) };

        f(&mut ansi);
        let inner_box = ansi.io;
        assert_eq!(inner_box.last(), expected);
    }

    #[test]
    fn it_escapes_a_str() {
        assert_results_in("\x1b[something", |ansi| { ansi.escape("something") });
    }

    #[test]
    fn it_clears_the_screen() {
        assert_results_in("\x1b[2J", |ansi| { ansi.clear() });
    }

    #[test]
    fn it_hides_the_cursor() {
        assert_results_in("\x1b[?251", |ansi| { ansi.hide_cursor() });
    }

    #[test]
    fn it_shows_the_cursor() {
        assert_results_in("\x1b[?25h", |ansi| { ansi.show_cursor() });
    }

    #[test]
    fn it_sets_the_position() {
        assert_results_in("\x1b[9;13H", |ansi| { ansi.set_position(8,12); });
    }

    #[test]
    fn it_prints_inverted() {
        assert_results_in("\x1b[7mtest\x1b[0m", |ansi| { ansi.inverted("test"); });
    }
}
