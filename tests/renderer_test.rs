#![feature(plugin)]
#![plugin(stainless)]
#![allow(unused_mut)]

extern crate stainless;
extern crate icepick;

mod test {
    pub use icepick::search::Search;
    pub use icepick::text::Text;
    pub use icepick::renderer::Renderer;
    pub use icepick::score::Match;
    pub use icepick::score::Quality;
    pub use icepick::score::Substring;

    describe! renderer_test {
        before_each {
            let choices = vec!["one".to_string(),
                               "one".to_string(),
                               "one".to_string()];

            let renderer = Renderer;
        }

        it "renders_selected_matches_with_a_highlight" {
            let search = Search::blank(&choices, None, 2).down();
            let output = renderer.render(&search);
            let ref text = choices[1];

            assert_eq!(vec![Text::Normal("3 > ".to_string()),
                            Text::Colored(Match::origin_only(text)),
                            Text::Highlight("one".to_string())], output);

        }

        it "renders_a_mismatch" {
            let search = Search::blank(&choices, None, 2).append_to_search("z");
            let output = renderer.render(&search);

            assert_eq!(vec![Text::Normal("0 > z".to_string()),
                            Text::Blank,
                            Text::Blank], output);
        }
    }
}
