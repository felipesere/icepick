use std::io::prelude::*;
use std::fs::{File, OpenOptions};
use std::process::{Command, Stdio};
use std::os::unix::AsRawFd;
use libc::{c_ushort, c_int, c_ulong};
use std::str;


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
    fn reset(&self);
}

impl IO for TTY {
    fn write(&mut self, line: &str) {
        let it = format!("{}", line);
        self.file.write(it.as_bytes()).unwrap();
    }

    fn read(&mut self) -> Option<String> {
        let mut buffer = [0] ;
        let res = match self.file.read(&mut buffer) {
            Ok(c) if  c > 0 => {
                str::from_utf8(&buffer).ok().map(|x| x.to_string())
            },
            _ => None,
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

    fn reset(&self) {
        TTY::stty(&self.file, &[self.original_state.as_slice()]);
    }
}

impl TTY {
    pub fn new() -> TTY {
        let path = Path::new("/dev/tty");
        let file = OpenOptions::new().read(true).write(true).open(&path).unwrap();
        let dimension = TTY::get_window_size(&file);
        let orig_state = TTY::previous_state(&file);

        TTY::no_echo_no_escaping(&file);

        TTY {
            original_state: orig_state,
            dimensions: dimension,
            file: file,
        }
    }

    fn get_window_size(path: &File) -> (usize, usize) {
        extern {
            fn ioctl(fd: c_int, request: c_ulong, ...) -> c_int;
        }
        #[cfg(any(target_os = "macos", target_os = "freebsd"))]
        const TIOCGWINSZ: c_ulong = 0x40087468;

        #[cfg(any(target_os = "linux", target_os = "android"))]
        const TIOCGWINSZ: c_ulong = 0x00005413;

        #[repr(C)]
        struct TermSize {
            rows: c_ushort,
            cols: c_ushort,
            x: c_ushort,
            y: c_ushort,
        }

        let size = TermSize { rows: 0, cols: 0, x: 0, y: 0 };
        if unsafe { ioctl(path.as_raw_fd(), TIOCGWINSZ, &size) } == 0 {
            (size.cols as usize, size.rows as usize)
        } else {
            panic!("Could not read winsize from /dev/tty")
        }
    }

    fn stty(file: &File, args: &[&str]) -> Option<String> {
        let output = Command::new("stty").args(args).stdin(Stdio::capture()).output().unwrap();
        String::from_utf8(output.stdout).ok()
    }

    fn no_echo_no_escaping(file: &File) {
        TTY::stty(file, &["-echo", "-icanon"]);
    }

    fn previous_state(file: &File) -> String {
        TTY::stty(file, &["-g"]).unwrap_or("".to_string())
    }
}
