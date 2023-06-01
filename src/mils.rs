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
        while (iter < *n && (self.bw > self.max_degree / 2)) {
            iter += 1;
            // let r1 = self.perturbacao(self.labels);
        }
    }

    // Proceeds with local search and change labels if a better labeling if found
    fn local_search(&self) {
        // let criticos = self.criticals_neighbours();
    }

    // println!("\tcriticals = {:?}", self.criticals);
    // let mut rng = rand::thread_rng();
    // let v: usize = rng.gen_range(1..self.m);
}
