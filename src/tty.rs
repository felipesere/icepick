#![allow(unstable)]

extern crate libc;

use std::io::{IoResult, standard_error, ResourceUnavailable, File, Open, ReadWrite};

struct TTY {
    file: File
}

impl TTY {
    pub fn new() -> TTY {
        let path = Path::new("/dev/tty");
        let file = match File::open_mode(&path, Open, ReadWrite) {
                Ok(f) => f,
                Err(e) => panic!("file error: {}", e),
        };

        TTY { file: file }
    }
}

#[repr(C)]
struct winsize {
    ws_row: libc::c_ushort,     /* rows, in characters */
    ws_col: libc::c_ushort,     /* columns, in characters */
    ws_xpixel: libc::c_ushort,  /* horizontal size, pixels */
    ws_ypixel: libc::c_ushort   /* vertical size, pixels */
}

const TIOCGWINSZ: libc::c_ulong = 0x40087468;

pub fn get_winsize() -> IoResult<(isize, isize)> {
    let w = winsize { ws_row: 0, ws_col: 0, ws_xpixel: 0, ws_ypixel: 0 };
    let r = unsafe { libc::funcs::bsd44::ioctl(libc::STDOUT_FILENO, TIOCGWINSZ, &w) };

    match r {
        0 => Ok((w.ws_col as isize, w.ws_row as isize)),
        _ => {
            return Err(standard_error(ResourceUnavailable))
        }
    }
}

#[cfg(test)]

#[test]
fn can_create_a_tty() {
    let tty = TTY::new();
}

#[test]
fn winsize_has_valid_width_and_height() {
    let (width, height) = get_winsize().unwrap();
    assert!(width > 0);
    assert!(height > 0);

}
