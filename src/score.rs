use std::cmp::min;

#[derive(Debug)]
#[derive(PartialEq)]
pub struct Quality(pub f32);

#[derive(Debug)]
#[derive(PartialEq)]
pub struct Substring(pub usize,pub usize);

#[derive(Debug)]
#[derive(PartialEq)]
pub struct Match {
    pub quality: Quality,
    pub range: Substring,
}

impl Match {
    pub fn new(quality: Quality, range: Substring) -> Match {
        Match { quality: quality, range: range }
    }
}

pub fn score(choice: &String, query: &String) -> Option<Match> {
    let choice_length = choice.len() as f32;
    let query_length = query.len() as f32;

    if query_length == 0.0 { return Some(Match { quality: Quality(1.0), range: Substring(0,0) }) }

    match new_compute_match_length(choice, query) {
        Some((start, match_length)) => {
            let quality = Quality( (query_length / match_length as f32) / choice_length);
            let substring = Substring(start, start+match_length);
            Some(Match::new(quality, substring))
        },
        None => None,
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

fn new_compute_match_length(choice: &String, query: &String) -> Option<(usize, usize)> {
    let (first, rest) = match query.slice_shift_char() {
        Some((c, r)) => (c,r),
        None => return None,
    };

    let impossible_match = choice.len() + 1;
    let mut shortest_match = impossible_match;
    let mut shortest_start = impossible_match;

    for_each_beginning(choice, first, |beginning| {
        match match_length_from(choice, rest, beginning) {
            Some(length) => {
                             shortest_match = min(length, shortest_match);
                             shortest_start = beginning;
            },
            None         => {},
        };
    });

    if shortest_match == impossible_match {None} else {Some((shortest_start, shortest_match))}
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
