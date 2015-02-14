#![feature(plugin)]
#![plugin(stainless)]
#![allow(unused_variables)]
#![allow(unused_mut)]

extern crate stainless;
extern crate athena;

mod tests {
    pub use athena::search::Search;

    pub fn input_times(n: usize) ->Vec<String> {
        let choices = vec!["choice".to_string()];
        let mut result: Vec<String> = Vec::new();
        for choice in choices.iter().cycle().take(n) {
            result.push(choice.clone());
        }
        result
    }

    describe! search_tests {
        before_each {
            let choices = vec!["one".to_string(),
                             "two".to_string(),
                             "three".to_string()];
            let mut search = Search::blank(&choices, None, None);
        }

        it "it_selects_the_first_choice_by_default" {
            assert_eq!(search.selection(), Some("one".to_string()));
        }

        it "it_selets_the_second_when_down_is_called" {
            assert_eq!(search.down().selection(), Some("two".to_string()));
        }

        it "it_loop_around_when_reaching_end_of_list" {
            assert_eq!(search.down().down().down().down().selection(), Some("two".to_string()));
        }

        it "it_loop_around_when_reaching_top_of_list" {
            assert_eq!(search.up().up().selection(), Some("two".to_string()));
        }

        it "it_loop_around_when_reaching_visible_limit" {
            assert_eq!(search.down().down().down().down().selection(), Some("two".to_string()));
        }

        it "it_moves_down_the_filtered_search_results" {
            assert_eq!(search.append_to_search("t").down().selection(), Some("three".to_string()));
        }

        it "it_moves_down_the_filtered_search_results_twice" {
            assert_eq!(search.append_to_search("t").append_to_search("w").selection(), Some("two".to_string()));
        }

        it "it_handles_not_matching_anything" {
            assert_eq!(search.append_to_search("x").selection(), None);
        }

        it "up_match_nothing_after_filtering_all_out" {
            assert_eq!(search.append_to_search("x").up().selection(), None);
        }

        it "down_match_nothing_after_filtering_all_out" {
            assert_eq!(search.append_to_search("x").down().selection(), None);
        }

        it "backspaces_over_characters" {
            search = search.append_to_search("e");

            assert_eq!(search.query, "e");
            assert_eq!(search.backspace().query, "");
        }

        it "resets_the_index_when_removing_char_from_search" {
            assert_eq!(search.append_to_search("e").down().backspace().current, 0);
        }

        it "resets_the_index_when_adding_char_to_seach" {
            assert_eq!(search.down().append_to_search("o").current, 0);
        }

        it "previous_results_appear_after_backspace" {
            assert_eq!(search.append_to_search("t").backspace().num_matches(), 3);
        }

        it "initial_search_is_not_done" {
            assert!(!search.is_done());
        }

        it "backspace_over_initial_doesnt_crash" {
            search.backspace();
        }

        it "done_search_is_done" {
            assert!(search.done().is_done());
        }

        it "done_search_has_selection" {
            assert_eq!(search.done().selection(), Some("one".to_string()));
        }

        it "loop_around_when_reaching_bottom_of_choices" {
            assert_eq!(search.append_to_search("n").down().selection(), Some("one".to_string()));
        }

        it "loop_around_when_reaching_top_of_choices" {
            assert_eq!(search.append_to_search("n").up().selection(), Some("one".to_string()));
        }

        it "search_is_case_insensitive" {
            assert_eq!(search.append_to_search("T").num_matches(), 2);
        }

        it "uses_configs_visible_limit_as_result_size" {
            let many_choices = input_times(30);
            let search = Search::blank(&many_choices, None, Some(20)).append_to_search("c");

            assert_eq!(search.result.len(), 20);
        }
    }
}
