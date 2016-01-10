extern crate icepick;

#[cfg(test)]
mod tests {
    pub use icepick::search::Search;
    pub use icepick::screen::Screen;
    pub use icepick::tty::TTY;

    #[test]
    fn moves_the_selection_down_for_ctrl_n() {
        let choices = vec!["one".to_string(), "two".to_string()];
        let screen = Screen::fake();

        let search = Search::blank(&choices, None, 10);
        let result = screen.handle_keystroke(search, "\u{e}");
        assert_eq!(result.selection(), Some("two".to_string()));
    }

    #[test]
    fn moves_the_selection_up_for_ctrl_p() {
        let choices = vec!["one".to_string(), "two".to_string()];
        let screen = Screen::fake();

        let search = Search::blank(&choices, None, 10).down();
        let result = screen.handle_keystroke(search, "\u{10}");
        assert_eq!(result.selection(), Some("one".to_string()));
    }

    #[test]
    fn removes_the_last_character_for_delete() {
        let choices = vec!["one".to_string(), "two".to_string()];
        let screen = Screen::fake();

        let search = Search::blank(&choices, None, 10).append_to_search("w").append_to_search("x");
        let result = screen.handle_keystroke(search, "\u{7f}");
        assert_eq!(result.selection(), Some("two".to_string()));
    }

    #[test]
    fn marks_a_search_as_done_for_enter() {
        let choices = vec!["one".to_string(), "two".to_string()];
        let screen = Screen::fake();

        let search = Search::blank(&choices, None, 10);
        let result = screen.handle_keystroke(search, "\n");
        assert!(result.is_done());
    }

    #[test]
    fn blanks_amount_of_lines() {
        let mut screen = Screen::fake();

        screen.clear(4);
        let lines = screen.ansi.io.lines();
        let expected_lines = vec!["\n", "\n", "\n", "\n"];

        assert_eq!(lines, expected_lines);
    }

    #[test]
    fn test_run_search_and_look_for_t() {
        let choices = vec!["one".to_string(), "two".to_string()];
        let mut screen = Screen::fake_with_input(vec!["t"]);

        let result = screen.run_search(choices, None);
        assert_eq!(result.unwrap(), "two")
    }

    #[test]
    fn run_search_immediatly_done() {
        let choices = vec!["one".to_string(), "two".to_string()];
        let mut screen = Screen::fake_with_input(vec!["\n"]);
        let result = screen.run_search(choices, None);
        assert_eq!(result.unwrap(), "one")
    }
}
