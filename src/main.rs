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
use selecta::text::Text;
use selecta::ansi::Ansi;

fn main() {
    let initial_query = extract_initial_query();
    let lines = read_lines();

    let config = Configuration::from_inputs(lines, initial_query, Some(20));
    let mut search = Search::blank(config);

    let mut tty = TTY::new();
    let mut screen = Screen::new();
    let renderer = Renderer;
    let ansi = Ansi { io: Box::new(TTY::new()) };

   for _ in 0..20 {
       tty.write("\n");
   };


   screen.print(&search);

    while !search.is_done() {
        let input = tty.read();
        match input {
            Some(n) => {
                search = screen.handle_keystroke(search,n.as_slice());
            },
            None => break,
        };
        screen.print(&search);
    }
    match search.selection {
        Some(ref t) => println!("{}\n",t),
        None => println!("None"),
    };
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
