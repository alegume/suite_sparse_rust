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
        let criticos = self.criticals();
        let mut bw_0 = self.bandwidth();
        for v in criticos {
            for u in self.neighbour_of_criticals(&v) {
                // println!("O{:?}", &self.labels);
                self.labels.swap(v, u);
                if self.bandwidth() < bw_0 {
                    bw_0 = self.bw;
                    println!("MELHOROU! {}", bw_0);
                    // println!("M{:?}", &self.labels);
                } else {
                    self.labels.swap(v, u);
                    // println!("V{:?}", &self.labels);
                }
            }
        }
    }

    fn neighbour_of_criticals(&self, v: &usize) -> Vec<usize> {
        // TODO: Ordenar em ordem crescente  do valor |mid(v) − f (u)|
        // TODO: IMPLEMENTAR REGRAS
        let neighbour_of_criticals: Vec<usize> = Vec::with_capacity(self.degree(*v));
        let neighbour = self.get_columns_of_row(*v).to_owned();
        // if (abs(mid(v) - f(u)) < abs(mid(v) - f(v)) {
        // }
        neighbour
    }

    // println!("\tcriticals = {:?}", self.criticals);
    // let mut rng = rand::thread_rng();
    // let v: usize = rng.gen_range(1..self.m);
}
