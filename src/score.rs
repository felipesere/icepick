use std::cmp::min;
use std::ops::Range;

#[derive(Clone, Debug, PartialEq)]
pub struct Quality(pub f32);

impl Quality {
    pub fn to_f32(&self) -> f32 {
        let Quality(q) = *self;
        q
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Match<'a> {
    pub quality: Quality,
    pub range: Range<usize>,
    pub original: &'a String,
}

impl<'a> Match<'a> {
    pub fn parts(&self) -> (String, String, String) {
        let start = self.range.start;
        let end = self.range.end;
        let input = self.original;
        (
            input[..start].to_string(),
            input[start..end].to_string(),
            input[end..].to_string(),
        )
    }
}

impl<'a> Match<'a> {
    pub fn new(quality: Quality, range: Range<usize>, original: &'a String) -> Match<'a> {
        Match {
            quality,
            range,
            original,
        }
    }

    pub fn with_empty_range(original: &'a String) -> Match<'a> {
        Match::new(Quality(1.0), Range { start: 0, end: 0 }, original)
    }
}

pub fn score<'a>(choice: &'a String, query: &String) -> Option<Match<'a>> {
    let choice_length = choice.len() as f32;
    let query_length = query.len() as f32;

    if query_length == 0.0 {
        return Some(Match::with_empty_range(choice));
    }
    let lower_choice = choice.to_ascii_lowercase();

    // TODO convert this over to compute_match_length(...).map(...)
    match compute_match_length(&lower_choice, query) {
        Some((start, match_length)) => {
            let quality = Quality((query_length / match_length as f32) / choice_length);
            let substring = Range {
                start,
                end: start + match_length,
            };
            Some(Match::new(quality, substring, choice))
        }
        None => None,
    }
}

fn slice_shift_char(line: &str) -> Option<(char, &str)> {
    if line.is_empty() {
        None
    } else {
        let mut chars = line.chars();
        let ch = chars.next().unwrap();
        let len = line.len();
        let next_s = &line[ch.len_utf8()..len];
        Some((ch, next_s))
    }
}

fn compute_match_length(choice: &str, query: &str) -> Option<(usize, usize)> {
    if query.is_empty() {
        return None;
    }
    let (first, rest) = slice_shift_char(query).unwrap();

    let impossible_match = choice.len() + 1;
    let mut shortest_match = impossible_match;
    let mut shortest_start = impossible_match;

    for_each_beginning(choice, first, |beginning| {
        if let Some(length) = match_length_from(choice, rest, beginning) {
            shortest_match = min(length, shortest_match);
            shortest_start = beginning;
        };
    });

    if shortest_match == impossible_match {
        None
    } else {
        Some((shortest_start, shortest_match))
    }
}

fn for_each_beginning<F: FnMut(usize)>(choice: &str, beginning: char, mut f: F) {
    for (idx, character) in choice.chars().enumerate() {
        if character == beginning {
            f(idx);
        }
    }
}

fn match_length_from(choice: &str, query: &str, beginning: usize) -> Option<usize> {
    let mut match_index = beginning;

    for query_char in query.chars() {
        match find_first_after(choice, query_char, match_index + 1) {
            Some(n) => match_index = n,
            None => return None,
        };
    }
    Some(match_index - beginning + 1)
}

fn find_first_after(choice: &str, query: char, offset: usize) -> Option<usize> {
    choice[offset..].find(query).map(|index| index + offset)
}
