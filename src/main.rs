#![feature(old_io, env, collections, rustc_private)]

extern crate getopts;
extern crate icepick;

use getopts::{optopt,getopts};
use std::old_io::stdio;

use icepick::screen::Screen;

#[allow(dead_code)]
fn main() {
    let initial_query = extract_initial_query();
    let lines = read_lines();
    let mut screen = Screen::new();

    let result = screen.run_search(lines, initial_query);
    screen.move_cursor_to_end();
    screen.reset();
    println!("{}", result.unwrap_or("".to_string()));
}

fn extract_initial_query() -> Option<String> {
    let args: Vec<String> = get_args();
    let opts = &[
        optopt("s", "search", "initial search query", ""),
    ];
    let matches = getopts(args.tail(), opts).unwrap();

    matches.opt_str("s")
}

fn get_args() -> Vec<String> {
    std::env::args().collect()
}

fn read_lines() -> Vec<String> {
    let mut stdin = stdio::stdin();
    let mut reader = stdin.lock();
    reader.lines().map( |line| {
        line.unwrap().trim().to_string()
    }).collect()
}
