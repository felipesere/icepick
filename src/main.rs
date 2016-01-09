extern crate getopts;
extern crate icepick;

use getopts::Options;
use std::io::BufRead;
use std::io;

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
    let mut opts = Options::new();
    opts.optopt("s", "search", "initial search query", "");

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m }
        Err(f) => { panic!(f.to_string()) }
    };

    matches.opt_str("s")
}

fn get_args() -> Vec<String> {
    std::env::args().collect()
}

fn read_lines() -> Vec<String> {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let l = reader.lines().map( |line| {
        line.unwrap().trim().to_string()
    }).collect();
    l
}
