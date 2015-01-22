#![allow(unstable)]

use std::io::{File, Open, ReadWrite, Command};
use std::io::process::StdioContainer;
use std::os::unix::prelude::AsRawFd;

use winsize;

struct TTY {
    file: File,
    original_state: String,
    dimensions: (usize, usize)
}

impl TTY {
    pub fn new() -> TTY {
        let path = Path::new("/dev/tty");
        let file = match File::open_mode(&path, Open, ReadWrite) {
                Ok(f) => f,
                Err(e) => panic!("file error: {}", e),
        };
        TTY::no_echo_no_escaping(&file);

        TTY {
            original_state: TTY::previous_state(&file),
            dimensions: winsize::get_winsize(&file).unwrap(),
            file: file,
        }
    }

    fn stty(file: &File, args: &[&str]) -> Option<String> {
        let container = StdioContainer::InheritFd(file.as_raw_fd());
        let output = match Command::new("stty").args(args).stdin(container).output() {
            Ok(r) => r,
            Err(e) => panic!("failed on process: {}", e),
        };
        String::from_utf8(output.output).ok()
    }

    fn no_echo_no_escaping(file: &File) {
        TTY::stty(file, &["-echo", "-icanon"]);
    }

    fn previous_state(file: &File) -> String {
        TTY::stty(file, &["-g"]).unwrap_or("".to_string())
    }

    fn reset(&mut self) {
        TTY::stty(&self.file, &[self.original_state.as_slice()]);
    }

    pub fn write(&mut self, line: &str) {
        self.file.write_str(line);
    }

    pub fn read(&mut self) -> Option<char> {
        let res = match self.file.read_byte() {
            Ok(c) => Some(c as char),
            Err(_) => None,
        };
        res
    }
}

#[cfg(test)]

#[test]
fn winsize_has_valid_width_and_height() {
    let tty = TTY::new();
    let (width, height) = tty.dimensions;
    assert!(width > 0);
    assert!(height > 0);
}

#[test]
fn can_read_and_write() {
    let mut tty = TTY::new();
    tty.write("#### winning ####\n");
   // let ch = tty.read();
   // println!("[{:?}]", ch);

    println!("dimensions: {:?}", tty.dimensions);
}
