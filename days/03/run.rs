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


fn trees_on_route(map_width: usize, mv_x: usize, mv_y: usize) -> usize {
    input()
        .lines()
        .enumerate()
        .filter(|(y, row)| y % mv_y == 0)
        .filter_map(|(y, row)| row.chars().nth(((mv_x * (y / mv_y)) % (map_width - 1))))
        .filter(|&tile| tile == '#')
        .count()
}

fn part_1() {
    let map_width = 32;
    let mv_x = 3;
    let mv_y = 1;

    let result = trees_on_route(map_width, mv_x, mv_y);

    println!("P1:");
    println!("ANSWER: {}", result);
}

fn part_2() {
    let map_width = 32;
    let result = trees_on_route(map_width, 1, 1)
        * trees_on_route(map_width, 3, 1)
        * trees_on_route(map_width, 5, 1)
        * trees_on_route(map_width, 7, 1)
        * trees_on_route(map_width, 1, 2);
    println!("\nP2:");
    println!("ANSWER: {}", result);
}

fn main() {
    part_1();
    part_2();
}
