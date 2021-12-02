#![allow(clippy::bool_comparison)]
#![allow(clippy::needless_bool)]
#![allow(dead_code)]
#![forbid(unsafe_code)]

#[macro_use]
extern crate aoc_runner_derive;

use aoc_runner_derive::aoc_lib;

mod common;
mod day_01;
mod day_02;

aoc_lib! { year = 2021 }
