#![allow(dead_code, unused)]
use std::env;
use std::fs;
use std::time::Instant;
mod cmr;
mod matrix_csr;
mod mils;
mod read_files;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut n: usize = 1;
    let mut k: usize = 3;
    let mut dir: String = String::from("input/tests/");

    if let Some(arg) = &args.get(1) {
        dir = format!("input/{}/", arg);
    }
    if let Some(arg) = &args.get(2) {
        n = arg.parse::<usize>().unwrap();
    }
    if let Some(arg) = &args.get(3) {
        k = arg.parse::<usize>().unwrap();
    }

    println!("instancia, n, bw_0, CMr, t(ms), MILS, t(ms)");

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
            &k,
        );
    }
}

fn experimentation(file: &str, n: &usize, k: &usize) {
    let mut matrix_original = matrix_csr::mm_file_to_csr(file, false);
    // !!! only for pattern matrix - drop v vector
    matrix_original.v = Vec::new();
    let file = &file[10..]; // Formating instance name
    let file = &file[..file.len() - 4];

    // CMr
    let mut matrix = matrix_original.clone();
    let now = Instant::now();
    let bw_0 = matrix.bandwidth();
    let p = matrix.pseudo_george_liu(0);
    matrix.cmr_labels(p);
    matrix.bandwidth();
    let total_time_cmr = now.elapsed().as_millis();

    // MILs
    let now = Instant::now();
    matrix_original.mils(n, k);
    let total_time_mils = now.elapsed().as_millis();

    // Output
    println!(
        "{}, {}, {}, {}, {}, {}, {}",
        file,
        matrix_original.m,
        bw_0,
        matrix.bw,
        total_time_cmr,
        matrix_original.bw,
        total_time_mils
    );

    // matrix.print();
    // print!("{:?}", matrix);
}
