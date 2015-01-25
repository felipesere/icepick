use configuration::Configuration;
use search::Search;

pub struct Screen;


impl Screen {
    pub fn handle_keystroke(&self, search: Search, input: &str) -> Search {
        match input {
           "\u{1b}" => search.done(),
           "\u{e}" => search.down(),
           "\u{10}" => search.up(),
           "\u{7f}" => search.backspace(),
           "\n" => search.done(),
            _ => search.append_to_search(input),
        }
    }
}


#[cfg(test)]

#[test]
pub fn search_is_done_when_esc() {
    let input = blank_search();
    let screen = Screen;
    let result = screen.handle_keystroke(input, "\u{1b}");
    assert!(result.is_done());
}

#[test]
pub fn moves_the_selection_down_for_ctrl_n() {
    let input = blank_search();
    let screen = Screen;
    let result = screen.handle_keystroke(input, "\u{e}");
    assert_eq!(result.selection, Some("two".to_string()));
}

#[test]
pub fn moves_the_selection_up_for_ctrl_p() {
    let input = blank_search().down();
    let screen = Screen;
    let result = screen.handle_keystroke(input, "\u{10}");
    assert_eq!(result.selection, Some("one".to_string()));
}

#[test]
pub fn removes_the_last_character_for_delete() {
    let input = blank_search().append_to_search("w").append_to_search("x");
    let screen = Screen;
    let result = screen.handle_keystroke(input, "\u{7f}");
    assert_eq!(result.selection, Some("two".to_string()));
}

#[test]
pub fn marks_a_search_as_done_for_enter() {
    let input = blank_search();
    let screen = Screen;
    let result = screen.handle_keystroke(input, "\n");
    assert!(result.is_done());
}

fn blank_search() -> Search {
    let input = vec!["one".to_string(), "two".to_string()];
    let config = Configuration::from_inputs(input, None, None);
    Search::blank(config)
}

