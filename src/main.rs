#![feature(io, os, core, collections, rustc_private)]

extern crate getopts;
extern crate selecta;

use getopts::{optopt,getopts};
use selecta::configuration::Configuration;
use selecta::search::Search;
use selecta::tty::TTY;
use selecta::tty::IO;
use selecta::screen::Screen;

fn main() {
    let initial_query = extract_initial_query();
    let lines = read_lines();

    let config = Configuration::from_inputs(lines, initial_query, Some(20));
    let mut search = Search::blank(config);

    let mut tty = TTY::new();
    let mut screen = Screen::new();

    screen.print(&search);

    while !search.is_done() {
        let input = tty.read();
        match input {
            Some(n) => {
                search = screen.handle_keystroke(search, n.as_slice());
            },
            None => break,
        };
        screen.print(&search);
    }
    println!("{}\n", search.selection.unwrap_or("None".to_string()));
    screen.ansi.show_cursor();
}

fn extract_initial_query() -> Option<String> {
    let args = std::os::args();
    let opts = &[
        optopt("s", "search", "initial search query", ""),
    ];
    let matches = getopts(args.tail(), opts).unwrap();

    matches.opt_str("s")
}

fn read_lines() -> Vec<String> {
    std::old_io::stdio::stdin().lock().lines().map( |line| {
        line.unwrap()
    }).collect()
}
