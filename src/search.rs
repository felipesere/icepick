use configuration::Configuration;

struct Search {
    selection: String,
    config: Configuration,
    query: String,
    current: uint,
    max_index: uint,
}

impl Search {
    fn blank(config: Configuration) -> Search {
        let max_index = config.visible_limit;
        let query = config.initial_search.clone();
        let selection = config.choices[0].to_string();

        Search { max_index: max_index,
                 query:     query,
                 config:    config,
                 current:   0,
                 selection: selection,
        }

    }

    fn down(self) -> Search {
        let next_index = self.next_index();

        self.new(next_index)
    }

    fn up(self) -> Search {
        let next_index = self.prev_index();

        self.new(next_index)
    }

    fn new(self, index: uint) -> Search {
        Search { selection: self.config.choices[index].to_string(),
                 config: self.config,
                 query: self.query,
                 max_index: self.max_index,
                 current: index }
    }

    fn append_to_search(self, input: String) -> Search {
        let index = self.current;
        let mut new_serach = self.new(index);
        new_serach.query.push_str(input.as_slice());

        new_serach
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
