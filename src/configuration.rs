use std::slice::SliceExt;

#[derive(Debug)]
pub struct Configuration<'c> {
    pub choices: &'c Vec<String>,
    pub visible_limit: usize,
    pub initial_search: String,
}

impl<'c> Configuration<'c> {
    pub fn from_inputs(choices: &'c Vec<String>,
                       initial_search: Option<String>,
                       visible_limit: Option<usize>) -> Configuration<'c> {

        let limit = visible_limit.unwrap_or(choices.len());
        let search = initial_search.unwrap_or("".to_string());

        Configuration { choices: choices,
                        initial_search: search,
                        visible_limit: limit }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_specify_initial_search() {
        let input = vec!["foo".to_string()];
        let config = Configuration::from_inputs(&input, Some("some search".to_string()), None);

        assert_eq!(config.initial_search.as_slice(), "some search");
    }

    #[test]
    fn initial_search_is_optional() {
        let input = vec!["foo".to_string()];
        let config = Configuration::from_inputs(&input, None, None);

        assert_eq!(config.initial_search, "");
    }
}
