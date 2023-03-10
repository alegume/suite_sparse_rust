#![allow(dead_code)]
use std::time::Instant;
// use std::time::{Duration};
// use std::thread::sleep;
mod matrix_csr;
mod read_files;

fn main() {
    // let file = "apache2.mtx"; // ~2.8M
    // let file = "pwtk.mtx"; //~6M
    // let file = "Roget.mtx"; // ~5k
    // let file = "nasa2910.mtx"; // ~88k
    // let file = "will199.mtx";
    // let file = "mcca.mtx";
    // let file = "lns__131.mtx";
    // let file = "bcspwr01.mtx";
    let file = "test2.mtx";
    // let file = "test3.mtx";
    // let file = "test1.mtx";

    let now = Instant::now();
    let mut matrix = matrix_csr::mm_file_to_csr(file);
    println!("Time to create Matrix = {}ms", now.elapsed().as_millis());
    // println!("{:?}", matrix);

    let now = Instant::now();
    println!("BW: {}", matrix.bandwidth());
    println!("Time to compute BW= {}ms", now.elapsed().as_millis());

    let now = Instant::now();
    matrix.cmr(matrix.col_index[0]);
    println!("Time of CMr= {}ms", now.elapsed().as_millis());
    // println!("{:?}", matrix);

    let now = Instant::now();
    println!("BW: {}", matrix.bandwidth());
    println!("Time to compute BW= {}ms", now.elapsed().as_millis());

    // println!("|V|{:?}; |row|{:?}; |col|{:?}; ", matrix.v.len(), matrix.row_index.len(), matrix.col_index.len());
    // sleep(Duration::new(5, 0));
}