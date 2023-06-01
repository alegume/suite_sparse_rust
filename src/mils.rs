use rand::Rng;

// use std::collections::HashMap;
// use std::collections::VecDeque;

use crate::matrix_csr::Matrix;

impl Matrix {
    // Main code for MILS
    pub fn mils(&mut self, n: &usize) {
        let mut nivel: usize = 1;
        let mut iter: usize = 0;
        // let mut bw = self.bandwidth();
        self.local_search();
        while (iter < *n && (self.bw > self.min_bw)) {
            iter += 1;
            // let r1 = self.perturbacao(self.labels);
        }
    }

    // Proceeds with local search and change labels if a better labeling if found
    fn local_search(&mut self) {
        let criticos = self.criticals_neighbours();
        // for v in criticos {
        //     for u in vizinhos_criticos(&v) {}
        // }
    }

    fn vizinhos_criticos(&self, v: &usize) {
        // TODO: Ordenar em ordem crescente  do valor |mid(v) − f (u)|
        // let vizinhos_criticos: Vec<usize> = Vec::with_capacity(self.degree(v));

        let vizinhos = self.get_columns_of_row(*v);
        // if (abs(mid(v) - f(u)) < abs(mid(v) - f(v)) {

        // }
    }

    // println!("\tcriticals = {:?}", self.criticals);
    // let mut rng = rand::thread_rng();
    // let v: usize = rng.gen_range(1..self.m);
}
