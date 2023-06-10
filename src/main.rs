#![allow(dead_code, unused)]
use std::time::Instant;
// use std::time::{Duration};
// use std::thread::sleep;
use std::env;
use std::fs;
use std::process::abort;
mod cmr;
mod matrix_csr;
mod mils;
mod read_files;

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
        experimentation(
            file.unwrap()
                .path()
                .into_os_string()
                .into_string()
                .unwrap()
                .as_str(),
            &n,
        );
    }
}

fn experimentation(file: &str, n: &usize) {
    let mut matrix_original = matrix_csr::mm_file_to_csr(file, false);
    // !!! only for pattern matrix - drop v vector
    matrix_original.v = Vec::new();
    let mut matrix = matrix_original.clone();

    // println!("\n\n{}", file);
    // println!("{:?}", matrix);
    // matrix.print();

    let now = Instant::now();
    let bw_0 = matrix.bandwidth();
    matrix.labels = matrix.cmr_reorder(0);
    matrix.bandwidth();
    let total_time = now.elapsed().as_millis();

    let file = &file[10..]; // Formating instance name
    let file = &file[..file.len() - 4];
    println!(
        "{}, n:{}, b0:{}, bf:{}, t:{}, CMr",
        file, matrix.m, bw_0, matrix.bw, total_time
    );

    // matrix_original.print();
    let p = matrix_original.pseudo_george_liu(0);
    // dbg!(p);
    // matrix_original.labels = matrix_original.cmr_reorder(p);
    matrix_original.cmr_labels(p);
    matrix_original.bandwidth();
    // abort();

    // MILs
    /*
    let now = Instant::now();
    // matrix.print();
    // matrix_original.labels = order;
    let bw_0 = matrix_original.bandwidth();
    matrix_original.old_labels = matrix_original.labels.clone();
    matrix_original.mils(n);
    let total_time = now.elapsed().as_millis();
     */
    println!(
        "{}, n:{}, b0:{}, bf:{}, t:{}, CMr-L,n={}",
        file, matrix_original.m, bw_0, matrix_original.bw, total_time, n
    );
    // matrix.print();
    // print!("{:?}", matrix);
}
