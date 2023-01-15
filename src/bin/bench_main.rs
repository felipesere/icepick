extern crate icepick;

use icepick::search::Search;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;

#[allow(dead_code)]
fn main() {
    let lines = read_lines("benches/30000.txt");

    let mut search = Search::blank(&lines, None, 20);

    search = search
        .append_to_search("t")
        .append_to_search("o")
        .append_to_search("a")
        .append_to_search("w")
        .append_to_search("c")
        .backspace()
        .backspace()
        .backspace()
        .append_to_search("w")
        .backspace()
        .append_to_search("a");

    println!(
        "\n{}",
        search.selection().unwrap_or_else(|| "None".to_string())
    );
}

fn read_lines(fname: &str) -> Vec<String> {
    let path = Path::new(fname);
    let mut file = BufReader::new(OpenOptions::new().read(true).open(path).unwrap());
    let mut result = Vec::new();
    loop {
        let mut buf = String::new();
        match file.read_line(&mut buf) {
            Ok(_) if !buf.is_empty() => {
                result.push(buf);
            }
            _ => break,
        }
    }
    result
}
