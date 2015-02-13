#![allow(missing_copy_implementations)]
#![feature(io, std_misc, core, collections, libc, path)]

extern crate libc;

pub mod score;
pub mod search;
pub mod sorted_result_set;
pub mod renderer;

pub mod tty;
pub mod fake_tty;

pub mod ansi;
pub mod text;

pub mod screen;
