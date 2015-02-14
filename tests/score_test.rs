#![feature(plugin)]
#![plugin(stainless)]

extern crate stainless;
extern crate athena;

mod test {
    use athena::score::score;

    pub fn do_score(choice: &str, query: &str) -> f32 {
       let choice_stirng = choice.to_string();
       let query_stirng = query.to_string();
       score(&choice_stirng,  &query_stirng)
    }

    describe! score_test {
        it "scores_zero_when_the_choice_is_emtpy" {
            assert_eq!(do_score("", "a"), 0.0);
        }

        it "scores_one_when_the_query_is_empty" {
            assert_eq!(do_score("a", ""), 1.0);
        }

        it "scores_zero_if_query_is_longer_than_the_choice" {
            assert_eq!(do_score("a", "aaaaa"), 0.0);
        }

        it "scores_zero_if_query_does_not_match_at_all" {
            assert_eq!(do_score("a", "b"), 0.0);
        }

        it "scores_greater_than_zero_if_query_matches_choice" {
            assert!(do_score("a", "a") > 0.0);
            assert!(do_score("ab", "a") > 0.0);
            assert!(do_score("ba", "a") > 0.0);
            assert!(do_score("bab", "a") > 0.0);
            assert!(do_score("babababab", "aaaa") > 0.0);
        }

        it "normalizes_score_based_on_length" {
            assert_eq!(do_score("a", "a"), 1.0);
            assert_eq!(do_score("ab", "ab"), 0.5);
            assert_eq!(do_score("a long string", "a long string"), 1.0 / "a long string".len() as f32);
            assert_eq!(do_score("spec/search_spec.rb", "sear"), 1.0 / "spec/search_spec.rb".len() as f32)
        }

        it "matches_punctuation" {
            assert!(do_score("/! symbols $^", "/!$^") > 0.0);
        }

        it "repeated_character_does_not_match" {
            assert_eq!(do_score("a", "aa"), 0.0);
        }

        it "scores_higher_for_better_matches" {
              assert!(do_score("ahtena.gemspec", "asp") > do_score("algorithm4_spec.rb", "asp"));
              assert!(do_score("readme.md", "em") > do_score("benchmark.rb", "em"));
              assert!(do_score("search.rb", "sear") > do_score("spec/search_spec.rb", "sear"));
        }

        it "scores_shorter_matches_higher" {
              assert!(do_score("fbb", "fbb") > do_score("foo bar baz", "fbb"));
              assert!(do_score("foo", "foo") > do_score("longer foo", "foo"));
              assert!(do_score("foo", "foo") > do_score("foo longer", "foo"));
              assert!(do_score("1/2/3/4", "1/2/3") > do_score("1/9/2/3/4", "1/2/3"));
        }

        it "scores_the_tighter_of_two_matches_regardless_of_order" {
              let beginning = "121padding2";
              let end = "1padding212";
              assert_eq!(do_score(beginning, "12"), do_score(end, "12"));
        }

        it "tighter_matches_score_higher" {
            assert!(do_score("long 12 long", "12") > do_score("1 long 2", "12"));
        }
    }
}
