extern crate icepick;

#[cfg(test)]
mod tests {
    pub use icepick::search::Search;

    pub fn input_times(n: usize) ->Vec<String> {
        let choices = vec!["choice".to_string()];
        let mut result: Vec<String> = Vec::new();
        for choice in choices.iter().cycle().take(n) {
            result.push(choice.clone());
        }
        result
    }

    #[test]
    fn it_selects_the_first_choice_by_default() {
        let choices = vec!["one".to_string(),
        "two".to_string(),
        "three".to_string()];
        let mut search = Search::blank(&choices, None, 20);

        assert_eq!(search.selection(), Some("one".to_string()));
    }

    #[test]
    fn it_selets_the_second_when_down_is_called() {
        let choices = vec!["one".to_string(),
        "two".to_string(),
        "three".to_string()];
        let mut search = Search::blank(&choices, None, 20);

        assert_eq!(search.down().selection(), Some("two".to_string()));
    }

    #[test]
    fn it_loop_around_when_reaching_end_of_list() {
        let choices = vec!["one".to_string(),
        "two".to_string(),
        "three".to_string()];
        let mut search = Search::blank(&choices, None, 20);

        assert_eq!(search.down().down().down().down().selection(), Some("two".to_string()));
    }

    #[test]
    fn it_loop_around_when_reaching_top_of_list() {
        let choices = vec!["one".to_string(), "two".to_string(), "three".to_string()];
        let mut search = Search::blank(&choices, None, 20);

        assert_eq!(search.up().up().selection(), Some("two".to_string()));
    }

    #[test]
    fn it_loop_around_when_reaching_visible_limit() {
        let choices = vec!["one".to_string(),
        "two".to_string(),
        "three".to_string()];
        let mut search = Search::blank(&choices, None, 20);

        assert_eq!(search.down().down().down().down().selection(), Some("two".to_string()));
    }

    #[test]
    fn it_moves_down_the_filtered_search_results() {
        let choices = vec!["one".to_string(),
        "two".to_string(),
        "three".to_string()];
        let mut search = Search::blank(&choices, None, 20);

        assert_eq!(search.append_to_search("t").down().selection(), Some("three".to_string()));
    }

    #[test]
    fn it_moves_down_the_filtered_search_results_twice() {
        let choices = vec!["one".to_string(),
        "two".to_string(),
        "three".to_string()];
        let mut search = Search::blank(&choices, None, 20);

        assert_eq!(search.append_to_search("t").append_to_search("w").selection(), Some("two".to_string()));
    }

    #[test]
    fn it_handles_not_matching_anything() {
        let choices = vec!["one".to_string(),
        "two".to_string(),
        "three".to_string()];
        let mut search = Search::blank(&choices, None, 20);

        assert_eq!(search.append_to_search("x").selection(), None);
    }

    #[test]
    fn up_match_nothing_after_filtering_all_out() {
        let choices = vec!["one".to_string(), "two".to_string(), "three".to_string()];
        let mut search = Search::blank(&choices, None, 20);

        assert_eq!(search.append_to_search("x").up().selection(), None);
    }

    #[test]
    fn down_match_nothing_after_filtering_all_out() {
        let choices = vec!["one".to_string(),
        "two".to_string(),
        "three".to_string()];
        let mut search = Search::blank(&choices, None, 20);

        assert_eq!(search.append_to_search("x").down().selection(), None);
    }

    #[test]
    fn backspaces_over_characters() {
        let choices = vec!["one".to_string(),
        "two".to_string(),
        "three".to_string()];
        let mut search = Search::blank(&choices, None, 20);

        search = search.append_to_search("e");

        assert_eq!(search.query, "e");
        assert_eq!(search.backspace().query, "");
    }

    #[test]
    fn resets_the_index_when_removing_char_from_search() {
        let choices = vec!["one".to_string(),
        "two".to_string(),
        "three".to_string()];
        let mut search = Search::blank(&choices, None, 20);

        assert_eq!(search.append_to_search("e").down().backspace().current, 0);
    }

    #[test]
    fn resets_the_index_when_adding_char_to_seach() {
        let choices = vec!["one".to_string(),
        "two".to_string(),
        "three".to_string()];
        let mut search = Search::blank(&choices, None, 20);

        assert_eq!(search.down().append_to_search("o").current, 0);
    }

    #[test]
    fn previous_results_appear_after_backspace() {
        let choices = vec!["one".to_string(),
        "two".to_string(),
        "three".to_string()];
        let mut search = Search::blank(&choices, None, 20);

        assert_eq!(search.append_to_search("t").backspace().num_matches(), 3);
    }

    #[test]
    fn initial_search_is_not_done() {
        let choices = vec!["one".to_string(),
        "two".to_string(),
        "three".to_string()];
        let mut search = Search::blank(&choices, None, 20);

        assert!(!search.is_done());
    }

    #[test]
    fn backspace_over_initial_doesnt_crash() {
        let choices = vec!["one".to_string(),
        "two".to_string(),
        "three".to_string()];
        let mut search = Search::blank(&choices, None, 20);

        search.backspace();
    }

    #[test]
    fn done_search_is_done() {
        let choices = vec!["one".to_string(),
        "two".to_string(),
        "three".to_string()];
        let mut search = Search::blank(&choices, None, 20);

        assert!(search.done().is_done());
    }

    #[test]
    fn done_search_has_selection() {
        let choices = vec!["one".to_string(),
        "two".to_string(),
        "three".to_string()];
        let mut search = Search::blank(&choices, None, 20);

        assert_eq!(search.done().selection(), Some("one".to_string()));
    }

    #[test]
    fn loop_around_when_reaching_bottom_of_choices() {
        let choices = vec!["one".to_string(),
        "two".to_string(),
        "three".to_string()];
        let mut search = Search::blank(&choices, None, 20);

        assert_eq!(search.append_to_search("n").down().selection(), Some("one".to_string()));
    }

    #[test]
    fn loop_around_when_reaching_top_of_choices() {
        let choices = vec!["one".to_string(),
        "two".to_string(),
        "three".to_string()];
        let mut search = Search::blank(&choices, None, 20);

        assert_eq!(search.append_to_search("n").up().selection(), Some("one".to_string()));
    }

    #[test]
    fn search_is_case_insensitive() {
        let choices = vec!["one".to_string(),
        "two".to_string(),
        "three".to_string()];
        let mut search = Search::blank(&choices, None, 20);

        assert_eq!(search.append_to_search("T").num_matches(), 2);
    }

    #[test]
    fn uses_configs_visible_limit_as_result_size() {
        let choices = vec!["one".to_string(),
        "two".to_string(),
        "three".to_string()];
        let mut search = Search::blank(&choices, None, 20);

        let many_choices = input_times(30);
        let search = Search::blank(&many_choices, None, 20).append_to_search("c");

        assert_eq!(search.result.len(), 20);
    }
}
