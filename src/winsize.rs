extern crate libc;

use std::old_io::{IoResult, standard_error, ResourceUnavailable, File};
use std::os::unix::prelude::AsRawFd;

#[repr(C)]
struct winsize {
    ws_row: libc::c_ushort,     /* rows, in characters */
    ws_col: libc::c_ushort,     /* columns, in characters */
    ws_xpixel: libc::c_ushort,  /* horizontal size, pixels */
    ws_ypixel: libc::c_ushort   /* vertical size, pixels */
}

#[cfg(any(target_os = "macos",
          target_os = "freebsd"))]
const TIOCGWINSZ: libc::c_ulong = 0x40087468;

#[cfg(any(target_os = "linux",
          target_os = "android"))]
const TIOCGWINSZ: libc::c_ulong = 0x00005413;

pub fn get_winsize(file: &File) -> IoResult<(usize, usize)> {
    let w = winsize { ws_row: 0, ws_col: 0, ws_xpixel: 0, ws_ypixel: 0 };
    let r = unsafe { libc::funcs::bsd44::ioctl(file.as_raw_fd(), TIOCGWINSZ, &w) };

    match r {
        0 => Ok((w.ws_col as usize, w.ws_row as usize)),
        _ => {
            return Err(standard_error(ResourceUnavailable))
        }
    }
}
