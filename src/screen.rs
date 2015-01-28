use configuration::Configuration;
use search::Search;
use ansi::Ansi;
use tty::TTY;
use fake_tty::FakeIO;
use renderer::Renderer;
use text::Text;


pub struct Screen <'a> {
    pub ansi: Ansi<'a>,
    height: usize,
}


impl <'a> Screen <'a>{
    pub fn new() -> Screen<'a>  {
        let ansi = Ansi { io: Box::new(TTY::new()) };
        let (_, height) = ansi.io.dimensions();
        Screen {
            ansi: ansi,
            height: height,
        }
    }

    fn fake() -> Screen<'a> {
        Screen {
            ansi: Ansi { io: Box::new(FakeIO::new()) },
            height: 20,
        }
    }

    pub fn handle_keystroke(&self, search: Search, input: &str) -> Search {
        match input {
           "\u{1b}" => search.abort(),
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

        let start_line = self.height - search.config.visible_limit -1;

        for (idx, text) in result.iter().enumerate() {
            self.write(start_line+idx, text);
        };
    }

    pub fn write(&mut self, line: usize, text: &Text) {
        self.ansi.blank_line(line);
        self.ansi.set_position(line,0);

        match *text {
            Text::Normal(ref t) => self.ansi.print(t.as_slice()),
            Text::Highlight(ref t) => self.ansi.inverted(t.as_slice()),
            Text::Blank => self.ansi.print("".as_slice()),
        };
    }
}


#[cfg(test)]

#[test]
pub fn search_is_done_when_esc() {
    let input = blank_search();
    let screen = Screen::new();
    let result = screen.handle_keystroke(input, "\u{1b}");
    assert!(result.is_done());
}

#[test]
pub fn moves_the_selection_down_for_ctrl_n() {
    let input = blank_search();
    let screen = Screen::new();
    let result = screen.handle_keystroke(input, "\u{e}");
    assert_eq!(result.selection, Some("two".to_string()));
}

#[test]
pub fn moves_the_selection_up_for_ctrl_p() {
    let input = blank_search().down();
    let screen = Screen::new();
    let result = screen.handle_keystroke(input, "\u{10}");
    assert_eq!(result.selection, Some("one".to_string()));
}

#[test]
pub fn removes_the_last_character_for_delete() {
    let input = blank_search().append_to_search("w").append_to_search("x");
    let screen = Screen::new();
    let result = screen.handle_keystroke(input, "\u{7f}");
    assert_eq!(result.selection, Some("two".to_string()));
}

#[test]
pub fn marks_a_search_as_done_for_enter() {
    let input = blank_search();
    let screen = Screen::new();
    let result = screen.handle_keystroke(input, "\n");
    assert!(result.is_done());
}

fn blank_search() -> Search {
    let input = vec!["one".to_string(), "two".to_string()];
    let config = Configuration::from_inputs(input, None, Some(10));
    Search::blank(config)
}

