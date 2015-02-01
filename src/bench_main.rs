#![feature(os, collections, rustc_private)]

extern crate getopts;
extern crate selecta;

use getopts::{optopt,getopts};
use selecta::configuration::Configuration;
use selecta::search::Search;
use selecta::tty::TTY;
use selecta::tty::IO;
use selecta::screen::Screen;

#[allow(dead_code)]
fn main() {
    let initial_query = extract_initial_query();
    let lines = read_lines(100000);

    let config = Configuration::from_inputs(lines, initial_query, Some(20));
    let mut search = Search::blank(config);

    search = search.append_to_search("t").backspace().append_to_search("o");
}

fn extract_initial_query() -> Option<String> {
    let args = std::os::args();
    let opts = &[
        optopt("s", "search", "initial search query", ""),
    ];
    let matches = getopts(args.tail(), opts).unwrap();

    matches.opt_str("s")
}

fn one_two_three() -> Vec<String> {
    vec!["one".to_string(),
         "two".to_string(),
         "three".to_string()]
}

fn read_lines(n: usize) -> Vec<String> {
    let mut result: Vec<String> = Vec::new();
    for thing in one_two_three().iter().cycle().take(n) {
        result.push(thing.clone());
    }
    result
}
