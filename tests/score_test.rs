#![feature(plugin)]
#![plugin(stainless)]

extern crate stainless;
extern crate athena;

mod test {
    use athena::score;
    use athena::score::Match;

    pub fn do_new_score(choice: &str, query: &str) -> Option<Match> {
       let choice_stirng = choice.to_string();
       let query_stirng = query.to_string();
       score::score(&choice_stirng,  &query_stirng)
    }

    pub fn match_quality(choice: &str, query: &str) -> f32 {
        let m = do_new_score(choice, query);
        let Quality(quality) = m.unwrap().quality;
        quality
    }

    pub use athena::score::Substring;
    pub use athena::score::Quality;

    describe! score_and_match {
        it "scores_greater_than_zero_and_shows_match" {
            let it = do_new_score("a", "a").unwrap();
            let Substring(a,b) = it.range;
            assert_eq!((a,b), (0,1));
        }

        it "match_range_ends_in_non_inclusive" {
            //                     0123456789
            let it = do_new_score("ana.gemspec", "asp").unwrap();
            let Quality(q) = it.quality;
            let Substring(a,b) = it.range;
            assert!(q > 0.0);
            assert_eq!((a,b), (2,9));
        }
    }

    describe! score_test {
        describe! mismatch {
            it "no_match_when_the_choice_is_emtpy" {
                assert_eq!(do_new_score("", "a"), None);
            }

            it "no_match_if_query_is_longer_than_the_choice" {
                assert_eq!(do_new_score("a", "aaaaa"), None);
            }

            it "no_match_if_query_does_not_match_at_all" {
                assert_eq!(do_new_score("a", "b"), None);
            }

            it "repeated_character_does_not_match" {
                assert_eq!(do_new_score("a", "aa"), None);
            }
        }

        describe! empty_query {
            it "scores_one_when_the_query_is_empty" {
                assert_eq!(match_quality("a", ""), 1.0);
            }
        }

        describe! match_quality {
            it "scores_greater_than_zero_if_query_matches_choice" {
                assert!(match_quality("a", "a") > 0.0);
                assert!(match_quality("ab", "a") > 0.0);
                assert!(match_quality("ba", "a") > 0.0);
                assert!(match_quality("bab", "a") > 0.0);
                assert!(match_quality("babababab", "aaaa") > 0.0);
            }

            it "matches_punctuation" {
                assert!(match_quality("/! symbols $^", "/!$^") > 0.0);
            }

            it "normalizes_score_based_on_length" {
                assert_eq!(match_quality("a", "a"), 1.0);
                assert_eq!(match_quality("ab", "ab"), 0.5);
                assert_eq!(match_quality("a long string", "a long string"), 1.0 / "a long string".len() as f32);
                assert_eq!(match_quality("spec/search_spec.rb", "sear"), 1.0 / "spec/search_spec.rb".len() as f32)
            }


            it "scores_higher_for_better_matches" {
                assert!(match_quality("ahtena.gemspec", "asp") > match_quality("algorithm4_spec.rb", "asp"));
                assert!(match_quality("readme.md", "em") > match_quality("benchmark.rb", "em"));
                assert!(match_quality("search.rb", "sear") > match_quality("spec/search_spec.rb", "sear"));
            }

            it "scores_shorter_matches_higher" {
                assert!(match_quality("fbb", "fbb") > match_quality("foo bar baz", "fbb"));
                assert!(match_quality("foo", "foo") > match_quality("longer foo", "foo"));
                assert!(match_quality("foo", "foo") > match_quality("foo longer", "foo"));
                assert!(match_quality("1/2/3/4", "1/2/3") > match_quality("1/9/2/3/4", "1/2/3"));
            }

            it "scores_the_tighter_of_two_matches_regardless_of_order" {
                let beginning = "121padding2";
                let end = "1padding212";
                assert_eq!(match_quality(beginning, "12"), match_quality(end, "12"));
            }

            it "tighter_matches_score_higher" {
                assert!(match_quality("long 12 long", "12") > match_quality("1 long 2", "12"));
            }
        }
    }
}
