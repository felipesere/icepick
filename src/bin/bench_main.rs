#![feature(os, collections, rustc_private)]

extern crate getopts;
extern crate selecta;

use getopts::{optopt,getopts};
use selecta::configuration::Configuration;
use selecta::search::Search;
use selecta::tty::TTY;
use selecta::tty::IO;
use selecta::screen::Screen;
use std::old_io::{File, BufferedReader};

#[allow(dead_code)]
fn main() {
    let initial_query = extract_initial_query();
    let lines = read_lines("benches/30000.txt".to_string());

    let config = Configuration::from_inputs(lines, initial_query, Some(20));
    let mut search = Search::blank(&config);

    search = search.append_to_search("t").append_to_search("o").append_to_search("a").append_to_search("w").append_to_search("c").backspace().backspace().backspace().append_to_search("w").backspace().append_to_search("a");

    println!("\n{}", search.selection().unwrap_or("None".to_string()));
}

fn extract_initial_query() -> Option<String> {
    let args = std::os::args();
    let opts = &[
        optopt("s", "search", "initial search query", ""),
    ];
    let matches = getopts(args.tail(), opts).unwrap();

    matches.opt_str("s")
}

fn read_lines(fname: String) -> Vec<String> {
    let path = Path::new(fname);
    let mut file = BufferedReader::new(File::open(&path));
    let mut result = Vec::new();
    loop {
        match file.read_line() {
            Ok(line) => result.push(line),
            Err(_) => break,
        }
    }
    result
}
