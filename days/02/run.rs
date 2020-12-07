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
    let result = input().lines().filter(|line| {
	let arr: Vec<&str> = line.split(|c| c == '-' || c == ' ' || c == ':').collect();

        let p = arr[2].chars().next().unwrap();
	let min: i32 = arr[0].parse().unwrap();
	let max: i32 = arr[1].parse().unwrap();

        let count = arr[4].chars().filter(|&c| c == p).count() as i32;

	return min <= count && max >= count
    }).count();

    println!("P1:");
    println!("ANSWER: {}", result);
}

fn part_2() {
    let result = input().lines().filter(|line| {
	let arr: Vec<&str> = line.split(|c| c == '-' || c == ' ' || c == ':').collect();

        let p = arr[2].chars().next().unwrap();
	let i = (arr[0].parse::<i32>().unwrap() - 1) as usize;
	let j = (arr[1].parse::<i32>().unwrap() - 1) as usize;

	
	let first = match arr[4].chars().nth(i) {
	    Some(c) => c == p,
	    None => false
	};
	let second = match arr[4].chars().nth(j) {
	    Some(c) => c == p,
	    None => false
	};

	return (first || second ) && ( first != second )

    }).count();
    println!("\nP2:");
    println!("ANSWER: {}", result);
}

fn main() {
    part_1();
    part_2();
}
