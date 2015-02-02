pub struct Score;

impl Score {
    pub fn score(choice: &str, query: &str) -> f32 {
        let choice_length = choice.len() as f32;
        let query_length = query.len() as f32;

        if query_length == 0.0 { return 1.0 }

        let possible_match_length = compute_match_length(choice.as_slice(), query.as_slice());
        match possible_match_length {
            Some(match_length) => {
                (query_length / match_length as f32) / choice_length
            },
            None => 0.0,
        }
    }
}

fn compute_match_length(choice: &str, query: &str) -> Option<usize> {
    let (first, rest) = match query.slice_shift_char() {
        Some((c, r)) => (c,r),
        None => return None,
    };

    let match_beginnings = find_positions(first, choice);

    let mut shortest_match: Option<usize> = None;
    for beginning in match_beginnings.iter() {
        let possible_match_length = find_match_length(choice, rest, *beginning);

        match (shortest_match, possible_match_length) {
            (Some(shortest), Some(length)) if shortest > length => shortest_match = possible_match_length,
            (None, Some(_)) => shortest_match = possible_match_length,
            (_, _) => continue,
        };
    }
    return shortest_match;
}

fn find_positions(first_char: char, choice: &str) -> Vec<usize> {
    let mut found: Vec<usize> = Vec::new();
    for (idx, character) in choice.chars().enumerate() {
        if character == first_char {
            found.push(idx);
        }
    }

    return found;
}

fn find_match_length(choice: &str, query: &str, beginning: usize) -> Option<usize> {
    let mut last_index = beginning;
    for query_char in query.chars() {
       let found = find_first_after(choice, query_char, last_index + 1);
       match found {
           Some(n) => last_index = n,
           None => return None,
       };
    }
    return Some(last_index - beginning + 1);
}

fn find_first_after(choice: &str, query: char, offset: usize) -> Option<usize> {
    match choice[offset..].find(query) {
        Some(index) => Some(index + offset),
        None => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
    fn normalizes_score_based_on_length() {
        assert_eq!(Score::score("a", "a"), 1.0);
        assert_eq!(Score::score("ab", "ab"), 0.5);
        assert_eq!(Score::score("a long string", "a long string"), 1.0 / "a long string".len() as f32);
        assert_eq!(Score::score("spec/search_spec.rb", "sear"), 1.0 / "spec/search_spec.rb".len() as f32)
    }

    #[test]
    fn matches_punctuation() {
        assert!(Score::score("/! symbols $^", "/!$^") > 0.0);
    }

    #[test]
    fn repeated_character_does_not_match() {
        assert_eq!(Score::score("a", "aa"), 0.0);
    }

    #[test]
    fn scores_higher_for_better_matches() {
          assert!(Score::score("selecta.gemspec", "asp") > Score::score("algorithm4_spec.rb", "asp"));
          assert!(Score::score("readme.md", "em") > Score::score("benchmark.rb", "em"));
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

    #[test]
    fn tighter_matches_score_higher() {
        assert!(Score::score("long 12 long", "12") > Score::score("1 long 2", "12"));
    }
}
