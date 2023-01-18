#![allow(dead_code)]
use std::time::{Instant};
// use std::time::{Duration};
// use std::thread::sleep;
mod matrix_csr;


fn main() {
    let now = Instant::now();
    // let file = "pwtk.mtx";
    // let file = "lp_nug05.mtx";
    // let file = "test1.mtx";
    // let file = "test2.mtx";
    let file = "bcspwr01.mtx";
    // let file = "apache2.mtx";
    // let file = "lns__131.mtx";
    let matrix = matrix_csr::mm_file_to_csr(file);
    // println!("{:?}", matrix);
    println!("BW: {}", matrix.bandwidth());
    // matrix.cmr();
    // println!("BW: {}", matrix.bandwidth());
    println!("time = {}ms", now.elapsed().as_millis());
    // sleep(Duration::new(5, 0));
}