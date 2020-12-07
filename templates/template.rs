#!/usr/bin/env scriptisto
// scriptisto-begin
// script_src: src/main.rs
// build_cmd: cargo build --release && strip ./target/release/script
// target_bin: ./target/release/script
// files:
//  - path: Cargo.toml
//    content: |
//     package = { name = "script", version = "0.1.0", edition = "2018"}
//     [dependencies]
//     itertools = "0.9.0"
//
// scriptisto-end

use itertools::Itertools;

fn input() -> String {
    std::fs::read_to_string("input").unwrap()
}

fn part_1() {
    println!("P1:");
    println!("CALCULATION: ");
    println!("ANSWER: ");
}

fn part_2() {
    println!("\nP2:");
    println!("CALCULATION: ");
    println!("ANSWER: ");
}

fn part_3() {
    println!("\nP3:");
    println!("CALCULATION: ");
    println!("ANSWER: ");
}

fn main() {
    part_1();
    //part_2();
    //part_3();
}
