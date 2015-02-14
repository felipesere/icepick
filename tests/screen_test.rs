#![feature(plugin)]
#![plugin(stainless)]
#![allow(unused_assignments)]
#![allow(unused_mut)]
#![allow(unused_variables)]
#![feature(core)]

extern crate stainless;
extern crate athena;

mod test {
    pub use athena::search::Search;
    pub use athena::screen::Screen;
    pub use athena::tty::TTY;
    
    describe! screen_test {
        before_each {
            let choices = vec!["one".to_string(), "two".to_string()];
            let mut screen = Screen::fake();
        }

        it "moves_the_selection_down_for_ctrl_n" {
            let search = Search::blank(&choices, None, Some(10));
            let result = screen.handle_keystroke(search, "\u{e}");
            assert_eq!(result.selection(), Some("two".to_string()));
        }

        it "moves_the_selection_up_for_ctrl_p" {
            let search = Search::blank(&choices, None, Some(10)).down();
            let result = screen.handle_keystroke(search, "\u{10}");
            assert_eq!(result.selection(), Some("one".to_string()));
        }

        it "removes_the_last_character_for_delete" {
            let search = Search::blank(&choices, None, Some(10)).append_to_search("w").append_to_search("x");
            let result = screen.handle_keystroke(search, "\u{7f}");
            assert_eq!(result.selection(), Some("two".to_string()));
        }

        it "marks_a_search_as_done_for_enter" {
            let search = Search::blank(&choices, None, Some(10));
            let result = screen.handle_keystroke(search, "\n");
            assert!(result.is_done());
        }

        it "blanks_amount_of_lines" {
            screen.blank(4);
            let lines = screen.ansi.io.lines();
            let expected_lines = vec!["\n", "\n", "\n", "\n"];

            assert_eq!(lines, expected_lines);
        }

        it "test_run_search_and_look_for_t" {
            screen = Screen::fake_with_input(vec!["t"]);
            let result = screen.run_search(choices, None);
            assert_eq!(result.unwrap().as_slice(), "two")
        }

        it "run_search_immediatly_done" {
            screen = Screen::fake_with_input(vec!["\n"]);
            let result = screen.run_search(choices, None);
            assert_eq!(result.unwrap().as_slice(), "one")
        }
    }
}
