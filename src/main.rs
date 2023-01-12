#[allow(dead_code)]
use std::time::{Instant};
// use std::time::{Duration};
// use std::thread::sleep;
mod matrix;


fn main() {
    let now = Instant::now();
    // let file = "pwtk.mtx";
    // let file = "lp_nug05.mtx";
    let file = "test1.mtx";
    // let file = "apache2.mtx";
    // let file = "lns__131.mtx";
    let matrix = matrix::read_matrix_market(file);
    println!("matrix:{:?}", matrix);
    println!("|i|: {:?}; |j|:{:?}", matrix.i_size, matrix.j_size);
    println!("BW: {}", matrix.bandwidth());
    println!("time = {}ms", now.elapsed().as_millis());
    // sleep(Duration::new(5, 0));
}