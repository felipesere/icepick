#![allow(unstable)]
#![allow(dead_code)]
#![allow(missing_copy_implementations)]
#![feature(io, std_misc, core, collections, libc, path)]

pub mod score;
pub mod search;
pub mod configuration;
pub mod renderer;

pub mod tty;
pub mod fake_tty;

pub mod ansi;
pub mod text;

pub mod screen;
