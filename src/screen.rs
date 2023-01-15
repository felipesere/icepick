use crate::ansi::Ansi;
use crate::fake_tty::FakeIO;
use crate::renderer::Renderer;
use crate::search::Search;
use crate::text::Printable;
use crate::text::Text;
use crate::tty::TTY;
use std::cmp::min;

pub struct Screen<'a> {
    pub ansi: Ansi<'a>,
    pub height: usize,
    pub width: usize,
}

impl<'a> Screen<'a> {
    pub fn new() -> Screen<'a> {
        let ansi = Ansi {
            io: Box::new(TTY::new()),
        };
        let (width, height) = ansi.io.dimensions();
        Screen {
            ansi,
            height,
            width: width - 1,
        }
    }

    pub fn fake() -> Screen<'a> {
        Screen::fake_with_input(vec![])
    }

    pub fn fake_with_input(input: Vec<&str>) -> Screen<'a> {
        Screen {
            ansi: Ansi {
                io: Box::new(FakeIO::new_with_input(input)),
            },
            height: 20,
            width: 10,
        }
    }

    pub fn handle_keystroke(&self, search: Search<'a>, input: &str) -> Search<'a> {
        match input {
            "\u{e}" => search.down(),
            "\u{10}" => search.up(),
            "\u{7f}" => search.backspace(),
            "\n" => search.done(),
            _ => search.append_to_search(input),
        }
    }

    pub fn print(&mut self, search: &Search) {
        let renderer = Renderer;
        let result = renderer.render(search);
        self.ansi.hide_cursor();

        let start_line = self.height - search.visible_limit;

        for (idx, text) in result.into_iter().enumerate() {
            self.write(start_line + idx, text);
        }
        self.ansi
            .set_position(start_line, renderer.header(search).len());
        self.ansi.show_cursor();
    }

    pub fn write(&mut self, line: usize, text: Text) {
        self.ansi.blank_line(line);
        self.ansi.set_position(line, 0);

        text.print(&mut self.ansi);
    }

    pub fn clear(&mut self, lines: usize) {
        for _ in 0..lines {
            self.ansi.print("\n");
        }
    }

    pub fn reset(&mut self) {
        self.ansi.io.reset();
    }

    pub fn move_cursor_to_end(&mut self) {
        self.ansi.set_position(self.height - 1, 0);
    }

    pub fn run_search(
        &mut self,
        lines: Vec<String>,
        initial_query: Option<String>,
    ) -> Option<String> {
        let height = min(20, self.height);
        let mut search = Search::blank(&lines, initial_query, height);

        self.clear(height);

        while !search.is_done() {
            self.print(&search);
            let input = self.ansi.io.read();

            match input {
                Some(character) => {
                    search = self.handle_keystroke(search, character.as_ref());
                }
                None => break,
            };
        }
        search.selection()
    }
}
impl<'a> Default for Screen<'a> {
    fn default() -> Self {
        Self::new()
    }
}
