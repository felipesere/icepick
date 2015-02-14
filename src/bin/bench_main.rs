#![feature(io, path)]

extern crate athena;

use athena::search::Search;
use std::old_io::{File, BufferedReader};

#[allow(dead_code)]
fn main() {
    let lines = read_lines("benches/30000.txt".to_string());

    let mut search = Search::blank(&lines, None, 20);

    search = search.append_to_search("t").append_to_search("o").append_to_search("a").append_to_search("w").append_to_search("c").backspace().backspace().backspace().append_to_search("w").backspace().append_to_search("a");

    println!("\n{}", search.selection().unwrap_or("None".to_string()));
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
