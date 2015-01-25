#![allow(unstable)]

extern crate getopts;
extern crate selecta;

use getopts::{optopt ,getopts};
use selecta::configuration::Configuration;
use selecta::search::Search;
use selecta::renderer::Renderer;
use selecta::tty::TTY;
use selecta::tty::IO;
use selecta::screen::Screen;

fn main() {
    let initial_query = extract_initial_query();
    let lines = read_lines();

    let config = Configuration::from_inputs(lines, initial_query, Some(20));
    let mut search = Search::blank(config);

    let mut tty = TTY::new();
    let screen = Screen;
    let renderer = Renderer;

    while !search.is_done() {
        let input = tty.read();
        match input {
            Some(n) => {
                search = screen.handle_keystroke(search,n.as_slice());
            },
            None => break,
        };
        let result = renderer.render(&search);
        println!("{:?}", result);
    }


    let result = renderer.render(&search);
    println!("{:?}", result);
}

fn extract_initial_query() -> Option<String> {
    let args: Vec<String> = std::os::args();
    let opts = &[
        optopt("s", "search", "initial search query", ""),
    ];
    let matches = match getopts(args.tail(), opts) {
        Ok(m) => { m }
        Err(f) => { panic!(f.to_string()) }
    };

    matches.opt_str("s")
}

fn read_lines() -> Vec<String> {
    let mut lines: Vec<String> = Vec::new();
    let mut reader = std::io::stdio::stdin();

    loop {
        match reader.read_line() {
            Err(_) => break,
            Ok(l) => {
                let message = l.trim_left().trim_right();
                lines.push(message.to_string());
            },
        };
    };
    lines
}
