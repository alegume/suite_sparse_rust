#![allow(dead_code)]
use std::time::Instant;
// use std::time::{Duration};
// use std::thread::sleep;
mod matrix_csr;
mod read_files;

fn main() {

    let files = vec![
    // "apache2.mtx", // ~2.8M
    // "pwtk.mtx", //~6M
    // "Roget.mtx", // ~5k
    // "nasa2910.mtx", // ~88k
    "will199.mtx",
    "mcca.mtx",
    "lns__131.mtx",
    "bcspwr01.mtx",
    "test2.mtx",
    "test3.mtx",
    "test1.mtx",
    ];
    println!("instancia, n, bw_0, bw_1, tempo(ms)");
    for file in files {
        experimentation(file);
    }
}

fn experimentation(file: &str) {
    let now = Instant::now();
    let mut matrix = matrix_csr::mm_file_to_csr(file);
    let bw_0 = matrix.bandwidth();
    matrix.cmr(matrix.col_index[0]);
    let bw_1 = matrix.bandwidth();
    let total_time = now.elapsed().as_millis();
    println!("{}, {}, {}, {}, {}", file, matrix.m, bw_0, bw_1, total_time);
}