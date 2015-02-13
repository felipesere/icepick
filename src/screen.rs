use search::Search;
use ansi::Ansi;
use tty::TTY;
use fake_tty::FakeIO;
use renderer::Renderer;
use text::Text;
use std::cmp::min;

pub struct Screen <'a> {
    pub ansi: Ansi<'a>,
    pub height: usize,
    pub width: usize,
}

impl <'a> Screen <'a>{
    pub fn new() -> Screen<'a> {
        let ansi = Ansi { io: Box::new(TTY::new()) };
        let (width, height) = ansi.io.dimensions();
        Screen {
            ansi: ansi,
            height: height,
            width: width - 1,
        }
    }

    pub fn fake() -> Screen<'a> {
        Screen::fake_with_input(vec![])
    }

    pub fn fake_with_input(input: Vec<&str>) -> Screen<'a> {
        Screen {
            ansi: Ansi { io: Box::new(FakeIO::new_with_input(input)) },
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

        let start_line = self.height - search.visible_limit - 1;

        for (idx, text) in result.iter().enumerate() {
            self.write(start_line + idx, text);
        };
        self.ansi.set_position(start_line, renderer.header(search).len());
        self.ansi.show_cursor();
    }

    pub fn write(&mut self, line: usize, text: &Text) {
        self.ansi.blank_line(line);
        self.ansi.set_position(line, 0);

        match *text {
            Text::Normal(ref text) => {
                let printable_length = self.printable_length(text);
                self.ansi.print(&text[..printable_length]);
            }
            Text::Highlight(ref text) => {
                let printable_length = self.printable_length(text);
                self.ansi.inverted(&text[..printable_length]);
            }
            Text::Blank => self.ansi.print("".as_slice()),
        };
    }

    pub fn blank(&mut self, lines: usize) {
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

    fn printable_length(&self, text: &str) -> usize {
        min(text.len(), self.width)
    }

    pub fn run_search(&mut self, lines: Vec<String>, initial_query: Option<String>) -> Option<String> {
        let height = min(20, self.height);
        let mut search = Search::blank(&lines, initial_query, Some(height));

        self.blank(height);

        while !search.is_done() {
            self.print(&search);
            let input = self.ansi.io.read();
            match input {
                Some(character) => {
                    search = self.handle_keystroke(search, character.as_slice());
                },
                None => break,
            };
        }
        search.selection()
    }
}

#[cfg(test)]
mod tests {
    use search::Search;
    use super::*;

    #[test]
    fn test_run_search_and_look_for_t() {
        let mut screen = Screen::fake_with_input(vec!["t"]);
        let input = input();
        let result = screen.run_search(input, None);
        assert_eq!(result.unwrap().as_slice(), "two")
    }

    #[test]
    fn run_search_immediatly_done() {
        let mut screen = Screen::fake_with_input(vec!["\n"]);
        let input = input();
        let result = screen.run_search(input, None);
        assert_eq!(result.unwrap().as_slice(), "one")
    }

    #[test]
    fn moves_the_selection_down_for_ctrl_n() {
        let input = input();
        let search = Search::blank(&input, None, Some(10));
        let screen = Screen::fake();
        let result = screen.handle_keystroke(search, "\u{e}");
        assert_eq!(result.selection(), Some("two".to_string()));
    }

    #[test]
    fn moves_the_selection_up_for_ctrl_p() {
        let input = input();
        let search = Search::blank(&input, None, Some(10)).down();
        let screen = Screen::fake();
        let result = screen.handle_keystroke(search, "\u{10}");
        assert_eq!(result.selection(), Some("one".to_string()));
    }

    #[test]
    fn removes_the_last_character_for_delete() {
        let input = input();
        let search = Search::blank(&input, None, Some(10)).append_to_search("w").append_to_search("x");
        let screen = Screen::fake();
        let result = screen.handle_keystroke(search, "\u{7f}");
        assert_eq!(result.selection(), Some("two".to_string()));
    }

    #[test]
    fn marks_a_search_as_done_for_enter() {
        let input = input();
        let search = Search::blank(&input, None, Some(10));
        let screen = Screen::fake();
        let result = screen.handle_keystroke(search, "\n");
        assert!(result.is_done());
    }

    #[test]
    fn blanks_amount_of_lines() {
        let mut screen = Screen::fake();
        screen.blank(4);
        let lines = screen.ansi.io.lines();
        let expected_lines = vec!["\n",
                                  "\n",
                                  "\n",
                                  "\n"];

        assert_eq!(lines, expected_lines);
    }

    fn input() -> Vec<String> {
        vec!["one".to_string(), "two".to_string()]
    }
}
