use configuration::Configuration;
use std::slice::SliceExt;
use std::cmp::Ordering;
use score::Score;

struct Search {
    config: Configuration,
    current: uint,
    max_index: uint,
    query: String,
    selection: String,
    result: Vec<String>,
}

impl Search {
    fn blank(config: Configuration) -> Search {
        let max_index = config.visible_limit;
        let query = config.initial_search.clone();

        Search::new(config, query, max_index, 0)
    }

    fn new(config: Configuration, query: String, max_index: uint, index: uint) -> Search {
        let mut result = config.choices.clone();
        result = Search::filter(query.as_slice(), result);

        Search { selection: config.choices[index].to_string(),
                 config: config,
                 query: query,
                 max_index: max_index,
                 result: result,
                 current: index }
    }

    fn filter(query: &str, mut choices: Vec<String>) -> Vec<String> {
        choices.sort_by( |a,b| Score::score(a.as_slice(), query).partial_cmp(&Score::score(b.as_slice(), query)).unwrap_or(Ordering::Equal) );
        choices
    }

    fn for_index(self, index: uint) -> Search {
        Search::new(self.config, self.query, self.max_index, index)
    }

    fn down(self) -> Search {
        let next_index = self.next_index();

        self.for_index(next_index)
    }

    fn up(self) -> Search {
        let next_index = self.prev_index();

        self.for_index(next_index)
    }


    fn append_to_search(self, input: String) -> Search {
        let mut new_query = self.query;
        new_query.push_str(input.as_slice());
        Search::new(self.config, new_query, self.max_index, self.current)
    }

    fn next_index(&self) -> uint {
        let mut next_index = self.current + 1;

        if next_index >= self.max_index {
            next_index = 0;
        }

        next_index
    }

    fn prev_index(&self) -> uint {
        if self.current == 0 {
            self.max_index - 1
        } else  {
            self.current - 1
        }
    }

}


#[cfg(test)]

#[test]
fn it_selects_the_first_choice_by_default() {
    let input =  one_two_three();

    let config = Configuration::from_inputs(input, None, None);
    let search = Search::blank(config);

    assert_eq!(search.selection, "one");
}

fn one_two_three() -> Vec<String> {
    vec!["one".to_string(),
         "two".to_string(),
         "three".to_string()]
}

#[test]
fn it_selets_the_second_when_down_is_called() {
    let input =  one_two_three();

    let config = Configuration::from_inputs(input, None, None);
    let search = Search::blank(config);

    assert_eq!(search.down().selection, "two");
}

#[test]
fn it_loop_around_when_reaching_end_of_list() {
    let input =  one_two_three();

    let config = Configuration::from_inputs(input, None, None);
    let search = Search::blank(config);

    assert_eq!(search.down().down().down().down().selection, "two");
}

#[test]
fn it_loop_around_when_reaching_top_of_list() {
    let input =  one_two_three();

    let config = Configuration::from_inputs(input, None, None);
    let search = Search::blank(config);

    assert_eq!(search.up().up().selection, "two");
}

#[test]
fn it_loop_around_when_reaching_visible_limit() {
    let input =  one_two_three();

    let config = Configuration::from_inputs(input, None, Some(2));
    let search = Search::blank(config);

    assert_eq!(search.down().down().down().selection, "two");
}

//#[test]
fn it_moves_down_the_filtered_search_results() {
    let input =  one_two_three();

    let config = Configuration::from_inputs(input, None, None);
    let search = Search::blank(config);
    println!("{}", search.result);
    assert_eq!(search.append_to_search("t".to_string()).down().selection, "three");
}
