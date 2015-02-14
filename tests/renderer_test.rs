#![feature(plugin)]
#![plugin(stainless)]
#![allow(unused_mut)]

extern crate stainless;
extern crate athena;

mod test {
    pub use athena::search::Search;
    pub use athena::text::Text;
    pub use athena::renderer::Renderer;

    describe! renderer_test {
        before_each {
            let choices = vec!["one".to_string(),
                             "one".to_string(),
                             "one".to_string()];

            let renderer = Renderer;
        }

        it "renders_selected_matches_with_a_highlight" {
            let search = Search::blank(&choices, None, Some(2)).down();
            let output = renderer.render(&search);

            assert_eq!(vec![Text::Normal("3 > ".to_string()),
                            Text::Normal("one".to_string()),
                            Text::Highlight("one".to_string())], output);
        }

       
        it "renders_a_mismatch" {
            let search = Search::blank(&choices, None, Some(2)).append_to_search("z");
            let output = renderer.render(&search);

            assert_eq!(vec![Text::Normal("0 > z".to_string()),
                            Text::Blank,
                            Text::Blank], output);
        }
    }
}
