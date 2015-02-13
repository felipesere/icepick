use search::Search;
use text::Text;

pub struct Renderer;

impl Renderer {
    pub fn render(&self, search: &Search) -> Vec<Text> {
        let mut result = Vec::new();
        result.push(Text::Normal(self.header(search)));

        for position in 0..search.visible_limit {
            let element = match search.result.get(position) {
                Some(choice) if position == search.current => Text::Highlight(choice.clone()),
                Some(choice) => Text::Normal(choice.clone()),
                None => Text::Blank
            };
            result.push(element);
        }
        result
    }

    pub fn header(&self, search: &Search) -> String {
        format!("{} > {}", search.num_matches(), search.query)
    }
}

#[cfg(test)]
mod tests {
    use search::Search;
    use text::Text;
    use super::*;

    #[test]
    fn it_renders_selected_matches_with_a_highlight() {
        let input = vec!["one".to_string(),
                         "one".to_string(),
                         "one".to_string()];

        let search = Search::blank(&input, None, Some(2)).down();
        let renderer = Renderer;

        let output = renderer.render(&search);

        assert_eq!(vec![Text::Normal("3 > ".to_string()),
                        Text::Normal("one".to_string()),
                        Text::Highlight("one".to_string())], output);
    }

    #[test]
    fn it_renders_a_mismatch() {
        let input = vec!["one".to_string(),
                         "two".to_string(),
                         "three".to_string()];

        let search = Search::blank(&input, None, Some(2)).append_to_search("z");
        let renderer = Renderer;

        let output = renderer.render(&search);

        assert_eq!(vec![Text::Normal("0 > z".to_string()),
                        Text::Blank,
                        Text::Blank], output);
    }
}
