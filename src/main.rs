#![allow(dead_code, unused)]
use std::time::Instant;
// use std::time::{Duration};
// use std::thread::sleep;
use std::fs;
use std::env;
use std::process::abort;
mod matrix_csr;
mod read_files;
mod cmr;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut n: usize = 1;
    let mut dir: String = String::from("input/tests/");

    if let Some(arg1) = &args.get(1) {
        n = arg1.parse::<usize>().unwrap();
    }
    if let Some(arg2) = &args.get(2) {
        dir = format!("input/{}/", arg2);
    }

    println!("instancia, n, bw_0, bw_1, max_degree, tempo(ms), Algo");

    let files = fs::read_dir(dir.as_str()).unwrap();
    for file in files {
        experimentation(file.unwrap().path().into_os_string().into_string().unwrap().as_str(), &n);
    }
}

fn experimentation(file: &str, n: &usize) {
    let matrix_original = matrix_csr::mm_file_to_csr(file);
    let mut matrix = matrix_original.clone();

    print!("\n\n{}", file);
    // println!("{:?}", matrix);
    let now = Instant::now();
    let bw_0 = matrix.bandwidth();
    let order = matrix.cmr(matrix.col_index[0]);
    // matrix.cmr(matrix.col_index[0]);
    matrix.bandwidth();
    let total_time = now.elapsed().as_millis();
    
    let file = &file[10..]; // Formating instance name
    let file = &file[..file.len()-4];
    println!("{}, n:{}, b0:{}, bf:{}, md:{}, t:{}, CMr ({})", file, matrix.m, bw_0, matrix.bw, matrix.max_degree, total_time, matrix.col_index[0]);

    
    // ----------------------
    matrix_original.print();
    println!("{:?}", matrix_original);
    // print!("o={:?}", order);
    // matrix.print();
    // // println!("{:?}\n", matrix);
    
    // matrix.reorder(&vec![2,3,1,0]);
    // matrix.print();
    // println!("{:?}\n", matrix);
    // abort();
    // ----------------------


    for _ in 0..*n {
        let now = Instant::now();
        let mut matrix = matrix_original.clone();
        // matrix.print();

        matrix.vertices_swap_bw_update(&1, &2);

        // matrix.ils();
        matrix.bandwidth();
        // let total_time = now.elapsed().as_millis();
        // println!("{}, n:{}, b0:{}, bf:{}, md:{}, t:{}, ils ({})", file, matrix.m, bw_0, matrix.bw, matrix.max_degree, total_time, matrix.col_index[0]);
 
        // matrix.print();
        print!("{:?}", matrix);
    }
}