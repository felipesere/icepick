use crate::tty::IO;

pub struct Ansi<'a> {
    pub io: Box<(dyn IO + 'a)>,
}

impl<'a> Ansi<'a> {
    pub fn escape(&mut self, message: &str) {
        let out = Ansi::esc(message);
        self.io.write(out.as_ref());
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
        self.escape(message.as_ref());
    }

    pub fn inverted(&mut self, line: &str) {
        let compound = format!("{}{}{}", Ansi::esc("7m"), line, Ansi::esc("0m"));
        self.io.write(compound.as_ref());
    }

    pub fn print(&mut self, line: &str) {
        self.io.write(line);
    }

    pub fn blank_line(&mut self, line: usize) {
        self.set_position(line, 0);
        self.escape("2K");
    }
}
