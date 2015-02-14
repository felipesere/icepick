use std::cmp::min;

pub fn score(choice: &String, query: &String) -> f32 {
    let choice_length = choice.len() as f32;
    let query_length = query.len() as f32;

    if query_length == 0.0 { return 1.0 }

    match compute_match_length(choice, query) {
        Some(match_length) => (query_length / match_length as f32) / choice_length,
        None => 0.0,
    }
}

fn compute_match_length(choice: &String, query: &String) -> Option<usize> {
    let (first, rest) = match query.slice_shift_char() {
        Some((c, r)) => (c,r),
        None => return None,
    };

    let impossible_match = choice.len() + 1;
    let mut shortest_match = impossible_match;

    for_each_beginning(choice, first, |beginning| {
        match match_length_from(choice, rest, beginning) {
            Some(length) => shortest_match = min(length, shortest_match),
            None         => {},
        };
    });

    if shortest_match == impossible_match {None} else {Some(shortest_match)}
}

fn for_each_beginning<F: FnMut(usize)>(choice: &String, beginning: char, mut f: F) {
    for (idx, character) in choice.chars().enumerate() {
        if character == beginning {
            f(idx);
        }
    }
}

fn match_length_from(choice: &String, query: &str, beginning: usize) -> Option<usize> {
    let mut match_index = beginning;

    for query_char in query.chars() {
       match find_first_after(choice, query_char, match_index + 1) {
           Some(n) => match_index = n,
           None => return None,
       };
    }
    Some(match_index - beginning + 1)
}

fn find_first_after(choice: &String, query: char, offset: usize) -> Option<usize> {
    choice[offset..]
        .find(query)
        .map(|index| index + offset)
}

#[cfg(test)]
mod tests {

    #[test]
    fn scores_zero_when_the_choice_is_emtpy() {
        assert_eq!(score("", "a"), 0.0);
    }

    fn score(choice: &str, query: &str) -> f32 {
       let choice_stirng = choice.to_string();
       let query_stirng = query.to_string();
       super:: score(&choice_stirng,  &query_stirng)
    }

    #[test]
    fn scores_one_when_the_query_is_empty() {
        assert_eq!(score("a", ""), 1.0);
    }

    #[test]
    fn scores_zero_if_query_is_longer_than_the_choice() {
        assert_eq!(score("a", "aaaaa"), 0.0);
    }

    #[test]
    fn scores_zero_if_query_does_not_match_at_all() {
        assert_eq!(score("a", "b"), 0.0);
    }

    #[test]
    fn scores_greater_than_zero_if_query_matches_choice() {
        assert!(score("a", "a") > 0.0);
        assert!(score("ab", "a") > 0.0);
        assert!(score("ba", "a") > 0.0);
        assert!(score("bab", "a") > 0.0);
        assert!(score("babababab", "aaaa") > 0.0);
    }

    #[test]
    fn normalizes_score_based_on_length() {
        assert_eq!(score("a", "a"), 1.0);
        assert_eq!(score("ab", "ab"), 0.5);
        assert_eq!(score("a long string", "a long string"), 1.0 / "a long string".len() as f32);
        assert_eq!(score("spec/search_spec.rb", "sear"), 1.0 / "spec/search_spec.rb".len() as f32)
    }

    #[test]
    fn matches_punctuation() {
        assert!(score("/! symbols $^", "/!$^") > 0.0);
    }

    #[test]
    fn repeated_character_does_not_match() {
        assert_eq!(score("a", "aa"), 0.0);
    }

    #[test]
    fn scores_higher_for_better_matches() {
          assert!(score("ahtena.gemspec", "asp") > score("algorithm4_spec.rb", "asp"));
          assert!(score("readme.md", "em") > score("benchmark.rb", "em"));
          assert!(score("search.rb", "sear") > score("spec/search_spec.rb", "sear"));
    }

    #[test]
    fn scores_shorter_matches_higher() {
          assert!(score("fbb", "fbb") > score("foo bar baz", "fbb"));
          assert!(score("foo", "foo") > score("longer foo", "foo"));
          assert!(score("foo", "foo") > score("foo longer", "foo"));
          assert!(score("1/2/3/4", "1/2/3") > score("1/9/2/3/4", "1/2/3"));
    }

    #[test]
    fn scores_the_tighter_of_two_matches_regardless_of_order() {
          let beginning = "121padding2";
          let end = "1padding212";
          assert_eq!(score(beginning, "12"), score(end, "12"));
    }

    #[test]
    fn tighter_matches_score_higher() {
        assert!(score("long 12 long", "12") > score("1 long 2", "12"));
    }
}
