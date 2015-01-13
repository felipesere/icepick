use configuration::Configuration;
use std::slice::SliceExt;
use std::cmp::Ordering;
use score::Score;

struct Search {
    config: Configuration,
    current: uint,
    query: String,
    selection: Option<String>,
    result: Vec<String>,
}

impl Search {
    fn blank(config: Configuration) -> Search {
        let query = config.initial_search.clone();
        let previous_result = config.choices.clone();
        let selection = Some(previous_result[0].clone());
        Search::new(config, query, previous_result, 0, selection)
    }

    fn new(config: Configuration, query: String, result: Vec<String>, index: uint, selection: Option<String>) -> Search {
        Search { config: config,
                 current: index,
                 query: query,
                 selection: selection,
                 result: result }
    }

    fn new_for_index(self, index: uint) -> Search {
        let new_selection = Search::select(&self.result, index);

        Search::new(self.config, self.query, self.result, index, new_selection)
    }

    fn filter(query: &str, choices: &Vec<String>) -> Vec<String> {
        let mut filtered = choices.iter().filter_map( |choice| {
            let quality = Score::score(choice.as_slice(), query);
            if quality > 0.0 {
                Some((quality, choice.to_string()))
            } else {
                None
            }
        }).collect::<Vec<(f32, String)>>();

        filtered.sort_by( |&(quality_a, _), &(quality_b, _)| {
            quality_a.partial_cmp(&quality_b).unwrap_or(Ordering::Equal).reverse()
        });

        filtered.iter().map( |&(_, ref choice)| choice.to_string() ).collect::<Vec<String>>()
    }

    fn select(result: &Vec<String>, index: uint) -> Option<String> {
        if result.len() > 0 {
            Some(result[index].to_string()) 
        } else { 
            None
        }
    }

    fn down(self) -> Search {
        let next_index = self.next_index();
        self.new_for_index(next_index)
    }

    fn up(self) -> Search {
        let next_index = self.prev_index();
        self.new_for_index(next_index)
    }

    fn append_to_search(self, input: &str) -> Search {
        let mut new_query = self.query;
        new_query.push_str(input.as_slice());

        let new_result    = Search::filter(new_query.as_slice(), &self.result);
        let new_selection = Search::select(&new_result, self.current);

        Search::new(self.config, new_query, new_result, self.current, new_selection)
    }

    fn next_index(&self) -> uint {
        let next_index = self.current + 1;

        if next_index >= self.config.visible_limit { 0 } else { next_index }
    }

    fn prev_index(&self) -> uint {
        if self.current == 0 { self.config.visible_limit - 1 } else  { self.current - 1 }
    }
}

#[cfg(test)]

fn one_two_three() -> Vec<String> {
    vec!["one".to_string(),
         "two".to_string(),
         "three".to_string()]
}

#[test]
fn it_selects_the_first_choice_by_default() {
    let input =  one_two_three();

    let config = Configuration::from_inputs(input, None, None);
    let search = Search::blank(config);
    assert_eq!(search.selection, Some("one".to_string()));
}

#[test]
fn it_selets_the_second_when_down_is_called() {
    let input =  one_two_three();

    let config = Configuration::from_inputs(input, None, None);
    let search = Search::blank(config);
    assert_eq!(search.down().selection, Some("two".to_string()));
}

#[test]
fn it_loop_around_when_reaching_end_of_list() {
    let input =  one_two_three();

    let config = Configuration::from_inputs(input, None, None);
    let search = Search::blank(config);
    assert_eq!(search.down().down().down().down().selection, Some("two".to_string()));
}

#[test]
fn it_loop_around_when_reaching_top_of_list() {
    let input =  one_two_three();

    let config = Configuration::from_inputs(input, None, None);
    let search = Search::blank(config);
    assert_eq!(search.up().up().selection, Some("two".to_string()));
}

#[test]
fn it_loop_around_when_reaching_visible_limit() {
    let input =  one_two_three();

    let config = Configuration::from_inputs(input, None, Some(2));
    let search = Search::blank(config);
    assert_eq!(search.down().down().down().selection, Some("two".to_string()));
}

#[test]
fn it_moves_down_the_filtered_search_results() {
    let input =  one_two_three();

    let config = Configuration::from_inputs(input, None, None);
    let search = Search::blank(config);
    assert_eq!(search.append_to_search("t").down().selection, Some("three".to_string()));
}

#[test]
fn it_moves_down_the_filtered_search_results_twice() {
    let input =  one_two_three();

    let config = Configuration::from_inputs(input, None, None);
    let search = Search::blank(config);
    assert_eq!(search.append_to_search("t").append_to_search("w").selection, Some("two".to_string()));
}

#[test]
fn it_handles_not_matching_anything() {
    let input = one_two_three();

    let config = Configuration::from_inputs(input, None, None);
    let search = Search::blank(config);
    assert_eq!(search.append_to_search("x").selection, None);
}
