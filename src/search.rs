use configuration::Configuration;

struct Search {
    selection: String,
    config: Configuration,
    index: uint,
}

impl Search {
    fn blank(config: Configuration) -> Search {
        Search { selection: config.choices[0].to_string(),
                 config: config,
                 index: 0 }
    }

    fn down(self) -> Search {
        let next_index = self.index + 1;
        Search { selection: self.config.choices[next_index].to_string(),
                 config: self.config,
                 index: next_index }
    }
}


#[cfg(test)]

#[test]
fn it_selects_the_first_choice_by_default() {
    let input = vec!["one".to_string(),
                     "two".to_string(),
                     "three".to_string()];

    let config = Configuration::from_inputs(input, None);
    let search = Search::blank(config);

    assert_eq!(search.selection, "one");
}

#[test]
fn it_selets_the_second_when_down_is_called() {
    let input = vec!["one".to_string(),
                     "two".to_string(),
                     "three".to_string()];

    let config = Configuration::from_inputs(input, None);
    let search = Search::blank(config);

    assert_eq!(search.down().selection, "two");
}
