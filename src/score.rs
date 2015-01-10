use regex::Regex;

struct Score;

impl Score {
    pub fn score(choice: &str, query: &str) -> f32 {
        let choice_length = choice.len();
        let query_length = query.len();
        if choice_length == 0 || query_length > choice_length  {
            return 0.0
        }
        if query_length == 0  {
            return 1.0
        }

        if create_regex(query).is_match(choice) {
            return 1.0 / (choice_length as f32)
        }

        0.0
    }
}

fn create_regex(query: &str) -> Regex {
   match Regex::new(expand_regex(query).as_slice()) {
            Ok(re) => re,
            Err(err) => panic!("{}", err),
   }
}

fn expand_regex(query: &str) -> String {
    let mut result = "(?iU).*".to_string();
    for c in query.chars() {
        result.push(c);
        result.push_str(".*");
    }
    result
}

#[cfg(test)]

#[test]
fn scores_zero_when_the_choice_is_emtpy() {
    assert_eq!(Score::score("", "a"), 0.0);
}

#[test]
fn scores_one_when_the_query_is_empty() {
    assert_eq!(Score::score("a", ""), 1.0);
}

#[test]
fn scores_zero_if_query_is_longer_than_the_choice() {
    assert_eq!(Score::score("a", "aaaaa"), 0.0);
}

#[test]
fn scores_zero_if_query_does_not_match_at_all() {
    assert_eq!(Score::score("a", "b"), 0.0);
}

#[test]
fn scores_greater_than_zero_if_query_matches_choice() {
    assert!(Score::score("a", "a") > 0.0);
    assert!(Score::score("ab", "a") > 0.0);
    assert!(Score::score("ba", "a") > 0.0);
    assert!(Score::score("bab", "a") > 0.0);
    assert!(Score::score("babababab", "aaaa") > 0.0);
}

#[test]
fn case_insensitive_matching() {
    assert!(Score::score("a", "A") == 1.0);
    assert!(Score::score("A", "a") == 1.0);
}

#[test]
fn normalizes_score_based_on_length() {
    assert_eq!(Score::score("a", "a") , 1.0);
    assert_eq!(Score::score("ab", "ab") , 0.5);
    assert_eq!(Score::score("a long string", "a long string") , 1.0 / "a long string".len() as f32);
    assert_eq!(Score::score("spec/search_spec.rb", "sear"), 1.0 / "spec/search_spec.rb".len() as f32)
}

//#[test]
fn matches_punctuation() {
    assert!(Score::score("/! symbols $^", "/!$^") > 0.0);
}

#[test]
fn repeated_character_does_not_match() {
    assert_eq!(Score::score("a", "aa") , 0.0);
}

#[test]
fn scores_higher_for_better_matches() {
      assert!(Score::score("selecta.gemspec", "asp") > Score::score("algorithm4_spec.rb", "asp"));
      assert!(Score::score("README.md", "em") > Score::score("benchmark.rb", "em"));
      assert!(Score::score("search.rb", "sear") > Score::score("spec/search_spec.rb", "sear"));
}

#[test]
fn scores_shorter_matches_higher() {
      assert!(Score::score("fbb", "fbb") > Score::score("foo bar baz", "fbb"));
      assert!(Score::score("foo", "foo") > Score::score("longer foo", "foo"));
      assert!(Score::score("foo", "foo") > Score::score("foo longer", "foo"));
      assert!(Score::score("1/2/3/4", "1/2/3") > Score::score("1/9/2/3/4", "1/2/3"));
}

#[test]
fn scores_the_tighter_of_two_matches_regardless_of_order() {
      let beginning = "121padding2";
      let end = "1padding212";
      assert_eq!(Score::score(beginning, "12"), Score::score(end, "12"));
}

