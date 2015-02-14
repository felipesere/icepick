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
