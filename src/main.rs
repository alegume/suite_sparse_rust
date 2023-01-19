#![allow(dead_code)]
use std::time::{Instant};
// use std::time::{Duration};
// use std::thread::sleep;
mod matrix_csr;


fn main() {
    let now = Instant::now();
    // let file = "apache2.mtx";
    // let file = "pwtk.mtx";
    // let file = "will199.mtx";
    // let file = "mcca.mtx";
    // let file = "bcspwr01.mtx";
    // let file = "lns__131.mtx";
    // let file = "test1.mtx";
    let file = "test2.mtx";
    let matrix = matrix_csr::mm_file_to_csr(file);
    println!("Time to create Matrix = {}ms", now.elapsed().as_millis());
    // println!("{:?}", matrix);
    let now = Instant::now();
    println!("Time to compute BW= {}ms", now.elapsed().as_millis());
    println!("BW: {}", matrix.bandwidth());
    let now = Instant::now();
    matrix.cmr();
    println!("Time of CMr= {}ms", now.elapsed().as_millis());
    println!("BW: {}", matrix.bandwidth());
    // sleep(Duration::new(5, 0));
}