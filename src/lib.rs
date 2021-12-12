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
mod day_03;
mod day_04;
mod day_05;
mod day_06;
mod day_07;
mod day_08;
mod day_09;
mod day_10;
mod day_11;
mod day_12;

aoc_lib! { year = 2021 }
