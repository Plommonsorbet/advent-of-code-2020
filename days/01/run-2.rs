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
// scriptisto-end

use std::io::{self, Read};

pub type Result = ::std::result::Result<(), Box<dyn std::error::Error>>;

fn input() -> String {
    let mut buffer = String::new();
    io::stdin()
        .read_to_string(&mut buffer)
        .expect("ERROR: Unable to read from stdin!");

    return buffer;
}

fn main() {
    let sum = 2020;
    let input: Vec<i32> = input().lines().map(|x| x.parse().unwrap()).collect();

    for n1 in &input {
        for n2 in &input {
            for n3 in &input {
                if n1 + n2 + n3 == sum {
                    println!(
                        "RESULT: {i} + {j} + {k} = {} :: {i} * {j} * {k} = {}",
                        sum,
                        n1 * n2 * n3,
                        i = n1,
                        j = n2,
                        k = n3
                    );
		    return
                };
            }
        }
    }
}
