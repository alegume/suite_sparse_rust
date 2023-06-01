use rand::Rng;

// use std::collections::HashMap;
// use std::collections::VecDeque;

use crate::matrix_csr::Matrix;

impl Matrix {
    // Main code for MILS
    pub fn mils(&mut self) {

        let mut rng = rand::thread_rng();
        let v: usize = rng.gen_range(1..self.m);
        
        // println!("\tcriticals = {:?}", self.criticals);
        for _ in 0..1 {
            let mut new_rows = self.cmr(self.col_index[0]);
            // self.print();
            // self.local_search(&mut new_rows);
        }
    }

}