#![allow(dead_code)]
use std::time::Instant;
// use std::time::{Duration};
// use std::thread::sleep;
use std::fs;
use std::env;
mod matrix_csr;
mod read_files;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut n:usize = 1;
    if let Some(arg1) = &args.get(1) {
        n = arg1.parse::<usize>().unwrap();
    }

    /*let files = vec![
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
    ];*/

    // let files = fs::read_dir("./instances/tests").unwrap();
    let files = fs::read_dir("./instances/IPO").unwrap();
    println!("instancia, n, bw_0, bw_1, tempo(ms), Algo");
    for file in files {
        experimentation(file.unwrap().path().into_os_string().into_string().unwrap().as_str(), &n);
    }
}

fn experimentation(file: &str, n: &usize) {
    let now = Instant::now();
    let matrix_original = matrix_csr::mm_file_to_csr(file);
    let mut matrix = matrix_original.clone();

    // print!("{}", file);
    // matrix_original.print();

    let bw_0 = matrix.bandwidth();
    matrix.cmr(matrix.col_index[0]);
    matrix.bandwidth();
    let total_time = now.elapsed().as_millis();
    let file = &file[16..]; // Formating instance name
    let file = &file[..file.len()-4];
    println!("{}, {}, {}, {}, {}, CMr ({})", file, matrix.m, bw_0, matrix.bw, total_time, matrix.col_index[0]);
    // ----------------------

    for _ in 0..*n {
        let now = Instant::now();
        let mut matrix = matrix_original.clone();
        matrix.ils();
        // matrix.bandwidth();
        let total_time = now.elapsed().as_millis();
        println!("{}, {}, {}, {}, {}, ILS", file, matrix.m, bw_0, matrix.bw, total_time);
        // print!("{}", file);
        // matrix.print();
    }
}