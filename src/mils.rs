use rand::seq::SliceRandom;
use rand::Rng;
use std::collections::{HashMap, HashSet};

// use std::collections::HashMap;
// use std::collections::VecDeque;

use crate::matrix_csr::Matrix;

impl Matrix {
    // Main code for MILS
    pub fn mils(&mut self, n: &usize) {
        let mut nivel: usize = 13;
        let mut iter: usize = 0;
        let mut bw_0 = self.bandwidth();
        let mut h: HashMap<usize, HashSet<usize>> = HashMap::new();
        self.local_search();
        while (iter < *n && (bw_0 > self.min_bw)) {
            iter += 1;
            // self.perturbation(nivel, &mut h);
            self.local_search();
            if (self.bw < bw_0) {
                bw_0 = self.bw;
                iter = 0;
                nivel = 1;
            } else {
                nivel += 1;
            }
        }
    }

    fn perturbation(&mut self, nivel: usize, h: &mut HashMap<usize, HashSet<usize>>) {
        let mut iter = 1;
        let criticos = self.criticals();
        // dbg!(&criticos);
        let options: HashSet<usize> = HashSet::from_iter(self.labels.iter().cloned());
        while (iter <= nivel) {
            let v = criticos.choose(&mut rand::thread_rng()).unwrap();
            // dbg!(v);
            let mut u: &usize = v;
            if let Some(n_v) = h.get_mut(v) {
                // Vec of vertices not in history of v
                let n_v_copy = n_v.clone();
                let diff: Vec<&usize> = options.symmetric_difference(&n_v_copy).collect();
                let u = *diff.choose(&mut rand::thread_rng()).unwrap_or(&v);
                n_v.insert(*u);
                // print!("v= {};", v);
                // print!("u={:?}; ", *u);
                // print!("o={:?}; ", &options);
                // println!("diff={:?}", diff);
                // dbg!(&h);
            } else {
                let u = self.labels.choose(&mut rand::thread_rng()).unwrap();
                h.insert(*v, HashSet::from([*u]));
                // print!("\t primeiro");
                // dbg!(&h);
            }
            self.labels.swap(*v, *u);
            iter += 1;
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
                if self.bandwidth() <= bw_0 {
                    bw_0 = self.bw;
                    // println!("MELHOROU! {}", bw_0);
                    // println!("M{:?}", &self.labels);
                } else {
                    // Return to previous situation
                    self.labels.swap(v, u);
                    self.bw = bw_0;
                    // println!("V{:?}", &self.labels);
                }
            }
        }
    }

    fn neighbour_of_criticals(&self, v: &usize) -> Vec<usize> {
        // TODO: Ordenar em ordem crescente  do valor |mid(v) − f (u)|
        // TODO: IMPLEMENTAR REGRAS
        let neighbour_of_criticals: Vec<usize> = Vec::with_capacity(self.degree(*v));
        dbg!(v);
        let neighbour = self.get_columns_of_row(*v).to_owned();
        // if (abs(mid(v) - f(u)) < abs(mid(v) - f(v)) {
        // }
        neighbour
    }

    // println!("\tcriticals = {:?}", self.criticals);
    // let mut rng = rand::thread_rng();
    // let v: usize = rng.gen_range(1..self.m);
}
