use configuration::Configuration;
use search::Search;

#[derive(PartialEq, Show)]
enum Text {
    Normal(String),
    Highlight(String),
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

    fn render(self) -> Vec<Text> {
        let mut result = Vec::new();
        result.push(Text::Normal("> ".to_string()));

        let selection = self.search.selection.unwrap_or("".to_string());
        let max_results = self.search.config.visible_limit - 1;

        for position in range(0, max_results) {
            let choice = self.search.result[position].clone();

            if choice == selection {
                result.push(Text::Highlight(choice));
            } else {
                result.push(Text::Normal(choice))
            }
        }
        result
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
//  it "renders selected matches" do
//    search = Search.blank(config).down
//    renderer = Renderer.new(search)
//    expect(renderer.render.choices).to eq [
//      "> ",
//      "one",
//      Text[:inverse, "two", :reset],
//    ]
//  end
