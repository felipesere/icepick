use std::cmp::min;
use std::ascii::AsciiExt;

#[derive(Clone, Debug,PartialEq)]
pub struct Quality(pub f32);

impl Quality {
    pub fn to_f32(&self) -> f32 {
        let Quality(q) = *self;
        q
    }
}

#[derive(Clone, Debug,PartialEq)]
pub struct Substring(pub usize,pub usize);

impl Substring {
    pub fn slice(&self, input: &String) -> (String, String, String) {
       let Substring(start, end) = *self;
       (input[0..start].to_string(),
        input[start..end].to_string(),
        input[end..].to_string())
    }
}

#[derive(Clone, Debug,PartialEq)]
pub struct Match<'a> {
    pub quality: Quality,
    pub range: Substring,
    pub original: &'a String,

}

impl <'a>Match<'a>{
    pub fn new(quality: Quality, range: Substring, original: &'a String) -> Match<'a> {
        Match { quality: quality, range: range, original: original }
    }

    pub fn origin_only(original: &'a String) -> Match<'a> {
        Match::new(Quality(1.0), Substring(0,0), original)
    }
}

pub fn score<'a>(choice: &'a String, query: &String) -> Option<Match<'a>> {
    let choice_length = choice.len() as f32;
    let query_length = query.len() as f32;

    if query_length == 0.0 { return Some(Match::new(Quality(1.0), Substring(0,0), choice)) }
    let lower_choice = choice.to_ascii_lowercase();

    match compute_match_length(&lower_choice, query) {
        Some((start, match_length)) => {
            let quality = Quality( (query_length / match_length as f32) / choice_length);
            let substring = Substring(start, start+match_length);
            Some(Match::new(quality, substring, choice))
        },
        None => None,
    }
}

fn compute_match_length(choice: &String, query: &String) -> Option<(usize, usize)> {
    if query.len() == 0 {
        return None;
    }
    let (first, rest) = query.slice_shift_char().unwrap();

    let impossible_match = choice.len() + 1;
    let mut shortest_match = impossible_match;
    let mut shortest_start = impossible_match;

    for_each_beginning(choice, first, |beginning| {
        match match_length_from(choice, rest, beginning) {
            Some(length) => {
                             shortest_match = min(length, shortest_match);
                             shortest_start = beginning;
            },
            None => {},
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
