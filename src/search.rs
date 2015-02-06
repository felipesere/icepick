use configuration::Configuration;
use std::slice::SliceExt;
use std::cmp::min;
use score::Score;
use sorted_result_set::SortedResultSet;
use std::ascii::AsciiExt;

#[derive(Debug)]
pub struct Search<'s> {
    pub config: &'s Configuration,
    current: usize,
    pub query: String,
    pub selection: Option<String>,
    pub result: Vec<String>,
    choice_stack: Vec<Vec<&'s String>>,
    done: bool,
}

impl<'s> Search<'s> {
    pub fn blank(config: &'s Configuration) -> Search<'s> {
        let query = config.initial_search.clone();
        let mut choice_stack: Vec<Vec<&'s String>> = Vec::new();
        let mut result = Vec::new();

        for choice in config.choices.iter().take(config.visible_limit) {
            result.push(choice.clone());
        }

        let mut first_stack_frame = Vec::new();
        for choice in config.choices.iter() {
            first_stack_frame.push(choice);
        }

        choice_stack.push(first_stack_frame);

        Search::new(config, query, choice_stack, result, 0, false)
    }

    pub fn is_done(&self) -> bool {
        self.done
    }

    pub fn done(self) -> Search<'s> {
        Search::new(self.config, self.query, self.choice_stack, self.result, self.current, true)
    }

    fn new(config: &'s Configuration, query: String, choice_stack: Vec<Vec<&'s String>>, result: Vec<String>, index: usize, done: bool) -> Search<'s> {
        let selection = Search::select(&result, index);

        Search { config: config,
                 current: index,
                 query: query,
                 selection: selection,
                 result: result,
                 choice_stack: choice_stack,
                 done: done}
    }

    fn new_for_index(self, index: usize) -> Search<'s> {
        Search::new(self.config, self.query, self.choice_stack, self.result, index, self.done)
    }

    fn new_for_query(mut self, new_query: String) -> Search<'s> {
        let mut results = SortedResultSet::new(self.config.visible_limit);

        let mut filtered_choices: Vec<&String> = Vec::new();

        Search::iter_matches(new_query.as_slice(), &self.config.choices,
                        |match_str, quality| { results.push(match_str, quality);
                                               filtered_choices.push(match_str)
                                             });

        self.choice_stack.push(filtered_choices);
        Search::new(self.config, new_query, self.choice_stack, results.as_sorted_vec(), 0, self.done)
    }

    pub fn iter_matches<F: FnMut(&'s String, f32)>(query: &str, choices: &'s Vec<String>, mut f: F) {
        let lower_query = query.to_ascii_lowercase();

        for choice in choices.iter() {
            let lower_choice = choice.to_ascii_lowercase();

            match Score::score(&lower_choice, &lower_query) {
                0.0     => continue,
                quality => f(choice, quality),
            };
        }
    }

    fn select(result: &Vec<String>, index: usize) -> Option<String> {
        match result.get(index) {
            Some(t) => Some(t.clone()),
            None => None,
        }
    }

    pub fn down(self) -> Search<'s> {
        let next_index = self.next_index();
        self.new_for_index(next_index)
    }

    pub fn up(self) -> Search<'s> {
        let next_index = self.prev_index();
        self.new_for_index(next_index)
    }

    pub fn append_to_search(self, input: &str) -> Search<'s> {
        let mut new_query = self.query.clone();
        new_query.push_str(input.as_slice());

        self.new_for_query(new_query)
    }

    pub fn backspace(self) -> Search<'s> {
        let mut new_query = self.query.clone();
        new_query.pop();

        self.new_for_query(new_query)
    }

    fn next_index(&self) -> usize {
        let next_index = self.current + 1;

        if next_index >= self.actual_limit() {
            0
        } else {
            next_index
        }
    }

    fn prev_index(&self) -> usize {
        if self.current == 0 {
            self.actual_limit() - 1
        } else {
            self.current - 1
        }
    }

    fn actual_limit(&self) -> usize {
        min(self.config.visible_limit, self.num_matches())
    }

    pub fn num_matches(&self) -> usize {
        self.choice_stack.last().unwrap().len()
    }
}

#[cfg(test)]
mod tests {
    extern crate test;
    use configuration::Configuration;
    use super::*;
    use self::test::Bencher;
    use sorted_result_set::SortedResultSet;

    fn one_two_three() -> Vec<String> {
        vec!["one".to_string(),
             "two".to_string(),
             "three".to_string()]
    }

    #[test]
    fn it_selects_the_first_choice_by_default() {
        let input = one_two_three();
        let config = Configuration::from_inputs(input, None, None);
        let search = Search::blank(&config);

        assert_eq!(search.selection, Some("one".to_string()));
    }

    #[test]
    fn it_selets_the_second_when_down_is_called() {
        let input = one_two_three();
        let config = Configuration::from_inputs(input, None, None);
        let search = Search::blank(&config);

        assert_eq!(search.down().selection, Some("two".to_string()));
    }

    #[test]
    fn it_loop_around_when_reaching_end_of_list() {
        let input = one_two_three();
        let config = Configuration::from_inputs(input, None, None);
        let search = Search::blank(&config);

        assert_eq!(search.down().down().down().down().selection, Some("two".to_string()));
    }

    #[test]
    fn it_loop_around_when_reaching_top_of_list() {
        let input = one_two_three();
        let config = Configuration::from_inputs(input, None, None);
        let search = Search::blank(&config);

        assert_eq!(search.up().up().selection, Some("two".to_string()));
    }

    #[test]
    fn it_loop_around_when_reaching_visible_limit() {
        let input = one_two_three();
        let config = Configuration::from_inputs(input, None, Some(2));
        let search = Search::blank(&config);

        assert_eq!(search.down().down().down().selection, Some("two".to_string()));
    }

    #[test]
    fn it_moves_down_the_filtered_search_results() {
        let input = one_two_three();
        let config = Configuration::from_inputs(input, None, None);
        let search = Search::blank(&config);

        assert_eq!(search.append_to_search("t").down().selection, Some("three".to_string()));
    }

    #[test]
    fn it_moves_down_the_filtered_search_results_twice() {
        let input = one_two_three();
        let config = Configuration::from_inputs(input, None, None);
        let search = Search::blank(&config);

        assert_eq!(search.append_to_search("t").append_to_search("w").selection, Some("two".to_string()));
    }

    #[test]
    fn it_handles_not_matching_anything() {
        let input = one_two_three();
        let config = Configuration::from_inputs(input, None, None);
        let search = Search::blank(&config);

        assert_eq!(search.append_to_search("x").selection, None);
    }

    #[test]
    fn up_match_nothing_after_filtering_all_out() {
        let input = one_two_three();
        let config = Configuration::from_inputs(input, None, None);
        let search = Search::blank(&config).append_to_search("x");

        assert_eq!(search.up().selection, None);
    }

    #[test]
    fn down_match_nothing_after_filtering_all_out() {
        let input = one_two_three();
        let config = Configuration::from_inputs(input, None, None);
        let search = Search::blank(&config).append_to_search("x");

        assert_eq!(search.down().selection, None);
    }

    #[test]
    fn backspaces_over_characters() {
        let input = one_two_three();
        let config = Configuration::from_inputs(input, None, None);
        let search = Search::blank(&config).append_to_search("e");

        assert_eq!(search.query, "e");
        assert_eq!(search.backspace().query, "");
    }

    #[test]
    fn resets_the_index_when_removing_char_from_search() {
        let input = one_two_three();
        let config = Configuration::from_inputs(input, None, None);
        let search = Search::blank(&config).append_to_search("e");

        assert_eq!(search.down().backspace().current, 0);
    }

    #[test]
    fn resets_the_index_when_adding_char_to_seach() {
        let input = one_two_three();
        let config = Configuration::from_inputs(input, None, None);
        let search = Search::blank(&config).down();

        assert_eq!(search.append_to_search("o").current, 0);
    }

    #[test]
    fn previous_results_appear_after_backspace() {
        let input = one_two_three();
        let config = Configuration::from_inputs(input, None, None);
        let search = Search::blank(&config).append_to_search("t");

        assert_eq!(search.backspace().num_matches(), 3);
    }

    #[test]
    fn initial_search_is_not_done() {
        let input = one_two_three();
        let config = Configuration::from_inputs(input, None, None);
        let search = Search::blank(&config);

        assert!(!search.is_done());
    }

    #[test]
    fn done_search_is_done() {
        let input = one_two_three();
        let config = Configuration::from_inputs(input, None, None);
        let search = Search::blank(&config).done();

        assert!(search.is_done());
    }

    #[test]
    fn done_search_has_selection() {
        let input = one_two_three();
        let config = Configuration::from_inputs(input, None, None);
        let search = Search::blank(&config).done();

        assert_eq!(search.selection, Some("one".to_string()));
    }

    #[test]
    fn loop_around_when_reaching_bottom_of_choices() {
        let input = one_two_three();
        let config = Configuration::from_inputs(input, None, None);
        let search = Search::blank(&config).append_to_search("n").down();

        assert_eq!(search.selection, Some("one".to_string()));
    }

    #[test]
    fn loop_around_when_reaching_top_of_choices() {
        let input = one_two_three();
        let config = Configuration::from_inputs(input, None, None);
        let search = Search::blank(&config).append_to_search("n").up();

        assert_eq!(search.selection, Some("one".to_string()));
    }

    #[test]
    fn search_is_case_insensitive() {
        let input = one_two_three();
        let config = Configuration::from_inputs(input, None, None);
        let search = Search::blank(&config).append_to_search("T");

        assert_eq!(search.num_matches(), 2);
    }

    #[test]
    fn uses_configs_visible_limit_as_result_size() {
        let input = input_times(30);
        let config = Configuration::from_inputs(input, None, Some(20));
        let search = Search::blank(&config).append_to_search("T");

        assert_eq!(search.num_matches(), 20);
    }

    fn input_times(n: usize) ->Vec<String> {
        let mut result: Vec<String> = Vec::new();
        for thing in one_two_three().iter().cycle().take(n) {
            result.push(thing.clone());
        }
        result
    }

    //84114 ns/iter (+/- 33515)
    #[bench]
    fn filter_speed(b: &mut Bencher) {
        let input = input_times(1000);
        let query = "t";

        b.iter(||{
            let mut results = SortedResultSet::new(20);
            Search::iter_matches(query, &input,
                                    |choice, quality| results.push(choice, quality));
            results
        });
    }
}
