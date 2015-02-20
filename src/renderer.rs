use search::Search;
use text::Text;

pub struct Renderer;

impl Renderer {
    pub fn render<'a>(&'a self, search: &'a Search) -> Vec<Text> {
        let mut result = Vec::new();
        result.push(Text::Normal(self.header(search)));

        for position in 0..search.visible_limit {
            let element = match search.result.get(position) {
                Some(choice) if position == search.current => Text::Highlight(choice.original.clone()),
                Some(choice) => Text::Colored(choice.clone()),
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
