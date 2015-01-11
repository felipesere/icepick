use std::slice::SliceExt;

pub struct Configuration {
    pub choices: Vec<String>,
    pub visible_limit: uint,
    initial_search: Option<String>,
}

impl Configuration {
    pub fn from_inputs(choice: Vec<String>,
                       initial_search: Option<String>,
                       visible_limit: Option<uint>)  -> Configuration {

        let cleaned = choice.iter().map(clean as fn(&String) -> String).collect::<Vec<String>>();
        let limit = match visible_limit {
            Some(n) => n,
            None => choice.len(),
        };

        Configuration { choices: cleaned,
                        initial_search: initial_search,
                        visible_limit: limit }
    }

    fn parse_options(input: Vec<String>) -> Option<String> {
        match input.position_elem(&"-s".to_string()) {
            Some(ref idx) => Some(input[*idx + 1].clone()),
            None => None,
        }
    }

    fn choices(self) -> Vec<String> {
        self.choices
    }
}

fn clean(input: &String) -> String {
    let slice = input.as_slice();
    slice.trim_left().trim_right().to_string()
}

#[cfg(test)]

#[test]
fn removes_leading_and_trailing_whitespace() {
    let input = vec![" a choice ".to_string()];

    let config = Configuration::from_inputs(input, None, None);

    assert_eq!(config.choices(), vec!("a choice"));
}

#[test]
fn can_specify_initial_search() {
    let input = vec!["foo".to_string()];
    let options = vec!["-s".to_string(),
                       "some search".to_string()];

    let config = Configuration::from_inputs(input, Configuration::parse_options(options), None);
    assert_eq!(config.initial_search, Some("some search".to_string()));
}

#[test]
fn initial_search_is_optional() {
    let input = vec!["foo".to_string()];
    let options = vec![];

    let config = Configuration::from_inputs(input, Configuration::parse_options(options), None);
    assert_eq!(config.initial_search, None);
}
