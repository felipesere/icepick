extern crate libc;

use std::old_io::{stdio, File, Open, ReadWrite, Command};
use std::old_io::process::StdioContainer;
use std::os::unix::prelude::AsRawFd;

pub struct TTY {
    file: File,
    dimensions: (usize, usize),
    original_state: String
}

pub trait IO {
    fn write(&mut self, line: &str);
    fn read(&mut self) -> Option<String>;
    fn last(&self) -> &str;
    fn lines(&self) -> Vec<String>;
    fn dimensions(&self) -> (usize, usize);
}

impl IO for TTY {
    fn write(&mut self, line: &str) {
        let it = format!("{}\n", line);
        self.file.write_str(it.as_slice());
    }

    fn read(&mut self) -> Option<String> {
        let res = match self.file.read_byte() {
            Ok(c) => {
                let character = c as char;
                Some(character.to_string())
            },
            Err(_) => None,
        };
        res
    }

    fn last(&self) -> &str {
        "fail"
    }

    fn lines(&self) -> Vec<String> {
        let mut lines: Vec<String> = Vec::new();
        lines.push("fail".to_string());
        lines
    }

    fn dimensions(&self) -> (usize, usize) {
        self.dimensions
    }
}

impl TTY {
    pub fn new() -> TTY {
        let path = Path::new("/dev/tty");
        let file = File::open_mode(&path, Open, ReadWrite).unwrap();
        let dimension = TTY::get_window_size();
        let orig_state = TTY::previous_state(&file);

        TTY::no_echo_no_escaping(&file);

        TTY {
            original_state: orig_state,
            dimensions: dimension,
            file: file,
        }
    }

    pub fn reset(&self) {
        TTY::stty(&self.file, &[self.original_state.as_slice()]);
    }

    fn get_window_size() -> (usize, usize) {
        let (w,h) = stdio::stdout_raw().winsize().unwrap();
        (w as usize, h as usize)
    }

    fn stty(file: &File, args: &[&str]) -> Option<String> {
        let container = StdioContainer::InheritFd(file.as_raw_fd());
        let output = Command::new("stty").args(args).stdin(container).output().unwrap();
        String::from_utf8(output.output).ok()
    }

    fn no_echo_no_escaping(file: &File) {
        TTY::stty(file, &["-echo", "-icanon"]);
    }

    fn previous_state(file: &File) -> String {
        TTY::stty(file, &["-g"]).unwrap_or("".to_string())
    }
}
