use configuration::Configuration;
use search::Search;

#[derive(PartialEq, Show)]
enum Text {
    Normal(String),
    Highlight(String),
    Blank,
}

struct Renderer {
    search: Search,
}

impl Renderer {
    fn new(search: Search) -> Renderer {
        Renderer {
            search: search,
        }
    }

    fn render(&self) -> Vec<Text> {
        let mut result = Vec::new();
        result.push(self.create_header());

        let selection = self.search.selection.clone().unwrap_or("".to_string());

        for position in range(0, self.search.config.visible_limit -1) {
            if position >= self.search.result.len() {
                result.push(Text::Blank);
                continue;
            }

            let choice = self.search.result[position].clone();

            if choice == selection {
                result.push(Text::Highlight(choice));
            } else {
                result.push(Text::Normal(choice))
            }
        }
        result
    }

    fn create_header(&self) -> Text {
        let mut line = String::from_str("> ");
        line.push_str(self.search.query.as_slice());
        Text::Normal(line)
    }
}


#[cfg(test)]

#[test]
fn it_renderes_selected_matches_with_a_highlight() {
    let config = Configuration::from_inputs(vec!["one".to_string(),
                                                 "two".to_string(),
                                                 "three".to_string()], None, Some(3));
    let search = Search::blank(config).down();
    let renderer = Renderer::new(search);

    let output = renderer.render();

    assert_eq!(vec![Text::Normal("> ".to_string()),
                    Text::Normal("one".to_string()),
                    Text::Highlight("two".to_string())], output);
}

#[test]
fn it_renders_a_missmatch() {
    let config = Configuration::from_inputs(vec!["one".to_string(),
                                                 "two".to_string(),
                                                 "three".to_string()], None, Some(3));

    let search = Search::blank(config).append_to_search("z");
    let renderer = Renderer::new(search);

    let output = renderer.render();

    assert_eq!(vec![Text::Normal("> z".to_string()),
                    Text::Blank,
                    Text::Blank], output);
}
