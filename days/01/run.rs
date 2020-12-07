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

fn input() -> String {
    std::fs::read_to_string("input").unwrap()
}

fn part_1() {
    println!("P1:");
    let sum = 2020;
    let input: Vec<i32> = input().lines().map(|x| x.parse().unwrap()).collect();

    for x in &input {
        for y in &input {
            if x + y == sum {
                println!("CALCULATION: {} + {} = {}", x, y, sum);
                println!("ANSWER: {}", x * y);
                return;
            }
        }
    }
}
fn part_2() {
    println!("\nP2:");
    let sum = 2020;
    let input: Vec<i32> = input().lines().map(|x| x.parse().unwrap()).collect();

    for x in &input {
        for y in &input {
            for z in &input {
                if x + y + z == sum {
                    println!("CALCULATION: {} + {} + {} = {}", x, y, z, sum);
                    println!("ANSWER: {}", x * y * z);
                    return;
                };
            }
        }
    }
}

fn main() {
    part_1();
    part_2();
}
