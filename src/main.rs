#![feature(io, os, env, core, collections, rustc_private)]

extern crate getopts;
extern crate selecta;

use getopts::{optopt,getopts};
use std::cmp::min;

use selecta::search::Search;
use selecta::tty::IO;
use selecta::screen::Screen;

#[allow(dead_code)]
fn main() {
    let initial_query = extract_initial_query();
    let lines = read_lines();
    let mut screen = Screen::new();

    let height = min(20, screen.height);
    let mut search = Search::blank(&lines, initial_query, Some(height));

    screen.blank(search.visible_limit);

    while !search.is_done() {
        screen.print(&search);
        let input = screen.ansi.io.read();
        match input {
            Some(character) => {
                search = screen.handle_keystroke(search, character.as_slice());
            },
            None => break,
        };
    }
    screen.move_cursor_to_end();
    screen.ansi.io.reset();
    println!("{}", search.selection().unwrap_or("".to_string()));
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
    std::old_io::stdio::stdin().lock().lines().map( |line| {
        line.unwrap().trim().to_string()
    }).collect()
}
