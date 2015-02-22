#![allow(missing_copy_implementations)]
#![feature(io, std_misc, core, collections, libc, fs, old_io, old_path)]

extern crate libc;
extern crate ansi_term;

pub mod score;
pub mod search;
pub mod sorted_result_set;
pub mod renderer;

pub mod tty;
pub mod fake_tty;

pub mod ansi;
pub mod text;

pub mod screen;
