use configuration::Configuration;
use std::slice::SliceExt;
use std::cmp::min;
use score::Score;
use sorted_result_set::SortedResultSet;
use std::ascii::AsciiExt;

#[derive(Debug)]
pub struct Search<'s> {
    pub config: &'s Configuration,
    pub current: usize,
    pub query: String,
    pub result: Vec<String>,
    choice_stack: Vec<Vec<&'s String>>,
    done: bool,
}

struct ChoiceStack<'s> {
    content: Vec<Vec<&'s String>>,
}

impl <'s>ChoiceStack<'s> {
    pub fn new(input: &'s Vec<String>) -> ChoiceStack<'s> {
        let mut first_stack_frame = Vec::new();
        for choice in input.iter() {
            first_stack_frame.push(choice);
        }

        ChoiceStack { content: vec![first_stack_frame] }
    }

    pub fn push(&mut self, frame: Vec<&'s String>) {
        self.content.push(frame);
    }

    pub fn pop(&mut self) {
        if self.content.len() > 1 {
            self.content.pop();
        }
    }

    pub fn peek(&self) -> &Vec<&'s String> {
        self.content.last().unwrap()
    }

    pub fn last_size(&self) -> uint {
        self.peek().len()
    }
}

impl<'s> Search<'s> {
    pub fn blank(config: &'s Configuration) -> Search<'s> {
        let query = config.initial_search.clone();

        let mut choice_stack: Vec<Vec<&'s String>> = Vec::new();
        let mut result = Vec::new();
        let mut first_stack_frame = Vec::new();

        let f = ChoiceStack::new(&config.choices);


        for choice in config.choices.iter().take(config.visible_limit) {
            result.push(choice.clone());
        }

        for choice in config.choices.iter() {
            first_stack_frame.push(choice);
        }

        choice_stack.push(first_stack_frame);

        Search::new(config, query, choice_stack, result, 0, false)
    }

    fn new(config: &'s Configuration, query: String, choice_stack: Vec<Vec<&'s String>>, result: Vec<String>, index: usize, done: bool) -> Search<'s> {
        Search { config: config,
                 current: index,
                 query: query,
                 result: result,
                 choice_stack: choice_stack,
                 done: done}
    }

    pub fn is_done(&self) -> bool {
        self.done
    }

    pub fn done(self) -> Search<'s> {
        Search::new(self.config, self.query, self.choice_stack, self.result, self.current, true)
    }

    pub fn selection(&self) -> Option<String> {
        if self.result.len() > 0 {
            Some(self.result[self.current].to_string())
        } else {
            None
        }
    }

    fn new_for_index(self, index: usize) -> Search<'s> {
        Search::new(self.config, self.query, self.choice_stack, self.result, index, self.done)
    }

    pub fn iter_matches<F: FnMut(&'s String, f32)>(query: &str, choices: &Vec<&'s String>, mut f: F) {
        let lower_query = query.to_ascii_lowercase();

        for choice in choices.iter() {
            let lower_choice = choice.to_ascii_lowercase();

            match Score::score(&lower_choice, &lower_query) {
                0.0     => continue,
                quality => f(choice, quality),
            };
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

    pub fn append_to_search(mut self, input: &str) -> Search<'s> {
        let mut new_query = self.query.clone();
        new_query.push_str(input.as_slice());

        let mut results = SortedResultSet::new(self.config.visible_limit);
        let mut filtered_choices: Vec<&String> = Vec::new();
        Search::iter_matches(new_query.as_slice(), &self.choice_stack.last().unwrap(),
                        |match_str, quality| {
                                               results.push(match_str, quality);
                                               filtered_choices.push(match_str)
                                             });

        self.choice_stack.push(filtered_choices);
        Search::new(self.config, new_query, self.choice_stack, results.as_sorted_vec(), 0, self.done)
    }

    pub fn backspace(mut self) -> Search<'s> {
        let mut new_query = self.query.clone();
        new_query.pop();

        if self.choice_stack.len() > 1 {
            self.choice_stack.pop();
        }

        let mut results = SortedResultSet::new(self.config.visible_limit);
        Search::iter_matches(new_query.as_slice(), &self.choice_stack.last().unwrap(),
                        |match_str, quality| {
                                               results.push(match_str, quality);
                                             });
        Search::new(self.config, new_query, self.choice_stack, results.as_sorted_vec(), 0, self.done)
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
    use configuration::Configuration;
    use super::*;

    fn one_two_three() -> Vec<String> {
        vec!["one".to_string(),
             "two".to_string(),
             "three".to_string()]
    }

    fn input_times(n: usize) ->Vec<String> {
        let mut result: Vec<String> = Vec::new();
        for thing in one_two_three().iter().cycle().take(n) {
            result.push(thing.clone());
        }
        result
    }

    fn basic_configuration() -> Configuration {
        let input = one_two_three();
        Configuration::from_inputs(input, None, None)
    }

    #[test]
    fn it_selects_the_first_choice_by_default() {
        let config = basic_configuration();
        let search = Search::blank(&config);

        assert_eq!(search.selection(), Some("one".to_string()));
    }

    #[test]
    fn it_selets_the_second_when_down_is_called() {
        let config = basic_configuration();
        let search = Search::blank(&config);

        assert_eq!(search.down().selection(), Some("two".to_string()));
    }

    #[test]
    fn it_loop_around_when_reaching_end_of_list() {
        let config = basic_configuration();
        let search = Search::blank(&config);

        assert_eq!(search.down().down().down().down().selection(), Some("two".to_string()));
    }

    #[test]
    fn it_loop_around_when_reaching_top_of_list() {
        let config = basic_configuration();
        let search = Search::blank(&config);

        assert_eq!(search.up().up().selection(), Some("two".to_string()));
    }

    #[test]
    fn it_loop_around_when_reaching_visible_limit() {
        let config = basic_configuration();
        let search = Search::blank(&config);

        assert_eq!(search.down().down().down().down().selection(), Some("two".to_string()));
    }

    #[test]
    fn it_moves_down_the_filtered_search_results() {
        let config = basic_configuration();
        let search = Search::blank(&config);

        assert_eq!(search.append_to_search("t").down().selection(), Some("three".to_string()));
    }

    #[test]
    fn it_moves_down_the_filtered_search_results_twice() {
        let config = basic_configuration();
        let search = Search::blank(&config);

        assert_eq!(search.append_to_search("t").append_to_search("w").selection(), Some("two".to_string()));
    }

    #[test]
    fn it_handles_not_matching_anything() {
        let config = basic_configuration();
        let search = Search::blank(&config);

        assert_eq!(search.append_to_search("x").selection(), None);
    }

    #[test]
    fn up_match_nothing_after_filtering_all_out() {
        let config = basic_configuration();
        let search = Search::blank(&config).append_to_search("x");

        assert_eq!(search.up().selection(), None);
    }

    #[test]
    fn down_match_nothing_after_filtering_all_out() {
        let config = basic_configuration();
        let search = Search::blank(&config).append_to_search("x");

        assert_eq!(search.down().selection(), None);
    }

    #[test]
    fn backspaces_over_characters() {
        let config = basic_configuration();
        let search = Search::blank(&config).append_to_search("e");

        assert_eq!(search.query, "e");
        assert_eq!(search.backspace().query, "");
    }

    #[test]
    fn resets_the_index_when_removing_char_from_search() {
        let config = basic_configuration();
        let search = Search::blank(&config).append_to_search("e");

        assert_eq!(search.down().backspace().current, 0);
    }

    #[test]
    fn resets_the_index_when_adding_char_to_seach() {
        let config = basic_configuration();
        let search = Search::blank(&config).down();

        assert_eq!(search.append_to_search("o").current, 0);
    }

    #[test]
    fn previous_results_appear_after_backspace() {
        let config = basic_configuration();
        let search = Search::blank(&config).append_to_search("t");

        assert_eq!(search.backspace().num_matches(), 3);
    }

    #[test]
    fn initial_search_is_not_done() {
        let config = basic_configuration();
        let search = Search::blank(&config);

        assert!(!search.is_done());
    }

    #[test]
    fn backspace_over_initial_doesnt_crash() {
        let config = basic_configuration();
        let search = Search::blank(&config);

        search.backspace();
    }

    #[test]
    fn done_search_is_done() {
        let config = basic_configuration();
        let search = Search::blank(&config).done();

        assert!(search.is_done());
    }

    #[test]
    fn done_search_has_selection() {
        let config = basic_configuration();
        let search = Search::blank(&config).done();

        assert_eq!(search.selection(), Some("one".to_string()));
    }

    #[test]
    fn loop_around_when_reaching_bottom_of_choices() {
        let config = basic_configuration();
        let search = Search::blank(&config).append_to_search("n").down();

        assert_eq!(search.selection(), Some("one".to_string()));
    }

    #[test]
    fn loop_around_when_reaching_top_of_choices() {
        let config = basic_configuration();
        let search = Search::blank(&config).append_to_search("n").up();

        assert_eq!(search.selection(), Some("one".to_string()));
    }

    #[test]
    fn search_is_case_insensitive() {
        let config = basic_configuration();
        let search = Search::blank(&config).append_to_search("T");

        assert_eq!(search.num_matches(), 2);
    }

    #[test]
    fn uses_configs_visible_limit_as_result_size() {
        let input = input_times(30);

        let mut bar = Vec::new();
        for f in input.iter() {
            bar.push((*f).clone());
        }

        let config = Configuration::from_inputs(bar, None, Some(20));
        let search = Search::blank(&config).append_to_search("T");

        assert_eq!(search.num_matches(), 20);
    }

}
