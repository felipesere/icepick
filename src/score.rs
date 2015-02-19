use std::cmp::min;

pub fn score(choice: &String, query: &String) -> f32 {
    let choice_length = choice.len() as f32;
    let query_length = query.len() as f32;

    let chars = choice.chars().collect::<Vec<char>>();
    if query_length == 0.0 { return 1.0 }

    match compute_match_length(&chars[], query) {
        Some(match_length) => (query_length / match_length as f32) / choice_length,
        None => 0.0,
    }
}

fn compute_match_length(choice: &[char], query: &String) -> Option<usize> {
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

fn for_each_beginning<F: FnMut(usize)>(choice: &[char], beginning: char, mut f: F) {
    for (idx, character) in choice.iter().enumerate() {
        if *character == beginning {
            f(idx);
        }
    }
}

fn match_length_from(choice: &[char], query: &str, beginning: usize) -> Option<usize> {
    let mut match_index = beginning;

    for query_char in query.chars() {
       match find_first_after(choice, query_char, match_index + 1) {
           Some(n) => match_index = n,
           None => return None,
       };
    }
    Some(match_index - beginning + 1)
}

fn find_first_after(choice: &[char], query: char, offset: usize) -> Option<usize> {
    for (idx, c) in choice[offset..].iter().enumerate() {
        if *c == query {
            return Some(idx + offset);
        }
    }
    None
}
